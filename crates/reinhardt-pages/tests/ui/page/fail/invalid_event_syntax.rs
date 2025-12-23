//! page! macro with invalid event syntax

use reinhardt_pages::page;

fn main() {
	// Missing handler after @event: - should fail
	let _invalid = page!(|| {
	button {
		@click: ,
		"Click"
	}
});
}
