//! HP-001: Basic form definition.
//!
//! Tests that a minimal form with a single CharField compiles successfully.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			name: CharField {},
		},
	};
}
