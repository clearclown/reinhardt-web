//! EP-007: Invalid closure arguments error.
//!
//! Tests that a validator closure with wrong argument count produces a compile error.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {},
		},
		validators: {
			username: [
				|a, b, c| true => "Too many arguments",
			],
		},
	};
}
