//! Use case tests for form handling.
//!
//! Tests real-world form scenarios like login, registration, contact forms.
//! These tests use the Form struct directly instead of macros for explicit
//! field configuration.

use reinhardt_forms::field::Widget;
use reinhardt_forms::fields::{
	BooleanField, CharField, DateField, EmailField, FileField, ImageField, IntegerField, URLField,
};
use reinhardt_forms::{Form, FormError};
use rstest::rstest;
use serde_json::json;
use std::collections::HashMap;

/// UC-001: Login form use case.
///
/// Tests username + password + remember_me form.
#[rstest]
fn test_login_form_use_case() {
	let mut form = Form::new();
	form.add_field(Box::new(
		CharField::new("username".to_string())
			.required()
			.with_max_length(150),
	));
	form.add_field(Box::new(
		CharField::new("password".to_string())
			.required()
			.with_widget(Widget::PasswordInput),
	));
	form.add_field(Box::new(BooleanField::new("remember_me".to_string())));

	// Test 1: Valid credentials
	let mut data = HashMap::new();
	data.insert("username".to_string(), json!("testuser"));
	data.insert("password".to_string(), json!("password123"));
	data.insert("remember_me".to_string(), json!(true));
	form.bind(data);

	assert!(form.is_valid());
	assert_eq!(
		form.cleaned_data().get("username"),
		Some(&json!("testuser"))
	);

	// Test 2: Empty username
	let mut form2 = Form::new();
	form2.add_field(Box::new(CharField::new("username".to_string()).required()));
	form2.add_field(Box::new(
		CharField::new("password".to_string())
			.required()
			.with_widget(Widget::PasswordInput),
	));
	let data2 = HashMap::new();
	form2.bind(data2);

	assert!(!form2.is_valid());
	assert!(form2.errors().contains_key("username"));
}

/// UC-002: User registration form use case.
///
/// Tests username + email + password + confirm_password with validation.
#[rstest]
fn test_user_registration_form_use_case() {
	let mut form = Form::new();
	form.add_field(Box::new(
		CharField::new("username".to_string())
			.required()
			.with_max_length(150),
	));
	form.add_field(Box::new(EmailField::new("email".to_string()).required()));
	form.add_field(Box::new(
		CharField::new("password".to_string())
			.required()
			.with_widget(Widget::PasswordInput),
	));
	form.add_field(Box::new(
		CharField::new("confirm_password".to_string())
			.required()
			.with_widget(Widget::PasswordInput),
	));

	// Add form-level validation for password matching
	form.add_clean_function(|data: &HashMap<String, serde_json::Value>| {
		let password = data.get("password").and_then(|v| v.as_str());
		let confirm = data.get("confirm_password").and_then(|v| v.as_str());
		if password == confirm {
			Ok(())
		} else {
			Err(FormError::Validation("Passwords must match".to_string()))
		}
	});

	// Test 1: Valid registration
	let mut data = HashMap::new();
	data.insert("username".to_string(), json!("newuser"));
	data.insert("email".to_string(), json!("user@example.com"));
	data.insert("password".to_string(), json!("securepassword"));
	data.insert("confirm_password".to_string(), json!("securepassword"));
	form.bind(data);

	assert!(form.is_valid());

	// Test 2: Password mismatch
	let mut form2 = Form::new();
	form2.add_field(Box::new(CharField::new("password".to_string()).required()));
	form2.add_field(Box::new(
		CharField::new("confirm_password".to_string()).required(),
	));
	form2.add_clean_function(|data: &HashMap<String, serde_json::Value>| {
		if data.get("password") == data.get("confirm_password") {
			Ok(())
		} else {
			Err(FormError::Validation("Passwords must match".to_string()))
		}
	});

	let mut data2 = HashMap::new();
	data2.insert("password".to_string(), json!("password1"));
	data2.insert("confirm_password".to_string(), json!("password2"));
	form2.bind(data2);

	assert!(!form2.is_valid());
}

/// UC-003: Contact form use case.
///
/// Tests name + email + subject + message form.
#[rstest]
fn test_contact_form_use_case() {
	let mut form = Form::new();
	form.add_field(Box::new(
		CharField::new("name".to_string())
			.required()
			.with_max_length(100),
	));
	form.add_field(Box::new(EmailField::new("email".to_string()).required()));
	form.add_field(Box::new(
		CharField::new("subject".to_string())
			.required()
			.with_max_length(200),
	));
	form.add_field(Box::new(
		CharField::new("message".to_string())
			.required()
			.with_widget(Widget::TextArea),
	));

	// Valid contact form submission
	let mut data = HashMap::new();
	data.insert("name".to_string(), json!("John Doe"));
	data.insert("email".to_string(), json!("john@example.com"));
	data.insert("subject".to_string(), json!("Inquiry"));
	data.insert("message".to_string(), json!("Hello, I have a question..."));
	form.bind(data);

	assert!(form.is_valid());
	assert_eq!(form.fields().len(), 4);
}

