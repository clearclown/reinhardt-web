//! Typed AST node definitions for the form! macro.
//!
//! This module provides typed versions of form AST nodes, where property values
//! have explicit type information. This enables stronger compile-time validation
//! and better error messages.
//!
//! The typed AST is produced by the validator after transforming and validating
//! the untyped AST from the parser.

use proc_macro2::Span;
use syn::{Ident, Path};

/// The top-level typed AST node representing a validated form! macro invocation.
///
/// This is the result of successful validation and transformation of an untyped
/// `FormMacro`. All validation rules have been enforced at this point.
#[derive(Debug)]
pub struct TypedFormMacro {
	/// Form struct name (validated identifier)
	pub name: Ident,
	/// Validated form action configuration
	pub action: TypedFormAction,
	/// HTTP method (validated, defaults to Post)
	pub method: FormMethod,
	/// Form-level styling configuration
	pub styling: TypedFormStyling,
	/// Validated field definitions
	pub fields: Vec<TypedFormFieldDef>,
	/// Validated server-side validators
	pub validators: Vec<TypedFormValidator>,
	/// Validated client-side validators
	pub client_validators: Vec<TypedClientValidator>,
	/// Span for error reporting
	pub span: Span,
}

/// Typed form action configuration.
///
/// Validated to ensure exactly one action method is specified.
#[derive(Debug, Clone)]
pub enum TypedFormAction {
	/// URL action with validated string
	Url(String),
	/// server_fn action with validated path
	ServerFn(Path),
	/// No action specified (form handles submission manually)
	None,
}

/// HTTP method for form submission.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormMethod {
	#[default]
	Post,
	Get,
	Put,
	Delete,
	Patch,
}

impl FormMethod {
	/// Returns the HTTP method string.
	pub fn as_str(&self) -> &'static str {
		match self {
			FormMethod::Post => "POST",
			FormMethod::Get => "GET",
			FormMethod::Put => "PUT",
			FormMethod::Delete => "DELETE",
			FormMethod::Patch => "PATCH",
		}
	}

	/// Returns the lowercase method string for HTML forms.
	pub fn as_html_method(&self) -> &'static str {
		match self {
			FormMethod::Post => "post",
			FormMethod::Get => "get",
			// HTML forms only support GET and POST natively
			// PUT/DELETE/PATCH need JavaScript handling
			FormMethod::Put => "post",
			FormMethod::Delete => "post",
			FormMethod::Patch => "post",
		}
	}

	/// Returns true if this method requires JavaScript handling.
	pub fn requires_js(&self) -> bool {
		matches!(
			self,
			FormMethod::Put | FormMethod::Delete | FormMethod::Patch
		)
	}
}

/// Form-level styling configuration.
#[derive(Debug, Clone, Default)]
pub struct TypedFormStyling {
	/// Form element CSS class
	pub class: Option<String>,
}

/// A validated field definition with typed properties.
#[derive(Debug)]
pub struct TypedFormFieldDef {
	/// Field name identifier
	pub name: Ident,
	/// Validated field type
	pub field_type: TypedFieldType,
	/// Validation properties
	pub validation: TypedFieldValidation,
	/// Display properties
	pub display: TypedFieldDisplay,
	/// Styling properties
	pub styling: TypedFieldStyling,
	/// Widget type
	pub widget: TypedWidget,
	/// Span for error reporting
	pub span: Span,
}

/// Validated field types with their associated Signal types.
#[derive(Debug, Clone)]
pub enum TypedFieldType {
	/// CharField -> Signal<String>
	CharField,
	/// EmailField -> Signal<String>
	EmailField,
	/// UrlField -> Signal<String>
	UrlField,
	/// SlugField -> Signal<String>
	SlugField,
	/// TextField -> Signal<String>
	TextField,
	/// IntegerField -> Signal<i64>
	IntegerField,
	/// FloatField -> Signal<f64>
	FloatField,
	/// DecimalField -> Signal<String> (for precision)
	DecimalField,
	/// BooleanField -> Signal<bool>
	BooleanField,
	/// DateField -> Signal<Option<NaiveDate>>
	DateField,
	/// TimeField -> Signal<Option<NaiveTime>>
	TimeField,
	/// DateTimeField -> Signal<Option<NaiveDateTime>>
	DateTimeField,
	/// ChoiceField -> Signal<String>
	ChoiceField,
	/// MultipleChoiceField -> Signal<Vec<String>>
	MultipleChoiceField,
	/// FileField -> Signal<Option<File>>
	FileField,
	/// ImageField -> Signal<Option<File>>
	ImageField,
	/// HiddenField -> Signal<String>
	HiddenField,
	/// PasswordField -> Signal<String>
	PasswordField,
	/// UUIDField -> Signal<String>
	UuidField,
	/// JsonField -> Signal<String>
	JsonField,
	/// IpAddressField -> Signal<String>
	IpAddressField,
}

