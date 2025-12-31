//! Profile components using React-like hooks
//!
//! Provides profile view and edit form components with hooks-styled state management.
//! Validation is handled server-side via server functions with automatic CSRF protection.

use crate::shared::types::{ProfileResponse, UpdateProfileRequest};
use reinhardt_pages::component::{ElementView, IntoView, View};
use reinhardt_pages::page;
use reinhardt_pages::reactive::hooks::use_state;
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
use {
	crate::server_fn::profile::{fetch_profile, update_profile},
	wasm_bindgen_futures::spawn_local,
};

#[cfg(not(target_arch = "wasm32"))]
use crate::server_fn::profile::fetch_profile;

/// Profile view component using hooks
///
/// Displays user profile information with loading and error states.
/// Uses React-like hooks for state management.
pub fn profile_view(user_id: Uuid) -> View {
	// Hook-styled state management
	let (profile, set_profile) = use_state(None::<ProfileResponse>);
	let (loading, set_loading) = use_state(true);
	let (error, set_error) = use_state(None::<String>);

	#[cfg(target_arch = "wasm32")]
	{
		// Clone setters for async use
		let set_profile = set_profile.clone();
		let set_loading = set_loading.clone();
		let set_error = set_error.clone();

		spawn_local(async move {
			set_loading(true);
			set_error(None);

			match fetch_profile(user_id).await {
				Ok(profile_data) => {
					set_profile(Some(profile_data));
					set_loading(false);
				}
				Err(e) => {
					set_error(Some(e.to_string()));
					set_loading(false);
				}
			}
		});
	}

	ElementView::new("div")
		.attr("class", "container mt-5")
		.child(
			ElementView::new("div")
				.attr("class", "row justify-content-center")
				.child(
					ElementView::new("div").attr("class", "col-md-8").child(
						ElementView::new("div").attr("class", "card").child(
							ElementView::new("div")
								.attr("class", "card-body")
								.child(
									ElementView::new("h2")
										.attr("class", "card-title mb-4")
										.child("Profile"),
								)
								.child({
									use crate::client::components::common::{
										error_alert, loading_spinner,
									};

									// Loading state
									if loading.get() {
										loading_spinner()
									} else if let Some(err) = error.get() {
										// Error state
										error_alert(&err, false)
									} else if let Some(profile_data) = profile.get() {
										// Profile data
										ElementView::new("div")
											.child(
												ElementView::new("div")
													.attr("class", "mb-3")
													.child(ElementView::new("strong").child("Bio:"))
													.child(ElementView::new("p").child(
														profile_data.bio.unwrap_or_else(|| {
															"No bio provided".to_string()
														}),
													)),
											)
											.child(
												ElementView::new("div")
													.attr("class", "mb-3")
													.child(
														ElementView::new("strong")
															.child("Location:"),
													)
													.child(ElementView::new("p").child(
														profile_data.location.unwrap_or_else(
															|| "Not specified".to_string(),
														),
													)),
											)
											.child(
												ElementView::new("div")
													.attr("class", "mb-3")
													.child(
														ElementView::new("strong")
															.child("Website:"),
													)
													.child(ElementView::new("p").child(
														if let Some(website) = profile_data.website
														{
															ElementView::new("a")
																.attr("href", website.clone())
																.attr("target", "_blank")
																.attr("rel", "noopener noreferrer")
																.child(website)
																.into_view()
														} else {
															ElementView::new("span")
																.child("No website")
																.into_view()
														},
													)),
											)
											.child(
												ElementView::new("div")
													.attr("class", "mt-4")
													.child(
														ElementView::new("a")
															.attr(
																"href",
																format!(
																	"/profile/{}/edit",
																	user_id
																),
															)
															.attr("class", "btn btn-primary")
															.child("Edit Profile"),
													),
											)
											.into_view()
									} else {
										ElementView::new("div").into_view()
									}
								}),
						),
					),
				),
		)
		.into_view()
}

