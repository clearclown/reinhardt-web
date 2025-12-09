//! examples-twitter library
//!
//! This is the main library crate for examples-twitter.

// Re-export internal crates for macro-generated code
// These are required by the #[post], #[get], etc. macros
pub use reinhardt::reinhardt_core;
pub use reinhardt::reinhardt_http;
pub use reinhardt::reinhardt_params;

pub mod config;
pub mod apps;
pub mod migrations;

#[cfg(test)]
pub mod test_utils;

// Re-export commonly used items
pub use config::settings::get_settings;
pub use config::urls::url_patterns;
