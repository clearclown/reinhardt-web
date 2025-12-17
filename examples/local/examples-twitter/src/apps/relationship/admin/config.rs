//! Admin configuration for relationship app models
//!
//! Note: Follow/Block relationships are managed through User's ManyToManyField,
//! so dedicated admin models are not needed here.

use reinhardt_admin_api::AdminSite;

/// Register all admin configurations for relationship app
pub fn register_admins(_site: &mut AdminSite) {
	// Relationships are managed through User model's ManyToMany fields
	// (following, blocked_users) rather than separate models
}
