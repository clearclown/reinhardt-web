//! Profile components using React-like hooks
//!
//! Provides profile view and edit form components with hooks-styled state management.
//! Validation is handled server-side via server functions with automatic CSRF protection.

use crate::shared::types::{ProfileResponse, UpdateProfileRequest};
use reinhardt::pages::Signal;
use reinhardt::pages::component::View;
use reinhardt::pages::page;
use reinhardt::pages::reactive::hooks::use_state;
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
/// Displays user profile information with modern SNS design.
/// Features cover image, large avatar, bio, location, and website.
/// Uses watch blocks for reactive UI updates when async data loads.
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

	// Clone signals for passing to page! macro
	let loading_signal = loading.clone();
	let error_signal = error.clone();
	let profile_signal = profile.clone();

	// Clone user_id for use in page! macro
	let user_id_str = user_id.to_string();

	page!(|loading_signal: Signal<bool>, error_signal: Signal<Option<String>>, profile_signal: Signal<Option<ProfileResponse>>, user_id_str: String| {
		div {
			class: "max-w-2xl mx-auto",
			watch {
				if loading_signal.get() {
					div {
						class: "flex flex-col items-center justify-center py-16",
						div {
							class: "spinner-lg mb-4"
						}
						p {
							class: "text-content-secondary text-sm",
							"Loading profile..."
						}
					}
				} else if error_signal.get().is_some() {
					div {
						class: "p-4",
						div {
							class: "alert-danger",
							role: "alert",
							div {
								class: "flex items-center gap-2",
								svg {
									class: "w-5 h-5 flex-shrink-0",
									fill: "currentColor",
									viewBox: "0 0 20 20",
									path {
										fill_rule: "evenodd",
										d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
										clip_rule: "evenodd"
									}
								}
								span { { error_signal.get().unwrap_or_default() } }
							}
						}
					}
				} else if profile_signal.get().is_some() {
					div {
						class: "card overflow-hidden animate-fade-in",
						// Cover image area
						div {
							class: "h-32 sm:h-48 bg-gradient-to-r from-brand to-brand-dark relative"
						}
						// Profile info section
						div {
							class: "px-4 pb-4",
							// Avatar and edit button row
							div {
								class: "flex justify-between items-end -mt-12 sm:-mt-16 mb-4",
								// Avatar - using initial letter (framework constraint: dynamic img src not allowed)
								div {
									class: "avatar-xl sm:w-32 sm:h-32 rounded-full border-4 border-surface-primary bg-surface-tertiary flex items-center justify-center text-3xl sm:text-4xl font-bold text-content-secondary",
									// Display user icon as fallback since ProfileResponse doesn't have username
									span {
										"👤"
									}
								}
								// Edit button
								a {
									href: format!("/profile/{}/edit", user_id_str),
									class: "btn-outline",
									"Edit profile"
								}
							}
							// Username section
							div {
								class: "mb-4",
								h1 {
									class: "text-xl font-bold text-content-primary",
									"@user"
								}
							}
							// Bio
							if let Some(ref data) = profile_signal.get() {
								if data.bio.is_some() {
									p {
										class: "text-content-primary mb-4 whitespace-pre-wrap",
										{ data.bio.clone().unwrap_or_default() }
									}
								}
							}
							// Meta info (location, website)
							div {
								class: "flex flex-wrap gap-4 text-content-secondary text-sm",
								if let Some(ref data) = profile_signal.get() {
									// Location
									if data.location.is_some() {
										div {
											class: "flex items-center gap-1",
											svg {
												class: "w-4 h-4",
												fill: "none",
												stroke: "currentColor",
												viewBox: "0 0 24 24",
												path {
													stroke_linecap: "round",
													stroke_linejoin: "round",
													stroke_width: "2",
													d: "M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"
												}
												path {
													stroke_linecap: "round",
													stroke_linejoin: "round",
													stroke_width: "2",
													d: "M15 11a3 3 0 11-6 0 3 3 0 016 0z"
												}
											}
											span {
												{ data.location.clone().unwrap_or_default() }
											}
										}
									}
									// Website
									if data.website.is_some() {
										a {
											class: "flex items-center gap-1 text-brand hover:underline",
											href: data.website.clone().unwrap_or_default(),
											target: "_blank",
											rel: "noopener noreferrer",
											svg {
												class: "w-4 h-4",
												fill: "none",
												stroke: "currentColor",
												viewBox: "0 0 24 24",
												path {
													stroke_linecap: "round",
													stroke_linejoin: "round",
													stroke_width: "2",
													d: "M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"
												}
											}
											span {
												{ data.website.clone().unwrap_or_default() }
											}
										}
									}
								}
							}
							// Stats section
							div {
								class: "flex gap-6 mt-4 pt-4 border-t border-border",
								div {
									class: "flex items-center gap-1",
									span {
										class: "font-bold text-content-primary",
										"0"
									}
									span {
										class: "text-content-secondary text-sm",
										"Following"
									}
								}
								div {
									class: "flex items-center gap-1",
									span {
										class: "font-bold text-content-primary",
										"0"
									}
									span {
										class: "text-content-secondary text-sm",
										"Followers"
									}
								}
							}
						}
					}
				}
			}
		}
	})(loading_signal, error_signal, profile_signal, user_id_str)
}

