//! URL configuration for examples-twitter project (RESTful)
//!
//! The `url_patterns` routes URLs to handlers.

use crate::apps;
use reinhardt::db::DatabaseConnection;
use reinhardt::prelude::*;
use reinhardt::register_url_patterns;
use std::sync::Arc;

use super::{admin, views};

/// Build URL patterns without admin panel
///
/// Use this when database connection is not available
/// or when you don't need the admin panel.
pub fn url_patterns() -> Arc<UnifiedRouter> {
	let mut router = build_api_router();

	// Register all routes before returning
	router.register_all_routes();

	Arc::new(router)
}

/// Build URL patterns with admin panel
///
/// Includes the admin panel under `/admin/api/` prefix.
/// Admin UI (WASM) should be served at `/admin/` via static files.
///
/// # Arguments
///
/// * `db` - Database connection for admin CRUD operations
pub fn url_patterns_with_admin(db: DatabaseConnection) -> Arc<UnifiedRouter> {
	use reinhardt_admin_api::{admin_routes, AdminDatabase};
	use reinhardt::reinhardt_di::SingletonScope;
	use std::sync::Arc as StdArc;

	let mut router = build_api_router();

	// Configure admin site and register models
	let admin_site = admin::configure_admin();

	// Create AdminDatabase wrapper
	let admin_db = AdminDatabase::new(db);

	// Configure DI container for admin panel
	let singleton = StdArc::new(SingletonScope::new());
	reinhardt_admin_api::AdminSite::configure_di(
		&singleton,
		StdArc::new(admin_site),
		admin_db,
		None, // No favicon data for now
	);

	// Get admin routes and include under /admin/api/ prefix
	let admin_router = admin_routes();
	router = router.include("/admin/api/", admin_router);

	// Admin UI static files (WASM)
	// Note: HTTPメソッドデコレーター（#[get(...)]）を使用しているため、
	// ここでの登録は不要（自動登録される）

	// Register all routes before returning
	router.register_all_routes();

	Arc::new(router)
}

/// Build the base API router
fn build_api_router() -> UnifiedRouter {
	UnifiedRouter::new()
		// Health check endpoint
		.endpoint(views::health_check)
		// Auth routes
		.endpoint(apps::auth::views::register)
		// Profile routes
		.endpoint(apps::profile::views::fetch_profile)
		.endpoint(apps::profile::views::create_profile)
		.endpoint(apps::profile::views::patch_profile)
		// Relationship routes
		.endpoint(apps::relationship::views::follow_user)
		.endpoint(apps::relationship::views::unfollow_user)
		.endpoint(apps::relationship::views::block_user)
		.endpoint(apps::relationship::views::unblock_user)
		.endpoint(apps::relationship::views::fetch_followers)
		.endpoint(apps::relationship::views::fetch_followings)
		.endpoint(apps::relationship::views::fetch_blockings)
		// DM routes
		.endpoint(apps::dm::views::list_rooms)
		.endpoint(apps::dm::views::get_room)
		.endpoint(apps::dm::views::create_room)
		.endpoint(apps::dm::views::delete_room)
		.endpoint(apps::dm::views::list_messages)
		.endpoint(apps::dm::views::send_message)
		.endpoint(apps::dm::views::get_message)
}

// Register URL patterns with admin panel for automatic discovery by the framework
register_url_patterns!(admin);
