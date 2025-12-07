//! URL configuration for api app

use reinhardt::UnifiedRouter;

use super::views;

pub fn url_patterns() -> UnifiedRouter {
	UnifiedRouter::new()
		.endpoint(views::list_articles)
		.endpoint(views::create_article)
		.endpoint(views::get_article)
		.endpoint(views::update_article)
		.endpoint(views::delete_article)
}
