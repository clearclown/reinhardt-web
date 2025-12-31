//! EP-001: Unknown field type error.
//!
//! Tests that using an unknown field type produces a compile error.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			name: FooField {},
		},
	};
}
