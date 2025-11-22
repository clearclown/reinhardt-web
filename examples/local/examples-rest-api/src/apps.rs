//! Application registry for examples-rest-api
//!
//! This file maintains the list of installed apps.

// Re-export apps from the apps directory
pub mod api;

pub use api::ApiConfig;
