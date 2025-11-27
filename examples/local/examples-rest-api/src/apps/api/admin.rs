//! Admin configuration for api app

use reinhardt::admin::ModelAdminConfig;

/// Admin configuration for Article model
///
/// This configures the admin panel display for Article items:
/// - List view with id, title, author, publication status, and creation date
/// - Filtering by publication status and creation date
/// - Search by title, author, and content
/// - Sorted by creation date (newest first)
/// - Read-only fields: created_at, updated_at
/// - 25 items per page
pub struct ArticleAdmin;

impl ArticleAdmin {
	/// Returns the ModelAdminConfig for Article model
	///
	/// # Example
	///
	/// ```rust,ignore
	/// use reinhardt_panel::AdminSite;
	/// use crate::apps::api::admin::ArticleAdmin;
	///
	/// let mut admin = AdminSite::new("Article Management");
	/// admin.register("Article", ArticleAdmin::config())?;
	/// ```
	pub fn config() -> ModelAdminConfig {
		ModelAdminConfig::builder()
			.model_name("Article")
			.list_display(vec!["id", "title", "author", "published", "created_at"])
			.list_filter(vec!["published", "created_at"])
			.search_fields(vec!["title", "author", "content"])
			.ordering(vec!["-created_at"])
			.readonly_fields(vec!["created_at", "updated_at"])
			.list_per_page(25)
			.build()
	}
}
