//! Field Reference Macro Integration Tests
//!
//! Tests that verify the `#[derive(Model)]` macro correctly generates
//! `field_*()` accessor methods that return type-safe `FieldRef<M, T>`.

use reinhardt_macros::Model;
use reinhardt_orm::{Model as ModelTrait, expressions::FieldRef};
use serde::{Deserialize, Serialize};

/// Test model with various field types
#[derive(Model, Debug, Clone, Serialize, Deserialize)]
#[model(app_label = "test_app", table_name = "test_users")]
struct User {
	#[field(primary_key = true)]
	id: i64,
	#[field(max_length = 100)]
	username: String,
	#[field(max_length = 255)]
	email: String,
	age: Option<i32>,
}

#[test]
fn test_macro_generates_field_id_accessor() {
	let id_ref = User::field_id();
	assert_eq!(id_ref.name(), "id");
	assert_eq!(id_ref.to_sql(), "id");
	assert_eq!(format!("{}", id_ref), "id");
}

#[test]
fn test_macro_generates_field_username_accessor() {
	let username_ref = User::field_username();
	assert_eq!(username_ref.name(), "username");
	assert_eq!(username_ref.to_sql(), "username");
}

#[test]
fn test_macro_generates_field_email_accessor() {
	let email_ref = User::field_email();
	assert_eq!(email_ref.name(), "email");
	assert_eq!(email_ref.to_sql(), "email");
}

#[test]
fn test_macro_generates_field_age_accessor() {
	let age_ref = User::field_age();
	assert_eq!(age_ref.name(), "age");
	assert_eq!(age_ref.to_sql(), "age");
}

#[test]
fn test_field_ref_preserves_type_safety() {
	// These assertions verify that the FieldRef types are correct at compile time
	let _id_ref: FieldRef<User, i64> = User::field_id();
	let _username_ref: FieldRef<User, String> = User::field_username();
	let _email_ref: FieldRef<User, String> = User::field_email();
	let _age_ref: FieldRef<User, Option<i32>> = User::field_age();
}

#[test]
fn test_field_ref_const_evaluation() {
	// FieldRef::new() is const, so these can be evaluated at compile time
	const ID_REF: FieldRef<User, i64> = FieldRef::new("id");
	assert_eq!(ID_REF.name(), "id");
}

#[test]
fn test_model_trait_still_works() {
	// Verify that Model trait implementation is not broken by field accessor generation
	assert_eq!(User::table_name(), "test_users");
	assert_eq!(User::app_label(), "test_app");
	assert_eq!(User::primary_key_field(), "id");
}
