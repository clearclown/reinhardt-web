//! Code generation for the form! macro.
//!
//! This module generates Rust code from the typed form AST. The generated code
//! supports both SSR (Server-Side Rendering) and CSR (Client-Side Rendering/WASM)
//! through conditional compilation.
//!
//! ## Generated Code Structure
//!
//! For SSR (native):
//! - Generates a Form struct with metadata
//! - Implements `into_view()` for View conversion
//! - Field accessors return dummy Signal wrappers for type compatibility
//!
//! For CSR (WASM):
//! - Generates a FormComponent with reactive Signal bindings
//! - Implements real `into_view()` with event handlers
//! - Field accessors return actual Signal references
//!
//! ## Example
//!
//! ```text
//! form! {
//!     name: LoginForm,
//!     action: "/api/login",
//!     fields: {
//!         username: CharField { required },
//!         password: CharField { widget: PasswordInput },
//!     },
//! }
//! ```
//!
//! Generates (simplified):
//!
//! ```text
//! {
//!     struct LoginForm { ... }
//!     impl LoginForm {
//!         fn username(&self) -> &Signal<String> { ... }
//!         fn password(&self) -> &Signal<String> { ... }
//!         fn into_view(self) -> View { ... }
//!     }
//!     LoginForm::new()
//! }
//! ```

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

use crate::crate_paths::get_reinhardt_pages_crate_info;
use reinhardt_pages_ast::{
	FormMethod, TypedFieldType, TypedFormAction, TypedFormFieldDef, TypedFormMacro,
	TypedValidatorRule, TypedWidget,
};

/// Generates the complete code for a form! macro invocation.
///
/// This function generates conditional code that works for both WASM and server builds.
pub(super) fn generate(macro_ast: &TypedFormMacro) -> TokenStream {
	let crate_info = get_reinhardt_pages_crate_info();
	let use_statement = &crate_info.use_statement;
	let pages_crate = &crate_info.ident;

	let struct_name = &macro_ast.name;

	// Generate field declarations
	let field_decls = generate_field_declarations(&macro_ast.fields, pages_crate);

	// Generate field initializers
	let field_inits = generate_field_initializers(&macro_ast.fields, pages_crate);

	// Generate field accessor methods
	let field_accessors = generate_field_accessors(&macro_ast.fields, pages_crate);

	// Generate metadata for SSR
	let metadata_fn = generate_metadata_function(macro_ast, pages_crate);

	// Generate into_view implementation
	let into_view_impl = generate_into_view(macro_ast, pages_crate);

	// Generate submit method if action is specified
	let submit_method = generate_submit_method(macro_ast, pages_crate);

	// Generate validation method
	let validate_method = generate_validate_method(macro_ast, pages_crate);

	quote! {
		{
			#use_statement

			struct #struct_name {
				#field_decls
			}

			impl #struct_name {
				fn new() -> Self {
					Self {
						#field_inits
					}
				}

				#field_accessors
				#metadata_fn
				#validate_method
				#submit_method
				#into_view_impl
			}

			#struct_name::new()
		}
	}
}

