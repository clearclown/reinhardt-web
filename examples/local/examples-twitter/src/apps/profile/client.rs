//! Profile client module (WASM)
//!
//! Contains WASM-only client components for profile functionality.

#[cfg(target_arch = "wasm32")]
pub mod components;

#[cfg(target_arch = "wasm32")]
pub use components::*;
