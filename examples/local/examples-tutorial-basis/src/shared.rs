//! Shared types and utilities
//!
//! This module contains types and utilities shared between client and server.

#[cfg(not(target_arch = "wasm32"))]
pub mod forms;
pub mod types;
