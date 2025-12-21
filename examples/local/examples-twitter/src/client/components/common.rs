//! Common UI components
//!
//! Provides reusable UI components for the Twitter clone application:
//! - `ButtonVariant` - Button style variants
//! - `button` - Styled button with click handler
//! - `loading_spinner` - Loading indicator
//! - `error_alert` - Error message display
//! - `success_alert` - Success message display
//! - `text_input` - Form text input
//! - `textarea` - Form textarea with character count
//! - `avatar` - User avatar image
//!
//! ## Design Note
//!
//! These components use ElementView for SSR compatibility.
//! Interactive components with event handlers are hydrated on the client side.

use reinhardt_pages::Signal;
use reinhardt_pages::component::{ElementView, IntoView, View};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

/// Button variant styles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
	/// Primary action button (blue)
	Primary,
	/// Secondary action button (gray)
	Secondary,
	/// Success action button (green)
	Success,
	/// Danger action button (red)
	Danger,
	/// Warning action button (yellow)
	Warning,
	/// Link style button (no background)
	Link,
	/// Outline primary button
	OutlinePrimary,
}

impl ButtonVariant {
	/// Get Bootstrap CSS class for this variant
	pub fn class(&self) -> &'static str {
		match self {
			ButtonVariant::Primary => "btn btn-primary",
			ButtonVariant::Secondary => "btn btn-secondary",
			ButtonVariant::Success => "btn btn-success",
			ButtonVariant::Danger => "btn btn-danger",
			ButtonVariant::Warning => "btn btn-warning",
			ButtonVariant::Link => "btn btn-link",
			ButtonVariant::OutlinePrimary => "btn btn-outline-primary",
		}
	}
}

/// Button component
///
/// Displays a styled button with various variants.
/// When clicked, sets the provided Signal to true.
///
/// # Arguments
///
/// * `text` - Button label text
/// * `variant` - Visual style variant
/// * `disabled` - Whether the button is disabled
/// * `on_click` - Signal that will be set to true when clicked
pub fn button(text: &str, variant: ButtonVariant, disabled: bool, on_click: Signal<bool>) -> View {
	let class = if disabled {
		format!("{} disabled", variant.class())
	} else {
		variant.class().to_string()
	};

	#[cfg(target_arch = "wasm32")]
	let button_view = {
		let on_click = on_click.clone();
		ElementView::new("button")
			.attr("class", &class)
			.attr("type", "button")
			.attr("disabled", if disabled { "true" } else { "" })
			.listener("click", move |_event: web_sys::Event| {
				on_click.set(true);
			})
			.child(text.to_string())
	};

	#[cfg(not(target_arch = "wasm32"))]
	let button_view = {
		let _ = on_click; // Suppress unused warning
		ElementView::new("button")
			.attr("class", &class)
			.attr("type", "button")
			.attr("disabled", if disabled { "true" } else { "" })
			.attr("data-reactive", "true")
			.child(text.to_string())
	};

	button_view.into_view()
}

/// Loading spinner component
///
/// Displays a Bootstrap spinner animation while content is loading.
pub fn loading_spinner() -> View {
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
}

/// Error alert component
///
/// Displays an error message in a styled alert box.
///
/// # Arguments
///
/// * `message` - Error message to display
/// * `dismissible` - Whether the alert can be dismissed
pub fn error_alert(message: &str, dismissible: bool) -> View {
	let class = if dismissible {
		"alert alert-danger alert-dismissible fade show"
	} else {
		"alert alert-danger"
	};

	let mut container = ElementView::new("div")
		.attr("class", class)
		.attr("role", "alert")
		.child(message.to_string());

	if dismissible {
		container = container.child(
			ElementView::new("button")
				.attr("type", "button")
				.attr("class", "btn-close")
				.attr("data-bs-dismiss", "alert")
				.attr("aria-label", "Close"),
		);
	}

	container.into_view()
}

/// Success alert component
///
/// Displays a success message in a styled alert box.
///
/// # Arguments
///
/// * `message` - Success message to display
pub fn success_alert(message: &str) -> View {
	ElementView::new("div")
		.attr("class", "alert alert-success")
		.attr("role", "alert")
		.child(message.to_string())
		.into_view()
}

