//! Validation and transformation logic for form! macro AST.
//!
//! This module transforms the untyped FormMacro AST into a typed TypedFormMacro,
//! performing semantic validation and type checking along the way.
//!
//! ## Validation Rules
//!
//! 1. **Field Types**: Must be valid field type identifiers (CharField, EmailField, etc.)
//! 2. **Field Properties**: Must match the expected type for each property
//! 3. **Widget Types**: Must be valid widget identifiers (TextInput, PasswordInput, etc.)
//! 4. **Required Name**: Form must have a name identifier
//! 5. **Unique Field Names**: Field names must be unique within the form
//! 6. **Valid Validators**: Validator closures must have correct signature

use proc_macro2::Span;
use std::collections::HashSet;
use syn::{Error, Result};

use reinhardt_pages_ast::{
	ClientValidator, ClientValidatorRule, FormAction, FormFieldDef, FormFieldProperty, FormMacro,
	FormMethod, FormValidator, TypedClientValidator, TypedClientValidatorRule, TypedFieldDisplay,
	TypedFieldStyling, TypedFieldType, TypedFieldValidation, TypedFormAction, TypedFormFieldDef,
	TypedFormMacro, TypedFormStyling, TypedFormValidator, TypedValidatorRule, TypedWidget,
	ValidatorRule,
};

/// Validates and transforms the FormMacro AST into a typed AST.
///
/// This is the main entry point for form! macro validation.
///
/// # Errors
///
/// Returns a compilation error if any validation rule is violated.
pub(super) fn validate(ast: &FormMacro) -> Result<TypedFormMacro> {
	// Validate unique field names
	validate_unique_field_names(&ast.fields)?;

	// Transform action
	let action = transform_action(&ast.action)?;

	// Transform method
	let method = transform_method(&ast.method)?;

	// Transform form-level styling
	let styling = transform_form_styling(ast)?;

	// Transform fields
	let fields = transform_fields(&ast.fields)?;

	// Transform server-side validators
	let validators = transform_validators(&ast.validators, &ast.fields)?;

	// Transform client-side validators
	let client_validators = transform_client_validators(&ast.client_validators, &ast.fields)?;

	Ok(TypedFormMacro {
		name: ast.name.clone(),
		action,
		method,
		styling,
		fields,
		validators,
		client_validators,
		span: ast.span,
	})
}

/// Validates that all field names are unique.
fn validate_unique_field_names(fields: &[FormFieldDef]) -> Result<()> {
	let mut seen = HashSet::new();

	for field in fields {
		let name = field.name.to_string();
		if !seen.insert(name.clone()) {
			return Err(Error::new(
				field.name.span(),
				format!("duplicate field name: '{}'", name),
			));
		}
	}

	Ok(())
}

/// Transforms FormAction to TypedFormAction.
fn transform_action(action: &FormAction) -> Result<TypedFormAction> {
	match action {
		FormAction::Url(lit) => Ok(TypedFormAction::Url(lit.value())),
		FormAction::ServerFn(ident) => Ok(TypedFormAction::ServerFn(ident.clone())),
		FormAction::None => Ok(TypedFormAction::None),
	}
}

/// Transforms method identifier to FormMethod enum.
fn transform_method(method: &Option<syn::Ident>) -> Result<FormMethod> {
	match method {
		Some(ident) => {
			let method_str = ident.to_string();
			match method_str.to_lowercase().as_str() {
				"get" => Ok(FormMethod::Get),
				"post" => Ok(FormMethod::Post),
				"put" => Ok(FormMethod::Put),
				"patch" => Ok(FormMethod::Patch),
				"delete" => Ok(FormMethod::Delete),
				_ => Err(Error::new(
					ident.span(),
					format!(
						"invalid HTTP method: '{}'. Expected: Get, Post, Put, Patch, or Delete",
						method_str
					),
				)),
			}
		}
		None => Ok(FormMethod::Post), // Default to POST
	}
}

/// Transforms form-level styling from FormMacro.
fn transform_form_styling(ast: &FormMacro) -> Result<TypedFormStyling> {
	Ok(TypedFormStyling {
		class: ast.class.as_ref().map(|lit| lit.value()),
	})
}

