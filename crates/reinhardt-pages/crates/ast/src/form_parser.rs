//! Parser implementation for the form! macro AST.
//!
//! This module provides the `Parse` trait implementation for `FormMacro`,
//! allowing it to be parsed from a `TokenStream`.

use proc_macro2::Span;
use syn::{
	Expr, ExprClosure, Ident, LitStr, Path, Result, Token, braced,
	parse::{Parse, ParseStream},
	token,
};

use crate::{
	ClientValidator, ClientValidatorRule, FormAction, FormFieldDef, FormFieldProperty, FormMacro,
	FormValidator, ValidatorRule,
};

impl Parse for FormMacro {
	fn parse(input: ParseStream) -> Result<Self> {
		let span = input.span();
		let mut form = FormMacro::new(Ident::new("_", Span::call_site()), span);

		// Parse key-value pairs until we hit fields, validators, or client_validators
		while !input.is_empty() {
			let key: Ident = input.parse()?;
			input.parse::<Token![:]>()?;

			match key.to_string().as_str() {
				"name" => {
					form.name = input.parse()?;
					parse_optional_comma(input)?;
				}
				"action" => {
					let url: LitStr = input.parse()?;
					form.action = FormAction::Url(url);
					parse_optional_comma(input)?;
				}
				"server_fn" => {
					let path: Path = input.parse()?;
					form.action = FormAction::ServerFn(path);
					parse_optional_comma(input)?;
				}
				"method" => {
					form.method = Some(input.parse()?);
					parse_optional_comma(input)?;
				}
				"class" => {
					form.class = Some(input.parse()?);
					parse_optional_comma(input)?;
				}
				"fields" => {
					let content;
					braced!(content in input);
					form.fields = parse_field_definitions(&content)?;
					parse_optional_comma(input)?;
				}
				"validators" => {
					let content;
					braced!(content in input);
					form.validators = parse_validators(&content)?;
					parse_optional_comma(input)?;
				}
				"client_validators" => {
					let content;
					braced!(content in input);
					form.client_validators = parse_client_validators(&content)?;
					parse_optional_comma(input)?;
				}
				_ => {
					return Err(syn::Error::new(
						key.span(),
						format!(
							"Unknown form property: '{}'. Expected: name, action, server_fn, method, class, fields, validators, client_validators",
							key
						),
					));
				}
			}
		}

		// Validate required fields
		if form.name == "_" {
			return Err(syn::Error::new(
				span,
				"form! macro requires 'name' property",
			));
		}

		Ok(form)
	}
}

/// Parses an optional trailing comma.
fn parse_optional_comma(input: ParseStream) -> Result<()> {
	if input.peek(Token![,]) {
		input.parse::<Token![,]>()?;
	}
	Ok(())
}

/// Parses field definitions inside the `fields: { ... }` block.
fn parse_field_definitions(input: ParseStream) -> Result<Vec<FormFieldDef>> {
	let mut fields = Vec::new();

	while !input.is_empty() {
		let span = input.span();
		let name: Ident = input.parse()?;
		input.parse::<Token![:]>()?;
		let field_type: Ident = input.parse()?;

		// Parse properties in braces: { required, max_length: 100, ... }
		let properties = if input.peek(token::Brace) {
			let content;
			braced!(content in input);
			parse_field_properties(&content)?
		} else {
			Vec::new()
		};

		fields.push(FormFieldDef {
			name,
			field_type,
			properties,
			span,
		});

		parse_optional_comma(input)?;
	}

	Ok(fields)
}

/// Parses field properties inside braces.
fn parse_field_properties(input: ParseStream) -> Result<Vec<FormFieldProperty>> {
	let mut properties = Vec::new();

	while !input.is_empty() {
		let span = input.span();

		// Check for widget keyword
		if input.peek(Ident) {
			let name: Ident = input.parse()?;

			if name == "widget" {
				// widget: WidgetType
				input.parse::<Token![:]>()?;
				let widget_type: Ident = input.parse()?;
				properties.push(FormFieldProperty::Widget { widget_type, span });
			} else if input.peek(Token![:]) {
				// name: value
				input.parse::<Token![:]>()?;
				let value: Expr = input.parse()?;
				properties.push(FormFieldProperty::Named { name, value, span });
			} else {
				// Flag property (just identifier, no value)
				properties.push(FormFieldProperty::Flag { name, span });
			}
		}

		parse_optional_comma(input)?;
	}

	Ok(properties)
}

