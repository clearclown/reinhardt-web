//! Basic view tests inspired by Django's view tests
//!
//! Tests for basic view functionality, HTTP method handling, and error responses

use crate::View;
use hyper::{Method, StatusCode};
use reinhardt_apps::{Error, Request, Response, Result};
use reinhardt_views_core::{
    assert_response_body_contains, assert_response_status, create_json_request, create_request,
    create_request_with_headers,
};
use std::collections::HashMap;

/// Test view that handles only GET requests
struct GetOnlyView {
    content: String,
}

impl GetOnlyView {
    fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl View for GetOnlyView {
    async fn dispatch(&self, request: Request) -> Result<Response> {
        match request.method {
            Method::GET => Ok(Response::ok().with_body(self.content.clone().into_bytes())),
            _ => Err(Error::Validation(format!(
                "Method {} not allowed",
                request.method
            ))),
        }
    }
}

/// Test view that handles GET and POST requests
struct GetPostView {
    content: String,
}

impl GetPostView {
    fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl View for GetPostView {
    async fn dispatch(&self, request: Request) -> Result<Response> {
        match request.method {
            Method::GET | Method::POST => {
                Ok(Response::ok().with_body(self.content.clone().into_bytes()))
            }
            _ => Err(Error::Validation(format!(
                "Method {} not allowed",
                request.method
            ))),
        }
    }
}

/// Test view that handles all HTTP methods
struct AllMethodsView;

#[async_trait::async_trait]
impl View for AllMethodsView {
    async fn dispatch(&self, request: Request) -> Result<Response> {
        let response_content = match request.method {
            Method::GET => "GET response",
            Method::POST => "POST response",
            Method::PUT => "PUT response",
            Method::PATCH => "PATCH response",
            Method::DELETE => "DELETE response",
            Method::HEAD => "HEAD response",
            Method::OPTIONS => "OPTIONS response",
            _ => "Unknown method response",
        };

        Ok(Response::ok().with_body(response_content))
    }
}

/// Test view that returns different status codes
struct StatusCodeView {
    status_code: StatusCode,
}

impl StatusCodeView {
    fn new(status_code: StatusCode) -> Self {
        Self { status_code }
    }
}

#[async_trait::async_trait]
impl View for StatusCodeView {
    async fn dispatch(&self, _request: Request) -> Result<Response> {
        Ok(Response::new(self.status_code).with_body("Status response"))
    }
}

/// Test view that returns JSON responses
struct JsonView {
    data: HashMap<String, serde_json::Value>,
}

impl JsonView {
    fn new(data: HashMap<String, serde_json::Value>) -> Self {
        Self { data }
    }
}

#[async_trait::async_trait]
impl View for JsonView {
    async fn dispatch(&self, request: Request) -> Result<Response> {
        let mut response_data = self.data.clone();
        response_data.insert(
            "method".to_string(),
            serde_json::Value::String(request.method.to_string()),
        );

        Response::ok().with_json(&response_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::header::HeaderMap;

    #[tokio::test]
    async fn test_get_only_view_get() {
        let view = GetOnlyView::new("This is a GET-only view");
        let request = create_request(Method::GET, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "This is a GET-only view");
    }

    #[tokio::test]
    async fn test_get_only_view_post() {
        let view = GetOnlyView::new("This is a GET-only view");
        let request = create_request(Method::POST, "/test/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_get_only_view_put() {
        let view = GetOnlyView::new("This is a GET-only view");
        let request = create_request(Method::PUT, "/test/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_get_only_view_delete() {
        let view = GetOnlyView::new("This is a GET-only view");
        let request = create_request(Method::DELETE, "/test/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_get_post_view_get() {
        let view = GetPostView::new("This handles GET and POST");
        let request = create_request(Method::GET, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "This handles GET and POST");
    }

    #[tokio::test]
    async fn test_get_post_view_post() {
        let view = GetPostView::new("This handles GET and POST");
        let request = create_request(Method::POST, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "This handles GET and POST");
    }

    #[tokio::test]
    async fn test_get_post_view_put() {
        let view = GetPostView::new("This handles GET and POST");
        let request = create_request(Method::PUT, "/test/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_all_methods_view_get() {
        let view = AllMethodsView;
        let request = create_request(Method::GET, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "GET response");
    }

    #[tokio::test]
    async fn test_all_methods_view_post() {
        let view = AllMethodsView;
        let request = create_request(Method::POST, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "POST response");
    }

    #[tokio::test]
    async fn test_all_methods_view_put() {
        let view = AllMethodsView;
        let request = create_request(Method::PUT, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "PUT response");
    }

    #[tokio::test]
    async fn test_all_methods_view_patch() {
        let view = AllMethodsView;
        let request = create_request(Method::PATCH, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "PATCH response");
    }

    #[tokio::test]
    async fn test_all_methods_view_delete() {
        let view = AllMethodsView;
        let request = create_request(Method::DELETE, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "DELETE response");
    }

    #[tokio::test]
    async fn test_all_methods_view_head() {
        let view = AllMethodsView;
        let request = create_request(Method::HEAD, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "HEAD response");
    }

    #[tokio::test]
    async fn test_all_methods_view_options() {
        let view = AllMethodsView;
        let request = create_request(Method::OPTIONS, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "OPTIONS response");
    }

    #[tokio::test]
    async fn test_status_code_view_ok() {
        let view = StatusCodeView::new(StatusCode::OK);
        let request = create_request(Method::GET, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "Status response");
    }

    #[tokio::test]
    async fn test_status_code_view_created() {
        let view = StatusCodeView::new(StatusCode::CREATED);
        let request = create_request(Method::POST, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::CREATED);
        assert_response_body_contains(&response, "Status response");
    }

    #[tokio::test]
    async fn test_status_code_view_not_found() {
        let view = StatusCodeView::new(StatusCode::NOT_FOUND);
        let request = create_request(Method::GET, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::NOT_FOUND);
        assert_response_body_contains(&response, "Status response");
    }

    #[tokio::test]
    async fn test_status_code_view_internal_server_error() {
        let view = StatusCodeView::new(StatusCode::INTERNAL_SERVER_ERROR);
        let request = create_request(Method::GET, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::INTERNAL_SERVER_ERROR);
        assert_response_body_contains(&response, "Status response");
    }

    #[tokio::test]
    async fn test_json_view() {
        let mut data = HashMap::new();
        data.insert(
            "message".to_string(),
            serde_json::Value::String("Hello, World!".to_string()),
        );
        data.insert("count".to_string(), serde_json::Value::Number(42.into()));

        let view = JsonView::new(data);
        let request = create_request(Method::GET, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);

        // Check that the response is JSON
        let body_str = String::from_utf8_lossy(&response.body);
        let json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert_eq!(json["message"], "Hello, World!");
        assert_eq!(json["count"], 42);
        assert_eq!(json["method"], "GET");
    }

    #[tokio::test]
    async fn test_simple_test_view() {
        let view = SimpleTestView::new("Simple test content");
        let request = create_request(Method::GET, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "Simple test content");
    }

    #[tokio::test]
    async fn test_simple_test_view_with_methods() {
        let view = SimpleTestView::new("Multi-method content")
            .with_methods(vec![Method::GET, Method::POST]);

        let request = create_request(Method::POST, "/test/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "Multi-method content");
    }

    #[tokio::test]
    async fn test_error_test_view_not_found() {
        let view = ErrorTestView::new(Error::NotFound("Test object not found".to_string()));
        let request = create_request(Method::GET, "/test/", None, None, None);
        let result = view.dispatch(request).await;

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn test_error_test_view_validation() {
        let view = ErrorTestView::new(Error::Validation("Invalid input".to_string()));
        let request = create_request(Method::GET, "/test/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_error_test_view_internal() {
        let view = ErrorTestView::new(Error::Internal("Internal server error".to_string()));
        let request = create_request(Method::GET, "/test/", None, None, None);
        let result = view.dispatch(request).await;

        assert_internal_error(result);
    }

    #[tokio::test]
    async fn test_view_with_query_params() {
        let view = GetOnlyView::new("Query params test");
        let mut query_params = HashMap::new();
        query_params.insert("page".to_string(), "1".to_string());
        query_params.insert("limit".to_string(), "10".to_string());

        let request = create_request(Method::GET, "/test/", Some(query_params), None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "Query params test");
    }

    #[tokio::test]
    async fn test_view_with_headers() {
        let view = GetOnlyView::new("Headers test");
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), "Test Agent".to_string());
        headers.insert("Accept".to_string(), "application/json".to_string());

        let request = create_request_with_headers(Method::GET, "/test/", headers, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_response_body_contains(&response, "Headers test");
    }

    #[tokio::test]
    async fn test_view_with_json_body() {
        let view = JsonView::new(HashMap::new());
        let json_data = serde_json::json!({
            "name": "Test User",
            "email": "test@example.com"
        });

        let request = create_json_request(Method::POST, "/test/", &json_data);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);

        let body_str = String::from_utf8_lossy(&response.body);
        let json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
        assert_eq!(json["method"], "POST");
    }
}
