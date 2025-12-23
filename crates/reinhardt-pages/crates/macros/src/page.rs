//! The page! macro implementation.
//!
//! This module provides the `page!` procedural macro for creating anonymous
//! WASM components with a concise, ergonomic DSL.
//!
//! ## Example
//!
//! ```ignore
//! use reinhardt_pages::page;
//!
//! // Define an anonymous component
//! let counter = page!(|initial: i32| {
//!     div {
//!         class: "counter",
//!         h1 { "Counter" }
//!         span { format!("Count: {}", initial) }
//!         button {
//!             @click: |_| { /* increment logic */ },
//!             "+"
//!         }
//!     }
//! });
//!
//! // Use it like a function
//! let view = counter(42);
//! ```

mod codegen;

use proc_macro::TokenStream;

// Re-export PageMacro from the shared ast crate for external use
pub(crate) use reinhardt_pages_ast::PageMacro;

/// Implementation of the page! macro.
///
/// This function parses the input TokenStream into a PageMacro AST,
/// then generates the corresponding Rust code.
pub(crate) fn page_impl(input: TokenStream) -> TokenStream {
	let input2 = proc_macro2::TokenStream::from(input);

	// Parse the input into AST
	let macro_ast: PageMacro = match syn::parse2(input2) {
		Ok(ast) => ast,
		Err(err) => return err.to_compile_error().into(),
	};

	// Generate the output code
	let output = codegen::generate(&macro_ast);

	output.into()
}

#[cfg(test)]
mod tests {
	use super::*;
	use quote::quote;

	#[test]
	fn test_page_macro_basic() {
		let input = quote!(|| { div { "hello" } });
		let ast: PageMacro = syn::parse2(input).unwrap();
		let output = codegen::generate(&ast);

		// Verify it generates valid tokens
		assert!(!output.is_empty());
	}

	#[test]
	fn test_page_macro_with_params() {
		let input = quote!(|name: String, count: i32| {
			div {
				class: "greeting",
				span { name }
				span { count.to_string() }
			}
		});
		let ast: PageMacro = syn::parse2(input).unwrap();
		let output = codegen::generate(&ast);

		let output_str = output.to_string();
		assert!(output_str.contains("name : String"));
		assert!(output_str.contains("count : i32"));
	}

	#[test]
	fn test_page_macro_with_events() {
		let input = quote!(|| {
			button {
				@click: |e| { handle_click(e); },
				@input: |e| { handle_input(e); },
				"Click me"
			}
		});
		let ast: PageMacro = syn::parse2(input).unwrap();
		let output = codegen::generate(&ast);

		let output_str = output.to_string();
		assert!(output_str.contains("EventType"));
		assert!(output_str.contains("Click"));
	}

	#[test]
	fn test_page_macro_nested_elements() {
		let input = quote!(|| {
			div {
				class: "container",
				header {
					h1 { "Title" }
				}
				main {
					p { "Content" }
				}
				footer {
					span { "Footer" }
				}
			}
		});
		let ast: PageMacro = syn::parse2(input).unwrap();
		let output = codegen::generate(&ast);

		let output_str = output.to_string();
		assert!(output_str.contains("\"div\""));
		assert!(output_str.contains("\"header\""));
		assert!(output_str.contains("\"main\""));
		assert!(output_str.contains("\"footer\""));
	}
}
