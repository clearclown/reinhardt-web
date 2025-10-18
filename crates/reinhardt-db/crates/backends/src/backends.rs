/// Database backend modules
///
/// This module contains database-specific implementations of schema editors
/// and other backend-specific functionality.

#[cfg(feature = "postgres")]
pub mod postgresql;

#[cfg(feature = "mysql")]
pub mod mysql;

#[cfg(feature = "sqlite")]
pub mod sqlite;
