//! Profile components
//!
//! Provides profile view and edit form components.

use crate::shared::types::{ProfileResponse, UpdateProfileRequest};
use reinhardt_pages::Signal;
use reinhardt_pages::component::{ElementView, IntoView, View};
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
use {
	crate::server::server_fn::profile::{fetch_profile, update_profile},
	wasm_bindgen_futures::spawn_local,
	web_sys::HtmlInputElement,
};

/// Profile view component
///
/// Displays user profile information with loading and error states.
pub fn profile_view(user_id: Uuid) -> View {
	let profile = Signal::new(None::<ProfileResponse>);
	let loading = Signal::new(true);
	let error = Signal::new(None::<String>);

	#[cfg(target_arch = "wasm32")]
	{
		let profile = profile.clone();
		let loading = loading.clone();
		let error = error.clone();

		spawn_local(async move {
			loading.set(true);
			error.set(None);

			match fetch_profile(user_id).await {
				Ok(profile_data) => {
					profile.set(Some(profile_data));
					loading.set(false);
				}
				Err(e) => {
					error.set(Some(e.to_string()));
					loading.set(false);
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
									// Loading state
									if loading.get() {
										ElementView::new("div")
											.attr("class", "text-center py-5")
											.child(
												ElementView::new("div")
													.attr("class", "spinner-border")
													.attr("role", "status")
													.child(
														ElementView::new("span")
															.attr("class", "visually-hidden")
															.child("Loading..."),
													),
											)
											.into_view()
									} else if let Some(err) = error.get() {
										// Error state
										ElementView::new("div")
											.attr("class", "alert alert-danger")
											.child(err)
											.into_view()
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
																.attr("href", &website)
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
																&format!(
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

/// Profile edit component
///
/// Provides form for editing user profile with validation.
pub fn profile_edit(user_id: Uuid) -> View {
	let bio = Signal::new(String::new());
	let avatar_url = Signal::new(String::new());
	let location = Signal::new(String::new());
	let website = Signal::new(String::new());
	let error = Signal::new(None::<String>);
	let loading = Signal::new(false);
	let success = Signal::new(false);

	// Load current profile data
	#[cfg(target_arch = "wasm32")]
	{
		let bio = bio.clone();
		let avatar_url = avatar_url.clone();
		let location = location.clone();
		let website = website.clone();

		spawn_local(async move {
			if let Ok(profile_data) = fetch_profile(user_id).await {
				bio.set(profile_data.bio.unwrap_or_default());
				avatar_url.set(profile_data.avatar_url.unwrap_or_default());
				location.set(profile_data.location.unwrap_or_default());
				website.set(profile_data.website.unwrap_or_default());
			}
		});
	}

	#[cfg(target_arch = "wasm32")]
	let on_submit = {
		let error = error.clone();
		let loading = loading.clone();
		let success = success.clone();
		let bio = bio.clone();
		let avatar_url = avatar_url.clone();
		let location = location.clone();
		let website = website.clone();

		move |event: web_sys::Event| {
			event.prevent_default();

			let error = error.clone();
			let loading = loading.clone();
			let success = success.clone();
			let bio_value = bio.get();
			let avatar_url_value = avatar_url.get();
			let location_value = location.get();
			let website_value = website.get();

			spawn_local(async move {
				loading.set(true);
				error.set(None);
				success.set(false);

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
						success.set(true);
						loading.set(false);
						// Redirect to profile view after 1 second
						if let Some(window) = web_sys::window() {
							let _ = window.location().set_href(&format!("/profile/{}", user_id));
						}
					}
					Err(e) => {
						error.set(Some(e.to_string()));
						loading.set(false);
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
									// Success message
									if success.get() {
										ElementView::new("div")
											.attr("class", "alert alert-success")
											.child("Profile updated successfully! Redirecting...")
											.into_view()
									} else {
										ElementView::new("div").into_view()
									}
								})
								.child({
									// Error message
									if let Some(msg) = error.get() {
										ElementView::new("div")
											.attr("class", "alert alert-danger")
											.child(msg)
											.into_view()
									} else {
										ElementView::new("div").into_view()
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
															&format!("/profile/{}", user_id),
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
