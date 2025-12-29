//! Integration modules for special macros.
//!
//! This module provides runtime support for special macros used in the page! macro.
//! Each integration module provides initialization and resolution functions for
//! their respective macros.

/// Runtime support for asset! macro.
///
/// Provides URL resolution for static files using a manifest.
#[cfg(feature = "static")]
pub mod static_context;

/// Runtime support for url! macro.
///
/// Provides URL resolution for named routes.
#[cfg(feature = "urls")]
pub mod url_resolver;
