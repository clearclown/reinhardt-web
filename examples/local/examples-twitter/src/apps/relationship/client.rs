//! Relationship client module
//!
//! WASM-only client components for user relationships (follow/unfollow).

#[cfg(target_arch = "wasm32")]
pub mod components;

#[cfg(target_arch = "wasm32")]
pub use components::*;
