//! URL configuration for database-integration example (RESTful)
//!
//! The `routes` function defines all URL patterns for this project.

use reinhardt::ServerRouter;
use reinhardt::routes;

use super::views;

#[routes]
pub fn routes() -> ServerRouter {
	ServerRouter::new()
		.endpoint(views::list_users)
		.mount("/api/todos/", crate::apps::todos::urls::url_patterns())
}