/// Text input component
///
/// Displays a labeled text input field.
///
/// # Arguments
///
/// * `id` - Input element ID
/// * `label` - Label text
/// * `placeholder` - Placeholder text
/// * `input_type` - HTML input type (e.g., "text", "email", "password")
/// * `value` - Reactive value signal
/// * `required` - Whether the field is required
pub fn text_input(
	id: &str,
	label: &str,
	placeholder: &str,
	input_type: &str,
	value: Signal<String>,
	required: bool,
) -> View {
	let current_value = value.get();

	#[cfg(target_arch = "wasm32")]
	let input = {
		let value = value.clone();
		ElementView::new("input")
			.attr("type", input_type)
			.attr("class", "form-control")
			.attr("id", id)
			.attr("name", id)
			.attr("placeholder", placeholder)
			.attr("value", &current_value)
			.attr("required", if required { "true" } else { "" })
			.listener("input", move |event: web_sys::Event| {
				if let Some(target) = event.target() {
					if let Ok(input) = target.dyn_into::<web_sys::HtmlInputElement>() {
						value.set(input.value());
					}
				}
			})
	};

	#[cfg(not(target_arch = "wasm32"))]
	let input = {
		let _ = value; // Suppress unused warning
		ElementView::new("input")
			.attr("type", input_type)
			.attr("class", "form-control")
			.attr("id", id)
			.attr("name", id)
			.attr("placeholder", placeholder)
			.attr("value", &current_value)
			.attr("required", if required { "true" } else { "" })
			.attr("data-reactive", "true")
	};

	ElementView::new("div")
		.attr("class", "mb-3")
		.child(
			ElementView::new("label")
				.attr("for", id)
				.attr("class", "form-label")
				.child(label.to_string()),
		)
		.child(input)
		.into_view()
}

/// Textarea component with character count
///
/// Displays a labeled textarea with optional character limit display.
///
/// # Arguments
///
/// * `id` - Textarea element ID
/// * `label` - Label text
/// * `placeholder` - Placeholder text
/// * `rows` - Number of visible rows
/// * `max_length` - Maximum character length (0 for no limit)
/// * `value` - Reactive value signal
pub fn textarea(
	id: &str,
	label: &str,
	placeholder: &str,
	rows: u32,
	max_length: usize,
	value: Signal<String>,
) -> View {
	let current_value = value.get();
	let char_count = current_value.len();

	#[cfg(target_arch = "wasm32")]
	let textarea_elem = {
		let value = value.clone();
		let mut elem = ElementView::new("textarea")
			.attr("class", "form-control")
			.attr("id", id)
			.attr("name", id)
			.attr("rows", &rows.to_string())
			.attr("placeholder", placeholder)
			.child(&current_value);

		if max_length > 0 {
			elem = elem.attr("maxlength", &max_length.to_string());
		}

		elem.listener("input", move |event: web_sys::Event| {
			if let Some(target) = event.target() {
				if let Ok(textarea) = target.dyn_into::<web_sys::HtmlTextAreaElement>() {
					value.set(textarea.value());
				}
			}
		})
	};

	#[cfg(not(target_arch = "wasm32"))]
	let textarea_elem = {
		let _ = value; // Suppress unused warning
		let mut elem = ElementView::new("textarea")
			.attr("class", "form-control")
			.attr("id", id)
			.attr("name", id)
			.attr("rows", &rows.to_string())
			.attr("placeholder", placeholder)
			.attr("data-reactive", "true")
			.child(&current_value);

		if max_length > 0 {
			elem = elem.attr("maxlength", &max_length.to_string());
		}

		elem
	};

	let mut container = ElementView::new("div")
		.attr("class", "mb-3")
		.child(
			ElementView::new("label")
				.attr("for", id)
				.attr("class", "form-label")
				.child(label.to_string()),
		)
		.child(textarea_elem);

	// Add character count if max_length is set
	if max_length > 0 {
		let count_class = if char_count > max_length {
			"text-danger"
		} else if char_count > max_length * 9 / 10 {
			"text-warning"
		} else {
			"text-muted"
		};

		container = container.child(
			ElementView::new("small")
				.attr("class", count_class)
				.child(format!("{}/{}", char_count, max_length)),
		);
	}

	container.into_view()
}

/// Avatar component
///
/// Displays a user avatar image with fallback.
///
/// # Arguments
///
/// * `url` - Avatar image URL (None for default avatar)
/// * `alt` - Alt text for the image
/// * `size` - Size in pixels
pub fn avatar(url: Option<&str>, alt: &str, size: u32) -> View {
	let src = url.unwrap_or("https://via.placeholder.com/150?text=User");
	let size_str = format!("{}px", size);

	ElementView::new("img")
		.attr("src", src)
		.attr("alt", alt)
		.attr("class", "rounded-circle")
		.attr("width", &size_str)
		.attr("height", &size_str)
		.attr("style", "object-fit: cover;")
		.into_view()
}

/// Empty placeholder component
///
/// Displays an empty div (useful for conditional rendering).
pub fn empty() -> View {
	ElementView::new("div").into_view()
}
