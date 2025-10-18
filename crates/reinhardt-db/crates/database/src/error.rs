//! Error types for database operations

use thiserror::Error;

/// Database operation errors
#[derive(Error, Debug)]
pub enum DatabaseError {
    /// Feature not supported by this database
    #[error("Feature '{feature}' is not supported by {database}")]
    UnsupportedFeature { database: String, feature: String },

    /// SQL syntax error
    #[error("SQL syntax error: {0}")]
    SyntaxError(String),

    /// Type conversion error
    #[error("Type conversion error: {0}")]
    TypeError(String),

    /// Connection error
    #[error("Connection error: {0}")]
    ConnectionError(String),

    /// Query execution error
    #[error("Query execution error: {0}")]
    QueryError(String),

    /// Serialization/deserialization error
    #[error("Serialization error: {0}")]
    SerializationError(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// Generic database error
    #[error("Database error: {0}")]
    Other(String),
}

/// Result type for database operations
pub type Result<T> = std::result::Result<T, DatabaseError>;

impl From<serde_json::Error> for DatabaseError {
    fn from(err: serde_json::Error) -> Self {
        DatabaseError::SerializationError(err.to_string())
    }
}