/// Transforms all field definitions.
fn transform_fields(fields: &[FormFieldDef]) -> Result<Vec<TypedFormFieldDef>> {
	fields.iter().map(transform_field).collect()
}

/// Transforms a single field definition.
fn transform_field(field: &FormFieldDef) -> Result<TypedFormFieldDef> {
	// Parse field type
	let field_type = parse_field_type(&field.field_type)?;

	// Extract properties into categories
	let validation = extract_validation_properties(&field.properties)?;
	let display = extract_display_properties(&field.properties)?;
	let styling = extract_styling_properties(&field.properties)?;
	let widget = extract_widget(&field.properties, &field_type)?;

	Ok(TypedFormFieldDef {
		name: field.name.clone(),
		field_type,
		widget,
		validation,
		display,
		styling,
		span: field.span,
	})
}

/// Parses field type identifier into TypedFieldType enum.
fn parse_field_type(ident: &syn::Ident) -> Result<TypedFieldType> {
	let type_str = ident.to_string();
	match type_str.as_str() {
		"CharField" => Ok(TypedFieldType::CharField),
		"TextField" => Ok(TypedFieldType::TextField),
		"EmailField" => Ok(TypedFieldType::EmailField),
		"PasswordField" => Ok(TypedFieldType::PasswordField),
		"IntegerField" => Ok(TypedFieldType::IntegerField),
		"FloatField" => Ok(TypedFieldType::FloatField),
		"DecimalField" => Ok(TypedFieldType::DecimalField),
		"BooleanField" => Ok(TypedFieldType::BooleanField),
		"DateField" => Ok(TypedFieldType::DateField),
		"TimeField" => Ok(TypedFieldType::TimeField),
		"DateTimeField" => Ok(TypedFieldType::DateTimeField),
		"ChoiceField" => Ok(TypedFieldType::ChoiceField),
		"MultipleChoiceField" => Ok(TypedFieldType::MultipleChoiceField),
		"FileField" => Ok(TypedFieldType::FileField),
		"ImageField" => Ok(TypedFieldType::ImageField),
		"UrlField" => Ok(TypedFieldType::UrlField),
		"SlugField" => Ok(TypedFieldType::SlugField),
		"UuidField" => Ok(TypedFieldType::UuidField),
		"IpAddressField" => Ok(TypedFieldType::IpAddressField),
		"JsonField" => Ok(TypedFieldType::JsonField),
		"HiddenField" => Ok(TypedFieldType::HiddenField),
		_ => Err(Error::new(
			ident.span(),
			format!(
				"unknown field type: '{}'. Expected one of: CharField, TextField, EmailField, \
				PasswordField, IntegerField, FloatField, DecimalField, BooleanField, DateField, \
				TimeField, DateTimeField, ChoiceField, MultipleChoiceField, FileField, ImageField, \
				UrlField, SlugField, UuidField, IpAddressField, JsonField, HiddenField",
				type_str
			),
		)),
	}
}

/// Extracts validation-related properties.
fn extract_validation_properties(properties: &[FormFieldProperty]) -> Result<TypedFieldValidation> {
	let mut required = false;
	let mut min_length = None;
	let mut max_length = None;
	let mut min_value = None;
	let mut max_value = None;
	let mut pattern = None;

	for prop in properties {
		match prop {
			FormFieldProperty::Flag { name, span: _ } => {
				if name == "required" {
					required = true;
				}
				// Ignore other flags
			}
			FormFieldProperty::Named { name, value, span } => {
				let name_str = name.to_string();
				match name_str.as_str() {
					"required" => {
						if let syn::Expr::Lit(lit) = value {
							if let syn::Lit::Bool(b) = &lit.lit {
								required = b.value;
							} else {
								return Err(Error::new(
									*span,
									"'required' must be a boolean value",
								));
							}
						} else {
							return Err(Error::new(*span, "'required' must be a boolean value"));
						}
					}
					"min_length" => {
						min_length = Some(extract_int_value_from_expr(value, "min_length", *span)?);
					}
					"max_length" => {
						max_length = Some(extract_int_value_from_expr(value, "max_length", *span)?);
					}
					"min_value" => {
						min_value = Some(extract_int_value_from_expr(value, "min_value", *span)?);
					}
					"max_value" => {
						max_value = Some(extract_int_value_from_expr(value, "max_value", *span)?);
					}
					"pattern" => {
						pattern = Some(extract_string_value_from_expr(value, "pattern", *span)?);
					}
					_ => {} // Ignore non-validation properties
				}
			}
			FormFieldProperty::Widget { .. } => {} // Ignore widget properties
		}
	}

	Ok(TypedFieldValidation {
		required,
		min_length,
		max_length,
		min_value,
		max_value,
		pattern,
	})
}

