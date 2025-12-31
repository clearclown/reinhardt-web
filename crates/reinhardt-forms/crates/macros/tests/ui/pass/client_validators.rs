//! HP-009: Client-side validators.
//!
//! Tests that client-side validators (JavaScript expressions) compile successfully.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {
				required,
				max_length: 150,
			},
			email: EmailField {
				required,
			},
			password: CharField {
				required,
			},
		},
		client_validators: {
			username: [
				"value.length >= 3" => "Username must be at least 3 characters",
				"!/\\s/.test(value)" => "Username cannot contain spaces",
			],
			email: [
				"/^[^@]+@[^@]+\\.[^@]+$/.test(value)" => "Please enter a valid email address",
			],
			password: [
				"value.length >= 8" => "Password must be at least 8 characters",
				"/[A-Z]/.test(value)" => "Password must contain an uppercase letter",
				"/[0-9]/.test(value)" => "Password must contain a number",
			],
		},
	};
}
