//! DM server module
//!
//! Server-only components for direct messaging.

#[cfg(not(target_arch = "wasm32"))]
pub mod server_fn;

#[cfg(not(target_arch = "wasm32"))]
pub use server_fn::*;
