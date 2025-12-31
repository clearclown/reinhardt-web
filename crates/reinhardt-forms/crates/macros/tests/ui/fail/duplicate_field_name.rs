//! EP-002: Duplicate field name error.
//!
//! Tests that duplicate field names produce a compile error.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {},
			username: EmailField {},
		},
	};
}
