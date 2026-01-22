//! Configuration and settings module.
//!
//! This module provides access to the Reinhardt configuration system,
//! including settings management and CLI utilities.
//!
//! # Examples
//!
//! ```rust,ignore
//! # #[cfg(feature = "conf")]
//! use reinhardt::conf::Settings;
//!
//! # #[cfg(feature = "conf")]
//! let settings = Settings::default();
//! ```

#[cfg(feature = "conf")]
pub use reinhardt_conf::*;
