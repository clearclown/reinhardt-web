//! URL configuration for api app

use reinhardt::{Method, UnifiedRouter};

pub fn url_patterns() -> UnifiedRouter {
	let mut router = UnifiedRouter::builder().build();

	// Add RESTful API endpoints
	router.function("/articles", Method::GET, super::views::list_articles);
	router.function("/articles", Method::POST, super::views::create_article);
	router.function("/articles/:id", Method::GET, super::views::get_article);
	router.function("/articles/:id", Method::PUT, super::views::update_article);
	router.function("/articles/:id", Method::DELETE, super::views::delete_article);

	router
}
