//! Tweet components using React-like hooks
//!
//! Provides tweet card, tweet form, and tweet list components with hooks-styled state management.

use crate::shared::types::{CreateTweetRequest, TweetInfo};
use reinhardt_pages::component::{ElementView, IntoView, View};
use reinhardt_pages::page;
use reinhardt_pages::reactive::hooks::use_state;
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
use {
	crate::server_fn::tweet::{create_tweet, delete_tweet, list_tweets},
	wasm_bindgen::JsCast,
	wasm_bindgen_futures::spawn_local,
	web_sys::HtmlTextAreaElement,
};

/// Tweet card component using hooks
///
/// Displays a single tweet with delete button if owned by current user.
/// Uses React-like hooks for state management.
pub fn tweet_card(tweet: &TweetInfo, show_delete: bool) -> View {
	let tweet_id = tweet.id;

	// Hook-styled state management
	let (deleted, set_deleted) = use_state(false);
	let (error, set_error) = use_state(None::<String>);

	#[cfg(target_arch = "wasm32")]
	let on_delete = {
		let set_deleted = set_deleted.clone();
		let set_error = set_error.clone();

		move |_event: web_sys::Event| {
			let set_deleted = set_deleted.clone();
			let set_error = set_error.clone();

			spawn_local(async move {
				match delete_tweet(tweet_id).await {
					Ok(()) => {
						set_deleted(true);
					}
					Err(e) => {
						set_error(Some(e.to_string()));
					}
				}
			});
		}
	};

	#[cfg(not(target_arch = "wasm32"))]
	let on_delete = |_event: web_sys::Event| {};

	if deleted.get() {
		// Tweet deleted - show nothing
		return ElementView::new("div").into_view();
	}

	ElementView::new("div")
		.attr("class", "card mb-3")
		.child(
			ElementView::new("div")
				.attr("class", "card-body")
				.child(
					ElementView::new("div")
						.attr("class", "d-flex justify-content-between align-items-start")
						.child(
							ElementView::new("div")
								.child(
									ElementView::new("h6")
										.attr("class", "card-subtitle mb-2 text-muted")
										.child(format!("@{}", tweet.username)),
								)
								.child(
									ElementView::new("p")
										.attr("class", "card-text")
										.child(&tweet.content),
								)
								.child(
									ElementView::new("small")
										.attr("class", "text-muted")
										.child(&tweet.created_at),
								),
						)
						.child(if show_delete {
							ElementView::new("button")
								.attr("class", "btn btn-sm btn-danger")
								.attr("type", "button")
								.listener("click", on_delete)
								.child("Delete")
								.into_view()
						} else {
							ElementView::new("div").into_view()
						}),
				)
				.child(if let Some(err) = error.get() {
					ElementView::new("div")
						.attr("class", "alert alert-danger mt-2")
						.child(err)
						.into_view()
				} else {
					ElementView::new("div").into_view()
				}),
		)
		.into_view()
}