/// Parses server-side validators inside the `validators: { ... }` block.
fn parse_validators(input: ParseStream) -> Result<Vec<FormValidator>> {
	let mut validators = Vec::new();

	while !input.is_empty() {
		let span = input.span();

		// Check for @form marker for form-level validators
		if input.peek(Token![@]) {
			input.parse::<Token![@]>()?;
			let form_ident: Ident = input.parse()?;
			if form_ident != "form" {
				return Err(syn::Error::new(
					form_ident.span(),
					"Expected '@form' for form-level validator",
				));
			}
			input.parse::<Token![:]>()?;

			// Parse rules array
			let rules = parse_validator_rules(input)?;
			validators.push(FormValidator::Form { rules, span });
		} else {
			// Field-level validator
			let field_name: Ident = input.parse()?;
			input.parse::<Token![:]>()?;

			// Parse rules array
			let rules = parse_validator_rules(input)?;
			validators.push(FormValidator::Field {
				field_name,
				rules,
				span,
			});
		}

		parse_optional_comma(input)?;
	}

	Ok(validators)
}

/// Parses validator rules: [ |v| condition => "message", ... ]
fn parse_validator_rules(input: ParseStream) -> Result<Vec<ValidatorRule>> {
	let content;
	syn::bracketed!(content in input);

	let mut rules = Vec::new();

	while !content.is_empty() {
		let span = content.span();

		// Parse closure: |v| condition
		let expr: ExprClosure = content.parse()?;

		// Parse arrow
		content.parse::<Token![=>]>()?;

		// Parse error message
		let message: LitStr = content.parse()?;

		rules.push(ValidatorRule {
			expr,
			message,
			span,
		});

		parse_optional_comma(&content)?;
	}

	Ok(rules)
}

/// Parses client-side validators inside the `client_validators: { ... }` block.
fn parse_client_validators(input: ParseStream) -> Result<Vec<ClientValidator>> {
	let mut validators = Vec::new();

	while !input.is_empty() {
		let span = input.span();
		let field_name: Ident = input.parse()?;
		input.parse::<Token![:]>()?;

		// Parse rules array
		let rules = parse_client_validator_rules(input)?;
		validators.push(ClientValidator {
			field_name,
			rules,
			span,
		});

		parse_optional_comma(input)?;
	}

	Ok(validators)
}

/// Parses client validator rules: [ "js_condition" => "message", ... ]
fn parse_client_validator_rules(input: ParseStream) -> Result<Vec<ClientValidatorRule>> {
	let content;
	syn::bracketed!(content in input);

	let mut rules = Vec::new();

	while !content.is_empty() {
		let span = content.span();

		// Parse JavaScript condition string
		let js_expr: LitStr = content.parse()?;

		// Parse arrow
		content.parse::<Token![=>]>()?;

		// Parse error message
		let message: LitStr = content.parse()?;

		rules.push(ClientValidatorRule {
			js_expr,
			message,
			span,
		});

		parse_optional_comma(&content)?;
	}

	Ok(rules)
}

#[cfg(test)]
mod tests {
	use super::*;
	use quote::quote;

	#[test]
	fn test_parse_simple_form() {
		let input = quote! {
			name: LoginForm,
			action: "/api/login",

			fields: {
				username: CharField { required, max_length: 150 },
				password: CharField { required, widget: PasswordInput },
			},
		};

		let result: Result<FormMacro> = syn::parse2(input);
		assert!(result.is_ok());

		let form = result.unwrap();
		assert_eq!(form.name.to_string(), "LoginForm");
		assert_eq!(form.fields.len(), 2);
		assert!(matches!(form.action, FormAction::Url(_)));
	}

	#[test]
	fn test_parse_server_fn_action() {
		let input = quote! {
			name: VoteForm,
			server_fn: submit_vote,

			fields: {
				choice_id: IntegerField { required },
			},
		};

		let result: Result<FormMacro> = syn::parse2(input);
		assert!(result.is_ok());

		let form = result.unwrap();
		assert!(matches!(form.action, FormAction::ServerFn(_)));
	}

	#[test]
	fn test_parse_missing_name() {
		let input = quote! {
			action: "/api/test",

			fields: {
				field1: CharField {},
			},
		};

		let result: Result<FormMacro> = syn::parse2(input);
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("name"));
	}
}