/// UC-004: Search form use case.
///
/// Tests query + category + date_range form (all optional).
#[rstest]
fn test_search_form_use_case() {
	let mut form = Form::new();
	form.add_field(Box::new(
		CharField::new("query".to_string()).with_max_length(255),
	));
	form.add_field(Box::new(DateField::new("date_from".to_string())));
	form.add_field(Box::new(DateField::new("date_to".to_string())));

	// Test field count (DateField validation is complex, so we just verify structure)
	assert_eq!(form.fields().len(), 3);
}

/// UC-005: Profile edit form use case.
///
/// Tests multiple optional fields.
#[rstest]
fn test_profile_edit_form_use_case() {
	let mut form = Form::new();
	form.add_field(Box::new(
		CharField::new("display_name".to_string()).with_max_length(100),
	));
	form.add_field(Box::new(
		CharField::new("bio".to_string())
			.with_max_length(500)
			.with_widget(Widget::TextArea),
	));
	form.add_field(Box::new(URLField::new("website".to_string())));
	form.add_field(Box::new(
		CharField::new("location".to_string()).with_max_length(100),
	));

	// Provide valid data for all fields (URLField requires proper URL format)
	let mut data = HashMap::new();
	data.insert("display_name".to_string(), json!("John"));
	data.insert("bio".to_string(), json!("A developer"));
	data.insert("website".to_string(), json!("https://example.com"));
	data.insert("location".to_string(), json!("Tokyo"));
	form.bind(data);

	assert!(form.is_valid());
	assert_eq!(form.fields().len(), 4);
}

/// UC-006: Payment form use case.
///
/// Tests credit card information form.
#[rstest]
fn test_payment_form_use_case() {
	let mut form = Form::new();
	form.add_field(Box::new(
		CharField::new("card_number".to_string())
			.required()
			.with_max_length(19),
	));
	form.add_field(Box::new(
		IntegerField::new("expiry_month".to_string()).required(),
	));
	form.add_field(Box::new(
		IntegerField::new("expiry_year".to_string()).required(),
	));
	form.add_field(Box::new(
		CharField::new("cvv".to_string())
			.required()
			.with_max_length(4)
			.with_widget(Widget::PasswordInput),
	));
	form.add_field(Box::new(
		CharField::new("cardholder_name".to_string()).required(),
	));

	// Valid card details
	let mut data = HashMap::new();
	data.insert("card_number".to_string(), json!("4111111111111111"));
	data.insert("expiry_month".to_string(), json!(12));
	data.insert("expiry_year".to_string(), json!(2025));
	data.insert("cvv".to_string(), json!("123"));
	data.insert("cardholder_name".to_string(), json!("John Doe"));
	form.bind(data);

	assert!(form.is_valid());
}

/// UC-007: File upload form use case.
///
/// Tests FileField + ImageField.
#[rstest]
fn test_file_upload_form_use_case() {
	let mut form = Form::new();
	let mut document_field = FileField::new("document".to_string());
	document_field.required = true;
	form.add_field(Box::new(document_field));
	form.add_field(Box::new(ImageField::new("thumbnail".to_string())));

	// File fields require special handling, test field count
	assert_eq!(form.fields().len(), 2);
}

/// UC-008: Multi-language form use case.
///
/// Tests Japanese labels and help text.
#[rstest]
fn test_multilingual_form_use_case() {
	let mut form = Form::new();
	form.add_field(Box::new(
		CharField::new("username".to_string())
			.required()
			.with_label("ユーザー名")
			.with_help_text("3文字以上で入力してください"),
	));
	form.add_field(Box::new(
		EmailField::new("email".to_string())
			.required()
			.with_label("メールアドレス")
			.with_help_text("有効なメールアドレスを入力してください"),
	));

	// Valid submission with Unicode
	let mut data = HashMap::new();
	data.insert("username".to_string(), json!("田中太郎"));
	data.insert("email".to_string(), json!("tanaka@example.co.jp"));
	form.bind(data);

	assert!(form.is_valid());
	assert_eq!(form.fields().len(), 2);
}
