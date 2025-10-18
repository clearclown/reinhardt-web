//! API view tests inspired by Django REST Framework's APIView tests
//!
//! Tests for API view functionality, exception handling, and response formatting

use crate::View;
use hyper::{Method, StatusCode};
use reinhardt_apps::{Error, Request, Response, Result};
use reinhardt_views_core::{
    assert_response_body_contains, assert_response_status, create_json_request, create_request,
};
use std::collections::HashMap;

/// Basic API view that handles GET and POST
pub struct BasicAPIView {
    data: HashMap<String, serde_json::Value>,
}

impl BasicAPIView {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn with_data(mut self, data: HashMap<String, serde_json::Value>) -> Self {
        self.data = data;
        self
    }
}

#[async_trait::async_trait]
impl View for BasicAPIView {
    async fn dispatch(&self, request: Request) -> Result<Response> {
        match request.method {
            Method::GET => {
                let mut response_data = HashMap::new();
                response_data.insert(
                    "method".to_string(),
                    serde_json::Value::String("GET".to_string()),
                );
                Response::ok().with_json(&response_data)
            }
            Method::POST => {
                let mut response_data = HashMap::new();
                response_data.insert(
                    "method".to_string(),
                    serde_json::Value::String("POST".to_string()),
                );
                response_data.insert(
                    "data".to_string(),
                    serde_json::to_value(&self.data).unwrap(),
                );
                Response::ok().with_json(&response_data)
            }
            Method::PUT => {
                let mut response_data = HashMap::new();
                response_data.insert(
                    "method".to_string(),
                    serde_json::Value::String("PUT".to_string()),
                );
                response_data.insert(
                    "data".to_string(),
                    serde_json::to_value(&self.data).unwrap(),
                );
                Response::ok().with_json(&response_data)
            }
            Method::PATCH => {
                let mut response_data = HashMap::new();
                response_data.insert(
                    "method".to_string(),
                    serde_json::Value::String("PATCH".to_string()),
                );
                response_data.insert(
                    "data".to_string(),
                    serde_json::to_value(&self.data).unwrap(),
                );
                Response::ok().with_json(&response_data)
            }
            _ => Err(Error::Validation(format!(
                "Method {} not allowed",
                request.method
            ))),
        }
    }
}

/// API view that raises an exception for testing error handling
pub struct ErrorAPIView;

#[async_trait::async_trait]
impl View for ErrorAPIView {
    async fn dispatch(&self, _request: Request) -> Result<Response> {
        Err(Error::Internal("Test error".to_string()))
    }
}

/// API view with custom exception handler
pub struct CustomExceptionAPIView {
    custom_handler: bool,
}

impl CustomExceptionAPIView {
    pub fn new() -> Self {
        Self {
            custom_handler: false,
        }
    }

    pub fn with_custom_handler(mut self, custom_handler: bool) -> Self {
        self.custom_handler = custom_handler;
        self
    }

    fn custom_exception_handler(&self, error: &Error) -> Result<Response> {
        match error {
            Error::Validation(msg) => {
                let error_response = HashMap::from([
                    (
                        "error".to_string(),
                        serde_json::Value::String("ValidationError".to_string()),
                    ),
                    (
                        "message".to_string(),
                        serde_json::Value::String(msg.clone()),
                    ),
                ]);
                Ok(Response::bad_request().with_json(&error_response).unwrap())
            }
            _ => {
                let error_response = HashMap::from([(
                    "error".to_string(),
                    serde_json::Value::String("UnknownError".to_string()),
                )]);
                Ok(Response::internal_server_error()
                    .with_json(&error_response)
                    .unwrap())
            }
        }
    }
}

