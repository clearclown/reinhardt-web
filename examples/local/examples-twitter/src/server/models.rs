//! Server-side models
//!
//! This module re-exports models from the apps/ structure.
//! In the future, models will be defined here directly.

// Re-export models from apps structure
#[cfg(not(target_arch = "wasm32"))]
pub use crate::apps::auth::models::*;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::apps::dm::models::*;
#[cfg(not(target_arch = "wasm32"))]
pub use crate::apps::profile::models::*;