impl TypedFieldType {
	/// Returns the Rust type used in the generated struct.
	pub fn rust_type(&self) -> &'static str {
		match self {
			TypedFieldType::CharField
			| TypedFieldType::EmailField
			| TypedFieldType::UrlField
			| TypedFieldType::SlugField
			| TypedFieldType::TextField
			| TypedFieldType::DecimalField
			| TypedFieldType::ChoiceField
			| TypedFieldType::HiddenField
			| TypedFieldType::PasswordField
			| TypedFieldType::UuidField
			| TypedFieldType::JsonField
			| TypedFieldType::IpAddressField => "String",
			TypedFieldType::IntegerField => "i64",
			TypedFieldType::FloatField => "f64",
			TypedFieldType::BooleanField => "bool",
			TypedFieldType::DateField => "Option<chrono::NaiveDate>",
			TypedFieldType::TimeField => "Option<chrono::NaiveTime>",
			TypedFieldType::DateTimeField => "Option<chrono::NaiveDateTime>",
			TypedFieldType::MultipleChoiceField => "Vec<String>",
			TypedFieldType::FileField | TypedFieldType::ImageField => "Option<web_sys::File>",
		}
	}

	/// Returns the default Signal wrapper type.
	pub fn signal_type(&self) -> String {
		format!("Signal<{}>", self.rust_type())
	}

	/// Returns the default widget for this field type.
	pub fn default_widget(&self) -> TypedWidget {
		match self {
			TypedFieldType::CharField
			| TypedFieldType::SlugField
			| TypedFieldType::UuidField
			| TypedFieldType::IpAddressField => TypedWidget::TextInput,
			TypedFieldType::EmailField => TypedWidget::EmailInput,
			TypedFieldType::UrlField => TypedWidget::UrlInput,
			TypedFieldType::TextField | TypedFieldType::JsonField => TypedWidget::Textarea,
			TypedFieldType::IntegerField
			| TypedFieldType::FloatField
			| TypedFieldType::DecimalField => TypedWidget::NumberInput,
			TypedFieldType::BooleanField => TypedWidget::CheckboxInput,
			TypedFieldType::DateField => TypedWidget::DateInput,
			TypedFieldType::TimeField => TypedWidget::TimeInput,
			TypedFieldType::DateTimeField => TypedWidget::DateTimeInput,
			TypedFieldType::ChoiceField => TypedWidget::Select,
			TypedFieldType::MultipleChoiceField => TypedWidget::SelectMultiple,
			TypedFieldType::FileField | TypedFieldType::ImageField => TypedWidget::FileInput,
			TypedFieldType::HiddenField => TypedWidget::HiddenInput,
			TypedFieldType::PasswordField => TypedWidget::PasswordInput,
		}
	}
}

/// Validated widget types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedWidget {
	TextInput,
	EmailInput,
	PasswordInput,
	NumberInput,
	UrlInput,
	TelInput,
	DateInput,
	TimeInput,
	DateTimeInput,
	ColorInput,
	RangeInput,
	HiddenInput,
	Textarea,
	Select,
	SelectMultiple,
	CheckboxInput,
	RadioInput,
	RadioSelect,
	FileInput,
	SearchInput,
}

impl TypedWidget {
	/// Returns the HTML input type attribute value.
	pub fn html_type(&self) -> &'static str {
		match self {
			TypedWidget::TextInput => "text",
			TypedWidget::EmailInput => "email",
			TypedWidget::PasswordInput => "password",
			TypedWidget::NumberInput => "number",
			TypedWidget::UrlInput => "url",
			TypedWidget::TelInput => "tel",
			TypedWidget::DateInput => "date",
			TypedWidget::TimeInput => "time",
			TypedWidget::DateTimeInput => "datetime-local",
			TypedWidget::ColorInput => "color",
			TypedWidget::RangeInput => "range",
			TypedWidget::HiddenInput => "hidden",
			TypedWidget::CheckboxInput => "checkbox",
			TypedWidget::RadioInput | TypedWidget::RadioSelect => "radio",
			TypedWidget::FileInput => "file",
			TypedWidget::SearchInput => "search",
			// These are not input types
			TypedWidget::Textarea => "text",
			TypedWidget::Select => "text",
			TypedWidget::SelectMultiple => "text",
		}
	}

	/// Returns true if this widget uses an <input> element.
	pub fn is_input(&self) -> bool {
		!matches!(
			self,
			TypedWidget::Textarea | TypedWidget::Select | TypedWidget::SelectMultiple
		)
	}

	/// Returns the HTML tag name for this widget.
	pub fn html_tag(&self) -> &'static str {
		match self {
			TypedWidget::Textarea => "textarea",
			TypedWidget::Select | TypedWidget::SelectMultiple => "select",
			_ => "input",
		}
	}
}