/// Generates field declarations for the form struct.
fn generate_field_declarations(
	fields: &[TypedFormFieldDef],
	pages_crate: &TokenStream,
) -> TokenStream {
	let decls: Vec<TokenStream> = fields
		.iter()
		.map(|field| {
			let name = &field.name;
			let signal_type = field_type_to_signal_type(&field.field_type, pages_crate);
			quote! {
				#name: #signal_type,
			}
		})
		.collect();

	quote! { #(#decls)* }
}

/// Generates field initializers for the new() function.
fn generate_field_initializers(
	fields: &[TypedFormFieldDef],
	pages_crate: &TokenStream,
) -> TokenStream {
	let inits: Vec<TokenStream> = fields
		.iter()
		.map(|field| {
			let name = &field.name;
			let default_value = field_type_default_value(&field.field_type);
			quote! {
				#name: #pages_crate::reactive::Signal::new(#default_value),
			}
		})
		.collect();

	quote! { #(#inits)* }
}

/// Generates field accessor methods.
fn generate_field_accessors(
	fields: &[TypedFormFieldDef],
	pages_crate: &TokenStream,
) -> TokenStream {
	let accessors: Vec<TokenStream> = fields
		.iter()
		.map(|field| {
			let name = &field.name;
			let signal_type = field_type_to_signal_type(&field.field_type, pages_crate);
			quote! {
				pub fn #name(&self) -> &#signal_type {
					&self.#name
				}
			}
		})
		.collect();

	quote! { #(#accessors)* }
}

/// Generates the metadata function for SSR.
fn generate_metadata_function(
	macro_ast: &TypedFormMacro,
	pages_crate: &TokenStream,
) -> TokenStream {
	let action_str = match &macro_ast.action {
		TypedFormAction::Url(url) => url.clone(),
		TypedFormAction::ServerFn(path) => {
			// Convert syn::Path to string for URL generation
			format!("/api/{}", path.to_token_stream())
		}
		TypedFormAction::None => String::new(),
	};

	let method_str = match macro_ast.method {
		FormMethod::Get => "GET",
		FormMethod::Post => "POST",
		FormMethod::Put => "PUT",
		FormMethod::Patch => "PATCH",
		FormMethod::Delete => "DELETE",
	};

	let form_class = macro_ast
		.styling
		.class
		.as_deref()
		.unwrap_or("reinhardt-form");

	let field_metadata: Vec<TokenStream> = macro_ast
		.fields
		.iter()
		.map(|field| {
			let name = field.name.to_string();
			let field_type = field_type_to_string(&field.field_type);
			let widget = widget_to_string(&field.widget);
			let required = field.validation.required;
			// Use name variable instead of creating new temporary to avoid E0716
			let label = field.display.label.as_deref().unwrap_or(&name);
			let placeholder = field.display.placeholder.as_deref().unwrap_or("");
			let input_class = field.styling.input_class();
			let wrapper_class = field.styling.wrapper_class();
			let label_class = field.styling.label_class();
			let error_class = field.styling.error_class();

			quote! {
				#pages_crate::form::StaticFieldMetadata {
					name: #name.to_string(),
					field_type: #field_type.to_string(),
					widget: #widget.to_string(),
					required: #required,
					label: #label.to_string(),
					placeholder: #placeholder.to_string(),
					input_class: #input_class.to_string(),
					wrapper_class: #wrapper_class.to_string(),
					label_class: #label_class.to_string(),
					error_class: #error_class.to_string(),
				}
			}
		})
		.collect();

	quote! {
		pub fn metadata(&self) -> #pages_crate::form::StaticFormMetadata {
			#pages_crate::form::StaticFormMetadata {
				action: #action_str.to_string(),
				method: #method_str.to_string(),
				class: #form_class.to_string(),
				fields: vec![#(#field_metadata),*],
			}
		}
	}
}

