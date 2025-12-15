//! Router wrapper that adds OpenAPI documentation endpoints
//!
//! This module provides a wrapper around any `Handler` implementation that
//! automatically serves OpenAPI documentation endpoints without modifying
//! user code.
//!
//! # Example
//!
//! ```rust,no_run
//! use reinhardt_openapi::OpenApiRouter;
//! use reinhardt_core::types::Handler;
//! use std::sync::Arc;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Wrap your existing router
//! let router = /* your router */;
//! # todo!();
//! let wrapped = OpenApiRouter::wrap(router);
//!
//! // The wrapped router now serves:
//! // - /api-docs/openapi.json (OpenAPI spec)
//! // - /docs (Swagger UI)
//! // - /docs-redoc (Redoc UI)
//! # Ok(())
//! # }
//! ```

use crate::endpoints::generate_openapi_schema;
use crate::swagger::{RedocUI, SwaggerUI};
use async_trait::async_trait;
use reinhardt_core::http::{Request, Response, Result};
use reinhardt_core::types::Handler;
use reinhardt_urls::prelude::Route;
use reinhardt_urls::routers::Router;
use std::sync::Arc;

/// Router wrapper that adds OpenAPI documentation endpoints
///
/// This wrapper intercepts requests to OpenAPI documentation paths and
/// serves them from memory, delegating all other requests to the wrapped
/// handler.
///
/// The OpenAPI schema is generated once at wrap time from the global
/// schema registry, ensuring minimal runtime overhead.
pub struct OpenApiRouter<H> {
	/// Base handler to delegate to
	inner: H,
	/// Pre-generated OpenAPI JSON schema
	openapi_json: Arc<String>,
	/// Swagger UI HTML
	swagger_html: Arc<String>,
	/// Redoc UI HTML
	redoc_html: Arc<String>,
}

impl<H> OpenApiRouter<H> {
	/// Wrap an existing handler with OpenAPI endpoints
	///
	/// This generates the OpenAPI schema from the global registry and
	/// pre-renders the Swagger and Redoc UIs.
	///
	/// # Example
	///
	/// ```rust,no_run
	/// use reinhardt_openapi::OpenApiRouter;
	///
	/// let router = /* your router */;
	/// # todo!();
	/// let wrapped = OpenApiRouter::wrap(router);
	/// ```
	pub fn wrap(handler: H) -> Self {
		// Generate OpenAPI schema from global registry
		let schema = generate_openapi_schema();
		let openapi_json =
			serde_json::to_string_pretty(&schema).expect("Failed to serialize OpenAPI schema");

		// Generate Swagger UI HTML
		let swagger_ui = SwaggerUI::new(schema.clone());
		let swagger_html = swagger_ui
			.render_html()
			.expect("Failed to render Swagger UI");

		// Generate Redoc UI HTML
		let redoc_ui = RedocUI::new(schema);
		let redoc_html = redoc_ui.render_html().expect("Failed to render Redoc UI");

		Self {
			inner: handler,
			openapi_json: Arc::new(openapi_json),
			swagger_html: Arc::new(swagger_html),
			redoc_html: Arc::new(redoc_html),
		}
	}

	/// Get a reference to the wrapped handler
	pub fn inner(&self) -> &H {
		&self.inner
	}
}

#[async_trait]
impl<H: Handler> Handler for OpenApiRouter<H> {
	/// Handle requests, intercepting OpenAPI documentation paths
	///
	/// Requests to `/api-docs/openapi.json`, `/docs`, or `/docs-redoc`
	/// are served from memory. All other requests are delegated to the
	/// wrapped handler.
	async fn handle(&self, request: Request) -> Result<Response> {
		// Match OpenAPI endpoints first
		match request.uri.path() {
			"/api-docs/openapi.json" => {
				let json = (*self.openapi_json).clone();
				Ok(Response::ok()
					.with_header("Content-Type", "application/json; charset=utf-8")
					.with_body(json))
			}
			"/docs" => {
				let html = (*self.swagger_html).clone();
				Ok(Response::ok()
					.with_header("Content-Type", "text/html; charset=utf-8")
					.with_body(html))
			}
			"/docs-redoc" => {
				let html = (*self.redoc_html).clone();
				Ok(Response::ok()
					.with_header("Content-Type", "text/html; charset=utf-8")
					.with_body(html))
			}
			_ => {
				// Delegate to base handler
				self.inner.handle(request).await
			}
		}
	}
}

