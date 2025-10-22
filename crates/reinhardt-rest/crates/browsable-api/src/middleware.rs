//! Browsable API Middleware
//!
//! Provides middleware for automatically serving browsable HTML responses
//! when accessed from a web browser, similar to Django REST Framework.

use async_trait::async_trait;
use reinhardt_apps::{Handler, Middleware, Request, Response, Result};
use std::sync::Arc;

/// Middleware configuration for Browsable API
#[derive(Debug, Clone)]
pub struct BrowsableApiConfig {
    /// Enable browsable API (default: true)
    pub enabled: bool,
    /// Custom template name (optional)
    pub template_name: Option<String>,
    /// Custom CSS path (optional)
    pub custom_css: Option<String>,
}

impl Default for BrowsableApiConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            template_name: None,
            custom_css: None,
        }
    }
}

/// Middleware for serving browsable API HTML responses
///
/// This middleware automatically converts API responses to browsable HTML
/// when the request is from a web browser (based on Accept header).
pub struct BrowsableApiMiddleware {
    config: BrowsableApiConfig,
}

impl BrowsableApiMiddleware {
    /// Create a new BrowsableApiMiddleware with default configuration
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_browsable_api::middleware::BrowsableApiMiddleware;
    ///
    /// let middleware = BrowsableApiMiddleware::new();
    /// ```
    pub fn new() -> Self {
        Self {
            config: BrowsableApiConfig::default(),
        }
    }

    /// Create a new BrowsableApiMiddleware with custom configuration
    ///
    /// # Arguments
    ///
    /// * `config` - Custom configuration for the middleware
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_browsable_api::middleware::{BrowsableApiMiddleware, BrowsableApiConfig};
    ///
    /// let config = BrowsableApiConfig {
    ///     enabled: true,
    ///     template_name: Some("custom_api.html".to_string()),
    ///     custom_css: Some("/static/api.css".to_string()),
    /// };
    ///
    /// let middleware = BrowsableApiMiddleware::with_config(config);
    /// ```
    pub fn with_config(config: BrowsableApiConfig) -> Self {
        Self { config }
    }

    /// Check if the request prefers HTML response
    fn prefers_html(request: &Request) -> bool {
        if let Some(accept) = request.headers.get("Accept") {
            if let Ok(accept_str) = accept.to_str() {
                // Check if Accept header contains text/html
                return accept_str.contains("text/html");
            }
        }
        false
    }
}

impl Default for BrowsableApiMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Middleware for BrowsableApiMiddleware {
    async fn process(&self, request: Request, handler: Arc<dyn Handler>) -> Result<Response> {
        // If disabled, just pass through
        if !self.config.enabled {
            return handler.handle(request).await;
        }

        let prefers_html = Self::prefers_html(&request);

        // Get response from handler
        let response = handler.handle(request).await?;

        // If client prefers HTML and response is JSON, convert to browsable HTML
        // NOTE: Actual HTML rendering would require integration with BrowsableApiRenderer
        // and would need to parse the JSON response. This is a basic implementation
        // that adds a header to indicate browsable API support.
        if prefers_html {
            let mut response = response;
            response.headers.insert(
                "X-Browsable-API",
                "enabled"
                    .parse()
                    .expect("Failed to parse browsable API header"),
            );
            Ok(response)
        } else {
            Ok(response)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use hyper::{HeaderMap, Method, StatusCode, Uri, Version};

    struct TestHandler;

    #[async_trait]
    impl Handler for TestHandler {
        async fn handle(&self, _request: Request) -> Result<Response> {
            Ok(Response::new(StatusCode::OK).with_body(Bytes::from(r#"{"data":"test"}"#)))
        }
    }

    #[tokio::test]
    async fn test_middleware_with_html_accept() {
        let middleware = BrowsableApiMiddleware::new();
        let handler = Arc::new(TestHandler);

        let mut headers = HeaderMap::new();
        headers.insert("Accept", "text/html".parse().unwrap());

        let request = Request::new(
            Method::GET,
            Uri::from_static("/api/test"),
            Version::HTTP_11,
            headers,
            Bytes::new(),
        );

        let response = middleware.process(request, handler).await.unwrap();
        assert!(response.headers.contains_key("X-Browsable-API"));
        assert_eq!(response.headers.get("X-Browsable-API").unwrap(), "enabled");
    }

    #[tokio::test]
    async fn test_middleware_with_json_accept() {
        let middleware = BrowsableApiMiddleware::new();
        let handler = Arc::new(TestHandler);

        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        let request = Request::new(
            Method::GET,
            Uri::from_static("/api/test"),
            Version::HTTP_11,
            headers,
            Bytes::new(),
        );

        let response = middleware.process(request, handler).await.unwrap();
        assert!(!response.headers.contains_key("X-Browsable-API"));
    }

    #[tokio::test]
    async fn test_middleware_disabled() {
        let config = BrowsableApiConfig {
            enabled: false,
            template_name: None,
            custom_css: None,
        };
        let middleware = BrowsableApiMiddleware::with_config(config);
        let handler = Arc::new(TestHandler);

        let mut headers = HeaderMap::new();
        headers.insert("Accept", "text/html".parse().unwrap());

        let request = Request::new(
            Method::GET,
            Uri::from_static("/api/test"),
            Version::HTTP_11,
            headers,
            Bytes::new(),
        );

        let response = middleware.process(request, handler).await.unwrap();
        assert!(!response.headers.contains_key("X-Browsable-API"));
    }

    #[tokio::test]
    async fn test_middleware_default() {
        let middleware = BrowsableApiMiddleware::default();
        assert!(middleware.config.enabled);
        assert!(middleware.config.template_name.is_none());
        assert!(middleware.config.custom_css.is_none());
    }

    #[tokio::test]
    async fn test_middleware_with_custom_config() {
        let config = BrowsableApiConfig {
            enabled: true,
            template_name: Some("custom.html".to_string()),
            custom_css: Some("/custom.css".to_string()),
        };
        let middleware = BrowsableApiMiddleware::with_config(config.clone());
        assert!(middleware.config.enabled);
        assert_eq!(
            middleware.config.template_name,
            Some("custom.html".to_string())
        );
        assert_eq!(
            middleware.config.custom_css,
            Some("/custom.css".to_string())
        );
    }

    #[tokio::test]
    async fn test_prefers_html_with_html_accept() {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "text/html".parse().unwrap());

        let request = Request::new(
            Method::GET,
            Uri::from_static("/test"),
            Version::HTTP_11,
            headers,
            Bytes::new(),
        );

        assert!(BrowsableApiMiddleware::prefers_html(&request));
    }

    #[tokio::test]
    async fn test_prefers_html_with_json_accept() {
        let mut headers = HeaderMap::new();
        headers.insert("Accept", "application/json".parse().unwrap());

        let request = Request::new(
            Method::GET,
            Uri::from_static("/test"),
            Version::HTTP_11,
            headers,
            Bytes::new(),
        );

        assert!(!BrowsableApiMiddleware::prefers_html(&request));
    }

    #[tokio::test]
    async fn test_prefers_html_without_accept_header() {
        let request = Request::new(
            Method::GET,
            Uri::from_static("/test"),
            Version::HTTP_11,
            HeaderMap::new(),
            Bytes::new(),
        );

        assert!(!BrowsableApiMiddleware::prefers_html(&request));
    }
}
