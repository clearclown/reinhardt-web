//! page! macro must have a body

use reinhardt_pages::page;

fn main() {
	// Missing body - should fail
	let _invalid = page!(||);
}
