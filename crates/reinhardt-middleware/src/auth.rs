use async_trait::async_trait;
use reinhardt_apps::{Handler, Middleware, Request, Response, Result};
use reinhardt_auth::{JwtAuth, User};
use std::marker::PhantomData;
use std::sync::Arc;

/// Authentication middleware
/// Extracts and validates JWT tokens from requests
pub struct AuthenticationMiddleware<U: User> {
    jwt_auth: Arc<JwtAuth>,
    _phantom: PhantomData<U>,
}

impl<U: User> AuthenticationMiddleware<U> {
    /// Create a new authentication middleware with the given JWT authentication
    ///
    /// # Arguments
    ///
    /// * `jwt_auth` - Shared JWT authentication handler
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use reinhardt_middleware::AuthenticationMiddleware;
    /// use reinhardt_auth::{JwtAuth, User};
    /// use reinhardt_apps::{Handler, Middleware, Request, Response};
    /// use hyper::{StatusCode, Method, Uri, Version, HeaderMap};
    /// use bytes::Bytes;
    ///
    /// #[derive(Debug, Clone)]
    /// struct MyUser {
    ///     id: String,
    /// }
    ///
    /// impl User for MyUser {
    ///     fn id(&self) -> String { self.id.clone() }
    ///     fn username(&self) -> &str { "user" }
    ///     fn is_authenticated(&self) -> bool { true }
    ///     fn is_active(&self) -> bool { true }
    ///     fn is_admin(&self) -> bool { false }
    /// }
    ///
    /// struct TestHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl Handler for TestHandler {
    ///     async fn handle(&self, _request: Request) -> reinhardt_apps::Result<Response> {
    ///         Ok(Response::new(StatusCode::OK).with_body(Bytes::from("Protected resource")))
    ///     }
    /// }
    ///
    /// # tokio_test::block_on(async {
    /// let jwt_auth = Arc::new(JwtAuth::new(b"secret_key"));
    /// let middleware = AuthenticationMiddleware::<MyUser>::new(jwt_auth.clone());
    /// let handler = Arc::new(TestHandler);
    ///
    /// // Create a token
    /// let token = jwt_auth.create_token("user123", 3600).unwrap();
    ///
    /// let mut headers = HeaderMap::new();
    /// headers.insert("Authorization", format!("Bearer {}", token).parse().unwrap());
    ///
    /// let request = Request::new(
    ///     Method::GET,
    ///     Uri::from_static("/api/protected"),
    ///     Version::HTTP_11,
    ///     headers,
    ///     Bytes::new(),
    /// );
    ///
    /// let response = middleware.process(request, handler).await.unwrap();
    /// assert_eq!(response.status, StatusCode::OK);
    /// # });
    /// ```
    pub fn new(jwt_auth: Arc<JwtAuth>) -> Self {
        Self {
            jwt_auth,
            _phantom: PhantomData,
        }
    }

    fn extract_token(&self, request: &Request) -> Option<String> {
        // Extract from Authorization header
        request
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| {
                if v.starts_with("Bearer ") {
                    Some(v[7..].to_string())
                } else {
                    None
                }
            })
    }
}

#[async_trait]
impl<U: User> Middleware for AuthenticationMiddleware<U> {
    async fn process(&self, request: Request, next: Arc<dyn Handler>) -> Result<Response> {
        // Try to extract and validate token
        if let Some(token) = self.extract_token(&request) {
            match self.jwt_auth.verify_token(&token) {
                Ok(_claims) => {
                    // Token is valid - you would typically attach user to request here
                    // For now, just pass through
                    next.handle(request).await
                }
                Err(_) => {
                    // Invalid token - you could return 401 or just pass through
                    // depending on your requirements
                    next.handle(request).await
                }
            }
        } else {
            // No token - pass through
            next.handle(request).await
        }
    }
}
