//! Injectable implementations for Server Functions
//!
//! This module provides Injectable trait implementations for types commonly
//! used in server function handlers, enabling automatic dependency injection.

use async_trait::async_trait;
use reinhardt_di::{DiError, DiResult, Injectable, InjectionContext};
use reinhardt_http::Request;
use std::sync::Arc;

/// Wrapper for Request that can be injected into server function handlers.
///
/// This allows server functions to access the HTTP request via dependency injection
/// rather than receiving it as a direct parameter.
pub struct ServerFnRequest(pub Arc<Request>);

impl ServerFnRequest {
	/// Returns a reference to the inner Request.
	pub fn inner(&self) -> &Request {
		&self.0
	}

	/// Consumes self and returns the inner Request.
	pub fn into_inner(self) -> Arc<Request> {
		self.0
	}
}

#[async_trait]
impl Injectable for ServerFnRequest {
	async fn inject(ctx: &InjectionContext) -> DiResult<Self> {
		// Try to get Request from the injection context
		// The request should be set via ctx.with_request() when handling the HTTP request
		ctx.get_request::<Request>()
			.map(ServerFnRequest)
			.ok_or_else(|| DiError::NotFound("Request not found in InjectionContext. Ensure the server function handler is invoked with a properly configured InjectionContext containing the Request".to_string()))
	}
}

/// Wrapper for the request body that can be injected into server function handlers.
///
/// This extracts and provides the request body as a String for server function
/// argument deserialization.
#[derive(Debug, Clone)]
pub struct ServerFnBody(pub String);

impl ServerFnBody {
	/// Returns a reference to the body string.
	pub fn inner(&self) -> &str {
		&self.0
	}

	/// Consumes self and returns the body string.
	pub fn into_inner(self) -> String {
		self.0
	}
}

#[async_trait]
impl Injectable for ServerFnBody {
	async fn inject(ctx: &InjectionContext) -> DiResult<Self> {
		// Get the request from context
		let request = ctx.get_request::<Request>().ok_or_else(|| {
			DiError::NotFound(
				"Cannot extract body: Request not found in InjectionContext".to_string(),
			)
		})?;

		// Read the body as string
		let body = request
			.read_body()
			.map_err(|e| DiError::ProviderError(format!("Failed to read request body: {}", e)))?;

		let body_string = String::from_utf8(body.to_vec()).map_err(|e| {
			DiError::ProviderError(format!("Request body is not valid UTF-8: {}", e))
		})?;

		Ok(ServerFnBody(body_string))
	}
}

/// Helper function to extract session ID from HTTP request cookies.
///
/// Searches for a cookie with the specified name in the Cookie header.
///
/// # Arguments
///
/// * `request` - The HTTP request to extract the session ID from
/// * `cookie_name` - The name of the session cookie (e.g., "sessionid")
///
/// # Returns
///
/// * `Ok(String)` - The session ID if found and valid
/// * `Err(DiError)` - If the cookie header is missing, invalid, or the session cookie is not found
// TODO: This function may be used in future for session management
#[allow(dead_code)]
fn extract_session_id_from_request(request: &Request, cookie_name: &str) -> DiResult<String> {
	let cookie_header = request
		.headers
		.get(hyper::header::COOKIE)
		.ok_or_else(|| DiError::NotFound("Cookie header not found".to_string()))?;

	let cookie_str = cookie_header
		.to_str()
		.map_err(|e| DiError::ProviderError(format!("Invalid cookie header: {}", e)))?;

	for cookie in cookie_str.split(';') {
		let parts: Vec<&str> = cookie.trim().splitn(2, '=').collect();
		if parts.len() == 2 && parts[0] == cookie_name {
			return Ok(parts[1].to_string());
		}
	}

	Err(DiError::NotFound(format!(
		"Session cookie '{}' not found",
		cookie_name
	)))
}

// TODO: Move these Injectable implementations to reinhardt-middleware to avoid orphan rule violations
// These implementations are commented out temporarily because they violate Rust's orphan rules:
// - Injectable trait is defined in reinhardt-di
// - SessionData and Arc<SessionStore> are defined in reinhardt-middleware
// - Neither trait nor types are defined in this crate (reinhardt-pages)
//
// Proper solution: Move these implementations to reinhardt-middleware crate

// #[async_trait]
// impl Injectable for reinhardt_middleware::session::SessionData {
// 	async fn inject(ctx: &InjectionContext) -> DiResult<Self> {
// 		// Get SessionStore from SingletonScope
// 		let store = ctx
// 			.get_singleton::<Arc<reinhardt_middleware::session::SessionStore>>()
// 			.ok_or_else(|| {
// 				DiError::NotFound(
// 					"SessionStore not found in SingletonScope. \
//                  Ensure SessionMiddleware is configured and its store is registered."
// 						.to_string(),
// 				)
// 			})?;
//
// 		// Get Request from context
// 		let request = ctx.get_request::<Request>().ok_or_else(|| {
// 			DiError::NotFound("Request not found in InjectionContext".to_string())
// 		})?;
//
// 		// Extract session ID from Cookie header
// 		let session_id = extract_session_id_from_request(&*request, "sessionid")?;
//
// 		// Load SessionData from store
// 		store
// 			.get(&session_id)
// 			.filter(|s| s.is_valid())
// 			.ok_or_else(|| {
// 				DiError::NotFound("Valid session not found. Session may have expired.".to_string())
// 			})
// 	}
// }
//
// #[async_trait]
// impl Injectable for Arc<reinhardt_middleware::session::SessionStore> {
// 	async fn inject(ctx: &InjectionContext) -> DiResult<Self> {
// 		ctx.get_singleton::<Arc<reinhardt_middleware::session::SessionStore>>()
// 			.ok_or_else(|| {
// 				DiError::NotFound(
// 					"SessionStore not found in SingletonScope. \
//                  Ensure SessionMiddleware is configured and its store is registered."
// 						.to_string(),
// 				)
// 			})
// 	}
// }

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_server_fn_request_wrapper() {
		let request = Request::builder().uri("/test").build().unwrap();
		let wrapped = ServerFnRequest(Arc::new(request));
		assert_eq!(wrapped.inner().uri.path(), "/test");
	}

	#[test]
	fn test_server_fn_body_wrapper() {
		let body = ServerFnBody("test body".to_string());
		assert_eq!(body.inner(), "test body");
		assert_eq!(body.into_inner(), "test body".to_string());
	}
}
