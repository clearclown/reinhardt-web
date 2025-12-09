//! Test data builders.
//!
//! Provides builder patterns for creating test data.

use serde_json::{json, Value};

/// Builder for registration request JSON.
pub struct RegisterRequestBuilder {
	email: String,
	username: String,
	password: String,
	password_confirmation: String,
}

impl RegisterRequestBuilder {
	/// Create a new builder with default values.
	pub fn new() -> Self {
		Self {
			email: "test@example.com".to_string(),
			username: "testuser".to_string(),
			password: "password123".to_string(),
			password_confirmation: "password123".to_string(),
		}
	}

	/// Set email.
	pub fn email(mut self, email: impl Into<String>) -> Self {
		self.email = email.into();
		self
	}

	/// Set username.
	pub fn username(mut self, username: impl Into<String>) -> Self {
		self.username = username.into();
		self
	}

	/// Set password (also sets confirmation).
	pub fn password(mut self, password: impl Into<String>) -> Self {
		let pwd = password.into();
		self.password = pwd.clone();
		self.password_confirmation = pwd;
		self
	}

	/// Set password and confirmation separately.
	pub fn password_with_confirmation(
		mut self,
		password: impl Into<String>,
		confirmation: impl Into<String>,
	) -> Self {
		self.password = password.into();
		self.password_confirmation = confirmation.into();
		self
	}

	/// Build the JSON value.
	pub fn build(self) -> Value {
		json!({
			"email": self.email,
			"username": self.username,
			"password": self.password,
			"password_confirmation": self.password_confirmation,
		})
	}
}

impl Default for RegisterRequestBuilder {
	fn default() -> Self {
		Self::new()
	}
}

/// Builder for signin request JSON.
pub struct SigninRequestBuilder {
	email: String,
	password: String,
}

impl SigninRequestBuilder {
	/// Create a new builder with default values.
	pub fn new() -> Self {
		Self {
			email: "test@example.com".to_string(),
			password: "password123".to_string(),
		}
	}

	/// Set email.
	pub fn email(mut self, email: impl Into<String>) -> Self {
		self.email = email.into();
		self
	}

	/// Set password.
	pub fn password(mut self, password: impl Into<String>) -> Self {
		self.password = password.into();
		self
	}

	/// Build the JSON value.
	pub fn build(self) -> Value {
		json!({
			"email": self.email,
			"password": self.password,
		})
	}
}

impl Default for SigninRequestBuilder {
	fn default() -> Self {
		Self::new()
	}
}
