//! URL configuration for issues app
//!
//! Routes are handled by the unified GraphQL schema in config/urls.rs.

use reinhardt::UnifiedRouter;

/// Returns an empty router as issue routes are served via unified GraphQL schema
pub fn url_patterns() -> UnifiedRouter {
	UnifiedRouter::new()
}