/// Extracts display-related properties.
fn extract_display_properties(properties: &[FormFieldProperty]) -> Result<TypedFieldDisplay> {
	let mut label = None;
	let mut placeholder = None;
	let mut help_text = None;
	let mut disabled = false;
	let mut readonly = false;
	let mut autofocus = false;

	for prop in properties {
		match prop {
			FormFieldProperty::Flag { name, .. } => {
				let name_str = name.to_string();
				match name_str.as_str() {
					"disabled" => disabled = true,
					"readonly" => readonly = true,
					"autofocus" => autofocus = true,
					_ => {} // Ignore other flags
				}
			}
			FormFieldProperty::Named { name, value, span } => {
				let name_str = name.to_string();
				match name_str.as_str() {
					"label" => {
						label = Some(extract_string_value_from_expr(value, "label", *span)?);
					}
					"placeholder" => {
						placeholder =
							Some(extract_string_value_from_expr(value, "placeholder", *span)?);
					}
					"help_text" => {
						help_text =
							Some(extract_string_value_from_expr(value, "help_text", *span)?);
					}
					"disabled" => {
						if let syn::Expr::Lit(lit) = value
							&& let syn::Lit::Bool(b) = &lit.lit
						{
							disabled = b.value;
						}
					}
					"readonly" => {
						if let syn::Expr::Lit(lit) = value
							&& let syn::Lit::Bool(b) = &lit.lit
						{
							readonly = b.value;
						}
					}
					"autofocus" => {
						if let syn::Expr::Lit(lit) = value
							&& let syn::Lit::Bool(b) = &lit.lit
						{
							autofocus = b.value;
						}
					}
					_ => {} // Ignore non-display properties
				}
			}
			FormFieldProperty::Widget { .. } => {} // Ignore widget properties
		}
	}

	Ok(TypedFieldDisplay {
		label,
		placeholder,
		help_text,
		disabled,
		readonly,
		autofocus,
	})
}

/// Extracts styling-related properties.
fn extract_styling_properties(properties: &[FormFieldProperty]) -> Result<TypedFieldStyling> {
	let mut class = None;
	let mut wrapper_class = None;
	let mut label_class = None;
	let mut error_class = None;

	for prop in properties {
		if let FormFieldProperty::Named { name, value, span } = prop {
			let name_str = name.to_string();
			match name_str.as_str() {
				"class" => {
					class = Some(extract_string_value_from_expr(value, "class", *span)?);
				}
				"wrapper_class" => {
					wrapper_class = Some(extract_string_value_from_expr(
						value,
						"wrapper_class",
						*span,
					)?);
				}
				"label_class" => {
					label_class =
						Some(extract_string_value_from_expr(value, "label_class", *span)?);
				}
				"error_class" => {
					error_class =
						Some(extract_string_value_from_expr(value, "error_class", *span)?);
				}
				_ => {} // Ignore non-styling properties
			}
		}
	}

	Ok(TypedFieldStyling {
		class,
		wrapper_class,
		label_class,
		error_class,
	})
}

