//! Database-backed session storage
//!
//! This module provides session storage using a database backend (PostgreSQL, MySQL, SQLite).
//! Sessions are persisted to a database table, making them survive application restarts.
//!
//! ## Features
//!
//! - Persistent session storage
//! - Automatic session expiration cleanup
//! - Support for multiple database backends
//!
//! ## Example
//!
//! ```rust,no_run
//! use reinhardt_sessions::backends::{DatabaseSessionBackend, SessionBackend};
//! use serde_json::json;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a database session backend
//! let backend = DatabaseSessionBackend::new("postgres://localhost/sessions").await?;
//!
//! // Store user session
//! let session_data = json!({
//!     "user_id": 42,
//!     "username": "alice",
//!     "authenticated": true,
//! });
//!
//! backend.save("session_key_123", &session_data, Some(3600)).await?;
//!
//! // Retrieve session
//! let retrieved: Option<serde_json::Value> = backend.load("session_key_123").await?;
//! assert!(retrieved.is_some());
//!
//! // Clean up expired sessions
//! backend.cleanup_expired().await?;
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Any, AnyPool, Row};
use std::sync::Arc;

use super::cache::{SessionBackend, SessionError};

/// Database session model
///
/// Represents a session stored in the database with expiration information.
///
/// ## Example
///
/// ```rust
/// use reinhardt_sessions::backends::database::SessionModel;
/// use chrono::Utc;
///
/// let session = SessionModel {
///     session_key: "abc123".to_string(),
///     session_data: serde_json::json!({"user_id": 42}),
///     expire_date: Utc::now() + chrono::Duration::hours(1),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionModel {
    /// Unique session key (primary key)
    pub session_key: String,
    /// Session data stored as JSON
    pub session_data: serde_json::Value,
    /// Session expiration timestamp
    pub expire_date: DateTime<Utc>,
}

/// Database-backed session storage
///
/// Stores sessions in a database table with automatic expiration handling.
/// Supports PostgreSQL, MySQL, and SQLite through sqlx's `Any` driver.
///
/// ## Database Schema
///
/// The backend expects a table with the following structure:
///
/// ```sql
/// CREATE TABLE sessions (
///     session_key VARCHAR(255) PRIMARY KEY,
///     session_data TEXT NOT NULL,
///     expire_date TIMESTAMP NOT NULL
/// );
/// CREATE INDEX idx_sessions_expire_date ON sessions(expire_date);
/// ```
///
/// ## Example
///
/// ```rust,no_run
/// use reinhardt_sessions::backends::{DatabaseSessionBackend, SessionBackend};
/// use serde_json::json;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Initialize backend with database URL
/// let backend = DatabaseSessionBackend::new("sqlite::memory:").await?;
///
/// // Create the sessions table
/// backend.create_table().await?;
///
/// // Store session with 1 hour TTL
/// let data = json!({"cart_total": 99.99});
/// backend.save("cart_xyz", &data, Some(3600)).await?;
///
/// // Check if session exists
/// let exists = backend.exists("cart_xyz").await?;
/// assert!(exists);
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct DatabaseSessionBackend {
    pool: Arc<AnyPool>,
}

impl DatabaseSessionBackend {
    /// Create a new database session backend
    ///
    /// Initializes a connection pool to the specified database URL.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use reinhardt_sessions::backends::DatabaseSessionBackend;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// // PostgreSQL
    /// let backend = DatabaseSessionBackend::new("postgres://localhost/mydb").await?;
    ///
    /// // MySQL
    /// let backend = DatabaseSessionBackend::new("mysql://localhost/mydb").await?;
    ///
    /// // SQLite
    /// let backend = DatabaseSessionBackend::new("sqlite::memory:").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn new(database_url: &str) -> Result<Self, SessionError> {
        let pool = AnyPool::connect(database_url)
            .await
            .map_err(|e| SessionError::CacheError(format!("Database connection error: {}", e)))?;

        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    /// Create a new backend from an existing pool
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use reinhardt_sessions::backends::DatabaseSessionBackend;
    /// use sqlx::AnyPool;
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let pool = AnyPool::connect("sqlite::memory:").await?;
    /// let backend = DatabaseSessionBackend::from_pool(Arc::new(pool));
    /// # Ok(())
    /// # }
    /// ```
    pub fn from_pool(pool: Arc<AnyPool>) -> Self {
        Self { pool }
    }

