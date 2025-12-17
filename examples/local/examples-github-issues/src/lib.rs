//! examples-github-issues library
//!
//! This is the main library crate for examples-github-issues.

pub mod config;
pub mod apps;

// Re-export commonly used items
pub use config::settings::get_settings;
