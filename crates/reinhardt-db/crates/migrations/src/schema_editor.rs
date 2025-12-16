//! Schema Editor for migration execution
//!
//! Provides atomic transaction support for DDL operations,
//! similar to Django's schema_editor.
//!
//! # Overview
//!
//! The `SchemaEditor` wraps database connections and optionally manages
//! transactions for atomic migration execution. It follows Django's pattern
//! where migrations can be wrapped in transactions for databases that support
//! transactional DDL.
//!
//! # Database Support
//!
//! | Database | Transactional DDL | Notes |
//! |----------|-------------------|-------|
//! | PostgreSQL | Yes | DDL can be rolled back |
//! | SQLite | Yes | DDL can be rolled back |
//! | MySQL | No | DDL causes implicit commit |
//!
//! # Example
//!
//! ```ignore
//! use reinhardt_migrations::schema_editor::SchemaEditor;
//! use reinhardt_backends::{DatabaseConnection, DatabaseType};
//!
//! let connection = DatabaseConnection::connect_postgres("postgres://...").await?;
//! let mut editor = SchemaEditor::new(connection.clone(), true, DatabaseType::Postgres).await?;
//!
//! editor.execute("CREATE TABLE users (id SERIAL PRIMARY KEY)").await?;
//! editor.execute("ALTER TABLE users ADD COLUMN name TEXT").await?;
//!
//! // Commit all changes atomically
//! editor.finish().await?;
//! ```

use crate::Result;
use reinhardt_backends::{
	connection::DatabaseConnection,
	types::{DatabaseType, TransactionExecutor},
};

/// Schema editor for executing DDL statements with optional transaction support
///
/// This struct wraps a database connection and optionally manages a transaction
/// for atomic migration execution. It follows Django's schema_editor pattern.
///
/// When `atomic` is `true` and the database supports transactional DDL,
/// all DDL operations are wrapped in a transaction that can be committed
/// or rolled back as a unit.
pub struct SchemaEditor {
	/// Database connection
	connection: DatabaseConnection,
	/// Transaction executor (if using atomic mode)
	executor: Option<Box<dyn TransactionExecutor>>,
	/// Whether this editor is using atomic transactions
	atomic: bool,
	/// Database type
	db_type: DatabaseType,
	/// Deferred SQL statements to execute at finish
	deferred_sql: Vec<String>,
}

impl SchemaEditor {
	/// Create a new schema editor
	///
	/// If `atomic` is true and the database supports transactional DDL,
	/// a transaction will be started automatically.
	///
	/// # Arguments
	///
	/// * `connection` - Database connection to use
	/// * `atomic` - Whether to wrap operations in a transaction
	/// * `db_type` - Database type for dialect-specific handling
	///
	/// # Returns
	///
	/// A new SchemaEditor instance
	///
	/// # Notes
	///
	/// If `atomic` is `true` but the database doesn't support transactional DDL
	/// (e.g., MySQL), a warning is logged and operations proceed without
	/// transaction wrapping.
	pub async fn new(
		connection: DatabaseConnection,
		atomic: bool,
		db_type: DatabaseType,
	) -> Result<Self> {
		let effective_atomic = atomic && db_type.supports_transactional_ddl();

		let executor = if effective_atomic {
			Some(connection.begin().await?)
		} else {
			if atomic && !db_type.supports_transactional_ddl() {
				tracing::warn!(
					"atomic=true requested but {:?} doesn't support transactional DDL. \
					 Proceeding without transaction wrapper.",
					db_type
				);
			}
			None
		};

		Ok(Self {
			connection,
			executor,
			atomic: effective_atomic,
			db_type,
			deferred_sql: Vec::new(),
		})
	}

	/// Execute a DDL statement
	///
	/// If in atomic mode, executes within the transaction.
	/// Otherwise, executes directly on the connection.
	///
	/// # Arguments
	///
	/// * `sql` - SQL statement to execute
	pub async fn execute(&mut self, sql: &str) -> Result<()> {
		if let Some(ref mut tx) = self.executor {
			tx.execute(sql, vec![]).await?;
		} else {
			self.connection.execute(sql, vec![]).await?;
		}

		Ok(())
	}

	/// Defer SQL execution until finish()
	///
	/// Some operations need to be executed after all other operations
	/// in the migration (e.g., creating indexes on newly created columns).
	///
	/// # Arguments
	///
	/// * `sql` - SQL statement to defer
	pub fn defer(&mut self, sql: String) {
		self.deferred_sql.push(sql);
	}

	/// Finish the schema editing session
	///
	/// Executes any deferred SQL and commits the transaction if atomic.
	///
	/// # Returns
	///
	/// Ok(()) on success
	pub async fn finish(mut self) -> Result<()> {
		// Execute deferred SQL
		for sql in self.deferred_sql.drain(..) {
			if let Some(ref mut tx) = self.executor {
				tx.execute(&sql, vec![]).await?;
			} else {
				self.connection.execute(&sql, vec![]).await?;
			}
		}

		// Commit if in transaction
		if let Some(tx) = self.executor.take() {
			tx.commit().await?;
		}

		Ok(())
	}

	/// Rollback any changes (only effective for transactional DDL databases)
	///
	/// For databases that don't support transactional DDL (MySQL),
	/// this is a no-op as DDL statements have already been implicitly committed.
	pub async fn rollback(mut self) -> Result<()> {
		if let Some(tx) = self.executor.take() {
			tx.rollback().await?;
		}
		Ok(())
	}

	/// Check if this editor is using atomic transactions
	pub fn is_atomic(&self) -> bool {
		self.atomic
	}

	/// Get the database type
	pub fn database_type(&self) -> DatabaseType {
		self.db_type
	}

	/// Get a reference to the underlying connection
	///
	/// This can be used for operations that need direct connection access
	/// outside of the transaction (e.g., checking table existence).
	pub fn connection(&self) -> &DatabaseConnection {
		&self.connection
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_database_type_transactional_ddl() {
		assert!(DatabaseType::Postgres.supports_transactional_ddl());
		assert!(DatabaseType::Sqlite.supports_transactional_ddl());
		assert!(!DatabaseType::Mysql.supports_transactional_ddl());
	}
}
