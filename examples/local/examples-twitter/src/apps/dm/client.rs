//! DM client module
//!
//! WASM-only client components for direct messaging.

#[cfg(target_arch = "wasm32")]
pub mod components;

#[cfg(target_arch = "wasm32")]
pub mod hooks;

#[cfg(target_arch = "wasm32")]
pub use components::*;

#[cfg(target_arch = "wasm32")]
pub use hooks::*;
