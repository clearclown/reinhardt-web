//! URL routing for hello app

use reinhardt::{Method, UnifiedRouter};

use crate::apps::hello::views;

pub fn url_patterns() -> UnifiedRouter {
	UnifiedRouter::new()
		.function("/", Method::GET, views::hello_world)
		.function("/health", Method::GET, views::health_check)
}
