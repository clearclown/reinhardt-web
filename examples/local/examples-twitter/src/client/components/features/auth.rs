//! Authentication components using React-like hooks
//!
//! Provides login and registration form components with hooks-styled state management.
//! Validation is handled server-side via server functions with automatic CSRF protection.

use crate::shared::types::{LoginRequest, RegisterRequest};
use reinhardt_pages::component::{ElementView, IntoView, View};
use reinhardt_pages::reactive::hooks::use_state;

#[cfg(target_arch = "wasm32")]
use {
	crate::client::state::set_current_user,
	crate::server_fn::auth::{login, register},
	wasm_bindgen::JsCast,
	wasm_bindgen_futures::spawn_local,
	web_sys::HtmlInputElement,
};

/// Login form component using hooks
///
/// Provides email/password login with:
/// - HTML5 validation for required fields and email format
/// - Server-side validation via server functions
/// - Automatic CSRF protection via server function headers
pub fn login_form() -> View {
	// Hook-styled state management
	let (error, set_error) = use_state(None::<String>);
	let (loading, set_loading) = use_state(false);

	#[cfg(target_arch = "wasm32")]
	let on_submit = {
		let set_error = set_error.clone();
		let set_loading = set_loading.clone();
		move |event: web_sys::Event| {
			event.prevent_default();

			let set_error = set_error.clone();
			let set_loading = set_loading.clone();

			// Get form data
			let form = event
				.target()
				.and_then(|t| t.dyn_into::<web_sys::HtmlFormElement>().ok());

			if let Some(form) = form {
				let email = form
					.elements()
					.named_item("email")
					.and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
					.map(|i| i.value())
					.unwrap_or_default();

				let password = form
					.elements()
					.named_item("password")
					.and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
					.map(|i| i.value())
					.unwrap_or_default();

				spawn_local(async move {
					set_loading(true);
					set_error(None);

					let request = LoginRequest { email, password };

					match login(request).await {
						Ok(user_info) => {
							set_current_user(Some(user_info));
							// Navigate to home/timeline
							if let Some(window) = web_sys::window() {
								let _ = window.location().set_href("/timeline");
							}
						}
						Err(e) => {
							set_error(Some(e.to_string()));
							set_loading(false);
						}
					}
				});
			}
		}
	};

	#[cfg(not(target_arch = "wasm32"))]
	let on_submit = |_event: web_sys::Event| {};

	ElementView::new("div")
		.attr("class", "container mt-5")
		.child(
			ElementView::new("div")
				.attr("class", "row justify-content-center")
				.child(
					ElementView::new("div").attr("class", "col-md-6").child(
						ElementView::new("div").attr("class", "card").child(
							ElementView::new("div")
								.attr("class", "card-body")
								.child(
									ElementView::new("h2")
										.attr("class", "card-title text-center mb-4")
										.child("Login"),
								)
								.child({
									// Error message
									let error_view = if let Some(msg) = error.get() {
										ElementView::new("div")
											.attr("class", "alert alert-danger")
											.child(msg)
											.into_view()
									} else {
										ElementView::new("div").into_view()
									};
									error_view
								})
								.child(
									ElementView::new("form")
										.listener("submit", on_submit)
										.child(
											ElementView::new("div")
												.attr("class", "mb-3")
												.child(
													ElementView::new("label")
														.attr("for", "email")
														.attr("class", "form-label")
														.child("Email"),
												)
												.child(
													ElementView::new("input")
														.attr("type", "email")
														.attr("class", "form-control")
														.attr("id", "email")
														.attr("name", "email")
														.attr("required", "true")
														.attr("placeholder", "Enter your email"),
												),
										)
										.child(
											ElementView::new("div")
												.attr("class", "mb-3")
												.child(
													ElementView::new("label")
														.attr("for", "password")
														.attr("class", "form-label")
														.child("Password"),
												)
												.child(
													ElementView::new("input")
														.attr("type", "password")
														.attr("class", "form-control")
														.attr("id", "password")
														.attr("name", "password")
														.attr("required", "true")
														.attr("placeholder", "Enter your password"),
												),
										)
										.child(
											ElementView::new("div").attr("class", "d-grid").child(
												ElementView::new("button")
													.attr("type", "submit")
													.attr(
														"class",
														if loading.get() {
															"btn btn-primary disabled"
														} else {
															"btn btn-primary"
														},
													)
													.child(if loading.get() {
														"Logging in..."
													} else {
														"Login"
													}),
											),
										)
										.child(
											ElementView::new("div")
												.attr("class", "text-center mt-3")
												.child("Don't have an account? ")
												.child(
													ElementView::new("a")
														.attr("href", "/register")
														.child("Register here"),
												),
										),
								),
						),
					),
				),
		)
		.into_view()
}

