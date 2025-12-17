//! Admin configuration for profile app models
//!
//! Provides admin panel registration for the Profile model.

use reinhardt_admin_api::{AdminSite, ModelAdminConfig};

/// Register all admin configurations for profile app
pub fn register_admins(site: &mut AdminSite) {
	let profile_admin = ModelAdminConfig::builder()
		.model_name("Profile")
		.table_name("profiles")
		.list_display(vec!["id", "user_id", "bio", "location"])
		.list_filter(vec!["location"])
		.search_fields(vec!["bio", "website"])
		.list_per_page(25)
		.build();

	site.register("Profile", profile_admin)
		.expect("Failed to register Profile admin");
}
