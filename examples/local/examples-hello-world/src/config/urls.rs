//! URL configuration for examples-hello-world project
//!
//! The `routes` function defines all URL patterns for this project.

use reinhardt::prelude::*;
use reinhardt::routes;

#[routes]
pub fn routes() -> ServerRouter {
	// Mount hello app routes
	ServerRouter::new().mount("/", crate::apps::hello::urls::url_patterns())
}
