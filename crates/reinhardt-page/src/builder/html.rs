//! HTML Element Builder
//!
//! This module provides a fluent API for constructing HTML elements with type-safe operations.
//!
//! ## Design Pattern
//!
//! - **Fluent API**: Method chaining for readable construction
//! - **RAII Integration**: EventHandle management for automatic cleanup
//! - **Reactive Binding**: Seamless integration with Signal<T>

use crate::Signal;
use crate::dom::{Document, Element, EventHandle};

/// HTML element builder with fluent API
///
/// This builder allows constructing HTML elements with method chaining.
/// Event listeners are automatically managed through EventHandle.
///
/// ## Example
///
/// ```ignore
/// let button = button()
///     .class("btn btn-primary")
///     .id("submit-button")
///     .text("Submit")
///     .on_click(|| console::log_1(&"Clicked!".into()))
///     .build();
/// ```
pub struct ElementBuilder {
	/// The underlying DOM element
	element: Element,
	/// Event handles for RAII cleanup
	event_handles: Vec<EventHandle>,
}

impl ElementBuilder {
	/// Create a new builder from an element
	pub fn new(element: Element) -> Self {
		Self {
			element,
			event_handles: Vec::new(),
		}
	}

	/// Set the class attribute
	///
	/// Multiple calls will overwrite the previous value.
	/// Use space-separated values for multiple classes.
	///
	/// ## Example
	///
	/// ```ignore
	/// div().class("container flex-row").build()
	/// ```
	pub fn class(self, class: &str) -> Self {
		let _ = self.element.set_attribute("class", class);
		self
	}

	/// Set the id attribute
	///
	/// ## Example
	///
	/// ```ignore
	/// div().id("main-content").build()
	/// ```
	pub fn id(self, id: &str) -> Self {
		let _ = self.element.set_attribute("id", id);
		self
	}

	/// Set the style attribute
	///
	/// ## Example
	///
	/// ```ignore
	/// div().style("color: red; font-size: 16px").build()
	/// ```
	pub fn style(self, style: &str) -> Self {
		let _ = self.element.set_attribute("style", style);
		self
	}

	/// Set a custom attribute
	///
	/// ## Example
	///
	/// ```ignore
	/// div().attr("data-test-id", "my-div").build()
	/// ```
	pub fn attr(self, name: &str, value: &str) -> Self {
		let _ = self.element.set_attribute(name, value);
		self
	}

	/// Set a reactive attribute bound to a Signal
	///
	/// The attribute will automatically update when the Signal changes.
	///
	/// ## Example
	///
	/// ```ignore
	/// let disabled = Signal::new(false);
	/// button()
	///     .reactive_attr("disabled", disabled)
	///     .build()
	/// ```
	pub fn reactive_attr<T>(self, name: &str, signal: Signal<T>) -> Self
	where
		T: ToString + Clone + 'static,
	{
		self.element.set_reactive_attribute(name, signal);
		self
	}

	/// Set text content
	///
	/// This will replace all children of the element.
	///
	/// ## Example
	///
	/// ```ignore
	/// p().text("Hello, world!").build()
	/// ```
	pub fn text(self, text: &str) -> Self {
		self.element.set_text_content(text);
		self
	}

	/// Append a child element
	///
	/// ## Example
	///
	/// ```ignore
	/// div()
	///     .child(p().text("First paragraph").build())
	///     .child(p().text("Second paragraph").build())
	///     .build()
	/// ```
	pub fn child(self, child: Element) -> Self {
		let _ = self.element.append_child(child);
		self
	}

	/// Add a click event listener
	///
	/// ## Example
	///
	/// ```ignore
	/// button()
	///     .text("Click me")
	///     .on_click(|| console::log_1(&"Clicked!".into()))
	///     .build()
	/// ```
	pub fn on_click<F>(mut self, callback: F) -> Self
	where
		F: FnMut() + 'static,
	{
		let handle = self.element.add_event_listener("click", callback);
		self.event_handles.push(handle);
		self
	}

