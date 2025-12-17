//! URL configuration for auth app (GraphQL)

use reinhardt::UnifiedRouter;

pub fn url_patterns() -> UnifiedRouter {
	let router = UnifiedRouter::new();

	// TODO: Add GraphQL endpoint here
	// Example:
	// router.function("/graphql", Method::POST, graphql_handler);

	router
}
