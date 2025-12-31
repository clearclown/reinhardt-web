//! HP-007: Field-level validators.
//!
//! Tests that field-level validators compile successfully.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {
				required,
				max_length: 150,
			},
			password: CharField {
				required,
			},
		},
		validators: {
			username: [
				|v| v.len() >= 3 => "Username must be at least 3 characters",
				|v| !v.contains(' ') => "Username cannot contain spaces",
			],
			password: [
				|v| v.len() >= 8 => "Password must be at least 8 characters",
				|v| v.chars().any(|c| c.is_uppercase()) => "Password must contain an uppercase letter",
				|v| v.chars().any(|c| c.is_numeric()) => "Password must contain a number",
			],
		},
	};
}
