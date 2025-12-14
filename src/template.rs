//! Template and rendering module.
//!
//! This module provides template engine, template macros, and renderers.
//!
//! # Examples
//!
//! ```rust,no_run
//! use reinhardt::template::templates::TemplateLoader;
//! use reinhardt::template::renderers::JSONRenderer;
//! ```

#[cfg(feature = "templates")]
pub use reinhardt_template::*;
