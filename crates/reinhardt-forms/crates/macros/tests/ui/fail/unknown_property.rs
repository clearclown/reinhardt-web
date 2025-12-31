//! EP-005: Unknown property error.
//!
//! Tests that an unknown property name produces a compile error.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {
				unknown_property: "value",
			},
		},
	};
}
