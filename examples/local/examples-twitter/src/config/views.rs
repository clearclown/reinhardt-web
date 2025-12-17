//! Common views for the project
//!
//! Health check and root endpoints

use reinhardt::get;
use reinhardt::http::ViewResult;
use reinhardt::{Request, Response, StatusCode};
use std::path::PathBuf;

/// Health check endpoint
#[get("/health", name = "health")]
pub async fn health_check() -> ViewResult<Response> {
	Ok(Response::new(StatusCode::OK).with_body("OK".as_bytes().to_vec()))
}

/// Serve admin static files (WASM UI)
#[get("/admin/*", name = "admin_static")]
pub async fn serve_admin_static(req: Request) -> ViewResult<Response> {
	// Extract path from request (e.g., /admin/index.html -> index.html)
	let req_path = req.uri.path();
	let path = req_path.strip_prefix("/admin/").unwrap_or(req_path);

	// If path is empty or ends with /, serve index.html
	let path = if path.is_empty() || path.ends_with('/') {
		"index.html"
	} else {
		path
	};

	// Serve from static/admin directory
	let root = PathBuf::from("static/admin");
	let handler = reinhardt::utils::r#static::StaticFileHandler::new(root);

	match handler.serve(path).await {
		Ok(file) => {
			Ok(Response::new(StatusCode::OK)
				.with_header("Content-Type", &file.mime_type)
				.with_body(file.content))
		}
		Err(e) => {
			eprintln!("Static file error for path '{}': {}", path, e);
			Ok(Response::new(StatusCode::NOT_FOUND)
				.with_body(format!("File not found: {}", path).into_bytes()))
		}
	}
}
