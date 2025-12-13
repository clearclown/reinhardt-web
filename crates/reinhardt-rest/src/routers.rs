//! REST API routers
//!
//! Re-exports router types from reinhardt-routers.
//!
//! ## Example
//!
//! ```rust,no_run
//! use reinhardt_rest::routers::DefaultRouter;
//! # use std::sync::Arc;
//! # struct UserViewSet;
//!
//! let mut router = DefaultRouter::new();
//! router.register_viewset("users", Arc::new(UserViewSet));
//! ```

// Re-export router types from reinhardt-routers
pub use reinhardt_urls::routers::{DefaultRouter, Router};

// Re-export additional types needed for URL patterns
pub use reinhardt_urls::routers::{Route, UrlPattern};
