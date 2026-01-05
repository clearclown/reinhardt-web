//! State transition tests for form handling.
//!
//! Tests form state transitions: New → Bound → Validated → CleanedData/Error.
//! These tests use the Form struct directly for explicit field configuration.

use reinhardt_forms::fields::{CharField, EmailField};
use reinhardt_forms::{Form, FormError};
use rstest::rstest;
use serde_json::json;
use std::collections::HashMap;

/// ST-001: Form state transition - happy path.
///
/// Tests the complete flow: New → Bound → Validated → CleanedData
#[rstest]
fn test_form_state_transition_happy_path() {
	// Create form (state: New)
	let mut form = Form::new();
	form.add_field(Box::new(
		CharField::new("username".to_string())
			.required()
			.with_max_length(150),
	));
	form.add_field(Box::new(EmailField::new("email".to_string()).required()));

	// State: New
	assert!(!form.is_bound());
	assert!(form.errors().is_empty());
	assert_eq!(form.fields().len(), 2);

	// Bind valid data (state: Bound)
	let mut data = HashMap::new();
	data.insert("username".to_string(), json!("testuser"));
	data.insert("email".to_string(), json!("test@example.com"));
	form.bind(data);

	// State: Bound
	assert!(form.is_bound());

	// Validate (state: Validated)
	assert!(form.is_valid());

	// Get cleaned data (state: CleanedData)
	let cleaned = form.cleaned_data();
	assert_eq!(cleaned.get("username"), Some(&json!("testuser")));
	assert_eq!(cleaned.get("email"), Some(&json!("test@example.com")));
}

/// ST-002: Form state transition - validation error.
///
/// Tests: New → Bound → ValidationError
#[rstest]
fn test_form_state_transition_validation_error() {
	// Create form with required field
	let mut form = Form::new();
	form.add_field(Box::new(CharField::new("username".to_string()).required()));

	// Bind empty data (missing required field)
	let data = HashMap::new();
	form.bind(data);

	// State: Bound
	assert!(form.is_bound());

	// Validate should fail
	assert!(!form.is_valid());

	// Errors should contain error for username
	assert!(!form.errors().is_empty());
	assert!(form.errors().contains_key("username"));
}

/// ST-004: Field validator execution order.
///
/// Tests: FieldClean → FormClean
#[rstest]
fn test_validator_execution_order() {
	// Create form with field and form validators
	let mut form = Form::new();
	form.add_field(Box::new(CharField::new("password".to_string()).required()));
	form.add_field(Box::new(CharField::new("confirm".to_string()).required()));

	// Add field-level validator for password length
	form.add_field_clean_function("password", |v: &serde_json::Value| {
		if v.as_str().is_some_and(|s| s.len() >= 8) {
			Ok(v.clone())
		} else {
			Err(FormError::Validation(
				"Password must be at least 8 characters".to_string(),
			))
		}
	});

	// Add form-level validator for password matching
	form.add_clean_function(|data: &HashMap<String, serde_json::Value>| {
		let password = data.get("password").and_then(|v| v.as_str());
		let confirm = data.get("confirm").and_then(|v| v.as_str());
		if password == confirm {
			Ok(())
		} else {
			Err(FormError::Validation("Passwords do not match".to_string()))
		}
	});

	// Bind valid data
	let mut data = HashMap::new();
	data.insert("password".to_string(), json!("password123"));
	data.insert("confirm".to_string(), json!("password123"));
	form.bind(data);

	assert!(form.is_valid());
}

/// ST-005: Multiple bindings.
///
/// Tests: Bound → Rebound → Validated
#[rstest]
fn test_multiple_bindings() {
	let mut form = Form::new();
	form.add_field(Box::new(CharField::new("username".to_string()).required()));

	// First binding with invalid data
	let data = HashMap::new();
	form.bind(data);
	assert!(!form.is_valid());

	// Second binding with valid data
	let mut data2 = HashMap::new();
	data2.insert("username".to_string(), json!("validuser"));
	form.bind(data2);

	// Previous errors should be cleared, new validation runs
	assert!(form.is_valid());
	assert!(form.errors().is_empty());
}