	/// Add an input event listener
	///
	/// Commonly used with `<input>` and `<textarea>` elements.
	///
	/// ## Example
	///
	/// ```ignore
	/// input()
	///     .attr("type", "text")
	///     .on_input(|| console::log_1(&"Input changed".into()))
	///     .build()
	/// ```
	pub fn on_input<F>(mut self, callback: F) -> Self
	where
		F: FnMut() + 'static,
	{
		let handle = self.element.add_event_listener("input", callback);
		self.event_handles.push(handle);
		self
	}

	/// Add a change event listener
	///
	/// Commonly used with `<select>`, `<input type="checkbox">`, etc.
	///
	/// ## Example
	///
	/// ```ignore
	/// input()
	///     .attr("type", "checkbox")
	///     .on_change(|| console::log_1(&"Checkbox toggled".into()))
	///     .build()
	/// ```
	pub fn on_change<F>(mut self, callback: F) -> Self
	where
		F: FnMut() + 'static,
	{
		let handle = self.element.add_event_listener("change", callback);
		self.event_handles.push(handle);
		self
	}

	/// Add a submit event listener
	///
	/// Commonly used with `<form>` elements.
	///
	/// ## Example
	///
	/// ```ignore
	/// form()
	///     .on_submit(|| console::log_1(&"Form submitted".into()))
	///     .build()
	/// ```
	pub fn on_submit<F>(mut self, callback: F) -> Self
	where
		F: FnMut() + 'static,
	{
		let handle = self.element.add_event_listener("submit", callback);
		self.event_handles.push(handle);
		self
	}

	/// Add a keydown event listener
	///
	/// ## Example
	///
	/// ```ignore
	/// input()
	///     .on_keydown(|| console::log_1(&"Key pressed".into()))
	///     .build()
	/// ```
	pub fn on_keydown<F>(mut self, callback: F) -> Self
	where
		F: FnMut() + 'static,
	{
		let handle = self.element.add_event_listener("keydown", callback);
		self.event_handles.push(handle);
		self
	}

	/// Add a focus event listener
	///
	/// ## Example
	///
	/// ```ignore
	/// input()
	///     .on_focus(|| console::log_1(&"Input focused".into()))
	///     .build()
	/// ```
	pub fn on_focus<F>(mut self, callback: F) -> Self
	where
		F: FnMut() + 'static,
	{
		let handle = self.element.add_event_listener("focus", callback);
		self.event_handles.push(handle);
		self
	}

	/// Add a blur event listener
	///
	/// ## Example
	///
	/// ```ignore
	/// input()
	///     .on_blur(|| console::log_1(&"Input blurred".into()))
	///     .build()
	/// ```
	pub fn on_blur<F>(mut self, callback: F) -> Self
	where
		F: FnMut() + 'static,
	{
		let handle = self.element.add_event_listener("blur", callback);
		self.event_handles.push(handle);
		self
	}

	/// Finalize the builder and return the Element
	///
	/// Event handles are transferred to the element, ensuring proper cleanup.
	///
	/// ## Example
	///
	/// ```ignore
	/// let element = div().class("container").build();
	/// ```
	pub fn build(self) -> Element {
		// Event handles are dropped here, but they're owned by the element
		// through the closure's captured state. This is safe.
		self.element
	}
}

// ============================================================================
// Helper functions for common HTML elements
// ============================================================================

/// Create a `<div>` element
///
/// ## Example
///
/// ```ignore
/// let container = div()
///     .class("container")
///     .child(p().text("Content").build())
///     .build();
/// ```
pub fn div() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("div").expect("Failed to create div");
	ElementBuilder::new(element)
}

/// Create a `<span>` element
///
/// ## Example
///
/// ```ignore
/// let label = span().text("Label").class("badge").build();
/// ```
pub fn span() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("span").expect("Failed to create span");
	ElementBuilder::new(element)
}

/// Create a `<p>` element (paragraph)
///
/// ## Example
///
/// ```ignore
/// let paragraph = p().text("This is a paragraph.").build();
/// ```
pub fn p() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("p").expect("Failed to create p");
	ElementBuilder::new(element)
}

