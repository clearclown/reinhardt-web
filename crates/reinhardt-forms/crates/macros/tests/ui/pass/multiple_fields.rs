//! HP-003: Multiple fields form.
//!
//! Tests that a form with multiple fields of different types compiles successfully.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {},
			email: EmailField {},
			age: IntegerField {},
			bio: CharField {},
			is_active: BooleanField {},
		},
	};
}
