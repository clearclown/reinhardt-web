//! EP-010: Invalid widget error.
//!
//! Tests that an unknown widget type produces a compile error.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {
				widget: UnknownWidget,
			},
		},
	};
}
