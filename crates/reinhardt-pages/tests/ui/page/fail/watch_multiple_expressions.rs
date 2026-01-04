//! Test: Watch block with multiple expressions (should fail)
//!
//! A watch block must contain exactly one expression.
//! Multiple child nodes are not allowed.

// reinhardt-fmt: ignore-all

use reinhardt_pages::page;

fn main() {
	// Error: Watch block cannot have multiple expressions
	let _invalid = page!(|| {
		div {
			watch {
				span { "First" }
				span { "Second" }
			}
		}
	});
}
