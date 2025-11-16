//! MySQL-specific Two-Phase Commit implementation
//!
//! This module implements the `TwoPhaseParticipant` trait for MySQL using
//! the XA transaction protocol (XA START, XA END, XA PREPARE, XA COMMIT, XA ROLLBACK).

use sqlx::pool::PoolConnection;
use sqlx::{MySql, MySqlPool, Row};
use std::sync::Arc;

use crate::error::{DatabaseError, Result};

/// XA transaction state machine
///
/// Represents the various states an XA transaction can be in during its lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XaState {
	/// Initial state, no transaction started
	Idle,
	/// Transaction started with XA START
	Started,
	/// Transaction ended with XA END
	Ended,
	/// Transaction prepared with XA PREPARE
	Prepared,
}

/// MySQL XA transaction session
///
/// Owns the MySQL connection and tracks the XA transaction state.
/// This ensures all XA operations occur on the same connection as required by MySQL.
pub struct XaSession {
	/// The dedicated MySQL connection for this XA transaction
	pub connection: PoolConnection<MySql>,
	/// The XA transaction identifier
	pub xid: String,
	/// Current state of the XA transaction
	pub state: XaState,
}

/// MySQL Two-Phase Commit participant using XA transactions
///
/// Manages two-phase commit transactions using MySQL's XA transaction protocol.
/// XA transactions in MySQL follow the X/Open XA standard for distributed
/// transaction processing.
///
/// # XA Transaction States
///
/// - ACTIVE: After XA START
/// - IDLE: After XA END
/// - PREPARED: After XA PREPARE
/// - COMMITTED: After XA COMMIT
/// - ROLLBACK: After XA ROLLBACK
///
/// # Examples
///
/// ```no_run
/// use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
/// use sqlx::MySqlPool;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
/// let participant = MySqlTwoPhaseParticipant::new(pool);
///
/// // Start an XA transaction and get a session
/// let mut session = participant.begin("txn_001").await?;
///
/// // ... perform operations ...
///
/// // End the XA transaction
/// participant.end(&mut session).await?;
///
/// // Prepare the transaction
/// participant.prepare(&mut session).await?;
///
/// // Commit the prepared transaction
/// participant.commit(session).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct MySqlTwoPhaseParticipant {
	pool: Arc<MySqlPool>,
	// Internal session storage for ORM layer compatibility
	// XID -> Session mapping for managing active transactions
	sessions: Arc<std::sync::Mutex<std::collections::HashMap<String, XaSession>>>,
}

impl MySqlTwoPhaseParticipant {
	/// Create a new MySQL two-phase commit participant
	///
	/// # Examples
	///
	/// ```no_run
	/// use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
	/// use sqlx::MySqlPool;
	///
	/// # async fn example() -> Result<(), sqlx::Error> {
	/// let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
	/// let participant = MySqlTwoPhaseParticipant::new(pool);
	/// # Ok(())
	/// # }
	/// ```
	pub fn new(pool: MySqlPool) -> Self {
		Self {
			pool: Arc::new(pool),
			sessions: Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
		}
	}

	/// Create from an Arc<MySqlPool>
	pub fn from_pool_arc(pool: Arc<MySqlPool>) -> Self {
		Self {
			pool,
			sessions: Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
		}
	}

	/// Get a reference to the underlying MySqlPool
	///
	/// This method is useful for tests and advanced use cases where direct
	/// access to the pool is required.
	pub fn pool(&self) -> &MySqlPool {
		self.pool.as_ref()
	}

	/// Start an XA transaction
	///
	/// This executes `XA START 'xid'` in MySQL and returns an XaSession that owns
	/// the connection. All subsequent XA operations must use this session.
	///
	/// # Examples
	///
	/// ```no_run
	/// # use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
	/// # use sqlx::MySqlPool;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// # let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
	/// # let participant = MySqlTwoPhaseParticipant::new(pool);
	/// let mut session = participant.begin("txn_001").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn begin(&self, xid: String) -> Result<XaSession> {
		// XAトランザクション用の新しいコネクションを取得
		let mut connection = self.pool.acquire().await.map_err(DatabaseError::from)?;

