//! URL configuration for {{ project_name }}

use reinhardt::prelude::*;
use reinhardt::register_url_patterns;
use std::sync::Arc;

/// Define URL patterns for the application
pub fn url_patterns() -> Arc<UnifiedRouter> {
    let router = UnifiedRouter::builder()
        .build();

    // Add your URL patterns here
    // Example:
    // router.add_function_route("/", Method::GET, home_view);
    // router.add_function_route("/about", Method::GET, about_view);
    //
    // Or include app routers:
    // router.include_router("/app/", app_router, Some("app".to_string()));

    Arc::new(router)
}

// Register URL patterns for automatic discovery by the framework
register_url_patterns!();
