//! Boundary value tests for the `form!` macro.
//!
//! Tests values at and around boundaries to ensure proper handling.

use quote::quote;
use reinhardt_forms_macros_ast::FormMacro;
use rstest::rstest;

/// BV-001: max_length boundary values.
/// Tests values at, below, and above typical boundaries.
#[rstest]
#[case(0, true)] // Minimum boundary
#[case(1, true)] // Just above minimum
#[case(255, true)] // Common maximum for strings
#[case(256, true)] // Just above 255
#[case(65535, true)] // u16 max
#[case(65536, true)] // Just above u16 max
fn test_max_length_boundary(#[case] max_len: usize, #[case] should_parse: bool) {
	let tokens = quote! {
		fields: {
			username: CharField {
				max_length: #max_len,
			},
		}
	};

	let result: syn::Result<FormMacro> = syn::parse2(tokens);
	assert_eq!(
		result.is_ok(),
		should_parse,
		"max_length: {} should {} parse",
		max_len,
		if should_parse {
			"successfully"
		} else {
			"fail to"
		}
	);
}

/// BV-002: min_length boundary values.
#[rstest]
#[case(0, true)] // Zero is valid (no minimum)
#[case(1, true)] // Minimum meaningful value
#[case(2, true)] // Just above minimum
#[case(100, true)] // Typical value
fn test_min_length_boundary(#[case] min_len: usize, #[case] should_parse: bool) {
	let tokens = quote! {
		fields: {
			username: CharField {
				min_length: #min_len,
			},
		}
	};

	let result: syn::Result<FormMacro> = syn::parse2(tokens);
	assert_eq!(
		result.is_ok(),
		should_parse,
		"min_length: {} should {} parse",
		min_len,
		if should_parse {
			"successfully"
		} else {
			"fail to"
		}
	);
}

/// BV-003: Field count boundaries.
#[rstest]
#[case(1, true)] // Minimum: at least one field
#[case(2, true)] // Just above minimum
#[case(10, true)] // Typical form
#[case(50, true)] // Large form
#[case(100, true)] // Very large form
fn test_field_count_boundary(#[case] field_count: usize, #[case] should_parse: bool) {
	let mut field_tokens = Vec::new();
	for i in 0..field_count {
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
	assert_eq!(
		result.is_ok(),
		should_parse,
		"{} fields should {} parse",
		field_count,
		if should_parse {
			"successfully"
		} else {
			"fail to"
		}
	);

	if result.is_ok() {
		assert_eq!(result.unwrap().fields.len(), field_count);
	}
}

/// BV-004: Validator count boundaries.
#[rstest]
#[case(0, true)] // No validators
#[case(1, true)] // Single validator
#[case(2, true)] // Two validators
#[case(5, true)] // Multiple validators
#[case(10, true)] // Many validators
fn test_validator_count_boundary(#[case] validator_count: usize, #[case] should_parse: bool) {
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
	assert_eq!(
		result.is_ok(),
		should_parse,
		"{} validators should {} parse",
		validator_count,
		if should_parse {
			"successfully"
		} else {
			"fail to"
		}
	);
}

/// BV-005: Property count per field boundaries.
#[rstest]
#[case(0, true)] // No properties (empty braces)
#[case(1, true)] // Single property
#[case(2, true)] // Two properties
#[case(5, true)] // Multiple properties
#[case(10, true)] // Many properties
fn test_property_count_boundary(#[case] property_count: usize, #[case] should_parse: bool) {
	let mut property_tokens = Vec::new();

	// Generate different property types to avoid duplicates
	let property_defs = vec![
		quote! { required },
		quote! { max_length: 100 },
		quote! { min_length: 1 },
		quote! { label: "Label" },
		quote! { help_text: "Help" },
		quote! { initial: "default" },
		quote! { widget: TextInput },
	];

	for i in 0..property_count.min(property_defs.len()) {
		property_tokens.push(&property_defs[i]);
	}

	let tokens = quote! {
		fields: {
			username: CharField {
				#(#property_tokens,)*
			},
		}
	};

	let result: syn::Result<FormMacro> = syn::parse2(tokens);
	assert_eq!(
		result.is_ok(),
		should_parse,
		"{} properties should {} parse",
		property_count,
		if should_parse {
			"successfully"
		} else {
			"fail to"
		}
	);
}