#[async_trait::async_trait]
impl View for CustomExceptionAPIView {
    async fn dispatch(&self, request: Request) -> Result<Response> {
        if self.custom_handler {
            // Simulate an error that would be handled by custom handler
            return Err(Error::Validation("Custom validation error".to_string()));
        }

        match request.method {
            Method::GET => {
                let mut response_data = HashMap::new();
                response_data.insert(
                    "method".to_string(),
                    serde_json::Value::String("GET".to_string()),
                );
                Response::ok().with_json(&response_data)
            }
            _ => Err(Error::Validation(format!(
                "Method {} not allowed",
                request.method
            ))),
        }
    }
}

/// API view that handles JSON parsing errors
pub struct JSONParseAPIView;

#[async_trait::async_trait]
impl View for JSONParseAPIView {
    async fn dispatch(&self, request: Request) -> Result<Response> {
        // Simulate JSON parsing error
        let body = request.read_body()?;
        if body.is_empty() {
            let error_response = HashMap::from([(
                "detail".to_string(),
                serde_json::Value::String("JSON parse error - Expecting value:".to_string()),
            )]);
            return Ok(Response::bad_request().with_json(&error_response).unwrap());
        }

        // Try to parse JSON
        match serde_json::from_slice::<serde_json::Value>(&body) {
            Ok(_) => {
                let mut response_data = HashMap::new();
                response_data.insert(
                    "status".to_string(),
                    serde_json::Value::String("success".to_string()),
                );
                Response::ok().with_json(&response_data)
            }
            Err(_) => {
                let error_response = HashMap::from([(
                    "detail".to_string(),
                    serde_json::Value::String("JSON parse error - Expecting value:".to_string()),
                )]);
                Ok(Response::bad_request().with_json(&error_response).unwrap())
            }
        }
    }
}

/// API view that handles different HTTP methods with specific responses
pub struct MethodBasedAPIView;

#[async_trait::async_trait]
impl View for MethodBasedAPIView {
    async fn dispatch(&self, request: Request) -> Result<Response> {
        match request.method {
            Method::GET => {
                let response_data = HashMap::from([
                    (
                        "method".to_string(),
                        serde_json::Value::String("GET".to_string()),
                    ),
                    ("data".to_string(), serde_json::Value::Null),
                ]);
                Response::ok().with_json(&response_data)
            }
            Method::POST => {
                let response_data = HashMap::from([
                    (
                        "method".to_string(),
                        serde_json::Value::String("POST".to_string()),
                    ),
                    (
                        "data".to_string(),
                        serde_json::to_value(&String::from_utf8_lossy(&request.read_body()?))
                            .unwrap(),
                    ),
                ]);
                Response::ok().with_json(&response_data)
            }
            Method::PUT => {
                let response_data = HashMap::from([
                    (
                        "method".to_string(),
                        serde_json::Value::String("PUT".to_string()),
                    ),
                    (
                        "data".to_string(),
                        serde_json::to_value(&String::from_utf8_lossy(&request.read_body()?))
                            .unwrap(),
                    ),
                ]);
                Response::ok().with_json(&response_data)
            }
            Method::PATCH => {
                let response_data = HashMap::from([
                    (
                        "method".to_string(),
                        serde_json::Value::String("PATCH".to_string()),
                    ),
                    (
                        "data".to_string(),
                        serde_json::to_value(&String::from_utf8_lossy(&request.read_body()?))
                            .unwrap(),
                    ),
                ]);
                Response::ok().with_json(&response_data)
            }
            Method::DELETE => {
                let response_data = HashMap::from([
                    (
                        "method".to_string(),
                        serde_json::Value::String("DELETE".to_string()),
                    ),
                    ("data".to_string(), serde_json::Value::Null),
                ]);
                Response::ok().with_json(&response_data)
            }
            _ => Err(Error::Validation(format!(
                "Method {} not allowed",
                request.method
            ))),
        }
    }
}

/// API view that handles authentication errors
pub struct AuthAPIView {
    require_auth: bool,
}

impl AuthAPIView {
    pub fn new() -> Self {
        Self {
            require_auth: false,
        }
    }