/// Extracts widget property and returns TypedWidget.
fn extract_widget(
	properties: &[FormFieldProperty],
	field_type: &TypedFieldType,
) -> Result<TypedWidget> {
	// Look for explicit widget property
	for prop in properties {
		match prop {
			FormFieldProperty::Widget {
				widget_type,
				span: _,
			} => {
				return parse_widget(widget_type);
			}
			FormFieldProperty::Named { name, value, span } if name == "widget" => {
				// Handle widget specified as named property: widget: PasswordInput
				if let syn::Expr::Path(path) = value
					&& let Some(ident) = path.path.get_ident()
				{
					return parse_widget(ident);
				}
				return Err(Error::new(
					*span,
					"'widget' must be a widget type identifier (e.g., TextInput, PasswordInput)",
				));
			}
			_ => {} // Continue searching
		}
	}

	// Return default widget for field type
	Ok(field_type.default_widget())
}

/// Parses widget identifier into TypedWidget enum.
fn parse_widget(ident: &syn::Ident) -> Result<TypedWidget> {
	let widget_str = ident.to_string();
	match widget_str.as_str() {
		"TextInput" => Ok(TypedWidget::TextInput),
		"PasswordInput" => Ok(TypedWidget::PasswordInput),
		"EmailInput" => Ok(TypedWidget::EmailInput),
		"NumberInput" => Ok(TypedWidget::NumberInput),
		"Textarea" => Ok(TypedWidget::Textarea),
		"CheckboxInput" => Ok(TypedWidget::CheckboxInput),
		"RadioSelect" => Ok(TypedWidget::RadioSelect),
		"Select" => Ok(TypedWidget::Select),
		"SelectMultiple" => Ok(TypedWidget::SelectMultiple),
		"DateInput" => Ok(TypedWidget::DateInput),
		"TimeInput" => Ok(TypedWidget::TimeInput),
		"DateTimeInput" => Ok(TypedWidget::DateTimeInput),
		"FileInput" => Ok(TypedWidget::FileInput),
		"HiddenInput" => Ok(TypedWidget::HiddenInput),
		"ColorInput" => Ok(TypedWidget::ColorInput),
		"RangeInput" => Ok(TypedWidget::RangeInput),
		"UrlInput" => Ok(TypedWidget::UrlInput),
		"TelInput" => Ok(TypedWidget::TelInput),
		"SearchInput" => Ok(TypedWidget::SearchInput),
		_ => Err(Error::new(
			ident.span(),
			format!(
				"unknown widget type: '{}'. Expected one of: TextInput, PasswordInput, \
				EmailInput, NumberInput, Textarea, CheckboxInput, RadioSelect, Select, \
				SelectMultiple, DateInput, TimeInput, DateTimeInput, FileInput, HiddenInput, \
				ColorInput, RangeInput, UrlInput, TelInput, SearchInput",
				widget_str
			),
		)),
	}
}

/// Transforms server-side validators.
fn transform_validators(
	validators: &[FormValidator],
	fields: &[FormFieldDef],
) -> Result<Vec<TypedFormValidator>> {
	let mut result = Vec::new();

	for validator in validators {
		match validator {
			FormValidator::Field {
				field_name,
				rules,
				span,
			} => {
				// Validate that field exists
				let field_exists = fields.iter().any(|f| f.name == *field_name);
				if !field_exists {
					return Err(Error::new(
						field_name.span(),
						format!("validator references unknown field: '{}'", field_name),
					));
				}

				let typed_rules = rules
					.iter()
					.map(transform_validator_rule)
					.collect::<Result<Vec<_>>>()?;

				result.push(TypedFormValidator {
					field_name: field_name.clone(),
					rules: typed_rules,
					span: *span,
				});
			}
			FormValidator::Form { rules: _, span: _ } => {
				// Form-level validators are not yet supported in TypedFormValidator
				// TODO: Support form-level validators
			}
		}
	}

	Ok(result)
}

/// Transforms a validator rule.
///
/// Converts the closure expression to a regular expression for code generation.
fn transform_validator_rule(rule: &ValidatorRule) -> Result<TypedValidatorRule> {
	// Convert ExprClosure body to Expr for use in validation
	let condition: syn::Expr = (*rule.expr.body).clone();

	Ok(TypedValidatorRule {
		condition,
		message: rule.message.value(),
		span: rule.span,
	})
}