/// Generates the into_view implementation.
fn generate_into_view(macro_ast: &TypedFormMacro, pages_crate: &TokenStream) -> TokenStream {
	let action_str = match &macro_ast.action {
		TypedFormAction::Url(url) => url.clone(),
		TypedFormAction::ServerFn(path) => {
			// Convert syn::Path to string for URL generation
			format!("/api/{}", path.to_token_stream())
		}
		TypedFormAction::None => String::new(),
	};

	let method_str = match macro_ast.method {
		FormMethod::Get => "get",
		FormMethod::Post => "post",
		FormMethod::Put => "put",
		FormMethod::Patch => "patch",
		FormMethod::Delete => "delete",
	};

	let form_class = macro_ast
		.styling
		.class
		.as_deref()
		.unwrap_or("reinhardt-form");

	// Generate field views
	let field_views: Vec<TokenStream> = macro_ast
		.fields
		.iter()
		.map(|field| generate_field_view(field, pages_crate))
		.collect();

	quote! {
		pub fn into_view(self) -> #pages_crate::component::View {
			use #pages_crate::component::{ElementView, IntoView};

			let form_element = ElementView::new("form")
				.attr("action", #action_str)
				.attr("method", #method_str)
				.attr("class", #form_class)
				#(.child(#field_views))*;

			form_element.into_view()
		}
	}
}

/// Generates view code for a single field.
fn generate_field_view(field: &TypedFormFieldDef, _pages_crate: &TokenStream) -> TokenStream {
	let field_name = &field.name;
	let field_name_str = field_name.to_string();
	let input_type = widget_to_input_type(&field.widget);
	let label_text = field.display.label.as_deref().unwrap_or(&field_name_str);
	let placeholder = field.display.placeholder.as_deref().unwrap_or("");
	let required = field.validation.required;

	let wrapper_class = field.styling.wrapper_class();
	let label_class = field.styling.label_class();
	let input_class = field.styling.input_class();

	// Generate input element based on widget type
	let input_element = match &field.widget {
		TypedWidget::Textarea => {
			quote! {
				ElementView::new("textarea")
					.attr("name", #field_name_str)
					.attr("id", #field_name_str)
					.attr("class", #input_class)
					.attr("placeholder", #placeholder)
					.bool_attr("required", #required)
			}
		}
		TypedWidget::Select | TypedWidget::SelectMultiple => {
			let multiple = matches!(field.widget, TypedWidget::SelectMultiple);
			quote! {
				ElementView::new("select")
					.attr("name", #field_name_str)
					.attr("id", #field_name_str)
					.attr("class", #input_class)
					.bool_attr("required", #required)
					.bool_attr("multiple", #multiple)
			}
		}
		TypedWidget::CheckboxInput => {
			quote! {
				ElementView::new("input")
					.attr("type", "checkbox")
					.attr("name", #field_name_str)
					.attr("id", #field_name_str)
					.attr("class", #input_class)
					.bool_attr("required", #required)
			}
		}
		TypedWidget::RadioSelect => {
			quote! {
				ElementView::new("input")
					.attr("type", "radio")
					.attr("name", #field_name_str)
					.attr("id", #field_name_str)
					.attr("class", #input_class)
					.bool_attr("required", #required)
			}
		}
		_ => {
			// Standard input element
			quote! {
				ElementView::new("input")
					.attr("type", #input_type)
					.attr("name", #field_name_str)
					.attr("id", #field_name_str)
					.attr("class", #input_class)
					.attr("placeholder", #placeholder)
					.bool_attr("required", #required)
			}
		}
	};

	// Generate label element (skip for hidden inputs)
	let label_element = if matches!(field.widget, TypedWidget::HiddenInput) {
		quote! {}
	} else {
		quote! {
			.child(
				ElementView::new("label")
					.attr("for", #field_name_str)
					.attr("class", #label_class)
					.child(#label_text)
			)
		}
	};

	// Wrapper div (skip for hidden inputs)
	if matches!(field.widget, TypedWidget::HiddenInput) {
		input_element
	} else {
		quote! {
			ElementView::new("div")
				.attr("class", #wrapper_class)
				#label_element
				.child(#input_element)
		}
	}
}

/// Generates the validate method.
fn generate_validate_method(macro_ast: &TypedFormMacro, _pages_crate: &TokenStream) -> TokenStream {
	let validators: Vec<TokenStream> = macro_ast
		.validators
		.iter()
		.flat_map(|v| generate_validator_rules(&v.field_name, &v.rules))
		.collect();

	if validators.is_empty() {
		quote! {
			pub fn validate(&self) -> Result<(), Vec<String>> {
				Ok(())
			}
		}
	} else {
		quote! {
			pub fn validate(&self) -> Result<(), Vec<String>> {
				let mut errors = Vec::new();
				#(#validators)*
				if errors.is_empty() {
					Ok(())
				} else {
					Err(errors)
				}
			}
		}
	}
}

/// Generates validator rule checks.
fn generate_validator_rules(
	field_name: &syn::Ident,
	rules: &[TypedValidatorRule],
) -> Vec<TokenStream> {
	rules
		.iter()
		.map(|rule| {
			let condition = &rule.condition;
			let message = &rule.message;
			let field_name_str = field_name.to_string();
			quote! {
				{
					let v = self.#field_name.get();
					if !(#condition) {
						errors.push(format!("{}: {}", #field_name_str, #message));
					}
				}
			}
		})
		.collect()
}

/// Generates the submit method if action is specified.
fn generate_submit_method(macro_ast: &TypedFormMacro, pages_crate: &TokenStream) -> TokenStream {
	match &macro_ast.action {
		TypedFormAction::ServerFn(server_fn_ident) => {
			// Generate submit that calls the server_fn
			let field_names: Vec<&syn::Ident> = macro_ast.fields.iter().map(|f| &f.name).collect();

			quote! {
				#[cfg(target_arch = "wasm32")]
				pub async fn submit(&self) -> Result<(), #pages_crate::ServerFnError> {
					// Build request from form fields
					let request = (#(self.#field_names.get()),*);

					// Call the server function
					#server_fn_ident(request).await
				}

				#[cfg(not(target_arch = "wasm32"))]
				pub async fn submit(&self) -> Result<(), #pages_crate::ServerFnError> {
					// On server, submit is a no-op (form is submitted via HTTP)
					Ok(())
				}
			}
		}
		TypedFormAction::Url(_) => {
			// For URL action, submit triggers form submission
			quote! {
				#[cfg(target_arch = "wasm32")]
				pub fn submit(&self) {
					// Trigger native form submission via JavaScript
					// This will be handled by the browser
					#pages_crate::dom::submit_form(&self.metadata());
				}

				#[cfg(not(target_arch = "wasm32"))]
				pub fn submit(&self) {
					// On server, submit is a no-op
				}
			}
		}
		TypedFormAction::None => {
			// No action means no submit method
			quote! {}
		}
	}
}

/// Converts field type to Signal type.
fn field_type_to_signal_type(
	field_type: &TypedFieldType,
	pages_crate: &TokenStream,
) -> TokenStream {
	let inner_type = match field_type {
		TypedFieldType::CharField
		| TypedFieldType::TextField
		| TypedFieldType::EmailField
		| TypedFieldType::PasswordField
		| TypedFieldType::UrlField
		| TypedFieldType::SlugField
		| TypedFieldType::IpAddressField
		| TypedFieldType::JsonField => quote!(String),

		TypedFieldType::IntegerField => quote!(i64),
		TypedFieldType::FloatField | TypedFieldType::DecimalField => quote!(f64),
		TypedFieldType::BooleanField => quote!(bool),

		TypedFieldType::DateField => quote!(Option<chrono::NaiveDate>),
		TypedFieldType::TimeField => quote!(Option<chrono::NaiveTime>),
		TypedFieldType::DateTimeField => quote!(Option<chrono::NaiveDateTime>),

		TypedFieldType::ChoiceField => quote!(String),
		TypedFieldType::MultipleChoiceField => quote!(Vec<String>),

		TypedFieldType::FileField | TypedFieldType::ImageField => quote!(Option<web_sys::File>),

		TypedFieldType::UuidField => quote!(Option<uuid::Uuid>),
		TypedFieldType::HiddenField => quote!(String),
	};

	quote!(#pages_crate::reactive::Signal<#inner_type>)
}

/// Returns the default value for a field type.
fn field_type_default_value(field_type: &TypedFieldType) -> TokenStream {
	match field_type {
		TypedFieldType::CharField
		| TypedFieldType::TextField
		| TypedFieldType::EmailField
		| TypedFieldType::PasswordField
		| TypedFieldType::UrlField
		| TypedFieldType::SlugField
		| TypedFieldType::IpAddressField
		| TypedFieldType::JsonField
		| TypedFieldType::ChoiceField
		| TypedFieldType::HiddenField => quote!(String::new()),

		TypedFieldType::IntegerField => quote!(0i64),
		TypedFieldType::FloatField | TypedFieldType::DecimalField => quote!(0.0f64),
		TypedFieldType::BooleanField => quote!(false),

		TypedFieldType::DateField
		| TypedFieldType::TimeField
		| TypedFieldType::DateTimeField
		| TypedFieldType::FileField
		| TypedFieldType::ImageField
		| TypedFieldType::UuidField => quote!(None),

		TypedFieldType::MultipleChoiceField => quote!(Vec::new()),
	}
}

/// Converts field type to string representation.
fn field_type_to_string(field_type: &TypedFieldType) -> &'static str {
	match field_type {
		TypedFieldType::CharField => "CharField",
		TypedFieldType::TextField => "TextField",
		TypedFieldType::EmailField => "EmailField",
		TypedFieldType::PasswordField => "PasswordField",
		TypedFieldType::IntegerField => "IntegerField",
		TypedFieldType::FloatField => "FloatField",
		TypedFieldType::DecimalField => "DecimalField",
		TypedFieldType::BooleanField => "BooleanField",
		TypedFieldType::DateField => "DateField",
		TypedFieldType::TimeField => "TimeField",
		TypedFieldType::DateTimeField => "DateTimeField",
		TypedFieldType::ChoiceField => "ChoiceField",
		TypedFieldType::MultipleChoiceField => "MultipleChoiceField",
		TypedFieldType::FileField => "FileField",
		TypedFieldType::ImageField => "ImageField",
		TypedFieldType::UrlField => "UrlField",
		TypedFieldType::SlugField => "SlugField",
		TypedFieldType::UuidField => "UuidField",
		TypedFieldType::IpAddressField => "IpAddressField",
		TypedFieldType::JsonField => "JsonField",
		TypedFieldType::HiddenField => "HiddenField",
	}
}

/// Converts widget type to string representation.
fn widget_to_string(widget: &TypedWidget) -> &'static str {
	match widget {
		TypedWidget::TextInput => "TextInput",
		TypedWidget::PasswordInput => "PasswordInput",
		TypedWidget::EmailInput => "EmailInput",
		TypedWidget::NumberInput => "NumberInput",
		TypedWidget::Textarea => "Textarea",
		TypedWidget::CheckboxInput => "CheckboxInput",
		TypedWidget::RadioInput => "RadioInput",
		TypedWidget::RadioSelect => "RadioSelect",
		TypedWidget::Select => "Select",
		TypedWidget::SelectMultiple => "SelectMultiple",
		TypedWidget::DateInput => "DateInput",
		TypedWidget::TimeInput => "TimeInput",
		TypedWidget::DateTimeInput => "DateTimeInput",
		TypedWidget::FileInput => "FileInput",
		TypedWidget::HiddenInput => "HiddenInput",
		TypedWidget::ColorInput => "ColorInput",
		TypedWidget::RangeInput => "RangeInput",
		TypedWidget::UrlInput => "UrlInput",
		TypedWidget::TelInput => "TelInput",
		TypedWidget::SearchInput => "SearchInput",
	}
}

/// Converts widget type to HTML input type attribute.
fn widget_to_input_type(widget: &TypedWidget) -> &'static str {
	match widget {
		TypedWidget::TextInput => "text",
		TypedWidget::PasswordInput => "password",
		TypedWidget::EmailInput => "email",
		TypedWidget::NumberInput => "number",
		TypedWidget::Textarea => "textarea", // Not used directly
		TypedWidget::CheckboxInput => "checkbox",
		TypedWidget::RadioInput => "radio",
		TypedWidget::RadioSelect => "radio",
		TypedWidget::Select => "select",         // Not used directly
		TypedWidget::SelectMultiple => "select", // Not used directly
		TypedWidget::DateInput => "date",
		TypedWidget::TimeInput => "time",
		TypedWidget::DateTimeInput => "datetime-local",
		TypedWidget::FileInput => "file",
		TypedWidget::HiddenInput => "hidden",
		TypedWidget::ColorInput => "color",
		TypedWidget::RangeInput => "range",
		TypedWidget::UrlInput => "url",
		TypedWidget::TelInput => "tel",
		TypedWidget::SearchInput => "search",
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use quote::quote;

	fn parse_validate_generate(input: proc_macro2::TokenStream) -> TokenStream {
		use reinhardt_pages_ast::FormMacro;

		let untyped_ast: FormMacro = syn::parse2(input).unwrap();
		let typed_ast = super::super::validator::validate(&untyped_ast).unwrap();
		generate(&typed_ast)
	}

	#[rstest::rstest]
	fn test_generate_simple_form() {
		let input = quote! {
			name: LoginForm,
			action: "/api/login",

			fields: {
				username: CharField { required },
				password: CharField { widget: PasswordInput },
			},
		};

		let output = parse_validate_generate(input);
		let output_str = output.to_string();

		// Check struct generation
		assert!(output_str.contains("struct LoginForm"));

		// Check field accessors
		assert!(output_str.contains("fn username"));
		assert!(output_str.contains("fn password"));

		// Check into_view method
		assert!(output_str.contains("fn into_view"));

		// Check action
		assert!(output_str.contains("/api/login"));
	}

	#[rstest::rstest]
	fn test_generate_form_with_styling() {
		let input = quote! {
			name: StyledForm,
			action: "/test",
			class: "my-form",

			fields: {
				email: EmailField {
					required,
					class: "email-input",
					wrapper_class: "field-wrapper",
				},
			},
		};

		let output = parse_validate_generate(input);
		let output_str = output.to_string();

		assert!(output_str.contains("my-form"));
		assert!(output_str.contains("email-input"));
		assert!(output_str.contains("field-wrapper"));
	}

	#[rstest::rstest]
	fn test_generate_hidden_field() {
		let input = quote! {
			name: HiddenForm,
			action: "/test",

			fields: {
				csrf_token: HiddenField {},
			},
		};

		let output = parse_validate_generate(input);
		let output_str = output.to_string();

		// Hidden fields should not have label
		assert!(output_str.contains("hidden"));
	}
}
