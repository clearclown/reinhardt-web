//! EP-009: Empty fields section error.
//!
//! Tests that an empty fields section produces a compile error.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {},
	};
}
