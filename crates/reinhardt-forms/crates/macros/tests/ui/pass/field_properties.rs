//! HP-005: Field properties.
//!
//! Tests that all field properties compile successfully.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {
				required,
				max_length: 150,
				min_length: 3,
				label: "Username",
				help_text: "Enter your username",
				initial: "anonymous",
			},
			email: EmailField {
				required,
				label: "Email Address",
				help_text: "We'll never share your email",
			},
			age: IntegerField {
				min_value: 0,
				max_value: 150,
				label: "Age",
			},
			website: URLField {
				label: "Website URL",
				help_text: "Your personal website",
			},
		},
	};
}
