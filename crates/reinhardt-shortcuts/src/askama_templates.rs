//! Askama template integration for compile-time type-safe templates
//!
//! This module provides support for Askama templates, which are compiled at build time
//! for maximum performance and type safety. Unlike Tera templates which are loaded at runtime,
//! Askama templates are embedded in the binary and validated during compilation.
//!
//! # Examples
//!
//! ```rust,ignore
//! use askama::Template;
//! use reinhardt_shortcuts::askama_templates::render_askama;
//!
//! #[derive(Template)]
//! #[template(path = "hello.html")]
//! struct HelloTemplate {
//!     name: String,
//!     count: u32,
//! }
//!
//! async fn hello_handler(request: Request) -> Result<Response, Response> {
//!     let template = HelloTemplate {
//!         name: "Alice".to_string(),
//!         count: 42,
//!     };
//!
//!     render_askama(&request, template)
//! }
//! ```

#[cfg(feature = "templates")]
use askama::Template;
#[cfg(feature = "templates")]
use reinhardt_http::{Request, Response};

/// Render an Askama template to an HTTP response
///
/// This function takes any type implementing the `askama::Template` trait
/// and renders it to an HTML response with proper content type headers.
///
/// # Type Safety
///
/// Unlike runtime template engines, Askama validates templates at compile time:
/// - Template syntax errors cause compilation to fail
/// - Variable names are checked against struct fields
/// - Type mismatches are caught during compilation
///
/// # Performance
///
/// Askama templates are compiled to native Rust code, providing:
/// - Zero runtime parsing overhead
/// - Optimal execution speed
/// - Smaller binary size (templates embedded)
///
/// # Arguments
///
/// * `_request` - The HTTP request (for future extensions)
/// * `template` - Any type implementing `askama::Template`
///
/// # Returns
///
/// An HTTP Response with the rendered HTML
///
/// # Errors
///
/// Returns `Err(Response)` with HTTP 500 if template rendering fails
///
/// # Examples
///
/// ```rust,ignore
/// use askama::Template;
/// use reinhardt_shortcuts::askama_templates::render_askama;
///
/// #[derive(Template)]
/// #[template(path = "user_profile.html")]
/// struct UserProfile {
///     username: String,
///     email: String,
///     is_active: bool,
/// }
///
/// async fn profile_handler(request: Request) -> Result<Response, Response> {
///     let template = UserProfile {
///         username: "john_doe".to_string(),
///         email: "john@example.com".to_string(),
///         is_active: true,
///     };
///
///     render_askama(&request, template)
/// }
/// ```
#[cfg(feature = "templates")]
pub fn render_askama<T>(_request: &Request, template: T) -> Result<Response, Response>
where
    T: Template,
{
    match template.render() {
        Ok(html) => {
            let mut response = Response::ok();
            response.headers.insert(
                hyper::header::CONTENT_TYPE,
                hyper::header::HeaderValue::from_static("text/html; charset=utf-8"),
            );
            response.body = bytes::Bytes::from(html);
            Ok(response)
        }
        Err(e) => {
            let mut response = Response::internal_server_error();
            response.body = bytes::Bytes::from(format!("Template rendering failed: {}", e));
            Err(response)
        }
    }
}

/// Render an Askama template to a string
///
/// This is a lower-level function that returns the rendered HTML as a String
/// instead of wrapping it in an HTTP Response. Useful for embedding templates
/// in other contexts or composing multiple templates.
///
/// # Arguments
///
/// * `template` - Any type implementing `askama::Template`
///
/// # Returns
///
/// The rendered template as a String
///
/// # Errors
///
/// Returns `askama::Error` if rendering fails
///
/// # Examples
///
/// ```rust,ignore
/// use askama::Template;
/// use reinhardt_shortcuts::askama_templates::render_askama_to_string;
///
/// #[derive(Template)]
/// #[template(source = "Hello, {{ name }}!", ext = "html")]
/// struct HelloTemplate {
///     name: String,
/// }
///
/// let template = HelloTemplate {
///     name: "World".to_string(),
/// };
///
/// let html = render_askama_to_string(template)?;
/// assert_eq!(html, "Hello, World!");
/// ```
#[cfg(feature = "templates")]
pub fn render_askama_to_string<T>(template: T) -> Result<String, askama::Error>
where
    T: Template,
{
    template.render()
}

#[cfg(all(test, feature = "templates"))]
mod tests {
    use super::*;
    use askama::Template;
    use bytes::Bytes;
    use hyper::{HeaderMap, Method, StatusCode, Uri, Version};
    use reinhardt_http::Request;

    fn create_test_request() -> Request {
        Request::new(
            Method::GET,
            Uri::from_static("/"),
            Version::HTTP_11,
            HeaderMap::new(),
            Bytes::new(),
        )
    }

    #[derive(Template)]
    #[template(source = "Hello, {{ name }}!", ext = "html")]
    struct SimpleTemplate {
        name: String,
    }

    #[test]
    fn test_render_askama_simple() {
        let request = create_test_request();
        let template = SimpleTemplate {
            name: "Alice".to_string(),
        };

        let result = render_askama(&request, template);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.status, StatusCode::OK);

        let body = String::from_utf8(response.body.to_vec()).unwrap();
        assert_eq!(body, "Hello, Alice!");
    }

    #[test]
    fn test_render_askama_to_string() {
        let template = SimpleTemplate {
            name: "Bob".to_string(),
        };

        let result = render_askama_to_string(template);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, Bob!");
    }

    #[derive(Template)]
    #[template(
        source = "<div>{% for item in items %}<p>{{ item }}</p>{% endfor %}</div>",
        ext = "html"
    )]
    struct LoopTemplate {
        items: Vec<String>,
    }

    #[test]
    fn test_render_askama_with_loop() {
        let request = create_test_request();
        let template = LoopTemplate {
            items: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
        };

        let result = render_askama(&request, template);
        assert!(result.is_ok());

        let body = String::from_utf8(result.unwrap().body.to_vec()).unwrap();
        assert!(body.contains("<p>foo</p>"));
        assert!(body.contains("<p>bar</p>"));
        assert!(body.contains("<p>baz</p>"));
    }

    #[derive(Template)]
    #[template(
        source = "{% if show %}<p>{{ message }}</p>{% endif %}",
        ext = "html"
    )]
    struct ConditionalTemplate {
        show: bool,
        message: String,
    }

    #[test]
    fn test_render_askama_conditional_true() {
        let template = ConditionalTemplate {
            show: true,
            message: "Visible".to_string(),
        };

        let result = render_askama_to_string(template);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "<p>Visible</p>");
    }

    #[test]
    fn test_render_askama_conditional_false() {
        let template = ConditionalTemplate {
            show: false,
            message: "Hidden".to_string(),
        };

        let result = render_askama_to_string(template);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_render_askama_content_type_header() {
        let request = create_test_request();
        let template = SimpleTemplate {
            name: "Test".to_string(),
        };

        let response = render_askama(&request, template).unwrap();
        assert_eq!(
            response.headers.get(hyper::header::CONTENT_TYPE),
            Some(&hyper::header::HeaderValue::from_static(
                "text/html; charset=utf-8"
            ))
        );
    }
}
