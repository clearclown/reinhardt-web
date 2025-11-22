//! URL configuration for hello app

use reinhardt_routers::UnifiedRouter;

pub fn url_patterns() -> UnifiedRouter {
	let router = UnifiedRouter::builder().build();

	// Add hello world endpoint
	// router.function("/", Method::GET, super::views::hello_world);

	router
}