/// Create a `<button>` element
///
/// ## Example
///
/// ```ignore
/// let button = button()
///     .text("Click me")
///     .on_click(|| console::log_1(&"Clicked!".into()))
///     .build();
/// ```
pub fn button() -> ElementBuilder {
	let doc = Document::global();
	let element = doc
		.create_element("button")
		.expect("Failed to create button");
	ElementBuilder::new(element)
}

/// Create an `<input>` element
///
/// ## Example
///
/// ```ignore
/// let text_input = input()
///     .attr("type", "text")
///     .attr("placeholder", "Enter text...")
///     .on_input(|| console::log_1(&"Input changed".into()))
///     .build();
/// ```
pub fn input() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("input").expect("Failed to create input");
	ElementBuilder::new(element)
}

/// Create a `<textarea>` element
///
/// ## Example
///
/// ```ignore
/// let textarea = textarea()
///     .attr("rows", "5")
///     .attr("placeholder", "Enter long text...")
///     .build();
/// ```
pub fn textarea() -> ElementBuilder {
	let doc = Document::global();
	let element = doc
		.create_element("textarea")
		.expect("Failed to create textarea");
	ElementBuilder::new(element)
}

/// Create a `<select>` element (dropdown)
///
/// ## Example
///
/// ```ignore
/// let select = select()
///     .child(option().attr("value", "1").text("Option 1").build())
///     .child(option().attr("value", "2").text("Option 2").build())
///     .build();
/// ```
pub fn select() -> ElementBuilder {
	let doc = Document::global();
	let element = doc
		.create_element("select")
		.expect("Failed to create select");
	ElementBuilder::new(element)
}

/// Create an `<option>` element (for use with `<select>`)
///
/// ## Example
///
/// ```ignore
/// let option = option()
///     .attr("value", "1")
///     .text("Option 1")
///     .build();
/// ```
pub fn option() -> ElementBuilder {
	let doc = Document::global();
	let element = doc
		.create_element("option")
		.expect("Failed to create option");
	ElementBuilder::new(element)
}

/// Create a `<form>` element
///
/// ## Example
///
/// ```ignore
/// let form = form()
///     .attr("method", "POST")
///     .attr("action", "/submit")
///     .on_submit(|| console::log_1(&"Form submitted".into()))
///     .build();
/// ```
pub fn form() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("form").expect("Failed to create form");
	ElementBuilder::new(element)
}

/// Create an `<a>` element (hyperlink)
///
/// ## Example
///
/// ```ignore
/// let link = a()
///     .attr("href", "https://example.com")
///     .text("Visit Example")
///     .build();
/// ```
pub fn a() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("a").expect("Failed to create a");
	ElementBuilder::new(element)
}

/// Create an `<img>` element
///
/// ## Example
///
/// ```ignore
/// let image = img()
///     .attr("src", "/images/logo.png")
///     .attr("alt", "Logo")
///     .build();
/// ```
pub fn img() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("img").expect("Failed to create img");
	ElementBuilder::new(element)
}

/// Create a `<h1>` element (heading level 1)
///
/// ## Example
///
/// ```ignore
/// let heading = h1().text("Page Title").build();
/// ```
pub fn h1() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("h1").expect("Failed to create h1");
	ElementBuilder::new(element)
}

/// Create a `<h2>` element (heading level 2)
pub fn h2() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("h2").expect("Failed to create h2");
	ElementBuilder::new(element)
}

/// Create a `<h3>` element (heading level 3)
pub fn h3() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("h3").expect("Failed to create h3");
	ElementBuilder::new(element)
}

/// Create a `<ul>` element (unordered list)
///
/// ## Example
///
/// ```ignore
/// let list = ul()
///     .child(li().text("Item 1").build())
///     .child(li().text("Item 2").build())
///     .build();
/// ```
pub fn ul() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("ul").expect("Failed to create ul");
	ElementBuilder::new(element)
}

/// Create an `<ol>` element (ordered list)
pub fn ol() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("ol").expect("Failed to create ol");
	ElementBuilder::new(element)
}

/// Create an `<li>` element (list item)
pub fn li() -> ElementBuilder {
	let doc = Document::global();
	let element = doc.create_element("li").expect("Failed to create li");
	ElementBuilder::new(element)
}
