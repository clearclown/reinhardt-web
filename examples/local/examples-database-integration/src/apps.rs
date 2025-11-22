//! Application registry for examples-database-integration
//!
//! This file maintains the list of installed apps.

// Re-export apps from the apps directory
pub mod todos;

pub use todos::TodosConfig;
