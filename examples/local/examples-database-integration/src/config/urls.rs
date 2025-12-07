//! URL configuration for database-integration example (RESTful)
//!
//! The `url_patterns` routes URLs to handlers.

use reinhardt::UnifiedRouter;

use super::views;

pub fn url_patterns() -> UnifiedRouter {
	UnifiedRouter::new()
		.endpoint(views::list_users)
		.mount("/api/todos/", crate::apps::todos::urls::url_patterns())
}
