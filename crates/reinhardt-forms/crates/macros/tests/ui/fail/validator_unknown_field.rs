//! EP-006: Validator references unknown field error.
//!
//! Tests that a validator referencing an undefined field produces a compile error.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {},
		},
		validators: {
			unknown_field: [
				|v| v.len() >= 3 => "Must be at least 3 characters",
			],
		},
	};
}
