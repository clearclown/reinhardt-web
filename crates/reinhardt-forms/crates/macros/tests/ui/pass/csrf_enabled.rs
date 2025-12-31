//! HP-006: CSRF enabled.
//!
//! Tests that CSRF configuration compiles successfully.

use reinhardt_forms_macros::form;

fn main() {
	// Simple CSRF enabled
	let _form1 = form! {
		csrf: true,
		fields: {
			name: CharField {},
		},
	};

	// CSRF disabled
	let _form2 = form! {
		csrf: false,
		fields: {
			name: CharField {},
		},
	};

	// CSRF with custom secret key
	let _form3 = form! {
		csrf: {
			enabled: true,
			secret_key: "my-secret-key",
		},
		fields: {
			name: CharField {},
		},
	};
}