/// Router trait implementation for OpenApiRouter
///
/// This implementation allows OpenApiRouter to be used where Router trait
/// is required. However, routes cannot be modified after wrapping - use
/// `add_route()` and `include()` on the base router before wrapping.
impl<H> Router for OpenApiRouter<H>
where
	H: Handler + Router,
{
	/// Add a route to the router
	///
	/// # Panics
	///
	/// This method always panics. Routes must be added to the base router
	/// before wrapping with `OpenApiRouter::wrap()`.
	fn add_route(&mut self, _route: Route) {
		panic!(
			"Cannot add routes to OpenApiRouter after wrapping. \
             Add routes to the base router before calling OpenApiRouter::wrap()."
		);
	}

	/// Include routes with a prefix
	///
	/// # Panics
	///
	/// This method always panics. Routes must be included in the base router
	/// before wrapping with `OpenApiRouter::wrap()`.
	fn include(&mut self, _prefix: &str, _routes: Vec<Route>, _namespace: Option<String>) {
		panic!(
			"Cannot include routes in OpenApiRouter after wrapping. \
             Include routes in the base router before calling OpenApiRouter::wrap()."
		);
	}

	/// Route a request through the OpenAPI wrapper
	///
	/// OpenAPI documentation endpoints (`/api-docs/openapi.json`, `/docs`,
	/// `/docs-redoc`) are handled directly. All other requests are delegated
	/// to the wrapped router's `route()` method.
	async fn route(&self, request: Request) -> Result<Response> {
		// Match OpenAPI endpoints first
		match request.uri.path() {
			"/api-docs/openapi.json" => {
				let json = (*self.openapi_json).clone();
				Ok(Response::ok()
					.with_header("Content-Type", "application/json; charset=utf-8")
					.with_body(json))
			}
			"/docs" => {
				let html = (*self.swagger_html).clone();
				Ok(Response::ok()
					.with_header("Content-Type", "text/html; charset=utf-8")
					.with_body(html))
			}
			"/docs-redoc" => {
				let html = (*self.redoc_html).clone();
				Ok(Response::ok()
					.with_header("Content-Type", "text/html; charset=utf-8")
					.with_body(html))
			}
			_ => {
				// Delegate to base router's route() method
				self.inner.route(request).await
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use reinhardt_core::http::StatusCode;

	struct DummyHandler;

	#[async_trait]
	impl Handler for DummyHandler {
		async fn handle(&self, _request: Request) -> Result<Response> {
			Ok(Response::new(StatusCode::OK).with_body("Hello from inner handler"))
		}
	}

	#[tokio::test]
	async fn test_openapi_json_endpoint() {
		let handler = DummyHandler;
		let wrapped = OpenApiRouter::wrap(handler);

		let request = Request::default().with_uri("/api-docs/openapi.json");
		let response = wrapped.handle(request).await.unwrap();

		assert_eq!(response.status, StatusCode::OK);
		let body_str = String::from_utf8(response.body.clone()).unwrap();
		assert!(body_str.contains("openapi"));
		assert!(body_str.contains("3.0")); // OpenAPI version
	}

	#[tokio::test]
	async fn test_swagger_docs_endpoint() {
		let handler = DummyHandler;
		let wrapped = OpenApiRouter::wrap(handler);

		let request = Request::default().with_uri("/docs");
		let response = wrapped.handle(request).await.unwrap();

		assert_eq!(response.status, StatusCode::OK);
		let body_str = String::from_utf8(response.body.clone()).unwrap();
		assert!(body_str.contains("swagger-ui"));
	}

	#[tokio::test]
	async fn test_redoc_docs_endpoint() {
		let handler = DummyHandler;
		let wrapped = OpenApiRouter::wrap(handler);

		let request = Request::default().with_uri("/docs-redoc");
		let response = wrapped.handle(request).await.unwrap();

		assert_eq!(response.status, StatusCode::OK);
		let body_str = String::from_utf8(response.body.clone()).unwrap();
		assert!(body_str.contains("redoc"));
	}

	#[tokio::test]
	async fn test_delegation_to_inner_handler() {
		let handler = DummyHandler;
		let wrapped = OpenApiRouter::wrap(handler);

		let request = Request::default().with_uri("/some/other/path");
		let response = wrapped.handle(request).await.unwrap();

		assert_eq!(response.status, StatusCode::OK);
		let body_str = String::from_utf8(response.body.clone()).unwrap();
		assert_eq!(body_str, "Hello from inner handler");
	}
}