    /// Create the sessions table if it doesn't exist
    ///
    /// Creates the required database table for session storage.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use reinhardt_sessions::backends::DatabaseSessionBackend;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let backend = DatabaseSessionBackend::new("sqlite::memory:").await?;
    /// backend.create_table().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_table(&self) -> Result<(), SessionError> {
        let sql = r#"
            CREATE TABLE IF NOT EXISTS sessions (
                session_key VARCHAR(255) PRIMARY KEY,
                session_data TEXT NOT NULL,
                expire_date TIMESTAMP NOT NULL
            )
        "#;

        sqlx::query(sql)
            .execute(&*self.pool)
            .await
            .map_err(|e| SessionError::CacheError(format!("Failed to create table: {}", e)))?;

        // Create index on expire_date for efficient cleanup
        let index_sql = r#"
            CREATE INDEX IF NOT EXISTS idx_sessions_expire_date
            ON sessions(expire_date)
        "#;

        sqlx::query(index_sql)
            .execute(&*self.pool)
            .await
            .map_err(|e| SessionError::CacheError(format!("Failed to create index: {}", e)))?;

        Ok(())
    }

    /// Clean up expired sessions
    ///
    /// Deletes all sessions that have passed their expiration time.
    /// This should be called periodically to prevent database bloat.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use reinhardt_sessions::backends::DatabaseSessionBackend;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let backend = DatabaseSessionBackend::new("sqlite::memory:").await?;
    /// backend.create_table().await?;
    ///
    /// // Clean up expired sessions
    /// let deleted_count = backend.cleanup_expired().await?;
    /// println!("Deleted {} expired sessions", deleted_count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cleanup_expired(&self) -> Result<u64, SessionError> {
        let sql = "DELETE FROM sessions WHERE expire_date < ?";
        let now = Utc::now();

        let result = sqlx::query(sql)
            .bind(now)
            .execute(&*self.pool)
            .await
            .map_err(|e| SessionError::CacheError(format!("Failed to cleanup sessions: {}", e)))?;

        Ok(result.rows_affected())
    }
}

#[async_trait]
impl SessionBackend for DatabaseSessionBackend {
    async fn load<T>(&self, session_key: &str) -> Result<Option<T>, SessionError>
    where
        T: for<'de> Deserialize<'de> + Send,
    {
        let sql = "SELECT session_data, expire_date FROM sessions WHERE session_key = ?";

        let row = sqlx::query(sql)
            .bind(session_key)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| SessionError::CacheError(format!("Failed to load session: {}", e)))?;

