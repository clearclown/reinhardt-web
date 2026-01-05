//! Untyped AST node definitions for the `form!` macro in reinhardt-pages.
//!
//! These structures represent the raw parse output before semantic validation.
//! The form! macro in reinhardt-pages is designed to work with both SSR (Server-Side Rendering)
//! and CSR (Client-Side Rendering/WASM) targets.
//!
//! ## DSL Structure
//!
//! ```text
//! form! {
//!     name: LoginForm,
//!     action: "/api/login",       // OR server_fn: submit_login
//!     method: Post,               // Optional, defaults to Post
//!     class: "form-container",    // Optional form-level styling
//!
//!     fields: {
//!         username: CharField {
//!             required,
//!             max_length: 150,
//!             label: "Username",
//!             placeholder: "Enter username",
//!             class: "input-field",
//!             wrapper_class: "field-group",
//!         },
//!         password: CharField {
//!             required,
//!             widget: PasswordInput,
//!             min_length: 8,
//!         },
//!     },
//!
//!     validators: {
//!         username: [
//!             |v| !v.trim().is_empty() => "Username cannot be empty",
//!         ],
//!     },
//!
//!     client_validators: {
//!         password: [
//!             "value.length >= 8" => "Password must be at least 8 characters",
//!         ],
//!     },
//! }
//! ```

use proc_macro2::Span;
use syn::{Expr, ExprClosure, Ident, LitStr, Path};

/// Top-level form macro AST.
///
/// Represents the entire `form! { ... }` invocation with support for
/// SSR and CSR rendering targets.
#[derive(Debug, Clone)]
pub struct FormMacro {
	/// Form struct name (required, e.g., `name: LoginForm`)
	pub name: Ident,
	/// Form action configuration
	pub action: FormAction,
	/// HTTP method (defaults to Post)
	pub method: Option<Ident>,
	/// Form-level CSS class
	pub class: Option<LitStr>,
	/// Field definitions
	pub fields: Vec<FormFieldDef>,
	/// Server-side validators
	pub validators: Vec<FormValidator>,
	/// Client-side validators (JavaScript expressions)
	pub client_validators: Vec<ClientValidator>,
	/// Span for error reporting
	pub span: Span,
}

/// Form action configuration.
///
/// Supports two modes:
/// - URL string: `action: "/api/login"`
/// - server_fn: `server_fn: submit_login`
#[derive(Debug, Clone)]
pub enum FormAction {
	/// URL action (traditional form submission)
	Url(LitStr),
	/// server_fn action (type-safe RPC)
	ServerFn(Path),
	/// No action specified (will be set programmatically)
	None,
}

/// A single field definition in the form macro.
///
/// Example:
/// ```ignore
/// username: CharField {
///     required,
///     max_length: 100,
///     label: "Username",
///     class: "input-field",
///     wrapper_class: "field-group",
/// }
/// ```
#[derive(Debug, Clone)]
pub struct FormFieldDef {
	/// Field name identifier
	pub name: Ident,
	/// Field type identifier (e.g., CharField, EmailField)
	pub field_type: Ident,
	/// Field properties (validation and styling)
	pub properties: Vec<FormFieldProperty>,
	/// Span for error reporting
	pub span: Span,
}

/// A property within a field definition.
#[derive(Debug, Clone)]
pub enum FormFieldProperty {
	/// Named property with a value: `max_length: 100`, `label: "Username"`
	Named {
		name: Ident,
		value: Expr,
		span: Span,
	},
	/// Flag property (boolean true): `required`
	Flag { name: Ident, span: Span },
	/// Widget specification: `widget: PasswordInput`
	Widget { widget_type: Ident, span: Span },
}

impl FormFieldProperty {
	/// Returns the property name for named properties and flags.
	///
	/// # Panics
	///
	/// Panics if called on a Widget property.
	pub fn name(&self) -> &Ident {
		match self {
			FormFieldProperty::Named { name, .. } => name,
			FormFieldProperty::Flag { name, .. } => name,
			FormFieldProperty::Widget { .. } => {
				panic!("Widget property has no direct name")
			}
		}
	}

	/// Returns the span for error reporting.
	pub fn span(&self) -> Span {
		match self {
			FormFieldProperty::Named { span, .. } => *span,
			FormFieldProperty::Flag { span, .. } => *span,
			FormFieldProperty::Widget { span, .. } => *span,
		}
	}

	/// Returns true if this is a styling property.
	pub fn is_styling(&self) -> bool {
		match self {
			FormFieldProperty::Named { name, .. } => {
				let name_str = name.to_string();
				matches!(
					name_str.as_str(),
					"class" | "wrapper_class" | "label_class" | "error_class"
				)
			}
			_ => false,
		}
	}
}

/// Server-side validator definition.
#[derive(Debug, Clone)]
pub enum FormValidator {
	/// Field-level validator: `username: [|v| ... => "error"]`
	Field {
		field_name: Ident,
		rules: Vec<ValidatorRule>,
		span: Span,
	},
	/// Form-level validator: `@form: [|data| ... => "error"]`
	Form {
		rules: Vec<ValidatorRule>,
		span: Span,
	},
}

/// A single validation rule with closure and error message.
#[derive(Debug, Clone)]
pub struct ValidatorRule {
	/// Validation closure expression
	pub expr: ExprClosure,
	/// Error message when validation fails
	pub message: LitStr,
	/// Span for error reporting
	pub span: Span,
}

