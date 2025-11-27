//! URL configuration for api app

use reinhardt::{Method, UnifiedRouter};

pub fn url_patterns() -> UnifiedRouter {
	// Use method chaining for UnifiedRouter builder pattern
	// Note: Routes should NOT have leading slash because after prefix stripping
	// from include("/api/", ...), the remaining path has no leading slash
	// (e.g., "/api/articles" -> "articles", "/api/articles/1" -> "articles/1")
	UnifiedRouter::new()
		.function("articles", Method::GET, super::views::list_articles)
		.function("articles", Method::POST, super::views::create_article)
		.function("articles/{id}", Method::GET, super::views::get_article)
		.function("articles/{id}", Method::PUT, super::views::update_article)
		.function(
			"articles/{id}",
			Method::DELETE,
			super::views::delete_article,
		)
}
