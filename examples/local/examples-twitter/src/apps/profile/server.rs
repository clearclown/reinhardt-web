//! Profile server module
//!
//! Contains server-only functionality for profile management.

#[cfg(not(target_arch = "wasm32"))]
pub mod server_fn;

#[cfg(not(target_arch = "wasm32"))]
pub use server_fn::*;
