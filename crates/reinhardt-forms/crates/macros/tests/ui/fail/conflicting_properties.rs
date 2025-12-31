//! EP-011: Conflicting properties error.
//!
//! Tests that conflicting properties produce a compile error.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {
				min_length: 10,
				max_length: 5,
			},
		},
	};
}
