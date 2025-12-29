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
//! the `page!` macro for JSX-like syntax with SPA navigation.

use crate::shared::types::UserInfo;
use reinhardt_pages::component::{Component, ElementView, IntoView, View};
use reinhardt_pages::page;
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
	// Generate navigation links as Views
	let nav_links: Vec<View> = nav_items
		.iter()
		.map(|item| {
			let class = if item.active {
				"nav-link active"
			} else {
				"nav-link"
			};
			let link_view = Link::new(item.href.clone(), item.label.clone())
				.class(class)
				.render();

			page!(|link_view: View| {
				li {
					class: "nav-item",
					{ link_view }
				}
			})(link_view)
		})
		.collect();
	let nav_links_view = View::fragment(nav_links);

	// Generate brand link
	let brand_link = Link::new("/".to_string(), site_name.to_string())
		.class("navbar-brand")
		.render();

	// Generate user menu based on authentication state
	let user_menu = if let Some(user) = current_user {
		let username = format!("@{}", user.username);
		let profile_link = Link::new(format!("/profile/{}", user.id), "Profile".to_string())
			.class("btn btn-outline-light btn-sm me-2")
			.render();

		page!(|username: String, profile_link: View| {
			div {
				class: "d-flex align-items-center",
				span {
					class: "navbar-text me-3",
					{ username }
				}
				{ profile_link }
				a {
					href: "/logout",
					class: "btn btn-outline-light btn-sm",
					"Logout"
				}
			}
		})(username, profile_link)
	} else {
		let login_link = Link::new("/login".to_string(), "Login".to_string())
			.class("btn btn-outline-light btn-sm me-2")
			.render();
		let register_link = Link::new("/register".to_string(), "Register".to_string())
			.class("btn btn-light btn-sm")
			.render();

		page!(|login_link: View, register_link: View| {
			div {
				class: "d-flex",
				{ login_link }
				{ register_link }
			}
		})(login_link, register_link)
	};

	page!(|brand_link: View, nav_links_view: View, user_menu: View| {
		nav {
			class: "navbar navbar-expand-lg navbar-dark bg-primary",
			div {
				class: "container-fluid",
				{ brand_link }
				button {
					class: "navbar-toggler",
					r#type: "button",
					data_bs_toggle: "collapse",
					data_bs_target: "#navbarNav",
					aria_controls: "navbarNav",
					aria_expanded: "false",
					aria_label: "Toggle navigation",
					span {
						class: "navbar-toggler-icon",
					}
				}
				div {
					class: "collapse navbar-collapse",
					id: "navbarNav",
					ul {
						class: "navbar-nav me-auto",
						{ nav_links_view }
					}
					{ user_menu }
				}
			}
		}
	})(brand_link, nav_links_view, user_menu)
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
	// Generate trending topics list as Views
	let topics_list: Vec<View> = trending_topics
		.iter()
		.map(|topic| {
			let href = format!("/search?q={}", topic.name);
			let name = topic.name.clone();
			let tweets_text = format!("{} tweets", topic.tweet_count);

			page!(|href: String, name: String, tweets_text: String| {
				a {
					href: href,
					class: "list-group-item list-group-item-action",
					div {
						class: "d-flex justify-content-between",
						strong {
							{ name }
						}
						small {
							class: "text-muted",
							{ tweets_text }
						}
					}
				}
			})(href, name, tweets_text)
		})
		.collect();

	let topics_empty = topics_list.is_empty();
	let topics_view = if topics_empty {
		page!(|| {
			div {
				class: "list-group-item text-muted",
				"No trending topics"
			}
		})()
	} else {
		View::fragment(topics_list)
	};

	// Generate suggested users list as Views
	let users_list: Vec<View> = suggested_users
		.iter()
		.map(|user| {
			let profile_link = Link::new(
				format!("/profile/{}", user.id),
				format!("@{}", user.username),
			)
			.class("fw-bold text-decoration-none")
			.render();

			let has_bio = user.bio.is_some();
			let bio_text = user.bio.clone().unwrap_or_default();

			page!(|profile_link: View, has_bio: bool, bio_text: String| {
				div {
					class: "list-group-item",
					div {
						class: "d-flex justify-content-between align-items-center",
						div {
							{ profile_link }
							if has_bio {
								small {
									class: "text-muted d-block",
									{ bio_text }
								}
							}
						}
						button {
							class: "btn btn-outline-primary btn-sm",
							"Follow"
						}
					}
				}
			})(profile_link, has_bio, bio_text)
		})
		.collect();

	let users_empty = users_list.is_empty();
	let users_view = if users_empty {
		page!(|| {
			div {
				class: "list-group-item text-muted",
				"No suggestions"
			}
		})()
	} else {
		View::fragment(users_list)
	};

	page!(|topics_view: View, users_view: View| {
		div {
			class: "sidebar",
			style: "position: sticky; top: 80px;",
			div {
				class: "card mb-4",
				div {
					class: "card-header",
					h6 {
						class: "mb-0",
						"Trending"
					}
				}
				div {
					class: "list-group list-group-flush",
					{ topics_view }
				}
			}
			div {
				class: "card",
				div {
					class: "card-header",
					h6 {
						class: "mb-0",
						"Who to follow"
					}
				}
				div {
					class: "list-group list-group-flush",
					{ users_view }
				}
			}
		}
	})(topics_view, users_view)
}

/// Footer component
///
/// Displays the footer with copyright and version information.
///
/// # Arguments
///
/// * `version` - Application version string
pub fn footer(version: &str) -> View {
	let version = version.to_string();
	page!(|version: String| {
		footer {
			class: "bg-light text-center py-3 mt-auto border-top",
			div {
				class: "container",
				span {
					class: "text-muted",
					{ format!("Twitter Clone v{} - Built with Reinhardt", version) }
				}
			}
		}
	})(version)
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
	// Build component views
	let header_view = header(site_name, current_user, nav_items);
	let footer_view = footer(version);

	// Build main content with conditional sidebar
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

	page!(|header_view: View, main_content: View, footer_view: View| {
		div {
			class: "d-flex flex-column min-vh-100",
			{ header_view }
			main {
				class: "flex-grow-1",
				div {
					class: "container py-4",
					{ main_content }
				}
			}
			{ footer_view }
		}
	})(header_view, main_content, footer_view)
}

/// Simple page layout without sidebar
///
/// A simplified layout for pages like login/register that don't need sidebar.
pub fn simple_layout(site_name: &str, nav_items: &[NavItem], content: View, version: &str) -> View {
	main_layout(site_name, None, nav_items, content, false, version)
}
