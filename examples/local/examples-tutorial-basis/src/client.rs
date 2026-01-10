//! Client-side code (WASM)
//!
//! This module contains all client-side code that runs in the browser.

#[cfg(wasm)]
pub mod lib;

#[cfg(wasm)]
pub mod router;

#[cfg(wasm)]
pub mod pages;

#[cfg(wasm)]
pub mod components;
