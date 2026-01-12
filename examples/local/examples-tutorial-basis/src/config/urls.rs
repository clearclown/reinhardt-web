//! URL configuration for examples-tutorial-basis project
//!
//! The `routes` function defines all URL patterns for this project.

use reinhardt::pages::server_fn::register_all_server_functions;
use reinhardt::prelude::*;
use reinhardt::routes;

#[routes]
pub fn routes() -> UnifiedRouter {
	// Register all server functions first
	let router = register_all_server_functions(UnifiedRouter::new());

	// Mount polls routes
	router.mount("/polls/", crate::apps::polls::urls::routes())
}
