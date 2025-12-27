//! Client-side code (WASM)
//!
//! This module contains all client-side code that runs in the browser.

#[cfg(target_arch = "wasm32")]
pub mod lib;

#[cfg(target_arch = "wasm32")]
pub mod router;

#[cfg(target_arch = "wasm32")]
pub mod pages;

#[cfg(target_arch = "wasm32")]
pub mod components;
