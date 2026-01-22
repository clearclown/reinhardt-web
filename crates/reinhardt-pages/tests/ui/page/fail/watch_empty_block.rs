//! Test: Watch block with empty content (should fail)
//!
//! A watch block must contain exactly one expression.
//! An empty watch block is not allowed.

// reinhardt-fmt: ignore-all

use reinhardt_pages::page;

fn main() {
	// Error: Empty watch block is not allowed
	let _invalid = page!(|| {
		div {
			watch {
			}
		}
	});
}
