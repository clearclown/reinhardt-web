//! Tweet components
//!
//! Provides tweet card, tweet form, and tweet list components.

use crate::shared::types::{CreateTweetRequest, TweetInfo};
use reinhardt_pages::Signal;
use reinhardt_pages::component::{ElementView, IntoView, View};
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
use {
	crate::server::server_fn::tweet::{create_tweet, delete_tweet, list_tweets},
	wasm_bindgen_futures::spawn_local,
	web_sys::HtmlTextAreaElement,
};

/// Tweet card component
///
/// Displays a single tweet with delete button if owned by current user.
pub fn tweet_card(tweet: &TweetInfo, show_delete: bool) -> View {
	let tweet_id = tweet.id;
	let deleted = Signal::new(false);
	let error = Signal::new(None::<String>);

	#[cfg(target_arch = "wasm32")]
	let on_delete = {
		let deleted = deleted.clone();
		let error = error.clone();

		move |_event: web_sys::Event| {
			let deleted = deleted.clone();
			let error = error.clone();

			spawn_local(async move {
				match delete_tweet(tweet_id).await {
					Ok(()) => {
						deleted.set(true);
					}
					Err(e) => {
						error.set(Some(e.to_string()));
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

/// Tweet form component
///
/// Provides form for creating a new tweet with 280 character limit.
pub fn tweet_form() -> View {
	let content = Signal::new(String::new());
	let error = Signal::new(None::<String>);
	let loading = Signal::new(false);
	let char_count = Signal::new(0);

	#[cfg(target_arch = "wasm32")]
	let on_input = {
		let content = content.clone();
		let char_count = char_count.clone();

		move |event: web_sys::Event| {
			if let Some(target) = event.target() {
				if let Ok(textarea) = target.dyn_into::<HtmlTextAreaElement>() {
					let value = textarea.value();
					char_count.set(value.len());
					content.set(value);
				}
			}
		}
	};

	#[cfg(not(target_arch = "wasm32"))]
	let on_input = |_event: web_sys::Event| {};

	#[cfg(target_arch = "wasm32")]
	let on_submit = {
		let error = error.clone();
		let loading = loading.clone();
		let content = content.clone();
		let char_count = char_count.clone();

		move |event: web_sys::Event| {
			event.prevent_default();

			let error = error.clone();
			let loading = loading.clone();
			let content_value = content.get();
			let content = content.clone();
			let char_count = char_count.clone();

			spawn_local(async move {
				loading.set(true);
				error.set(None);

				let request = CreateTweetRequest {
					content: content_value,
				};

				match create_tweet(request).await {
					Ok(_) => {
						// Clear form
						content.set(String::new());
						char_count.set(0);
						loading.set(false);
						// Reload page to show new tweet
						if let Some(window) = web_sys::window() {
							let _ = window.location().reload();
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

/// Tweet list component
///
/// Displays list of tweets with loading and error states.
pub fn tweet_list(user_id: Option<Uuid>) -> View {
	let tweets = Signal::new(Vec::<TweetInfo>::new());
	let loading = Signal::new(true);
	let error = Signal::new(None::<String>);

	#[cfg(target_arch = "wasm32")]
	{
		let tweets = tweets.clone();
		let loading = loading.clone();
		let error = error.clone();

		spawn_local(async move {
			loading.set(true);
			error.set(None);

			match list_tweets(user_id, 0).await {
				Ok(tweet_list) => {
					tweets.set(tweet_list);
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
		.child(if loading.get() {
			// Loading state
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
		} else {
			// Tweet list
			let tweet_list = tweets.get();
			if tweet_list.is_empty() {
				ElementView::new("div")
					.attr("class", "text-center py-5")
					.child(
						ElementView::new("p")
							.attr("class", "text-muted")
							.child("No tweets yet. Be the first to post!"),
					)
					.into_view()
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
