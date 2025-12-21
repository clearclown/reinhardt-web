//! Layout components
//!
//! Provides layout components for the Twitter clone application:
//! - `NavItem` - Navigation item structure
//! - `header` - Top navigation bar
//! - `sidebar` - Side panel with trending topics and suggested users
//! - `footer` - Footer component
//! - `main_layout` - Main layout wrapper with header, sidebar, and content
//!
//! ## Design Note
//!
//! These components follow Bootstrap 5 styling conventions and use
//! reinhardt-pages components for SPA navigation.

use crate::shared::types::UserInfo;
use reinhardt_pages::component::{Component, ElementView, IntoView, View};
use reinhardt_pages::router::Link;

/// Navigation item for the header menu
#[derive(Debug, Clone)]
pub struct NavItem {
	/// Display label
	pub label: String,
	/// URL path
	pub href: String,
	/// Whether this item is currently active
	pub active: bool,
}

impl NavItem {
	/// Create a new navigation item
	pub fn new(label: impl Into<String>, href: impl Into<String>) -> Self {
		Self {
			label: label.into(),
			href: href.into(),
			active: false,
		}
	}

	/// Set whether this item is active
	pub fn active(mut self, active: bool) -> Self {
		self.active = active;
		self
	}
}

/// Trending topic for sidebar display
#[derive(Debug, Clone)]
pub struct TrendingTopic {
	/// Topic name (e.g., "#RustLang")
	pub name: String,
	/// Number of tweets
	pub tweet_count: u64,
}

/// Suggested user for sidebar display
#[derive(Debug, Clone)]
pub struct SuggestedUser {
	/// User ID
	pub id: uuid::Uuid,
	/// Username
	pub username: String,
	/// User bio (short)
	pub bio: Option<String>,
}

/// Header component
///
/// Displays the top navigation bar with site branding and user menu.
///
/// # Arguments
///
/// * `site_name` - Site/app name to display
/// * `current_user` - Currently logged in user (None if not authenticated)
/// * `nav_items` - Navigation menu items
pub fn header(site_name: &str, current_user: Option<&UserInfo>, nav_items: &[NavItem]) -> View {
	let nav_links: Vec<View> = nav_items
		.iter()
		.map(|item| {
			let class = if item.active {
				"nav-link active"
			} else {
				"nav-link"
			};

			ElementView::new("li")
				.attr("class", "nav-item")
				.child(
					Link::new(item.href.clone(), item.label.clone())
						.class(class)
						.render(),
				)
				.into_view()
		})
		.collect();

	let user_menu = if let Some(user) = current_user {
		ElementView::new("div")
			.attr("class", "d-flex align-items-center")
			.child(
				ElementView::new("span")
					.attr("class", "navbar-text me-3")
					.child(format!("@{}", user.username)),
			)
			.child(
				Link::new(format!("/profile/{}", user.id), "Profile".to_string())
					.class("btn btn-outline-light btn-sm me-2")
					.render(),
			)
			.child(
				ElementView::new("a")
					.attr("href", "/logout")
					.attr("class", "btn btn-outline-light btn-sm")
					.child("Logout"),
			)
			.into_view()
	} else {
		ElementView::new("div")
			.attr("class", "d-flex")
			.child(
				Link::new("/login".to_string(), "Login".to_string())
					.class("btn btn-outline-light btn-sm me-2")
					.render(),
			)
			.child(
				Link::new("/register".to_string(), "Register".to_string())
					.class("btn btn-light btn-sm")
					.render(),
			)
			.into_view()
	};

	ElementView::new("nav")
		.attr("class", "navbar navbar-expand-lg navbar-dark bg-primary")
		.child(
			ElementView::new("div")
				.attr("class", "container-fluid")
				.child(
					Link::new("/".to_string(), site_name.to_string())
						.class("navbar-brand")
						.render(),
				)
				.child(
					ElementView::new("button")
						.attr("class", "navbar-toggler")
						.attr("type", "button")
						.attr("data-bs-toggle", "collapse")
						.attr("data-bs-target", "#navbarNav")
						.attr("aria-controls", "navbarNav")
						.attr("aria-expanded", "false")
						.attr("aria-label", "Toggle navigation")
						.child(ElementView::new("span").attr("class", "navbar-toggler-icon")),
				)
				.child(
					ElementView::new("div")
						.attr("class", "collapse navbar-collapse")
						.attr("id", "navbarNav")
						.child(
							ElementView::new("ul")
								.attr("class", "navbar-nav me-auto")
								.children(nav_links),
						)
						.child(user_menu),
				),
		)
		.into_view()
}

