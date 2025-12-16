//! URL patterns registration for compile-time discovery
//!
//! This module provides types and macros for registering URL pattern functions
//! at compile time using the `inventory` crate. This allows the framework to
//! automatically discover and register routers without manual boilerplate in
//! management commands.
//!
//! # Architecture
//!
//! The URL patterns registration system follows the same pattern as other
//! compile-time registration systems in Reinhardt (DI, Signals, OpenAPI, ViewSets):
//!
//! 1. User code calls `register_url_patterns!()` macro in `src/config/urls.rs`
//! 2. Macro generates an `inventory::submit!` call with function pointers
//! 3. Framework code retrieves registrations via `inventory::iter::<UrlPatternsRegistration>()`
//! 4. Framework calls the registered functions and registers routers
//!
//! # Examples
//!
//! ## Standard Project (no admin)
//!
//! ```rust,ignore
//! // src/config/urls.rs
//! use reinhardt::prelude::*;
//! use reinhardt::register_url_patterns;
//! use std::sync::Arc;
//!
//! pub fn url_patterns() -> Arc<UnifiedRouter> {
//!     let router = UnifiedRouter::new();
//!     // ... add routes
//!     Arc::new(router)
//! }
//!
//! // Register for automatic discovery
//! register_url_patterns!();
//! ```
//!
//! ## Admin-Enabled Project
//!
//! ```rust,ignore
//! // src/config/urls.rs
//! use reinhardt::prelude::*;
//! use reinhardt::register_url_patterns;
//! use reinhardt::db::DatabaseConnection;
//! use std::sync::Arc;
//!
//! pub fn url_patterns() -> Arc<UnifiedRouter> {
//!     let router = UnifiedRouter::new();
//!     // ... add routes
//!     Arc::new(router)
//! }
//!
//! pub fn url_patterns_with_admin(db: DatabaseConnection) -> Arc<UnifiedRouter> {
//!     let router = UnifiedRouter::new();
//!     // ... add routes with admin panel
//!     Arc::new(router)
//! }
//!
//! // Register both functions for automatic discovery
//! register_url_patterns!(admin);
//! ```

use crate::UnifiedRouter;
use std::sync::Arc;

// Re-export DatabaseConnection type from reinhardt-db
// This avoids requiring users to import reinhardt-db directly
#[cfg(feature = "database")]
pub use reinhardt_db::orm::DatabaseConnection;

#[cfg(not(feature = "database"))]
/// Placeholder type when database feature is not enabled
pub struct DatabaseConnection;

/// URL patterns registration for compile-time discovery
///
/// This type is used with the `inventory` crate to register URL pattern
/// functions at compile time, allowing the framework to automatically
/// discover and register routers without manual boilerplate in management
/// commands like `runserver` or `check`.
///
/// # Fields
///
/// * `get_router` - Function pointer to get the standard router
/// * `get_admin_router` - Optional function pointer to get the admin router (requires database)
///
/// # Implementation Details
///
/// This struct is collected by `inventory::collect!` and can be iterated
/// at runtime using `inventory::iter::<UrlPatternsRegistration>()`.
///
/// The framework automatically calls these functions in `execute_from_command_line()`
/// to register routers before executing management commands.
#[derive(Clone)]
pub struct UrlPatternsRegistration {
	/// Function to get the standard router
	///
	/// This function returns an `Arc<UnifiedRouter>` with all application routes.
	/// It is always called, regardless of whether admin functionality is enabled.
	pub get_router: fn() -> Arc<UnifiedRouter>,

	/// Optional function to get the admin router (requires database)
	///
	/// This function takes a `DatabaseConnection` parameter and returns an
	/// `Arc<UnifiedRouter>` with admin panel routes included. If this is `Some`,
	/// the framework will automatically obtain a database connection and call
	/// this function instead of `get_router`.
	pub get_admin_router: Option<fn(DatabaseConnection) -> Arc<UnifiedRouter>>,
}

impl UrlPatternsRegistration {
	/// Create a new registration with just the standard router
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use reinhardt_routers::registration::UrlPatternsRegistration;
	///
	/// let registration = UrlPatternsRegistration::new(url_patterns);
	/// ```
	pub const fn new(get_router: fn() -> Arc<UnifiedRouter>) -> Self {
		Self {
			get_router,
			get_admin_router: None,
		}
	}

	/// Create a new registration with both standard and admin routers
	///
	/// # Examples
	///
	/// ```rust,ignore
	/// use reinhardt_routers::registration::UrlPatternsRegistration;
	///
	/// let registration = UrlPatternsRegistration::with_admin(
	///     url_patterns,
	///     url_patterns_with_admin,
	/// );
	/// ```
	pub const fn with_admin(
		get_router: fn() -> Arc<UnifiedRouter>,
		get_admin_router: fn(DatabaseConnection) -> Arc<UnifiedRouter>,
	) -> Self {
		Self {
			get_router,
			get_admin_router: Some(get_admin_router),
		}
	}
}

// Collect registrations for runtime iteration
inventory::collect!(UrlPatternsRegistration);
