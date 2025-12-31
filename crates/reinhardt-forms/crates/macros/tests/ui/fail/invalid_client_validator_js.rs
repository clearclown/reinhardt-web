//! EP-012: Invalid client validator JavaScript error.
//!
//! Tests that an invalid JavaScript expression in client_validators produces a compile error.
//! Note: The actual JS validation will be done at runtime, but syntax errors should be caught.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {},
		},
		client_validators: {
			username: [
				123 => "Expected a string expression",
			],
		},
	};
}
