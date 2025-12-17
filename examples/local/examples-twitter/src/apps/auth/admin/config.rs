//! Admin configuration for auth app models
//!
//! Provides admin panel registration for the User model.

use reinhardt_admin_api::{AdminSite, ModelAdminConfig};

/// Register all admin configurations for auth app
pub fn register_admins(site: &mut AdminSite) {
	let user_admin = ModelAdminConfig::builder()
		.model_name("User")
		.table_name("users")
		.list_display(vec!["id", "username", "email", "is_active"])
		.list_filter(vec!["is_active"])
		.search_fields(vec!["username", "email"])
		.list_per_page(25)
		.build();

	site.register("User", user_admin)
		.expect("Failed to register User admin");
}