/// Validation-related properties of a field.
#[derive(Debug, Clone, Default)]
pub struct TypedFieldValidation {
	/// Whether the field is required
	pub required: bool,
	/// Maximum length constraint
	pub max_length: Option<i64>,
	/// Minimum length constraint
	pub min_length: Option<i64>,
	/// Minimum value constraint (for numeric fields)
	pub min_value: Option<i64>,
	/// Maximum value constraint (for numeric fields)
	pub max_value: Option<i64>,
	/// Regex pattern for validation
	pub pattern: Option<String>,
}

/// Display-related properties of a field.
#[derive(Debug, Clone, Default)]
pub struct TypedFieldDisplay {
	/// Label text
	pub label: Option<String>,
	/// Placeholder text
	pub placeholder: Option<String>,
	/// Help text
	pub help_text: Option<String>,
	/// Whether the field is disabled
	pub disabled: bool,
	/// Whether the field is readonly
	pub readonly: bool,
	/// Whether to autofocus this field
	pub autofocus: bool,
}

/// Styling-related properties of a field.
#[derive(Debug, Clone, Default)]
pub struct TypedFieldStyling {
	/// CSS class for the input element
	pub class: Option<String>,
	/// CSS class for the wrapper element
	pub wrapper_class: Option<String>,
	/// CSS class for the label element
	pub label_class: Option<String>,
	/// CSS class for the error element
	pub error_class: Option<String>,
}

impl TypedFieldStyling {
	/// Returns the CSS class for the input element, with default fallback.
	pub fn input_class(&self) -> &str {
		self.class.as_deref().unwrap_or("reinhardt-input")
	}

	/// Returns the CSS class for the wrapper element, with default fallback.
	pub fn wrapper_class(&self) -> &str {
		self.wrapper_class.as_deref().unwrap_or("reinhardt-field")
	}

	/// Returns the CSS class for the label element, with default fallback.
	pub fn label_class(&self) -> &str {
		self.label_class.as_deref().unwrap_or("reinhardt-label")
	}

	/// Returns the CSS class for the error element, with default fallback.
	pub fn error_class(&self) -> &str {
		self.error_class.as_deref().unwrap_or("reinhardt-error")
	}
}

/// Typed server-side validator for a specific field.
#[derive(Debug)]
pub struct TypedFormValidator {
	/// Field name being validated
	pub field_name: Ident,
	/// Validation rules for this field
	pub rules: Vec<TypedValidatorRule>,
	/// Span for error reporting
	pub span: Span,
}

/// A typed validation rule with condition expression and error message.
#[derive(Debug)]
pub struct TypedValidatorRule {
	/// Validation condition expression (should evaluate to bool)
	pub condition: syn::Expr,
	/// Error message when validation fails
	pub message: String,
	/// Span for error reporting
	pub span: Span,
}

/// Typed client-side validator.
#[derive(Debug)]
pub struct TypedClientValidator {
	/// Field name to validate
	pub field_name: Ident,
	/// Validation rules
	pub rules: Vec<TypedClientValidatorRule>,
	/// Span for error reporting
	pub span: Span,
}

/// A typed client-side validation rule.
#[derive(Debug)]
pub struct TypedClientValidatorRule {
	/// JavaScript condition expression for validation
	pub js_condition: String,
	/// Error message when validation fails
	pub message: String,
	/// Span for error reporting
	pub span: Span,
}

impl TypedFormMacro {
	/// Creates a new TypedFormMacro with the given name and action.
	pub fn new(name: Ident, action: TypedFormAction, span: Span) -> Self {
		Self {
			name,
			action,
			method: FormMethod::default(),
			styling: TypedFormStyling::default(),
			fields: Vec::new(),
			validators: Vec::new(),
			client_validators: Vec::new(),
			span,
		}
	}

	/// Returns true if this form uses server_fn for submission.
	pub fn uses_server_fn(&self) -> bool {
		matches!(self.action, TypedFormAction::ServerFn(_))
	}

	/// Returns the action URL if using URL mode.
	pub fn action_url(&self) -> Option<&str> {
		match &self.action {
			TypedFormAction::Url(url) => Some(url),
			_ => None,
		}
	}

	/// Returns the server_fn path if using server_fn mode.
	pub fn server_fn_path(&self) -> Option<&Path> {
		match &self.action {
			TypedFormAction::ServerFn(path) => Some(path),
			_ => None,
		}
	}

	/// Returns the form-level CSS class with default fallback.
	pub fn form_class(&self) -> &str {
		self.styling.class.as_deref().unwrap_or("reinhardt-form")
	}
}