/// Transforms client-side validators.
fn transform_client_validators(
	validators: &[ClientValidator],
	fields: &[FormFieldDef],
) -> Result<Vec<TypedClientValidator>> {
	validators
		.iter()
		.map(|v| transform_client_validator(v, fields))
		.collect()
}

/// Transforms a single client-side validator.
fn transform_client_validator(
	validator: &ClientValidator,
	fields: &[FormFieldDef],
) -> Result<TypedClientValidator> {
	// Validate that field exists
	let field_exists = fields.iter().any(|f| f.name == validator.field_name);
	if !field_exists {
		return Err(Error::new(
			validator.field_name.span(),
			format!(
				"client validator references unknown field: '{}'",
				validator.field_name
			),
		));
	}

	let rules = validator
		.rules
		.iter()
		.map(transform_client_validator_rule)
		.collect::<Result<Vec<_>>>()?;

	Ok(TypedClientValidator {
		field_name: validator.field_name.clone(),
		rules,
		span: validator.span,
	})
}

/// Transforms a client validator rule.
fn transform_client_validator_rule(rule: &ClientValidatorRule) -> Result<TypedClientValidatorRule> {
	Ok(TypedClientValidatorRule {
		js_condition: rule.js_expr.value(),
		message: rule.message.value(),
		span: rule.span,
	})
}

/// Extracts an integer value from an optional expression.
/// Reserved for future enhanced validation.
#[allow(dead_code)]
fn extract_int_value(value: &Option<syn::Expr>, prop_name: &str, span: Span) -> Result<i64> {
	match value {
		Some(syn::Expr::Lit(lit)) => {
			if let syn::Lit::Int(int_lit) = &lit.lit {
				int_lit.base10_parse::<i64>().map_err(|_| {
					Error::new(span, format!("'{}' must be a valid integer", prop_name))
				})
			} else {
				Err(Error::new(
					span,
					format!("'{}' must be an integer value", prop_name),
				))
			}
		}
		Some(syn::Expr::Unary(unary)) => {
			// Handle negative numbers like -10
			if let syn::UnOp::Neg(_) = unary.op
				&& let syn::Expr::Lit(lit) = &*unary.expr
				&& let syn::Lit::Int(int_lit) = &lit.lit
			{
				let val = int_lit.base10_parse::<i64>().map_err(|_| {
					Error::new(span, format!("'{}' must be a valid integer", prop_name))
				})?;
				return Ok(-val);
			}
			Err(Error::new(
				span,
				format!("'{}' must be an integer value", prop_name),
			))
		}
		None => Err(Error::new(
			span,
			format!("'{}' requires a value", prop_name),
		)),
		_ => Err(Error::new(
			span,
			format!("'{}' must be an integer value", prop_name),
		)),
	}
}

/// Extracts a string value from an optional expression.
/// Reserved for future enhanced validation.
#[allow(dead_code)]
fn extract_string_value(value: &Option<syn::Expr>, prop_name: &str, span: Span) -> Result<String> {
	match value {
		Some(syn::Expr::Lit(lit)) => {
			if let syn::Lit::Str(str_lit) = &lit.lit {
				Ok(str_lit.value())
			} else {
				Err(Error::new(
					span,
					format!("'{}' must be a string value", prop_name),
				))
			}
		}
		None => Err(Error::new(
			span,
			format!("'{}' requires a value", prop_name),
		)),
		_ => Err(Error::new(
			span,
			format!("'{}' must be a string value", prop_name),
		)),
	}
}

/// Extracts an integer value from an expression (non-optional version).
fn extract_int_value_from_expr(value: &syn::Expr, prop_name: &str, span: Span) -> Result<i64> {
	match value {
		syn::Expr::Lit(lit) => {
			if let syn::Lit::Int(int_lit) = &lit.lit {
				int_lit.base10_parse::<i64>().map_err(|_| {
					Error::new(span, format!("'{}' must be a valid integer", prop_name))
				})
			} else {
				Err(Error::new(
					span,
					format!("'{}' must be an integer value", prop_name),
				))
			}
		}
		syn::Expr::Unary(unary) => {
			// Handle negative numbers like -10
			if let syn::UnOp::Neg(_) = unary.op
				&& let syn::Expr::Lit(lit) = &*unary.expr
				&& let syn::Lit::Int(int_lit) = &lit.lit
			{
				let val = int_lit.base10_parse::<i64>().map_err(|_| {
					Error::new(span, format!("'{}' must be a valid integer", prop_name))
				})?;
				return Ok(-val);
			}
			Err(Error::new(
				span,
				format!("'{}' must be an integer value", prop_name),
			))
		}
		_ => Err(Error::new(
			span,
			format!("'{}' must be an integer value", prop_name),
		)),
	}
}

