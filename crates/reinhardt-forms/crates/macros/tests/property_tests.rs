//! Property-based tests for the `form!` macro using proptest.
//!
//! These tests verify invariants that should hold for all valid inputs.

use proptest::prelude::*;
use quote::quote;
use reinhardt_forms_macros_ast::FormMacro;

/// Strategy for generating valid field names.
fn field_name_strategy() -> impl Strategy<Value = String> {
	// Rust identifier: start with letter or underscore, followed by alphanumeric or underscore
	prop::string::string_regex("[a-z][a-z0-9_]{0,20}")
		.expect("Valid regex")
		.prop_filter("avoid Rust keywords", |s| {
			!matches!(
				s.as_str(),
				"type"
					| "struct" | "enum"
					| "fn" | "let" | "mut"
					| "const" | "static"
					| "if" | "else" | "match"
					| "for" | "while"
					| "loop" | "break"
					| "continue" | "return"
					| "pub" | "mod" | "use"
					| "impl" | "trait"
					| "where" | "async"
					| "await" | "move"
					| "self" | "super"
					| "crate" | "true"
					| "false" | "as"
					| "in" | "ref" | "dyn"
			)
		})
}

/// Strategy for generating field counts.
fn field_count_strategy() -> impl Strategy<Value = usize> {
	1usize..20
}

/// Strategy for generating max_length values.
fn max_length_strategy() -> impl Strategy<Value = usize> {
	0usize..10000
}

proptest! {
	/// PB-001: Field count is preserved after parsing.
	#[test]
	fn property_field_count_preserved(field_count in field_count_strategy()) {
		let mut field_tokens = Vec::new();
		for i in 0..field_count {
			let field_name = syn::Ident::new(
				&format!("field_{}", i),
				proc_macro2::Span::call_site(),
			);
			field_tokens.push(quote! {
				#field_name: CharField {},
			});
		}

		let tokens = quote! {
			fields: {
				#(#field_tokens)*
			}
		};

		let result: syn::Result<FormMacro> = syn::parse2(tokens);
		prop_assert!(result.is_ok(), "Parsing should succeed for {} fields", field_count);

		let form = result.unwrap();
		prop_assert_eq!(
			form.fields.len(),
			field_count,
			"Field count should be preserved"
		);
	}

	/// PB-002: Field order is preserved after parsing.
	#[test]
	fn property_field_order_preserved(field_count in 2usize..10) {
		let field_names: Vec<String> = (0..field_count)
			.map(|i| format!("field_{}", i))
			.collect();

		let mut field_tokens = Vec::new();
		for name in &field_names {
			let field_name = syn::Ident::new(name, proc_macro2::Span::call_site());
			field_tokens.push(quote! {
				#field_name: CharField {},
			});
		}

		let tokens = quote! {
			fields: {
				#(#field_tokens)*
			}
		};

		let result: syn::Result<FormMacro> = syn::parse2(tokens);
		prop_assert!(result.is_ok());

		let form = result.unwrap();
		for (i, field) in form.fields.iter().enumerate() {
			prop_assert_eq!(
				field.name.to_string(),
				field_names[i].clone(),
				"Field order should be preserved at index {}", i
			);
		}
	}

	/// PB-003: Validator count is preserved after parsing.
	#[test]
	fn property_validator_count_preserved(validator_count in 0usize..10) {
		let mut validator_tokens = Vec::new();
		for i in 0..validator_count {
			let msg = format!("Validation error {}", i);
			validator_tokens.push(quote! {
				|v| v.len() > #i => #msg,
			});
		}

		let tokens = if validator_count > 0 {
			quote! {
				fields: {
					username: CharField {},
				},
				validators: {
					username: [
						#(#validator_tokens)*
					],
				}
			}
		} else {
			quote! {
				fields: {
					username: CharField {},
				}
			}
		};

		let result: syn::Result<FormMacro> = syn::parse2(tokens);
		prop_assert!(result.is_ok());

		let form = result.unwrap();

		let total_rules: usize = form
			.validators
			.iter()
			.map(|v| match v {
				reinhardt_forms_macros_ast::FormValidator::Field { rules, .. } => rules.len(),
				reinhardt_forms_macros_ast::FormValidator::Form { rules, .. } => rules.len(),
			})
			.sum();

		prop_assert_eq!(
			total_rules,
			validator_count,
			"Validator count should be preserved"
		);
	}

	/// PB-004: max_length property value is preserved.
	#[test]
	fn property_max_length_preserved(max_len in max_length_strategy()) {
		let tokens = quote! {
			fields: {
				username: CharField {
					max_length: #max_len,
				},
			}
		};

		let result: syn::Result<FormMacro> = syn::parse2(tokens);
		prop_assert!(result.is_ok());

		let form = result.unwrap();
		let field = &form.fields[0];

		// Find the max_length property
		let max_length_prop = field.get_property("max_length");
		prop_assert!(
			max_length_prop.is_some(),
			"max_length property should be present"
		);

		// Parse the value and compare
		if let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Int(lit_int), .. }) = max_length_prop.unwrap() {
			let parsed_value: usize = lit_int.base10_parse().unwrap();
			prop_assert_eq!(
				parsed_value,
				max_len,
				"max_length value should be preserved"
			);
		} else {
			prop_assert!(false, "max_length should be an integer literal");
		}
	}

	/// PB-005: Valid field names always parse successfully.
	#[test]
	fn property_valid_field_names_parse(name in field_name_strategy()) {
		let field_name = syn::Ident::new(&name, proc_macro2::Span::call_site());

		let tokens = quote! {
			fields: {
				#field_name: CharField {},
			}
		};

		let result: syn::Result<FormMacro> = syn::parse2(tokens);
		prop_assert!(
			result.is_ok(),
			"Valid field name '{}' should parse successfully",
			name
		);

		let form = result.unwrap();
		prop_assert_eq!(
			form.fields[0].name.to_string(),
			name,
			"Field name should be preserved"
		);
	}
}
