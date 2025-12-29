//! URL configuration for examples-twitter project
//!
//! This project uses reinhardt-pages with Server Functions for API communication.
//! Server Functions are automatically registered via the inventory crate.

use reinhardt::pages::server_fn::register_all_server_functions;
use reinhardt::prelude::*;
use reinhardt::routes;

/// Build URL patterns for the application
///
/// This project primarily uses Server Functions (`#[server_fn]`) for API communication.
/// All server functions decorated with `#[server_fn]` are automatically collected
/// and registered via the inventory crate.
#[routes]
pub fn routes() -> UnifiedRouter {
	// Automatically register all server functions collected by inventory
	// Each #[server_fn] macro generates an inventory::submit! that adds
	// the function to the global registry
	register_all_server_functions(UnifiedRouter::new())
}
