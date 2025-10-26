//! URL routing and proxy utilities for Reinhardt framework.
//!
//! This crate provides URL routing, pattern matching, and proxy functionality
//! for the Reinhardt web framework. It is a unified interface over the following
//! internal crates:
//!
//! - `reinhardt-routers`: URL routing and pattern matching
//! - `reinhardt-routers-macros`: Compile-time URL validation macros
//! - `reinhardt-proxy`: Lazy relationship loading for ORM
//!
//! ## Planned Features
//!
//! ### Route Middleware Support
//!
//! Support for per-route middleware configuration is planned for future releases.
//! This will allow attaching middleware to specific routes or route groups:
//!
//! ```rust,ignore
//! use reinhardt_urls::prelude::*;
//!
//! let router = Router::new()
//!     .route("/public", handler)
//!     .route("/protected", handler)
//!         .with_middleware(AuthMiddleware::new()) // Route-specific middleware
//!     .group("/admin")
//!         .with_middleware(AdminAuthMiddleware::new()) // Group middleware
//!         .route("/users", users_handler)
//!         .route("/settings", settings_handler);
//! ```
//!
//! **Implementation Status**: Planned
//!
//! **Design Considerations**:
//! - Should middleware be applied at route registration time or dynamically?
//! - How to handle middleware ordering (global vs route-specific)?
//! - Should middleware be composable (chain multiple middleware)?
//! - What's the best API for route groups with shared middleware?
//!
//! **Required Changes**:
//! 1. Extend `Route` struct to store middleware stack
//! 2. Add `with_middleware()` method to router builders
//! 3. Implement middleware execution in request handling pipeline
//! 4. Support middleware inheritance for nested route groups
//!
//! See reinhardt-routers crate for the underlying routing implementation.

#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "routers")]
#[cfg_attr(docsrs, doc(cfg(feature = "routers")))]
pub use reinhardt_routers as routers;

#[cfg(feature = "routers-macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "routers-macros")))]
pub use reinhardt_routers_macros as routers_macros;

#[cfg(feature = "proxy")]
#[cfg_attr(docsrs, doc(cfg(feature = "proxy")))]
pub use reinhardt_proxy as proxy;

// Re-export commonly used types from routers
#[cfg(feature = "routers")]
#[cfg_attr(docsrs, doc(cfg(feature = "routers")))]
pub mod prelude {
    pub use reinhardt_routers::{
        clear_script_prefix, get_script_prefix, set_script_prefix, PathPattern, Route, Router,
    };
}