/// Profile edit component using hooks
///
/// Provides form for editing user profile with:
/// - Initial values loaded from server
/// - HTML5 validation for URL fields
/// - Server-side validation via server functions
/// - Automatic CSRF protection via server function headers
pub fn profile_edit(user_id: Uuid) -> View {
	// Hook-styled state for form fields
	let (bio, set_bio) = use_state(String::new());
	let (avatar_url, set_avatar_url) = use_state(String::new());
	let (location, set_location) = use_state(String::new());
	let (website, set_website) = use_state(String::new());
	let (error, set_error) = use_state(None::<String>);
	let (loading, set_loading) = use_state(false);
	let (success, set_success) = use_state(false);

	// Load current profile data
	#[cfg(target_arch = "wasm32")]
	{
		let set_bio = set_bio.clone();
		let set_avatar_url = set_avatar_url.clone();
		let set_location = set_location.clone();
		let set_website = set_website.clone();

		spawn_local(async move {
			// Fetch profile data for initial values
			if let Ok(profile_data) = fetch_profile(user_id).await {
				set_bio(profile_data.bio.unwrap_or_default());
				set_avatar_url(profile_data.avatar_url.unwrap_or_default());
				set_location(profile_data.location.unwrap_or_default());
				set_website(profile_data.website.unwrap_or_default());
			}
		});
	}

	// Submit handler using closures with Signal access
	#[cfg(target_arch = "wasm32")]
	let on_submit = {
		let set_error = set_error.clone();
		let set_loading = set_loading.clone();
		let set_success = set_success.clone();
		let bio = bio.clone();
		let avatar_url = avatar_url.clone();
		let location = location.clone();
		let website = website.clone();

		move |event: web_sys::Event| {
			event.prevent_default();

			let set_error = set_error.clone();
			let set_loading = set_loading.clone();
			let set_success = set_success.clone();
			let bio_value = bio.get();
			let avatar_url_value = avatar_url.get();
			let location_value = location.get();
			let website_value = website.get();

			spawn_local(async move {
				set_loading(true);
				set_error(None);
				set_success(false);

				let request = UpdateProfileRequest {
					bio: if bio_value.is_empty() {
						None
					} else {
						Some(bio_value)
					},
					avatar_url: if avatar_url_value.is_empty() {
						None
					} else {
						Some(avatar_url_value)
					},
					location: if location_value.is_empty() {
						None
					} else {
						Some(location_value)
					},
					website: if website_value.is_empty() {
						None
					} else {
						Some(website_value)
					},
				};

				match update_profile(request).await {
					Ok(_) => {
						set_success(true);
						set_loading(false);
						// Redirect to profile view after success
						if let Some(window) = web_sys::window() {
							let _ = window.location().set_href(&format!("/profile/{}", user_id));
						}
					}
					Err(e) => {
						set_error(Some(e.to_string()));
						set_loading(false);
					}
				}
			});
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
					ElementView::new("div").attr("class", "col-md-8").child(
						ElementView::new("div").attr("class", "card").child(
							ElementView::new("div")
								.attr("class", "card-body")
								.child(
									ElementView::new("h2")
										.attr("class", "card-title mb-4")
										.child("Edit Profile"),
								)
								.child({
									use crate::client::components::common::{
										error_alert, success_alert,
									};

									// Success message
									if success.get() {
										success_alert(
											"Profile updated successfully! Redirecting...",
										)
									} else if let Some(msg) = error.get() {
										// Error message
										error_alert(&msg, false)
									} else {
										page!(|| { div {} })()
									}
								})
								.child(
									ElementView::new("form")
										.listener("submit", on_submit)
										.child(
											ElementView::new("div")
												.attr("class", "mb-3")
												.child(
													ElementView::new("label")
														.attr("for", "bio")
														.attr("class", "form-label")
														.child("Bio"),
												)
												.child(
													ElementView::new("textarea")
														.attr("class", "form-control")
														.attr("id", "bio")
														.attr("name", "bio")
														.attr("rows", "3")
														.attr(
															"placeholder",
															"Tell us about yourself",
														),
												),
										)
										.child(
											ElementView::new("div")
												.attr("class", "mb-3")
												.child(
													ElementView::new("label")
														.attr("for", "avatar_url")
														.attr("class", "form-label")
														.child("Avatar URL"),
												)
												.child(
													ElementView::new("input")
														.attr("type", "url")
														.attr("class", "form-control")
														.attr("id", "avatar_url")
														.attr("name", "avatar_url")
														.attr(
															"placeholder",
															"https://example.com/avatar.jpg",
														),
												),
										)
										.child(
											ElementView::new("div")
												.attr("class", "mb-3")
												.child(
													ElementView::new("label")
														.attr("for", "location")
														.attr("class", "form-label")
														.child("Location"),
												)
												.child(
													ElementView::new("input")
														.attr("type", "text")
														.attr("class", "form-control")
														.attr("id", "location")
														.attr("name", "location")
														.attr("placeholder", "New York, NY"),
												),
										)
										.child(
											ElementView::new("div")
												.attr("class", "mb-3")
												.child(
													ElementView::new("label")
														.attr("for", "website")
														.attr("class", "form-label")
														.child("Website"),
												)
												.child(
													ElementView::new("input")
														.attr("type", "url")
														.attr("class", "form-control")
														.attr("id", "website")
														.attr("name", "website")
														.attr("placeholder", "https://example.com"),
												),
										)
										.child(
											ElementView::new("div")
												.attr(
													"class",
													"d-grid gap-2 d-md-flex justify-content-md-end",
												)
												.child(
													ElementView::new("a")
														.attr(
															"href",
															format!("/profile/{}", user_id),
														)
														.attr("class", "btn btn-secondary")
														.child("Cancel"),
												)
												.child(
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
															"Saving..."
														} else {
															"Save Changes"
														}),
												),
										),
								),
						),
					),
				),
		)
		.into_view()
}