		let sql = format!("XA START '{}'", Self::escape_xid(&xid));
		// MySQL XA commands are not supported in prepared statement protocol
		sqlx::raw_sql(&sql)
			.execute(&mut *connection)
			.await
			.map_err(DatabaseError::from)?;

		Ok(XaSession {
			connection,
			xid,
			state: XaState::Started,
		})
	}

	/// End an XA transaction
	///
	/// This executes `XA END 'xid'` in MySQL. Must be called before XA PREPARE.
	/// Transitions the session state from Started to Ended.
	///
	/// # Examples
	///
	/// ```no_run
	/// # use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
	/// # use sqlx::MySqlPool;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// # let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
	/// # let participant = MySqlTwoPhaseParticipant::new(pool);
	/// let mut session = participant.begin("txn_001").await?;
	/// // ... perform operations ...
	/// participant.end(&mut session).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn end(&self, session: &mut XaSession) -> Result<()> {
		let sql = format!("XA END '{}'", Self::escape_xid(&session.xid));
		sqlx::raw_sql(&sql)
			.execute(&mut *session.connection)
			.await
			.map_err(DatabaseError::from)?;

		session.state = XaState::Ended;
		Ok(())
	}

	/// Prepare an XA transaction for two-phase commit
	///
	/// This executes `XA PREPARE 'xid'` in MySQL.
	/// Transitions the session state from Ended to Prepared.
	///
	/// # Errors
	///
	/// Returns an error if:
	/// - The XA transaction is not in IDLE state (must call `end()` first)
	/// - The transaction ID does not exist
	///
	/// # Examples
	///
	/// ```no_run
	/// # use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
	/// # use sqlx::MySqlPool;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// # let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
	/// # let participant = MySqlTwoPhaseParticipant::new(pool);
	/// let mut session = participant.begin("txn_001").await?;
	/// // ... perform operations ...
	/// participant.end(&mut session).await?;
	/// participant.prepare(&mut session).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn prepare(&self, session: &mut XaSession) -> Result<()> {
		let sql = format!("XA PREPARE '{}'", Self::escape_xid(&session.xid));
		sqlx::raw_sql(&sql)
			.execute(&mut *session.connection)
			.await
			.map_err(DatabaseError::from)?;

		session.state = XaState::Prepared;
		Ok(())
	}

	/// Commit a prepared XA transaction
	///
	/// This executes `XA COMMIT 'xid'` in MySQL. Consumes the session.
	///
	/// # Errors
	///
	/// Returns an error if the prepared transaction does not exist.
	///
	/// # Examples
	///
	/// ```no_run
	/// # use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
	/// # use sqlx::MySqlPool;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// # let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
	/// # let participant = MySqlTwoPhaseParticipant::new(pool);
	/// let mut session = participant.begin("txn_001").await?;
	/// participant.end(&mut session).await?;
	/// participant.prepare(&mut session).await?;
	/// participant.commit(session).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn commit(&self, mut session: XaSession) -> Result<()> {
		let sql = format!("XA COMMIT '{}'", Self::escape_xid(&session.xid));
		sqlx::raw_sql(&sql)
			.execute(&mut *session.connection)
			.await
			.map_err(DatabaseError::from)?;

		// Session is consumed and connection is dropped
		Ok(())
	}

	/// Commit a prepared XA transaction by XID (for recovery scenarios)
	///
	/// This executes `XA COMMIT 'xid'` in MySQL using a new connection.
	/// Use this for recovery scenarios where you don't have the original session.
	///
	/// # Examples
	///
	/// ```no_run
	/// # use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
	/// # use sqlx::MySqlPool;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// # let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
	/// # let participant = MySqlTwoPhaseParticipant::new(pool);
	/// // Recovery scenario: commit by XID directly
	/// participant.commit_by_xid("txn_001").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn commit_by_xid(&self, xid: &str) -> Result<()> {
		let mut conn = self.pool.acquire().await.map_err(DatabaseError::from)?;
		let sql = format!("XA COMMIT '{}'", Self::escape_xid(xid));
		sqlx::raw_sql(&sql)
			.execute(&mut *conn)
			.await
			.map_err(DatabaseError::from)?;
		Ok(())
	}

	/// Commit an XA transaction with one-phase optimization
	///
	/// This executes `XA COMMIT 'xid' ONE PHASE` in MySQL. This is an optimization
	/// for single-phase commit when the transaction is in IDLE state (after XA END).
	/// Consumes the session.
	///
	/// # Examples
	///
	/// ```no_run
	/// # use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
	/// # use sqlx::MySqlPool;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// # let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
	/// # let participant = MySqlTwoPhaseParticipant::new(pool);
	/// let mut session = participant.begin("txn_001").await?;
	/// // ... perform operations ...
	/// participant.end(&mut session).await?;
	/// participant.commit_one_phase(session).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn commit_one_phase(&self, mut session: XaSession) -> Result<()> {
		let sql = format!("XA COMMIT '{}' ONE PHASE", Self::escape_xid(&session.xid));
		sqlx::raw_sql(&sql)
			.execute(&mut *session.connection)
			.await
			.map_err(DatabaseError::from)?;

		// Session is consumed and connection is dropped
		Ok(())
	}

	/// Rollback a prepared XA transaction
	///
	/// This executes `XA ROLLBACK 'xid'` in MySQL. Consumes the session.
	///
	/// # Examples
	///
	/// ```no_run
	/// # use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
	/// # use sqlx::MySqlPool;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// # let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
	/// # let participant = MySqlTwoPhaseParticipant::new(pool);
	/// let mut session = participant.begin("txn_001").await?;
	/// participant.end(&mut session).await?;
	/// participant.prepare(&mut session).await?;
	/// participant.rollback(session).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn rollback(&self, mut session: XaSession) -> Result<()> {
		let sql = format!("XA ROLLBACK '{}'", Self::escape_xid(&session.xid));
		sqlx::raw_sql(&sql)
			.execute(&mut *session.connection)
			.await
			.map_err(DatabaseError::from)?;

		// Session is consumed and connection is dropped
		Ok(())
	}

	/// Rollback a prepared XA transaction by XID (for recovery scenarios)
	///
	/// This executes `XA ROLLBACK 'xid'` in MySQL using a new connection.
	/// Use this for recovery scenarios where you don't have the original session.
	///
	/// # Examples
	///
	/// ```no_run
	/// # use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
	/// # use sqlx::MySqlPool;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// # let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
	/// # let participant = MySqlTwoPhaseParticipant::new(pool);
	/// // Recovery scenario: rollback by XID directly
	/// participant.rollback_by_xid("txn_001").await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn rollback_by_xid(&self, xid: &str) -> Result<()> {
		let mut conn = self.pool.acquire().await.map_err(DatabaseError::from)?;
		let sql = format!("XA ROLLBACK '{}'", Self::escape_xid(xid));
		sqlx::raw_sql(&sql)
			.execute(&mut *conn)
			.await
			.map_err(DatabaseError::from)?;
		Ok(())
	}

	/// Query all prepared XA transactions using XA RECOVER
	///
	/// Returns a list of prepared transaction IDs. This is useful for recovery
	/// scenarios where you need to find orphaned prepared transactions.
	///
	/// # Examples
	///
	/// ```no_run
	/// # use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
	/// # use sqlx::MySqlPool;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// # let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
	/// # let participant = MySqlTwoPhaseParticipant::new(pool);
	/// let prepared_txns = participant.list_prepared_transactions().await?;
	/// for txn_info in prepared_txns {
	///     println!("Prepared XA transaction: {:?}", txn_info);
	/// }
	/// # Ok(())
	/// # }
	/// ```
	pub async fn list_prepared_transactions(&self) -> Result<Vec<XaTransactionInfo>> {
		// MySQL XA RECOVER is not supported in prepared statement protocol
		let rows = sqlx::raw_sql("XA RECOVER")
			.fetch_all(self.pool.as_ref())
			.await
			.map_err(DatabaseError::from)?;

		let mut transactions = Vec::new();
		for row in rows {
			// XA RECOVER returns: formatID, gtrid_length, bqual_length, data
			let format_id: i32 = row.try_get("formatID").map_err(DatabaseError::from)?;
			let gtrid_length: i32 = row.try_get("gtrid_length").map_err(DatabaseError::from)?;
			let bqual_length: i32 = row.try_get("bqual_length").map_err(DatabaseError::from)?;
			let data: Vec<u8> = row.try_get("data").map_err(DatabaseError::from)?;

			transactions.push(XaTransactionInfo {
				format_id,
				gtrid_length,
				bqual_length,
				data: data.clone(),
				xid: String::from_utf8_lossy(&data).to_string(),
			});
		}

		Ok(transactions)
	}

	/// Find a specific prepared XA transaction
	///
	/// # Examples
	///
	/// ```no_run
	/// # use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
	/// # use sqlx::MySqlPool;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// # let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
	/// # let participant = MySqlTwoPhaseParticipant::new(pool);
	/// if let Some(info) = participant.find_prepared_transaction("txn_001").await? {
	///     println!("Found prepared XA transaction: {:?}", info);
	///     // Decide whether to commit or rollback using XID-based methods
	///     participant.commit_by_xid("txn_001").await?;
	/// }
	/// # Ok(())
	/// # }
	/// ```
	pub async fn find_prepared_transaction(&self, xid: &str) -> Result<Option<XaTransactionInfo>> {
		let all_txns = self.list_prepared_transactions().await?;
		Ok(all_txns.into_iter().find(|txn| txn.xid == xid))
	}

	/// Cleanup stale prepared XA transactions
	///
	/// This method queries all prepared transactions and attempts to rollback
	/// those matching a specific pattern. Use with caution as this may affect
	/// in-progress distributed transactions.
	///
	/// # Examples
	///
	/// ```no_run
	/// # use reinhardt_db::backends::mysql::two_phase::MySqlTwoPhaseParticipant;
	/// # use sqlx::MySqlPool;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// # let pool = MySqlPool::connect("mysql://localhost/mydb").await?;
	/// # let participant = MySqlTwoPhaseParticipant::new(pool);
	/// // Cleanup all transactions starting with "stale_"
	/// let cleaned = participant.cleanup_stale_transactions("stale_").await?;
	/// println!("Cleaned up {} stale XA transactions", cleaned);
	/// # Ok(())
	/// # }
	/// ```
	pub async fn cleanup_stale_transactions(&self, prefix: &str) -> Result<usize> {
		let all_txns = self.list_prepared_transactions().await?;
		let mut cleaned = 0;

		for txn in all_txns {
			if txn.xid.starts_with(prefix) && self.rollback_by_xid(&txn.xid).await.is_ok() {
				cleaned += 1;
			}
		}

		Ok(cleaned)
	}

	/// Escape XID to prevent SQL injection
	///
	/// Note: MySQL XA transaction IDs have specific format requirements.
	/// This is a simple escaping mechanism that removes single quotes.
	fn escape_xid(xid: &str) -> String {
		xid.replace('\'', "''")
	}

	// XID-based wrapper methods for ORM layer compatibility
	// These methods manage sessions internally using the sessions HashMap

	/// Begin an XA transaction by XID (ORM layer wrapper)
	///
	/// Creates a session and stores it internally for later use.
	pub async fn begin_by_xid(&self, xid: String) -> Result<()> {
		let xid_copy = xid.clone();
		let session = self.begin(xid).await?;
		self.sessions.lock().unwrap().insert(xid_copy, session);
		Ok(())
	}

	/// End an XA transaction by XID (ORM layer wrapper)
	///
	/// Executes XA END without exposing the session to the caller.
	pub async fn end_by_xid(&self, xid: String) -> Result<()> {
		// Extract the session temporarily to avoid holding the lock across await
		let mut session = {
			let mut sessions = self.sessions.lock().unwrap();
			sessions.remove(&xid).ok_or_else(|| {
				DatabaseError::QueryError(format!("No active session for XID: {}", xid))
			})?
		};

		// Perform the end operation directly without calling self.end()
		let sql = format!("XA END '{}'", Self::escape_xid(&xid));
		sqlx::raw_sql(&sql)
			.execute(&mut *session.connection)
			.await
			.map_err(DatabaseError::from)?;

		// Update state and re-insert
		session.state = XaState::Ended;
		self.sessions.lock().unwrap().insert(xid, session);

		Ok(())
	}

	/// Prepare an XA transaction by XID (ORM layer wrapper)
	///
	/// Executes XA PREPARE without exposing the session to the caller.
	pub async fn prepare_by_xid(&self, xid: String) -> Result<()> {
		// Extract the session temporarily to avoid holding the lock across await
		let mut session = {
			let mut sessions = self.sessions.lock().unwrap();
			sessions.remove(&xid).ok_or_else(|| {
				DatabaseError::QueryError(format!("No active session for XID: {}", xid))
			})?
		};

		// Perform the prepare operation directly without calling self.prepare()
		let sql = format!("XA PREPARE '{}'", Self::escape_xid(&xid));
		sqlx::raw_sql(&sql)
			.execute(&mut *session.connection)
			.await
			.map_err(DatabaseError::from)?;

		// Update state and re-insert
		session.state = XaState::Prepared;
		self.sessions.lock().unwrap().insert(xid, session);

		Ok(())
	}

	/// Commit an XA transaction by XID (ORM layer wrapper)
	///
	/// Removes the session from internal storage, executes XA COMMIT, and consumes the session.
	pub async fn commit_managed(&self, xid: String) -> Result<()> {
		let mut session = self.sessions.lock().unwrap().remove(&xid).ok_or_else(|| {
			DatabaseError::QueryError(format!("No active session for XID: {}", xid))
		})?;

		// Execute commit directly without calling self.commit()
		let sql = format!("XA COMMIT '{}'", Self::escape_xid(&xid));
		sqlx::raw_sql(&sql)
			.execute(&mut *session.connection)
			.await
			.map_err(DatabaseError::from)?;

		// Session is consumed and connection is dropped
		Ok(())
	}

	/// Rollback an XA transaction by XID (ORM layer wrapper)
	///
	/// Removes the session from internal storage, executes XA ROLLBACK, and consumes the session.
	pub async fn rollback_managed(&self, xid: String) -> Result<()> {
		let mut session = self.sessions.lock().unwrap().remove(&xid).ok_or_else(|| {
			DatabaseError::QueryError(format!("No active session for XID: {}", xid))
		})?;

		// Execute rollback directly without calling self.rollback()
		let sql = format!("XA ROLLBACK '{}'", Self::escape_xid(&xid));
		sqlx::raw_sql(&sql)
			.execute(&mut *session.connection)
			.await
			.map_err(DatabaseError::from)?;

		// Session is consumed and connection is dropped
		Ok(())
	}
}

