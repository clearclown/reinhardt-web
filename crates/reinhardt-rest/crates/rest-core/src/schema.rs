//! OpenAPI/Swagger schema generation
//!
//! Re-exports schema types from openapi crate.

// Re-export all types from openapi crate
pub use ::openapi::*;

/// OpenAPI version constant
pub const OPENAPI_VERSION: &str = "3.0.3";
