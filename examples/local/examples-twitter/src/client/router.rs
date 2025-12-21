//! Client-side routing
//!
//! This module defines the client-side router for the Twitter clone application.

use reinhardt_pages::component::View;
use reinhardt_pages::router::Router;
use std::cell::RefCell;

/// Application routes
#[derive(Debug, Clone, PartialEq)]
pub enum AppRoute {
	/// Home/Timeline page
	Home,
	/// Login page
	Login,
	/// Register page
	Register,
	/// User profile page
	Profile { user_id: String },
	/// Timeline page
	Timeline,
	/// Not found page
	NotFound,
}

// Global Router instance
thread_local! {
	static ROUTER: RefCell<Option<Router>> = const { RefCell::new(None) };
}

/// Initialize the global router instance
///
/// This must be called once at application startup before any routing operations.
pub fn init_global_router() {
	ROUTER.with(|r| {
		*r.borrow_mut() = Some(init_router());
	});
}

/// Provides access to the global router instance
///
/// # Panics
///
/// Panics if the router has not been initialized via `init_global_router()`.
pub fn with_router<F, R>(f: F) -> R
where
	F: FnOnce(&Router) -> R,
{
	ROUTER.with(|r| {
		f(r.borrow()
			.as_ref()
			.expect("Router not initialized. Call init_global_router() first."))
	})
}

/// Initialize the router with all application routes
fn init_router() -> Router {
	use reinhardt_pages::component::{ElementView, IntoView};

	let router = Router::new();

	// Home route
	router.add_route("/", || home_page());

	// Authentication routes
	router.add_route("/login", || login_page());
	router.add_route("/register", || register_page());

	// User profile routes (with dynamic parameter)
	router.add_route("/profile/:user_id/edit", || {
		with_router(|r| {
			let params = r.current_params().get();
			let user_id_str = params
				.get("user_id")
				.cloned()
				.unwrap_or_else(|| "unknown".to_string());
			profile_edit_page(user_id_str)
		})
	});

	router.add_route("/profile/:user_id", || {
		with_router(|r| {
			let params = r.current_params().get();
			let user_id_str = params
				.get("user_id")
				.cloned()
				.unwrap_or_else(|| "unknown".to_string());
			profile_page(user_id_str)
		})
	});

	// Timeline route
	router.add_route("/timeline", || timeline_page());

	// 404 not found (fallback)
	router.set_not_found_handler(|| not_found_page());

	router
}

/// Home page view
pub fn home_page_view() -> View {
	use reinhardt_pages::component::{ElementView, IntoView};

	ElementView::new("div")
		.attr("class", "container mt-5")
		.child(
			ElementView::new("h1")
				.attr("class", "mb-4")
				.child("Twitter Clone - Home")
				.into_view(),
		)
		.child(
			ElementView::new("p")
				.attr("class", "lead")
				.child("Welcome to the Twitter Clone!")
				.into_view(),
		)
		.child(
			ElementView::new("div")
				.attr("class", "mt-4")
				.child(
					ElementView::new("a")
						.attr("href", "/login")
						.attr("class", "btn btn-primary me-2")
						.child("Login")
						.into_view(),
				)
				.child(
					ElementView::new("a")
						.attr("href", "/register")
						.attr("class", "btn btn-secondary")
						.child("Register")
						.into_view(),
				)
				.into_view(),
		)
		.into_view()
}

/// Login page view
pub fn login_page_view() -> View {
	use crate::client::components::features::auth::login_form;
	login_form()
}

/// Register page view
pub fn register_page_view() -> View {
	use crate::client::components::features::auth::register_form;
	register_form()
}

/// Profile page view
pub fn profile_page_view(user_id_str: String) -> View {
	use crate::client::components::features::profile::profile_view;
	use reinhardt_pages::component::{ElementView, IntoView};
	use uuid::Uuid;

	// Parse UUID from string
	match Uuid::parse_str(&user_id_str) {
		Ok(user_id) => profile_view(user_id),
		Err(_) => {
			// Invalid UUID - show error
			ElementView::new("div")
				.attr("class", "container mt-5")
				.child(
					ElementView::new("div")
						.attr("class", "alert alert-danger")
						.child("Invalid user ID format"),
				)
				.into_view()
		}
	}
}

/// Profile edit page view
pub fn profile_edit_page_view(user_id_str: String) -> View {
	use crate::client::components::features::profile::profile_edit;
	use reinhardt_pages::component::{ElementView, IntoView};
	use uuid::Uuid;

	// Parse UUID from string
	match Uuid::parse_str(&user_id_str) {
		Ok(user_id) => profile_edit(user_id),
		Err(_) => {
			// Invalid UUID - show error
			ElementView::new("div")
				.attr("class", "container mt-5")
				.child(
					ElementView::new("div")
						.attr("class", "alert alert-danger")
						.child("Invalid user ID format"),
				)
				.into_view()
		}
	}
}

/// Timeline page view
pub fn timeline_page_view() -> View {
	use crate::client::components::features::tweet::{tweet_form, tweet_list};
	use reinhardt_pages::component::{ElementView, IntoView};

	ElementView::new("div")
		.attr("class", "container mt-5")
		.child(
			ElementView::new("h1")
				.attr("class", "mb-4")
				.child("Timeline"),
		)
		.child(
			// Tweet form
			tweet_form(),
		)
		.child(ElementView::new("hr").attr("class", "my-4"))
		.child(
			// Tweet list (all tweets)
			tweet_list(None),
		)
		.into_view()
}

/// Not found page view
pub fn not_found_page_view() -> View {
	use reinhardt_pages::component::{ElementView, IntoView};

	ElementView::new("div")
		.attr("class", "container mt-5")
		.child(
			ElementView::new("h1")
				.attr("class", "mb-4")
				.child("404 - Page Not Found")
				.into_view(),
		)
		.child(
			ElementView::new("p")
				.child("The page you are looking for does not exist.")
				.into_view(),
		)
		.child(
			ElementView::new("a")
				.attr("href", "/")
				.attr("class", "btn btn-primary")
				.child("Go Home")
				.into_view(),
		)
		.into_view()
}
