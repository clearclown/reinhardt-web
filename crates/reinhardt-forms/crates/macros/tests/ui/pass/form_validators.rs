//! HP-008: Form-level validators.
//!
//! Tests that form-level validators (@form) compile successfully.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			password: CharField {
				required,
			},
			confirm_password: CharField {
				required,
			},
			start_date: DateField {},
			end_date: DateField {},
		},
		validators: {
			@form: [
				|data| data["password"] == data["confirm_password"]
					=> "Passwords must match",
				|data| data["start_date"] <= data["end_date"]
					=> "Start date must be before or equal to end date",
			],
		},
	};
}
