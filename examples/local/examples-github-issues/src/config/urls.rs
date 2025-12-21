//! URL configuration for examples-github-issues project (RESTful)
//!
//! The `url_patterns` routes URLs to handlers.

use reinhardt::UnifiedRouter;
use reinhardt::db::DatabaseConnection;
use reinhardt::register_url_patterns;
use std::sync::Arc;

use super::views;

/// Build URL patterns without admin panel
///
/// Use this when database connection is not available
/// or when you don't need the admin panel.
pub fn url_patterns() -> Arc<UnifiedRouter> {
	let router = UnifiedRouter::new().endpoint(views::health_check);

	Arc::new(router)
}

/// Build URL patterns with admin panel
///
/// Includes the admin panel under `/admin` prefix.
///
/// # Arguments
///
/// * `_db` - Database connection for admin CRUD operations
pub fn url_patterns_with_admin(_db: DatabaseConnection) -> Arc<UnifiedRouter> {
	let router = UnifiedRouter::new().endpoint(views::health_check);

	// TODO: Include admin panel when implemented
	// let admin_router = admin::configure_admin(_db);
	// router = router.include("/admin/", admin_router);

	Arc::new(router)
}

// Register URL patterns with admin panel for automatic discovery by the framework
register_url_patterns!(admin);
