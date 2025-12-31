//! HP-013: Help text.
//!
//! Tests that help text specifications compile successfully.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {
				required,
				help_text: "Choose a unique username (3-150 characters)",
			},
			email: EmailField {
				required,
				help_text: "We'll never share your email with anyone else",
			},
			password: CharField {
				required,
				widget: PasswordInput,
				help_text: "Must be at least 8 characters with uppercase and numbers",
			},
			bio: CharField {
				widget: TextArea,
				help_text: "Tell us about yourself (optional, max 500 characters)",
			},
			website: URLField {
				help_text: "Enter the full URL including https://",
			},
		},
	};
}
