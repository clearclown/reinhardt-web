//! Relationship components (follow/block)
//!
//! Provides follow button and user list components for managing user relationships.

use crate::shared::types::UserInfo;
use reinhardt_pages::Signal;
use reinhardt_pages::component::{ElementView, IntoView, View};
use uuid::Uuid;

#[cfg(target_arch = "wasm32")]
use {
	crate::server::server_fn::relationship::{
		fetch_followers, fetch_following, follow_user, unfollow_user,
	},
	wasm_bindgen_futures::spawn_local,
};

/// Type of user list to display
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UserListType {
	/// List of followers
	Followers,
	/// List of users being followed
	Following,
}

/// Follow button component
///
/// Provides a button to follow/unfollow a user with state management.
pub fn follow_button(target_user_id: Uuid, is_following_initial: bool) -> View {
	let is_following = Signal::new(is_following_initial);
	let loading = Signal::new(false);
	let error = Signal::new(None::<String>);

	#[cfg(target_arch = "wasm32")]
	let on_click = {
		let is_following = is_following.clone();
		let loading = loading.clone();
		let error = error.clone();

		move |_event: web_sys::Event| {
			let is_following = is_following.clone();
			let loading = loading.clone();
			let error = error.clone();
			let currently_following = is_following.get();

			spawn_local(async move {
				loading.set(true);
				error.set(None);

				let result = if currently_following {
					unfollow_user(target_user_id).await
				} else {
					follow_user(target_user_id).await
				};

				match result {
					Ok(()) => {
						is_following.set(!currently_following);
						loading.set(false);
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
	let on_click = |_event: web_sys::Event| {};

	ElementView::new("div")
		.child(
			ElementView::new("button")
				.attr("type", "button")
				.attr(
					"class",
					if loading.get() {
						"btn btn-secondary disabled"
					} else if is_following.get() {
						"btn btn-outline-primary"
					} else {
						"btn btn-primary"
					},
				)
				.listener("click", on_click)
				.child(if loading.get() {
					"Processing..."
				} else if is_following.get() {
					"Unfollow"
				} else {
					"Follow"
				}),
		)
		.child(if let Some(err) = error.get() {
			ElementView::new("div")
				.attr("class", "alert alert-danger mt-2")
				.child(err)
				.into_view()
		} else {
			ElementView::new("div").into_view()
		})
		.into_view()
}

/// User card component
///
/// Displays a single user in a list.
fn user_card(user: &UserInfo) -> View {
	ElementView::new("div")
		.attr("class", "card mb-2")
		.child(
			ElementView::new("div").attr("class", "card-body").child(
				ElementView::new("div")
					.attr("class", "d-flex justify-content-between align-items-center")
					.child(
						ElementView::new("div")
							.child(
								ElementView::new("h6")
									.attr("class", "card-subtitle mb-1")
									.child(format!("@{}", user.username)),
							)
							.child(
								ElementView::new("small")
									.attr("class", "text-muted")
									.child(&user.email),
							),
					)
					.child(
						ElementView::new("a")
							.attr("href", &format!("/profile/{}", user.id))
							.attr("class", "btn btn-sm btn-outline-primary")
							.child("View Profile"),
					),
			),
		)
		.into_view()
}

/// User list component
///
/// Displays a list of users (followers or following) with loading and error states.
pub fn user_list(user_id: Uuid, list_type: UserListType) -> View {
	let users = Signal::new(Vec::<UserInfo>::new());
	let loading = Signal::new(true);
	let error = Signal::new(None::<String>);

	#[cfg(target_arch = "wasm32")]
	{
		let users = users.clone();
		let loading = loading.clone();
		let error = error.clone();

		spawn_local(async move {
			loading.set(true);
			error.set(None);

			let result = match list_type {
				UserListType::Followers => fetch_followers(user_id).await,
				UserListType::Following => fetch_following(user_id).await,
			};

			match result {
				Ok(user_list) => {
					users.set(user_list);
					loading.set(false);
				}
				Err(e) => {
					error.set(Some(e.to_string()));
					loading.set(false);
				}
			}
		});
	}

	let title = match list_type {
		UserListType::Followers => "Followers",
		UserListType::Following => "Following",
	};

	ElementView::new("div")
		.child(ElementView::new("h3").attr("class", "mb-4").child(title))
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
			// User list
			let user_list = users.get();
			if user_list.is_empty() {
				ElementView::new("div")
					.attr("class", "text-center py-5")
					.child(ElementView::new("p").attr("class", "text-muted").child(
						match list_type {
							UserListType::Followers => "No followers yet.",
							UserListType::Following => "Not following anyone yet.",
						},
					))
					.into_view()
			} else {
				ElementView::new("div")
					.children(
						user_list
							.iter()
							.map(|user| user_card(user))
							.collect::<Vec<_>>(),
					)
					.into_view()
			}
		})
		.into_view()
}