/// Extracts a string value from an expression (non-optional version).
fn extract_string_value_from_expr(
	value: &syn::Expr,
	prop_name: &str,
	span: Span,
) -> Result<String> {
	match value {
		syn::Expr::Lit(lit) => {
			if let syn::Lit::Str(str_lit) = &lit.lit {
				Ok(str_lit.value())
			} else {
				Err(Error::new(
					span,
					format!("'{}' must be a string value", prop_name),
				))
			}
		}
		_ => Err(Error::new(
			span,
			format!("'{}' must be a string value", prop_name),
		)),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use quote::quote;

	fn parse_and_validate(input: proc_macro2::TokenStream) -> Result<TypedFormMacro> {
		let ast: FormMacro = syn::parse2(input)?;
		validate(&ast)
	}

	#[rstest::rstest]
	fn test_validate_simple_form() {
		let input = quote! {
			name: LoginForm,
			action: "/api/login",

			fields: {
				username: CharField { required, max_length: 150 },
				password: CharField { required, widget: PasswordInput },
			},
		};

		let result = parse_and_validate(input);
		assert!(result.is_ok());

		let typed = result.unwrap();
		assert_eq!(typed.name.to_string(), "LoginForm");
		assert_eq!(typed.fields.len(), 2);
		assert!(matches!(typed.action, TypedFormAction::Url(_)));
	}

	#[rstest::rstest]
	fn test_validate_server_fn_action() {
		let input = quote! {
			name: VoteForm,
			server_fn: submit_vote,

			fields: {
				choice_id: IntegerField { required },
			},
		};

		let result = parse_and_validate(input);
		assert!(result.is_ok());

		let typed = result.unwrap();
		assert!(matches!(typed.action, TypedFormAction::ServerFn(_)));
	}

	#[rstest::rstest]
	fn test_validate_duplicate_field_names() {
		let input = quote! {
			name: TestForm,
			action: "/test",

			fields: {
				username: CharField { required },
				username: EmailField { required },
			},
		};

		let result = parse_and_validate(input);
		assert!(result.is_err());
		assert!(
			result
				.unwrap_err()
				.to_string()
				.contains("duplicate field name")
		);
	}

	#[rstest::rstest]
	fn test_validate_unknown_field_type() {
		let input = quote! {
			name: TestForm,
			action: "/test",

			fields: {
				unknown: UnknownField { required },
			},
		};

		let result = parse_and_validate(input);
		assert!(result.is_err());
		assert!(
			result
				.unwrap_err()
				.to_string()
				.contains("unknown field type")
		);
	}

	#[rstest::rstest]
	fn test_validate_validator_unknown_field() {
		let input = quote! {
			name: TestForm,
			action: "/test",

			fields: {
				username: CharField { required },
			},

			validators: {
				nonexistent: [
					|v| !v.is_empty() => "Cannot be empty",
				],
			},
		};

		let result = parse_and_validate(input);
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("unknown field"));
	}

	#[rstest::rstest]
	fn test_validate_styling_properties() {
		let input = quote! {
			name: StyledForm,
			action: "/test",
			class: "my-form",

			fields: {
				username: CharField {
					required,
					class: "input-field",
					wrapper_class: "field-wrapper",
					label_class: "field-label",
					error_class: "field-error",
				},
			},
		};

		let result = parse_and_validate(input);
		assert!(result.is_ok());

		let typed = result.unwrap();
		assert_eq!(typed.styling.class, Some("my-form".to_string()));
		assert_eq!(
			typed.fields[0].styling.class,
			Some("input-field".to_string())
		);
	}
}
