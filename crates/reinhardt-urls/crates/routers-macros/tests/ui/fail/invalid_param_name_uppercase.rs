use reinhardt_urls::routers_macros::path;

fn main() {
	let _ = path!("/users/{userId}/");
}