/// Tweet form component using hooks
///
/// Provides form for creating a new tweet with 280 character limit.
/// Uses React-like hooks for state management.
/// CSRF protection is handled automatically by #[server_fn] macro via headers.
pub fn tweet_form() -> View {
	// Hook-styled state for form fields
	let (content, set_content) = use_state(String::new());
	let (error, set_error) = use_state(None::<String>);
	let (loading, set_loading) = use_state(false);
	let (char_count, set_char_count) = use_state(0usize);

	#[cfg(target_arch = "wasm32")]
	let on_input = {
		let set_content = set_content.clone();
		let set_char_count = set_char_count.clone();

		move |event: web_sys::Event| {
			if let Some(target) = event.target() {
				if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
					let value = textarea.value();
					set_char_count(value.len());
					set_content(value);
				}
			}
		}
	};

	#[cfg(not(target_arch = "wasm32"))]
	let on_input = |_event: web_sys::Event| {};

	#[cfg(target_arch = "wasm32")]
	let on_submit = {
		let set_error = set_error.clone();
		let set_loading = set_loading.clone();
		let content = content.clone();
		let set_content = set_content.clone();
		let set_char_count = set_char_count.clone();

		move |event: web_sys::Event| {
			event.prevent_default();

			let set_error = set_error.clone();
			let set_loading = set_loading.clone();
			let content_value = content.get();
			let set_content = set_content.clone();
			let set_char_count = set_char_count.clone();

			spawn_local(async move {
				set_loading(true);
				set_error(None);

				let request = CreateTweetRequest {
					content: content_value,
				};

				match create_tweet(request).await {
					Ok(_) => {
						// Clear form
						set_content(String::new());
						set_char_count(0);
						set_loading(false);
						// Reload page to show new tweet
						if let Some(window) = web_sys::window() {
							let _ = window.location().reload();
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
		.attr("class", "card mb-4")
		.child(
			ElementView::new("div")
				.attr("class", "card-body")
				.child(
					ElementView::new("h5")
						.attr("class", "card-title")
						.child("What's happening?"),
				)
				.child(if let Some(msg) = error.get() {
					ElementView::new("div")
						.attr("class", "alert alert-danger")
						.child(msg)
						.into_view()
				} else {
					ElementView::new("div").into_view()
				})
				.child(
					ElementView::new("form")
						.listener("submit", on_submit)
						.child(
							ElementView::new("div")
								.attr("class", "mb-3")
								.child(
									ElementView::new("textarea")
										.attr("class", "form-control")
										.attr("id", "content")
										.attr("name", "content")
										.attr("rows", "3")
										.attr("maxlength", "280")
										.attr("placeholder", "What's on your mind?")
										.listener("input", on_input),
								)
								.child(
									ElementView::new("div")
										.attr(
											"class",
											"d-flex justify-content-between align-items-center mt-2",
										)
										.child(
											ElementView::new("small")
												.attr(
													"class",
													if char_count.get() > 280 {
														"text-danger"
													} else if char_count.get() > 250 {
														"text-warning"
													} else {
														"text-muted"
													},
												)
												.child(format!("{}/280", char_count.get())),
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
												.attr(
													"disabled",
													if char_count.get() == 0
														|| char_count.get() > 280
													{
														"true"
													} else {
														""
													},
												)
												.child(if loading.get() {
													"Posting..."
												} else {
													"Post"
												}),
										),
								),
						),
				),
		)
		.into_view()
}

/// Tweet list component using hooks
///
/// Displays list of tweets with loading and error states.
/// Uses React-like hooks for state management.
pub fn tweet_list(user_id: Option<Uuid>) -> View {
	use crate::client::components::common::{error_alert, loading_spinner};

	// Hook-styled state management
	let (tweets, set_tweets) = use_state(Vec::<TweetInfo>::new());
	let (loading, set_loading) = use_state(true);
	let (error, set_error) = use_state(None::<String>);

	#[cfg(target_arch = "wasm32")]
	{
		let set_tweets = set_tweets.clone();
		let set_loading = set_loading.clone();
		let set_error = set_error.clone();

		spawn_local(async move {
			set_loading(true);
			set_error(None);

			match list_tweets(user_id, 0).await {
				Ok(tweet_list) => {
					set_tweets(tweet_list);
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
		.child(if loading.get() {
			// Loading state - use shared component
			loading_spinner()
		} else if let Some(err) = error.get() {
			// Error state - use shared component
			error_alert(&err, false)
		} else {
			// Tweet list
			let tweet_list = tweets.get();
			if tweet_list.is_empty() {
				page!(|| {
					div {
						class: "text-center py-5",
						p {
							class: "text-muted",
							"No tweets yet. Be the first to post!"
						}
					}
				})()
			} else {
				ElementView::new("div")
					.children(
						tweet_list
							.iter()
							.map(|tweet| tweet_card(tweet, false))
							.collect::<Vec<_>>(),
					)
					.into_view()
			}
		})
		.into_view()
}