/// Client-side validator definition (JavaScript expressions).
#[derive(Debug, Clone)]
pub struct ClientValidator {
	/// Field name to validate
	pub field_name: Ident,
	/// Validation rules
	pub rules: Vec<ClientValidatorRule>,
	/// Span for error reporting
	pub span: Span,
}

/// A single client-side validation rule.
#[derive(Debug, Clone)]
pub struct ClientValidatorRule {
	/// JavaScript expression for validation
	pub js_expr: LitStr,
	/// Error message when validation fails
	pub message: LitStr,
	/// Span for error reporting
	pub span: Span,
}

impl FormMacro {
	/// Creates a new FormMacro with the given name and span.
	pub fn new(name: Ident, span: Span) -> Self {
		Self {
			name,
			action: FormAction::None,
			method: None,
			class: None,
			fields: Vec::new(),
			validators: Vec::new(),
			client_validators: Vec::new(),
			span,
		}
	}

	/// Returns true if this form uses server_fn for submission.
	pub fn uses_server_fn(&self) -> bool {
		matches!(self.action, FormAction::ServerFn(_))
	}

	/// Returns the action URL if set.
	pub fn action_url(&self) -> Option<&LitStr> {
		match &self.action {
			FormAction::Url(url) => Some(url),
			_ => None,
		}
	}

	/// Returns the server_fn path if set.
	pub fn server_fn_path(&self) -> Option<&Path> {
		match &self.action {
			FormAction::ServerFn(path) => Some(path),
			_ => None,
		}
	}
}

impl FormFieldDef {
	/// Creates a new field definition.
	pub fn new(name: Ident, field_type: Ident, span: Span) -> Self {
		Self {
			name,
			field_type,
			properties: Vec::new(),
			span,
		}
	}

	/// Returns true if this field has the `required` flag.
	pub fn is_required(&self) -> bool {
		self.properties
			.iter()
			.any(|p| matches!(p, FormFieldProperty::Flag { name, .. } if name == "required"))
	}

	/// Gets a named property value by name.
	pub fn get_property(&self, prop_name: &str) -> Option<&Expr> {
		self.properties.iter().find_map(|p| {
			if let FormFieldProperty::Named { name, value, .. } = p
				&& name == prop_name
			{
				return Some(value);
			}
			None
		})
	}

	/// Gets the widget type if specified.
	pub fn get_widget(&self) -> Option<&Ident> {
		self.properties.iter().find_map(|p| {
			if let FormFieldProperty::Widget { widget_type, .. } = p {
				Some(widget_type)
			} else {
				None
			}
		})
	}

	/// Gets the CSS class for the input element.
	pub fn get_class(&self) -> Option<&Expr> {
		self.get_property("class")
	}

	/// Gets the CSS class for the wrapper element.
	pub fn get_wrapper_class(&self) -> Option<&Expr> {
		self.get_property("wrapper_class")
	}

	/// Gets the CSS class for the label element.
	pub fn get_label_class(&self) -> Option<&Expr> {
		self.get_property("label_class")
	}

	/// Gets the CSS class for the error element.
	pub fn get_error_class(&self) -> Option<&Expr> {
		self.get_property("error_class")
	}

	/// Gets the label text if specified.
	pub fn get_label(&self) -> Option<&Expr> {
		self.get_property("label")
	}

	/// Gets the placeholder text if specified.
	pub fn get_placeholder(&self) -> Option<&Expr> {
		self.get_property("placeholder")
	}

	/// Gets the max_length constraint if specified.
	pub fn get_max_length(&self) -> Option<&Expr> {
		self.get_property("max_length")
	}

	/// Gets the min_length constraint if specified.
	pub fn get_min_length(&self) -> Option<&Expr> {
		self.get_property("min_length")
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_form_field_property_is_styling() {
		let class_prop = FormFieldProperty::Named {
			name: Ident::new("class", Span::call_site()),
			value: syn::parse_quote!("input-field"),
			span: Span::call_site(),
		};
		assert!(class_prop.is_styling());

		let wrapper_class_prop = FormFieldProperty::Named {
			name: Ident::new("wrapper_class", Span::call_site()),
			value: syn::parse_quote!("field-wrapper"),
			span: Span::call_site(),
		};
		assert!(wrapper_class_prop.is_styling());

		let label_prop = FormFieldProperty::Named {
			name: Ident::new("label", Span::call_site()),
			value: syn::parse_quote!("Username"),
			span: Span::call_site(),
		};
		assert!(!label_prop.is_styling());

		let required_flag = FormFieldProperty::Flag {
			name: Ident::new("required", Span::call_site()),
			span: Span::call_site(),
		};
		assert!(!required_flag.is_styling());
	}

	#[test]
	fn test_form_field_def_is_required() {
		let mut field = FormFieldDef::new(
			Ident::new("username", Span::call_site()),
			Ident::new("CharField", Span::call_site()),
			Span::call_site(),
		);
		assert!(!field.is_required());

		field.properties.push(FormFieldProperty::Flag {
			name: Ident::new("required", Span::call_site()),
			span: Span::call_site(),
		});
		assert!(field.is_required());
	}

	#[test]
	fn test_form_macro_uses_server_fn() {
		let mut form = FormMacro::new(
			Ident::new("LoginForm", Span::call_site()),
			Span::call_site(),
		);
		assert!(!form.uses_server_fn());

		form.action = FormAction::ServerFn(syn::parse_quote!(submit_login));
		assert!(form.uses_server_fn());

		form.action = FormAction::Url(syn::parse_str("/api/login").unwrap());
		assert!(!form.uses_server_fn());
	}
}