/// Registration form component using hooks
///
/// Provides username/email/password registration with:
/// - HTML5 validation for required fields and email format
/// - Server-side validation including password matching
/// - Automatic CSRF protection via server function headers
pub fn register_form() -> View {
	// Hook-styled state management
	let (error, set_error) = use_state(None::<String>);
	let (loading, set_loading) = use_state(false);

	#[cfg(target_arch = "wasm32")]
	let on_submit = {
		let set_error = set_error.clone();
		let set_loading = set_loading.clone();
		move |event: web_sys::Event| {
			event.prevent_default();

			let set_error = set_error.clone();
			let set_loading = set_loading.clone();

			// Get form data
			let form = event
				.target()
				.and_then(|t| t.dyn_into::<web_sys::HtmlFormElement>().ok());

			if let Some(form) = form {
				let username = form
					.elements()
					.named_item("username")
					.and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
					.map(|i| i.value())
					.unwrap_or_default();

				let email = form
					.elements()
					.named_item("email")
					.and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
					.map(|i| i.value())
					.unwrap_or_default();

				let password = form
					.elements()
					.named_item("password")
					.and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
					.map(|i| i.value())
					.unwrap_or_default();

				let password_confirmation = form
					.elements()
					.named_item("password_confirmation")
					.and_then(|e| e.dyn_into::<HtmlInputElement>().ok())
					.map(|i| i.value())
					.unwrap_or_default();

				// Simple client-side password match validation
				if password != password_confirmation {
					set_error(Some("Passwords do not match".to_string()));
					return;
				}

				spawn_local(async move {
					set_loading(true);
					set_error(None);

					let request = RegisterRequest {
						username,
						email,
						password,
						password_confirmation,
					};

					match register(request).await {
						Ok(()) => {
							// Navigate to login page
							if let Some(window) = web_sys::window() {
								let _ = window.location().set_href("/login");
							}
						}
						Err(e) => {
							set_error(Some(e.to_string()));
							set_loading(false);
						}
					}
				});
			}
		}
	};

	#[cfg(not(target_arch = "wasm32"))]
	let on_submit = |_event: web_sys::Event| {};

	ElementView::new("div")
		.attr("class", "container mt-5")
		.child(
			ElementView::new("div")
				.attr("class", "row justify-content-center")
				.child(
					ElementView::new("div").attr("class", "col-md-6").child(
						ElementView::new("div").attr("class", "card").child(
							ElementView::new("div")
								.attr("class", "card-body")
								.child(
									ElementView::new("h2")
										.attr("class", "card-title text-center mb-4")
										.child("Register"),
								)
								.child({
									// Error message
									let error_view = if let Some(msg) = error.get() {
										ElementView::new("div")
											.attr("class", "alert alert-danger")
											.child(msg)
											.into_view()
									} else {
										ElementView::new("div").into_view()
									};
									error_view
								})
								.child(
									ElementView::new("form")
										.listener("submit", on_submit)
										.child(
											ElementView::new("div")
												.attr("class", "mb-3")
												.child(
													ElementView::new("label")
														.attr("for", "username")
														.attr("class", "form-label")
														.child("Username"),
												)
												.child(
													ElementView::new("input")
														.attr("type", "text")
														.attr("class", "form-control")
														.attr("id", "username")
														.attr("name", "username")
														.attr("required", "true")
														.attr("placeholder", "Choose a username"),
												),
										)
										.child(
											ElementView::new("div")
												.attr("class", "mb-3")
												.child(
													ElementView::new("label")
														.attr("for", "email")
														.attr("class", "form-label")
														.child("Email"),
												)
												.child(
													ElementView::new("input")
														.attr("type", "email")
														.attr("class", "form-control")
														.attr("id", "email")
														.attr("name", "email")
														.attr("required", "true")
														.attr("placeholder", "Enter your email"),
												),
										)
										.child(
											ElementView::new("div")
												.attr("class", "mb-3")
												.child(
													ElementView::new("label")
														.attr("for", "password")
														.attr("class", "form-label")
														.child("Password"),
												)
												.child(
													ElementView::new("input")
														.attr("type", "password")
														.attr("class", "form-control")
														.attr("id", "password")
														.attr("name", "password")
														.attr("required", "true")
														.attr("placeholder", "Choose a password"),
												),
										)
										.child(
											ElementView::new("div")
												.attr("class", "mb-3")
												.child(
													ElementView::new("label")
														.attr("for", "password_confirmation")
														.attr("class", "form-label")
														.child("Confirm Password"),
												)
												.child(
													ElementView::new("input")
														.attr("type", "password")
														.attr("class", "form-control")
														.attr("id", "password_confirmation")
														.attr("name", "password_confirmation")
														.attr("required", "true")
														.attr(
															"placeholder",
															"Confirm your password",
														),
												),
										)
										.child(
											ElementView::new("div").attr("class", "d-grid").child(
												ElementView::new("button")
													.attr("type", "submit")
													.attr(
														"class",
														if loading.get() {
															"btn btn-primary disabled"
														} else {
															"btn btn-primary"
														},
													)
													.child(if loading.get() {
														"Registering..."
													} else {
														"Register"
													}),
											),
										)
										.child(
											ElementView::new("div")
												.attr("class", "text-center mt-3")
												.child("Already have an account? ")
												.child(
													ElementView::new("a")
														.attr("href", "/login")
														.child("Login here"),
												),
										),
								),
						),
					),
				),
		)
		.into_view()
}