/// Sidebar component
///
/// Displays trending topics and suggested users in a side panel.
///
/// # Arguments
///
/// * `trending_topics` - List of trending topics
/// * `suggested_users` - List of suggested users to follow
pub fn sidebar(trending_topics: &[TrendingTopic], suggested_users: &[SuggestedUser]) -> View {
	let topics_list: Vec<View> = trending_topics
		.iter()
		.map(|topic| {
			ElementView::new("a")
				.attr("href", &format!("/search?q={}", topic.name))
				.attr("class", "list-group-item list-group-item-action")
				.child(
					ElementView::new("div")
						.attr("class", "d-flex justify-content-between")
						.child(ElementView::new("strong").child(&topic.name))
						.child(
							ElementView::new("small")
								.attr("class", "text-muted")
								.child(format!("{} tweets", topic.tweet_count)),
						),
				)
				.into_view()
		})
		.collect();

	let users_list: Vec<View> = suggested_users
		.iter()
		.map(|user| {
			ElementView::new("div")
				.attr("class", "list-group-item")
				.child(
					ElementView::new("div")
						.attr("class", "d-flex justify-content-between align-items-center")
						.child(
							ElementView::new("div")
								.child(
									Link::new(
										format!("/profile/{}", user.id),
										format!("@{}", user.username),
									)
									.class("fw-bold text-decoration-none")
									.render(),
								)
								.child(if let Some(bio) = &user.bio {
									ElementView::new("small")
										.attr("class", "text-muted d-block")
										.child(bio.clone())
										.into_view()
								} else {
									ElementView::new("span").into_view()
								}),
						)
						.child(
							ElementView::new("button")
								.attr("class", "btn btn-outline-primary btn-sm")
								.child("Follow"),
						),
				)
				.into_view()
		})
		.collect();

	ElementView::new("div")
		.attr("class", "sidebar")
		.attr("style", "position: sticky; top: 80px;")
		// Trending Topics section
		.child(
			ElementView::new("div")
				.attr("class", "card mb-4")
				.child(
					ElementView::new("div")
						.attr("class", "card-header")
						.child(ElementView::new("h6").attr("class", "mb-0").child("Trending")),
				)
				.child(
					ElementView::new("div")
						.attr("class", "list-group list-group-flush")
						.children(if topics_list.is_empty() {
							vec![ElementView::new("div")
								.attr("class", "list-group-item text-muted")
								.child("No trending topics")
								.into_view()]
						} else {
							topics_list
						}),
				),
		)
		// Suggested Users section
		.child(
			ElementView::new("div")
				.attr("class", "card")
				.child(
					ElementView::new("div")
						.attr("class", "card-header")
						.child(
							ElementView::new("h6")
								.attr("class", "mb-0")
								.child("Who to follow"),
						),
				)
				.child(
					ElementView::new("div")
						.attr("class", "list-group list-group-flush")
						.children(if users_list.is_empty() {
							vec![ElementView::new("div")
								.attr("class", "list-group-item text-muted")
								.child("No suggestions")
								.into_view()]
						} else {
							users_list
						}),
				),
		)
		.into_view()
}

/// Footer component
///
/// Displays the footer with copyright and version information.
///
/// # Arguments
///
/// * `version` - Application version string
pub fn footer(version: &str) -> View {
	ElementView::new("footer")
		.attr("class", "bg-light text-center py-3 mt-auto border-top")
		.child(
			ElementView::new("div").attr("class", "container").child(
				ElementView::new("span")
					.attr("class", "text-muted")
					.child(format!("Twitter Clone v{} - Built with Reinhardt", version)),
			),
		)
		.into_view()
}

/// Main layout wrapper
///
/// Wraps content with header, optional sidebar, and footer.
/// This layout is responsive and adjusts for different screen sizes.
///
/// # Arguments
///
/// * `site_name` - Site/app name
/// * `current_user` - Currently logged in user
/// * `nav_items` - Navigation menu items
/// * `content` - Main content to display
/// * `show_sidebar` - Whether to show the sidebar
/// * `version` - Application version
pub fn main_layout(
	site_name: &str,
	current_user: Option<&UserInfo>,
	nav_items: &[NavItem],
	content: View,
	show_sidebar: bool,
	version: &str,
) -> View {
	let main_content = if show_sidebar {
		ElementView::new("div")
			.attr("class", "row")
			.child(
				ElementView::new("div")
					.attr("class", "col-lg-8")
					.child(content),
			)
			.child(
				ElementView::new("div")
					.attr("class", "col-lg-4")
					.child(sidebar(&[], &[])),
			)
			.into_view()
	} else {
		ElementView::new("div")
			.attr("class", "row justify-content-center")
			.child(
				ElementView::new("div")
					.attr("class", "col-lg-8")
					.child(content),
			)
			.into_view()
	};

	ElementView::new("div")
		.attr("class", "d-flex flex-column min-vh-100")
		.child(header(site_name, current_user, nav_items))
		.child(
			ElementView::new("main").attr("class", "flex-grow-1").child(
				ElementView::new("div")
					.attr("class", "container py-4")
					.child(main_content),
			),
		)
		.child(footer(version))
		.into_view()
}

/// Simple page layout without sidebar
///
/// A simplified layout for pages like login/register that don't need sidebar.
pub fn simple_layout(site_name: &str, nav_items: &[NavItem], content: View, version: &str) -> View {
	main_layout(site_name, None, nav_items, content, false, version)
}
