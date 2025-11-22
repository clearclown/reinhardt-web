//! Views for hello app
//!
//! Simple hello world view

use reinhardt_http::{Request, Response, StatusCode, ViewResult};
use reinhardt_macros::endpoint;

/// Hello World view
///
/// Returns a simple "Hello, World!" response
#[endpoint]
pub async fn hello_world(_req: Request) -> ViewResult<Response> {
	Ok(Response::new(
		StatusCode::OK,
		"Hello, World!".as_bytes().to_vec(),
	))
}
