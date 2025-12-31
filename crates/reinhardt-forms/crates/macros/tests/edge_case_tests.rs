//! Edge case tests for the `form!` macro.
//!
//! Tests unusual but valid inputs that might expose edge case bugs.

use quote::quote;
use reinhardt_forms_macros_ast::FormMacro;
use rstest::rstest;

/// EC-001: Very long field name (255 characters).
#[rstest]
fn test_long_field_name() {
	let long_name = "a".repeat(255);
	let long_ident = syn::Ident::new(&long_name, proc_macro2::Span::call_site());

	let tokens = quote! {
		fields: {
			#long_ident: CharField {},
		}
	};

	let result: syn::Result<FormMacro> = syn::parse2(tokens);
	assert!(result.is_ok(), "Long field name should parse successfully");

	let form = result.unwrap();
	assert_eq!(form.fields.len(), 1);
	assert_eq!(form.fields[0].name.to_string(), long_name);
}

/// EC-002: Unicode characters in labels.
#[rstest]
#[case("ユーザー名")]
#[case("用户名")]
#[case("사용자 이름")]
#[case("имя пользователя")]
#[case("اسم المستخدم")]
fn test_unicode_label(#[case] label: &str) {
	let tokens = quote! {
		fields: {
			username: CharField {
				label: #label,
			},
		}
	};

	let result: syn::Result<FormMacro> = syn::parse2(tokens);
	assert!(
		result.is_ok(),
		"Unicode label '{}' should parse successfully",
		label
	);
}

/// EC-003: Special characters in help text.
#[rstest]
#[case("<script>alert('xss')</script>")]
#[case("Line1\nLine2")]
#[case("Tab\there")]
#[case("Special chars: <>&\"'")]
#[case("Emoji: 🎉🚀💻")]
fn test_special_chars_help_text(#[case] help_text: &str) {
	let tokens = quote! {
		fields: {
			username: CharField {
				help_text: #help_text,
			},
		}
	};

	let result: syn::Result<FormMacro> = syn::parse2(tokens);
	assert!(
		result.is_ok(),
		"Special characters in help_text should parse successfully"
	);
}

/// EC-004: max_length = 0.
#[rstest]
fn test_max_length_zero() {
	let tokens = quote! {
		fields: {
			username: CharField {
				max_length: 0,
			},
		}
	};

	let result: syn::Result<FormMacro> = syn::parse2(tokens);
	assert!(result.is_ok(), "max_length: 0 should parse successfully");
}

/// EC-005: Large max_length value.
#[rstest]
fn test_max_length_large_value() {
	let tokens = quote! {
		fields: {
			username: CharField {
				max_length: 1000000,
			},
		}
	};

	let result: syn::Result<FormMacro> = syn::parse2(tokens);
	assert!(
		result.is_ok(),
		"Large max_length value should parse successfully"
	);
}

/// EC-006: Empty label string.
#[rstest]
fn test_empty_label() {
	let tokens = quote! {
		fields: {
			username: CharField {
				label: "",
			},
		}
	};

	let result: syn::Result<FormMacro> = syn::parse2(tokens);
	assert!(result.is_ok(), "Empty label should parse successfully");
}

/// EC-007: Many fields (100+).
#[rstest]
fn test_many_fields() {
	// Generate 100 fields dynamically
	let mut field_tokens = Vec::new();
	for i in 0..100 {
		let field_name = syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site());
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
	assert!(result.is_ok(), "100 fields should parse successfully");

	let form = result.unwrap();
	assert_eq!(form.fields.len(), 100);
}

/// EC-008: Complex nested validator expression.
#[rstest]
fn test_nested_validator_expression() {
	let tokens = quote! {
		fields: {
			username: CharField {},
		},
		validators: {
			username: [
				|v| {
					let len = v.len();
					let is_valid = len >= 3 && len <= 100;
					let has_valid_chars = v.chars().all(|c| c.is_alphanumeric() || c == '_');
					is_valid && has_valid_chars
				} => "Invalid username format",
			],
		}
	};

	let result: syn::Result<FormMacro> = syn::parse2(tokens);
	assert!(
		result.is_ok(),
		"Complex nested validator expression should parse successfully"
	);
}

/// EC-009: Rust reserved word as field name (with raw identifier).
/// Note: Direct reserved words would cause syntax errors, but raw identifiers work.
#[rstest]
#[case("r#type")]
#[case("r#async")]
#[case("r#await")]
#[case("r#struct")]
fn test_raw_identifier_field_name(#[case] raw_ident: &str) {
	let ident = syn::Ident::new_raw(&raw_ident[2..], proc_macro2::Span::call_site());

	let tokens = quote! {
		fields: {
			#ident: CharField {},
		}
	};

	let result: syn::Result<FormMacro> = syn::parse2(tokens);
	assert!(
		result.is_ok(),
		"Raw identifier '{}' should parse successfully",
		raw_ident
	);
}

/// EC-010: Field name starting with underscore.
#[rstest]
fn test_underscore_prefix_field_name() {
	let tokens = quote! {
		fields: {
			_private_field: CharField {},
			__dunder_field: CharField {},
		}
	};

	let result: syn::Result<FormMacro> = syn::parse2(tokens);
	assert!(
		result.is_ok(),
		"Field names starting with underscore should parse successfully"
	);
}