        match row {
            Some(row) => {
                // Check if session has expired
                let expire_date: DateTime<Utc> = row
                    .try_get("expire_date")
                    .map_err(|e| SessionError::CacheError(format!("Invalid expire_date: {}", e)))?;

                if expire_date < Utc::now() {
                    // Session expired, delete it
                    let _ = self.delete(session_key).await;
                    return Ok(None);
                }

                let session_data: String = row.try_get("session_data").map_err(|e| {
                    SessionError::CacheError(format!("Invalid session_data: {}", e))
                })?;

                let data: T = serde_json::from_str(&session_data).map_err(|e| {
                    SessionError::SerializationError(format!("Deserialization error: {}", e))
                })?;

                Ok(Some(data))
            }
            None => Ok(None),
        }
    }

    async fn save<T>(
        &self,
        session_key: &str,
        data: &T,
        ttl: Option<u64>,
    ) -> Result<(), SessionError>
    where
        T: Serialize + Send + Sync,
    {
        let session_data = serde_json::to_string(data)
            .map_err(|e| SessionError::SerializationError(format!("Serialization error: {}", e)))?;

        let expire_date = match ttl {
            Some(seconds) => Utc::now() + Duration::seconds(seconds as i64),
            None => Utc::now() + Duration::days(14), // Default 14 days
        };

        // Use REPLACE for SQLite compatibility or INSERT ... ON CONFLICT UPDATE for others
        // For simplicity, we'll delete then insert
        let _ = self.delete(session_key).await;

        let sql = "INSERT INTO sessions (session_key, session_data, expire_date) VALUES (?, ?, ?)";

        sqlx::query(sql)
            .bind(session_key)
            .bind(session_data)
            .bind(expire_date)
            .execute(&*self.pool)
            .await
            .map_err(|e| SessionError::CacheError(format!("Failed to save session: {}", e)))?;

        Ok(())
    }

    async fn delete(&self, session_key: &str) -> Result<(), SessionError> {
        let sql = "DELETE FROM sessions WHERE session_key = ?";

        sqlx::query(sql)
            .bind(session_key)
            .execute(&*self.pool)
            .await
            .map_err(|e| SessionError::CacheError(format!("Failed to delete session: {}", e)))?;

        Ok(())
    }

    async fn exists(&self, session_key: &str) -> Result<bool, SessionError> {
        let sql = "SELECT 1 FROM sessions WHERE session_key = ? AND expire_date > ?";
        let now = Utc::now();

        let row = sqlx::query(sql)
            .bind(session_key)
            .bind(now)
            .fetch_optional(&*self.pool)
            .await
            .map_err(|e| {
                SessionError::CacheError(format!("Failed to check session existence: {}", e))
            })?;

        Ok(row.is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    async fn create_test_backend() -> DatabaseSessionBackend {
        let backend = DatabaseSessionBackend::new("sqlite::memory:")
            .await
            .expect("Failed to create test backend");
        backend
            .create_table()
            .await
            .expect("Failed to create table");
        backend
    }

    #[tokio::test]
    async fn test_save_and_load_session() {
        let backend = create_test_backend().await;
        let session_key = "test_session_1";
        let data = json!({
            "user_id": 123,
            "username": "testuser",
        });

        // Save session
        backend
            .save(session_key, &data, Some(3600))
            .await
            .expect("Failed to save session");

        // Load session
        let loaded: Option<serde_json::Value> = backend
            .load(session_key)
            .await
            .expect("Failed to load session");

        assert_eq!(loaded, Some(data));
    }

    #[tokio::test]
    async fn test_session_exists() {
        let backend = create_test_backend().await;
        let session_key = "test_session_2";
        let data = json!({"test": "data"});

        // Session should not exist initially
        let exists = backend
            .exists(session_key)
            .await
            .expect("Failed to check existence");
        assert!(!exists);

        // Save session
        backend
            .save(session_key, &data, Some(3600))
            .await
            .expect("Failed to save session");

        // Session should now exist
        let exists = backend
            .exists(session_key)
            .await
            .expect("Failed to check existence");
        assert!(exists);
    }

    #[tokio::test]
    async fn test_delete_session() {
        let backend = create_test_backend().await;
        let session_key = "test_session_3";
        let data = json!({"test": "data"});

        // Save session
        backend
            .save(session_key, &data, Some(3600))
            .await
            .expect("Failed to save session");

        // Verify session exists
        assert!(backend
            .exists(session_key)
            .await
            .expect("Failed to check existence"));

        // Delete session
        backend
            .delete(session_key)
            .await
            .expect("Failed to delete session");

        // Verify session no longer exists
        assert!(!backend
            .exists(session_key)
            .await
            .expect("Failed to check existence"));
    }

    #[tokio::test]
    async fn test_expired_session() {
        let backend = create_test_backend().await;
        let session_key = "test_session_4";
        let data = json!({"test": "data"});

        // Save session with 0 second TTL (immediately expired)
        backend
            .save(session_key, &data, Some(0))
            .await
            .expect("Failed to save session");

        // Wait a moment to ensure expiration
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Try to load expired session
        let loaded: Option<serde_json::Value> = backend
            .load(session_key)
            .await
            .expect("Failed to load session");

        assert_eq!(loaded, None);
    }

    #[tokio::test]
    async fn test_cleanup_expired() {
        let backend = create_test_backend().await;

        // Save some expired sessions
        for i in 0..5 {
            let key = format!("expired_{}", i);
            backend
                .save(&key, &json!({ "test": i }), Some(0))
                .await
                .expect("Failed to save session");
        }

        // Save some active sessions
        for i in 0..3 {
            let key = format!("active_{}", i);
            backend
                .save(&key, &json!({ "test": i }), Some(3600))
                .await
                .expect("Failed to save session");
        }

        // Wait for expiration
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Clean up expired sessions
        let deleted = backend.cleanup_expired().await.expect("Failed to cleanup");

        assert_eq!(deleted, 5);

        // Verify active sessions still exist
        for i in 0..3 {
            let key = format!("active_{}", i);
            assert!(backend
                .exists(&key)
                .await
                .expect("Failed to check existence"));
        }
    }
}
