//! HP-002: Single field form.
//!
//! Tests that a form with a single CharField compiles successfully.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {},
		},
	};
}