    pub fn with_auth_required(mut self, require_auth: bool) -> Self {
        self.require_auth = require_auth;
        self
    }
}

#[async_trait::async_trait]
impl View for AuthAPIView {
    async fn dispatch(&self, request: Request) -> Result<Response> {
        if self.require_auth {
            // Check for authorization header
            if let Some(auth_header) = request.headers.get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if auth_str.starts_with("Bearer ") {
                        let response_data = HashMap::from([
                            (
                                "method".to_string(),
                                serde_json::Value::String("GET".to_string()),
                            ),
                            ("authenticated".to_string(), serde_json::Value::Bool(true)),
                        ]);
                        return Response::ok().with_json(&response_data);
                    }
                }
            }

            let error_response = HashMap::from([(
                "detail".to_string(),
                serde_json::Value::String(
                    "Authentication credentials were not provided.".to_string(),
                ),
            )]);
            return Ok(Response::unauthorized().with_json(&error_response).unwrap());
        }

        let response_data = HashMap::from([
            (
                "method".to_string(),
                serde_json::Value::String("GET".to_string()),
            ),
            ("authenticated".to_string(), serde_json::Value::Bool(false)),
        ]);
        Response::ok().with_json(&response_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_api_view_get() {
        let view = BasicAPIView::new();
        let request = create_request(Method::GET, "/api/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "method",
            &serde_json::Value::String("GET".to_string()),
        );
    }

    #[tokio::test]
    async fn test_basic_api_view_post() {
        let mut data = HashMap::new();
        data.insert(
            "key".to_string(),
            serde_json::Value::String("value".to_string()),
        );

        let view = BasicAPIView::new().with_data(data);
        let request = create_request(Method::POST, "/api/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "method",
            &serde_json::Value::String("POST".to_string()),
        );
    }

    #[tokio::test]
    async fn test_basic_api_view_put() {
        let view = BasicAPIView::new();
        let request = create_request(Method::PUT, "/api/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "method",
            &serde_json::Value::String("PUT".to_string()),
        );
    }

    #[tokio::test]
    async fn test_basic_api_view_patch() {
        let view = BasicAPIView::new();
        let request = create_request(Method::PATCH, "/api/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "method",
            &serde_json::Value::String("PATCH".to_string()),
        );
    }

    #[tokio::test]
    async fn test_basic_api_view_delete_not_allowed() {
        let view = BasicAPIView::new();
        let request = create_request(Method::DELETE, "/api/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_error_api_view() {
        let view = ErrorAPIView;
        let request = create_request(Method::GET, "/api/", None, None, None);
        let result = view.dispatch(request).await;

        assert_internal_error(result);
    }

    #[tokio::test]
    async fn test_custom_exception_api_view_get() {
        let view = CustomExceptionAPIView::new();
        let request = create_request(Method::GET, "/api/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_custom_exception_api_view_with_custom_handler() {
        let view = CustomExceptionAPIView::new().with_custom_handler(true);
        let request = create_request(Method::GET, "/api/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_json_parse_api_view_valid_json() {
        let view = JSONParseAPIView;
        let json_data = serde_json::json!({"key": "value"});
        let request = create_json_request(Method::POST, "/api/", &json_data);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "status",
            &serde_json::Value::String("success".to_string()),
        );
    }

    #[tokio::test]
    async fn test_json_parse_api_view_invalid_json() {
        let view = JSONParseAPIView;
        let body = bytes::Bytes::from("invalid json");
        let request = create_request(Method::POST, "/api/", None, None, Some(body));
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::BAD_REQUEST);
        assert_json_response_contains(
            &response,
            "detail",
            &serde_json::Value::String("JSON parse error - Expecting value:".to_string()),
        );
    }

    #[tokio::test]
    async fn test_json_parse_api_view_empty_body() {
        let view = JSONParseAPIView;
        let request = create_request(Method::POST, "/api/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::BAD_REQUEST);
        assert_json_response_contains(
            &response,
            "detail",
            &serde_json::Value::String("JSON parse error - Expecting value:".to_string()),
        );
    }

    #[tokio::test]
    async fn test_method_based_api_view_get() {
        let view = MethodBasedAPIView;
        let request = create_request(Method::GET, "/api/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "method",
            &serde_json::Value::String("GET".to_string()),
        );
    }

    #[tokio::test]
    async fn test_method_based_api_view_post() {
        let view = MethodBasedAPIView;
        let json_data = serde_json::json!({"data": "test"});
        let request = create_json_request(Method::POST, "/api/", &json_data);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "method",
            &serde_json::Value::String("POST".to_string()),
        );
    }

    #[tokio::test]
    async fn test_method_based_api_view_put() {
        let view = MethodBasedAPIView;
        let json_data = serde_json::json!({"data": "test"});
        let request = create_json_request(Method::PUT, "/api/", &json_data);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "method",
            &serde_json::Value::String("PUT".to_string()),
        );
    }

    #[tokio::test]
    async fn test_method_based_api_view_patch() {
        let view = MethodBasedAPIView;
        let json_data = serde_json::json!({"data": "test"});
        let request = create_json_request(Method::PATCH, "/api/", &json_data);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "method",
            &serde_json::Value::String("PATCH".to_string()),
        );
    }

    #[tokio::test]
    async fn test_method_based_api_view_delete() {
        let view = MethodBasedAPIView;
        let request = create_request(Method::DELETE, "/api/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "method",
            &serde_json::Value::String("DELETE".to_string()),
        );
    }

    #[tokio::test]
    async fn test_auth_api_view_no_auth_required() {
        let view = AuthAPIView::new();
        let request = create_request(Method::GET, "/api/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(&response, "authenticated", &serde_json::Value::Bool(false));
    }

    #[tokio::test]
    async fn test_auth_api_view_auth_required_no_header() {
        let view = AuthAPIView::new().with_auth_required(true);
        let request = create_request(Method::GET, "/api/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::UNAUTHORIZED);
        assert_json_response_contains(
            &response,
            "detail",
            &serde_json::Value::String("Authentication credentials were not provided.".to_string()),
        );
    }

    #[tokio::test]
    async fn test_auth_api_view_auth_required_with_header() {
        let view = AuthAPIView::new().with_auth_required(true);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Bearer token123".to_string());
        let request = create_request_with_headers(Method::GET, "/api/", headers, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(&response, "authenticated", &serde_json::Value::Bool(true));
    }

    #[tokio::test]
    async fn test_auth_api_view_auth_required_invalid_header() {
        let view = AuthAPIView::new().with_auth_required(true);
        let mut headers = HashMap::new();
        headers.insert("Authorization".to_string(), "Invalid token123".to_string());
        let request = create_request_with_headers(Method::GET, "/api/", headers, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::UNAUTHORIZED);
        assert_json_response_contains(
            &response,
            "detail",
            &serde_json::Value::String("Authentication credentials were not provided.".to_string()),
        );
    }

    #[tokio::test]
    async fn test_basic_api_view_head() {
        let view = BasicAPIView::new();
        let request = create_request(Method::HEAD, "/api/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_basic_api_view_options() {
        let view = BasicAPIView::new();
        let request = create_request(Method::OPTIONS, "/api/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_api_view_with_query_params() {
        let view = BasicAPIView::new();
        let mut query_params = HashMap::new();
        query_params.insert("page".to_string(), "1".to_string());
        query_params.insert("limit".to_string(), "10".to_string());

        let request = create_request(Method::GET, "/api/", Some(query_params), None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_view_with_custom_headers() {
        let view = BasicAPIView::new();
        let mut headers = HashMap::new();
        headers.insert("X-Custom-Header".to_string(), "custom-value".to_string());
        headers.insert("Accept".to_string(), "application/json".to_string());

        let request = create_request_with_headers(Method::GET, "/api/", headers, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }
}
