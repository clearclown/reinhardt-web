//! Admin panel configuration
//!
//! Configures and builds the admin interface for examples-twitter.

use reinhardt_admin_api::AdminSite;

use crate::apps;

/// Configure the admin site
///
/// Creates an AdminSite and registers all model admins from each app.
/// Database connection will be configured via DI container in urls.rs.
///
/// # Endpoints (via admin_routes() in urls.rs)
///
/// - `GET /admin/api/` - Dashboard (list of registered models)
/// - `GET /admin/api/{model}/` - List model instances
/// - `GET /admin/api/{model}/{id}/` - Get model instance detail
/// - `POST /admin/api/{model}/` - Create model instance
/// - `PUT /admin/api/{model}/{id}/` - Update model instance
/// - `DELETE /admin/api/{model}/{id}/` - Delete model instance
/// - `GET /admin/api/{model}/export/` - Export model data
/// - `POST /admin/api/{model}/import/` - Import model data
pub fn configure_admin() -> AdminSite {
	let mut site = AdminSite::new("Twitter Admin");

	// Register admin configurations from each app
	apps::auth::admin::register_admins(&mut site);
	apps::profile::admin::register_admins(&mut site);
	apps::relationship::admin::register_admins(&mut site);
	apps::dm::admin::register_admins(&mut site);

	site
}
