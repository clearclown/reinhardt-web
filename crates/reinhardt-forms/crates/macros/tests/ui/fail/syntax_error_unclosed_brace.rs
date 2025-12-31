//! EP-008: Syntax error - unclosed brace.
//!
//! Tests that an unclosed brace produces a compile error.

use reinhardt_forms_macros::form;

fn main() {
	let _form = form! {
		fields: {
			username: CharField {
		},
	};
}
