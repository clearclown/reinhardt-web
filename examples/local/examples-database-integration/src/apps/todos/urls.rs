//! URL configuration for todos app

use reinhardt::UnifiedRouter;

use super::views;

pub fn url_patterns() -> UnifiedRouter {
	UnifiedRouter::new()
		.endpoint(views::list_todos)
		.endpoint(views::create_todo)
		.endpoint(views::get_todo)
		.endpoint(views::update_todo)
		.endpoint(views::delete_todo)
}
