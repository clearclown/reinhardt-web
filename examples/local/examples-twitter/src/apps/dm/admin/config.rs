//! Admin configuration for dm app models
//!
//! Provides admin panel registration for DM models.

use reinhardt_admin_api::{AdminSite, ModelAdminConfig};

/// Register all admin configurations for dm app
pub fn register_admins(site: &mut AdminSite) {
	let room_admin = ModelAdminConfig::builder()
		.model_name("DMRoom")
		.table_name("dm_rooms")
		.list_display(vec!["id", "name", "is_group", "created_at"])
		.list_filter(vec!["is_group"])
		.search_fields(vec!["name"])
		.list_per_page(25)
		.build();

	site.register("DMRoom", room_admin)
		.expect("Failed to register DMRoom admin");

	let message_admin = ModelAdminConfig::builder()
		.model_name("DMMessage")
		.table_name("dm_messages")
		.list_display(vec!["id", "room_id", "sender_id", "is_read", "created_at"])
		.list_filter(vec!["is_read"])
		.search_fields(vec!["content"])
		.list_per_page(50)
		.build();

	site.register("DMMessage", message_admin)
		.expect("Failed to register DMMessage admin");
}
