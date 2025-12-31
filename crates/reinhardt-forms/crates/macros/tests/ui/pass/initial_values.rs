//! HP-012: Initial values.
//!
//! Tests that initial value specifications compile successfully.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {
				initial: "anonymous",
			},
			email: EmailField {
				initial: "user@example.com",
			},
			age: IntegerField {
				initial: 25,
			},
			rating: FloatField {
				initial: 4.5,
			},
			is_active: BooleanField {
				initial: true,
			},
			country: ChoiceField {
				initial: "US",
			},
		},
	};
}
