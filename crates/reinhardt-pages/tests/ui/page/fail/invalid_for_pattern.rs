//! page! macro with invalid for loop pattern

use reinhardt_pages::page;

fn main() {
	// Missing 'in' keyword in for loop
	let _invalid = page!(|items: Vec<String>| {
	ul {
		for item
		items {
			li {
 {
					item
				}
			}
		}
	}
});
}