/// Information about an XA transaction from XA RECOVER
#[derive(Debug, Clone, PartialEq)]
pub struct XaTransactionInfo {
	/// Format identifier
	pub format_id: i32,
	/// Global transaction ID length
	pub gtrid_length: i32,
	/// Branch qualifier length
	pub bqual_length: i32,
	/// Raw transaction data
	pub data: Vec<u8>,
	/// String representation of the XID
	pub xid: String,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_xa_transaction_info_creation() {
		let info = XaTransactionInfo {
			format_id: 1,
			gtrid_length: 7,
			bqual_length: 0,
			data: b"txn_001".to_vec(),
			xid: "txn_001".to_string(),
		};

		assert_eq!(info.format_id, 1);
		assert_eq!(info.gtrid_length, 7);
		assert_eq!(info.bqual_length, 0);
		assert_eq!(info.xid, "txn_001");
	}

	#[test]
	fn test_escape_xid() {
		assert_eq!(MySqlTwoPhaseParticipant::escape_xid("simple"), "simple");
		assert_eq!(MySqlTwoPhaseParticipant::escape_xid("it's"), "it''s");
		assert_eq!(MySqlTwoPhaseParticipant::escape_xid("a'b'c"), "a''b''c");
	}

	#[tokio::test]
	async fn test_participant_clone() {
		// Test that MySqlTwoPhaseParticipant can be cloned
		let pool = Arc::new(
			MySqlPool::connect_lazy("mysql://localhost/testdb")
				.expect("Failed to create lazy pool"),
		);
		let participant1 = MySqlTwoPhaseParticipant::from_pool_arc(pool.clone());
		let participant2 = participant1.clone();

		// Both should reference the same pool
		assert!(Arc::ptr_eq(&participant1.pool, &participant2.pool));
	}
}
