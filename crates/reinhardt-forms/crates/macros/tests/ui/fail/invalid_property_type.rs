//! EP-003: Invalid property type error.
//!
//! Tests that incorrect property value types produce a compile error.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {
				max_length: "not a number",
			},
		},
	};
}