impl TypedFormFieldDef {
	/// Creates a new TypedFormFieldDef with the given name and type.
	pub fn new(name: Ident, field_type: TypedFieldType, span: Span) -> Self {
		let widget = field_type.default_widget();
		Self {
			name,
			field_type,
			validation: TypedFieldValidation::default(),
			display: TypedFieldDisplay::default(),
			styling: TypedFieldStyling::default(),
			widget,
			span,
		}
	}

	/// Returns the HTML name attribute for this field.
	pub fn html_name(&self) -> String {
		self.name.to_string()
	}

	/// Returns the HTML id attribute for this field.
	pub fn html_id(&self) -> String {
		format!("id_{}", self.name)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_form_method_as_str() {
		assert_eq!(FormMethod::Post.as_str(), "POST");
		assert_eq!(FormMethod::Get.as_str(), "GET");
		assert_eq!(FormMethod::Put.as_str(), "PUT");
		assert_eq!(FormMethod::Delete.as_str(), "DELETE");
		assert_eq!(FormMethod::Patch.as_str(), "PATCH");
	}

	#[test]
	fn test_form_method_requires_js() {
		assert!(!FormMethod::Post.requires_js());
		assert!(!FormMethod::Get.requires_js());
		assert!(FormMethod::Put.requires_js());
		assert!(FormMethod::Delete.requires_js());
		assert!(FormMethod::Patch.requires_js());
	}

	#[test]
	fn test_typed_field_type_rust_type() {
		assert_eq!(TypedFieldType::CharField.rust_type(), "String");
		assert_eq!(TypedFieldType::IntegerField.rust_type(), "i64");
		assert_eq!(TypedFieldType::BooleanField.rust_type(), "bool");
		assert_eq!(
			TypedFieldType::DateField.rust_type(),
			"Option<chrono::NaiveDate>"
		);
	}

	#[test]
	fn test_typed_field_type_default_widget() {
		assert_eq!(
			TypedFieldType::CharField.default_widget(),
			TypedWidget::TextInput
		);
		assert_eq!(
			TypedFieldType::EmailField.default_widget(),
			TypedWidget::EmailInput
		);
		assert_eq!(
			TypedFieldType::PasswordField.default_widget(),
			TypedWidget::PasswordInput
		);
		assert_eq!(
			TypedFieldType::BooleanField.default_widget(),
			TypedWidget::CheckboxInput
		);
	}

	#[test]
	fn test_typed_widget_html_type() {
		assert_eq!(TypedWidget::TextInput.html_type(), "text");
		assert_eq!(TypedWidget::EmailInput.html_type(), "email");
		assert_eq!(TypedWidget::PasswordInput.html_type(), "password");
		assert_eq!(TypedWidget::NumberInput.html_type(), "number");
		assert_eq!(TypedWidget::DateInput.html_type(), "date");
	}

	#[test]
	fn test_typed_widget_is_input() {
		assert!(TypedWidget::TextInput.is_input());
		assert!(TypedWidget::EmailInput.is_input());
		assert!(!TypedWidget::Textarea.is_input());
		assert!(!TypedWidget::Select.is_input());
	}

	#[test]
	fn test_typed_widget_html_tag() {
		assert_eq!(TypedWidget::TextInput.html_tag(), "input");
		assert_eq!(TypedWidget::Textarea.html_tag(), "textarea");
		assert_eq!(TypedWidget::Select.html_tag(), "select");
	}

	#[test]
	fn test_typed_field_styling_defaults() {
		let styling = TypedFieldStyling::default();
		assert_eq!(styling.input_class(), "reinhardt-input");
		assert_eq!(styling.wrapper_class(), "reinhardt-field");
		assert_eq!(styling.label_class(), "reinhardt-label");
		assert_eq!(styling.error_class(), "reinhardt-error");
	}

	#[test]
	fn test_typed_field_styling_custom() {
		let styling = TypedFieldStyling {
			class: Some("custom-input".to_string()),
			wrapper_class: Some("custom-wrapper".to_string()),
			label_class: Some("custom-label".to_string()),
			error_class: Some("custom-error".to_string()),
		};
		assert_eq!(styling.input_class(), "custom-input");
		assert_eq!(styling.wrapper_class(), "custom-wrapper");
		assert_eq!(styling.label_class(), "custom-label");
		assert_eq!(styling.error_class(), "custom-error");
	}

	#[test]
	fn test_typed_form_field_def_html_name() {
		let field = TypedFormFieldDef::new(
			Ident::new("username", Span::call_site()),
			TypedFieldType::CharField,
			Span::call_site(),
		);
		assert_eq!(field.html_name(), "username");
		assert_eq!(field.html_id(), "id_username");
	}
}
