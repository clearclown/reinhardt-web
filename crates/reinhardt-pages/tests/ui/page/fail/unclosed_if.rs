//! page! macro with unclosed if block

use reinhardt_pages::page;

fn main() {
	// Missing closing brace for if block
	let _invalid = page!(|show: bool| {
	div {
		if show {
			span { "Visible" }
		// Missing closing brace here
	}
});
}
