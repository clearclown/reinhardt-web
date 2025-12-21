//! WASM entry point
//!
//! This is the main entry point for the WASM application.

use wasm_bindgen::prelude::*;

mod router;
mod state;

pub use router::{AppRoute, init_global_router, with_router};
pub use state::{
	auth_state, get_current_user, init_auth_state, is_authenticated, set_current_user,
};

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
	// Set panic hook for better error messages in browser console
	console_error_panic_hook::set_once();

	// Initialize global state
	state::init_auth_state();

	// Initialize router
	router::init_global_router();

	// Get the root element
	let window = web_sys::window().expect("no global `window` exists");
	let document = window.document().expect("should have a document on window");
	let root = document
		.get_element_by_id("root")
		.expect("should have #root element");

	// Clear loading spinner
	root.set_inner_html("");

	// Mount the router
	router::with_router(|router| {
		router.mount(&root);
	});

	Ok(())
}
