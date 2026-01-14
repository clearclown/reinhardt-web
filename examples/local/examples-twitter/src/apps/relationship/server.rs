//! Relationship server module
//!
//! Server-only components for user relationships.

#[cfg(not(target_arch = "wasm32"))]
pub mod server_fn;

#[cfg(not(target_arch = "wasm32"))]
pub use server_fn::*;
