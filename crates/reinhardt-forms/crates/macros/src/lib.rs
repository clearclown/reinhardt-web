//! Procedural macros for reinhardt-forms.
//!
//! This crate provides the `form!` macro for declaratively defining forms.
//!
//! ## Example
//!
//! ```ignore
//! use reinhardt_forms::form;
//!
//! let login_form = form! {
//!     fields: {
//!         username: CharField {
//!             required,
//!             max_length: 150,
//!         },
//!         password: CharField {
//!             required,
//!             widget: PasswordInput,
//!         },
//!     },
//! };
//! ```

use proc_macro::TokenStream;

/// Creates a form with the declarative DSL.
///
/// # Syntax
///
/// ```ignore
/// form! {
///     name: "form_name",           // Optional
///     csrf: true,                   // Optional, default false
///     fields: {                     // Required
///         field_name: FieldType {
///             property: value,
///         },
///     },
///     validators: {                 // Optional
///         field_name: [
///             |v| expr => "error message",
///         ],
///         @form: [
///             |data| expr => "error message",
///         ],
///     },
///     client_validators: {          // Optional
///         field_name: [
///             "js_expr" => "error message",
///         ],
///     },
/// }
/// ```
///
/// # Field Types
///
/// - `CharField` - Text input
/// - `EmailField` - Email input with validation
/// - `IntegerField` - Integer input
/// - `FloatField` - Float input
/// - `BooleanField` - Checkbox
/// - `DateField` - Date picker
/// - `DateTimeField` - Date and time picker
/// - `TimeField` - Time picker
/// - `ChoiceField` - Select dropdown
/// - `FileField` - File upload
/// - `ImageField` - Image upload
/// - `PasswordField` - Password input
/// - And more...
///
/// # Field Properties
///
/// - `required` - Field is required (flag)
/// - `max_length: usize` - Maximum length constraint
/// - `min_length: usize` - Minimum length constraint
/// - `label: "text"` - Display label
/// - `help_text: "text"` - Help text
/// - `initial: value` - Initial value
/// - `widget: WidgetType` - Widget specification
#[proc_macro]
pub fn form(input: TokenStream) -> TokenStream {
	let _input = proc_macro2::TokenStream::from(input);

	// TODO: Implement form macro
	// 1. Parse input using reinhardt-forms-macros-ast
	// 2. Validate AST
	// 3. Generate code

	// Stub implementation: return empty struct for now
	let output = quote::quote! {
		compile_error!("form! macro is not yet implemented")
	};

	TokenStream::from(output)
}