/// Profile edit component using hooks
///
/// Provides form for editing user profile with modern design.
/// Features icon-prefixed inputs and clean form layout.
/// Uses watch blocks for reactive UI updates for error/success states.
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

	// Clone signals for passing to page! macro
	let error_signal = error.clone();
	let loading_signal = loading.clone();
	let success_signal = success.clone();
	let bio_signal = bio.clone();
	let avatar_url_signal = avatar_url.clone();
	let location_signal = location.clone();
	let website_signal = website.clone();

	// Clone user_id for use in page! macro
	let user_id_str = user_id.to_string();

	page!(|error_signal: Signal<Option<String>>, loading_signal: Signal<bool>, success_signal: Signal<bool>, bio_signal: Signal<String>, avatar_url_signal: Signal<String>, location_signal: Signal<String>, website_signal: Signal<String>, user_id_str: String| {
		div {
			class: "max-w-2xl mx-auto p-4",
			div {
				class: "card animate-fade-in",
				div {
					class: "card-header flex items-center gap-3",
					a {
						href: format!("/profile/{}", user_id_str.clone()),
						class: "btn-icon",
						svg {
							class: "w-5 h-5",
							fill: "none",
							stroke: "currentColor",
							viewBox: "0 0 24 24",
							path {
								stroke_linecap: "round",
								stroke_linejoin: "round",
								stroke_width: "2",
								d: "M10 19l-7-7m0 0l7-7m-7 7h18",
							}
						}
					}
					h1 {
						class: "text-xl font-bold",
						"Edit Profile"
					}
				}
				div {
					class: "card-body",
					watch {
						if success_signal.get() {
							div {
								class: "alert-success mb-4",
								role: "alert",
								div {
									class: "flex items-center gap-2",
									svg {
										class: "w-5 h-5 flex-shrink-0",
										fill: "currentColor",
										viewBox: "0 0 20 20",
										path {
											fill_rule: "evenodd",
											d: "M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z",
											clip_rule: "evenodd",
										}
									}
									span {
										"Profile updated successfully! Redirecting..."
									}
								}
							}
						}
					}
					watch {
						if error_signal.get().is_some() {
							div {
								class: "alert-danger mb-4",
								role: "alert",
								div {
									class: "flex items-center gap-2",
									svg {
										class: "w-5 h-5 flex-shrink-0",
										fill: "currentColor",
										viewBox: "0 0 20 20",
										path {
											fill_rule: "evenodd",
											d: "M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z",
											clip_rule: "evenodd",
										}
									}
									span {
										{ error_signal.get().unwrap_or_default() }
									}
								}
							}
						}
					}
					form {
						class: "space-y-5",
						@submit: {
									let set_error = set_error.clone();
									let set_loading = set_loading.clone();
									let set_success = set_success.clone();
									let bio = bio.clone();
									let avatar_url = avatar_url.clone();
									let location = location.clone();
									let website = website.clone();
									move |event: web_sys::Event| {
										#[cfg(target_arch = "wasm32")]
										{
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
														if let Some(window) = web_sys::window() {
															let _ =
																window.location().set_href(&format!("/profile/{}", user_id));
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
								},
						div {
							label {
								r#for: "avatar_url",
								class: "form-label",
								"Avatar URL"
							}
							div {
								class: "relative",
								div {
									class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
									svg {
										class: "w-5 h-5 text-content-tertiary",
										fill: "none",
										stroke: "currentColor",
										viewBox: "0 0 24 24",
										path {
											stroke_linecap: "round",
											stroke_linejoin: "round",
											stroke_width: "2",
											d: "M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z",
										}
									}
								}
								input {
									r#type: "url",
									class: "form-input pl-10",
									id: "avatar_url",
									name: "avatar_url",
									placeholder: "https://example.com/avatar.jpg",
									value: avatar_url_signal.get(),
									@input: {
												let set_avatar_url = set_avatar_url.clone();
												move |event: web_sys::Event| {
													#[cfg(target_arch = "wasm32")]
													{
														use wasm_bindgen::JsCast;
														use web_sys::HtmlInputElement;
														if let Some(target) = event.target() {
															if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
																set_avatar_url(input.value());
															}
														}
													}
												}
											},
								}
							}
						}
						div {
							label {
								r#for: "bio",
								class: "form-label",
								"Bio"
							}
							textarea {
								class: "form-textarea",
								id: "bio",
								name: "bio",
								rows: 4,
								placeholder: "Tell the world about yourself...",
								@input: {
											let set_bio = set_bio.clone();
											move |event: web_sys::Event| {
												#[cfg(target_arch = "wasm32")]
												{
													use wasm_bindgen::JsCast;
													use web_sys::HtmlTextAreaElement;
													if let Some(target) = event.target() {
														if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
															set_bio(textarea.value());
														}
													}
												}
											}
										},
								{ bio_signal.get() }
							}
							p {
								class: "mt-1 text-sm text-content-tertiary",
								"Brief description for your profile. URLs are hyperlinked."
							}
						}
						div {
							label {
								r#for: "location",
								class: "form-label",
								"Location"
							}
							div {
								class: "relative",
								div {
									class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
									svg {
										class: "w-5 h-5 text-content-tertiary",
										fill: "none",
										stroke: "currentColor",
										viewBox: "0 0 24 24",
										path {
											stroke_linecap: "round",
											stroke_linejoin: "round",
											stroke_width: "2",
											d: "M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z",
										}
										path {
											stroke_linecap: "round",
											stroke_linejoin: "round",
											stroke_width: "2",
											d: "M15 11a3 3 0 11-6 0 3 3 0 016 0z",
										}
									}
								}
								input {
									r#type: "text",
									class: "form-input pl-10",
									id: "location",
									name: "location",
									placeholder: "San Francisco, CA",
									value: location_signal.get(),
									@input: {
												let set_location = set_location.clone();
												move |event: web_sys::Event| {
													#[cfg(target_arch = "wasm32")]
													{
														use wasm_bindgen::JsCast;
														use web_sys::HtmlInputElement;
														if let Some(target) = event.target() {
															if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
																set_location(input.value());
															}
														}
													}
												}
											},
								}
							}
						}
						div {
							label {
								r#for: "website",
								class: "form-label",
								"Website"
							}
							div {
								class: "relative",
								div {
									class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
									svg {
										class: "w-5 h-5 text-content-tertiary",
										fill: "none",
										stroke: "currentColor",
										viewBox: "0 0 24 24",
										path {
											stroke_linecap: "round",
											stroke_linejoin: "round",
											stroke_width: "2",
											d: "M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1",
										}
									}
								}
								input {
									r#type: "url",
									class: "form-input pl-10",
									id: "website",
									name: "website",
									placeholder: "https://example.com",
									value: website_signal.get(),
									@input: {
												let set_website = set_website.clone();
												move |event: web_sys::Event| {
													#[cfg(target_arch = "wasm32")]
													{
														use wasm_bindgen::JsCast;
														use web_sys::HtmlInputElement;
														if let Some(target) = event.target() {
															if let Ok(input) = target.dyn_into::<HtmlInputElement>() {
																set_website(input.value());
															}
														}
													}
												}
											},
								}
							}
						}
						div {
							class: "flex justify-end gap-3 pt-4 border-t border-border",
							a {
								href: format!("/profile/{}", user_id_str),
								class: "btn-secondary",
								"Cancel"
							}
							watch {
								if loading_signal.get() {
									button {
										r#type: "submit",
										class: "btn-primary opacity-50 cursor-not-allowed",
										disabled: loading_signal.get(),
										div {
											class: "flex items-center gap-2",
											div {
												class: "spinner-sm border-white border-t-transparent",
											}
											"Saving..."
										}
									}
								} else {
									button {
										r#type: "submit",
										class: "btn-primary",
										"Save"
									}
								}
							}
						}
					}
				}
			}
		}
	})(
		error_signal,
		loading_signal,
		success_signal,
		bio_signal,
		avatar_url_signal,
		location_signal,
		website_signal,
		user_id_str,
	)
}
