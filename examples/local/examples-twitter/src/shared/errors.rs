//! Shared error types
//!
//! Error types that can be serialized and sent between client and server.

use serde::{Deserialize, Serialize};

/// Application error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
	/// Authentication failed
	Unauthorized,
	/// Resource not found
	NotFound,
	/// Permission denied
	Forbidden,
	/// Validation error
	Validation(String),
	/// Internal server error
	Internal(String),
	/// Database error
	Database(String),
}

impl std::fmt::Display for AppError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			AppError::Unauthorized => write!(f, "Unauthorized"),
			AppError::NotFound => write!(f, "Not found"),
			AppError::Forbidden => write!(f, "Forbidden"),
			AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
			AppError::Internal(msg) => write!(f, "Internal error: {}", msg),
			AppError::Database(msg) => write!(f, "Database error: {}", msg),
		}
	}
}

impl std::error::Error for AppError {}

// ============================================================================
// Conversion to/from ServerFnError (server-side only)
// ============================================================================

#[cfg(not(target_arch = "wasm32"))]
impl From<AppError> for reinhardt::pages::server_fn::ServerFnError {
	fn from(err: AppError) -> Self {
		use reinhardt::pages::server_fn::ServerFnError;
		match err {
			AppError::Unauthorized => ServerFnError::server(401, "Unauthorized"),
			AppError::NotFound => ServerFnError::server(404, "Not found"),
			AppError::Forbidden => ServerFnError::server(403, "Forbidden"),
			AppError::Validation(msg) => {
				ServerFnError::server(400, format!("Validation error: {}", msg))
			}
			AppError::Internal(msg) => {
				ServerFnError::server(500, format!("Internal error: {}", msg))
			}
			AppError::Database(msg) => {
				ServerFnError::server(500, format!("Database error: {}", msg))
			}
		}
	}
}

#[cfg(not(target_arch = "wasm32"))]
impl From<validator::ValidationErrors> for AppError {
	fn from(err: validator::ValidationErrors) -> Self {
		AppError::Validation(err.to_string())
	}
}

#[cfg(not(target_arch = "wasm32"))]
impl From<reinhardt::db::Error> for AppError {
	fn from(err: reinhardt::db::Error) -> Self {
		AppError::Database(err.to_string())
	}
}
