use async_trait::async_trait;
use reinhardt_exception::Result;
use std::sync::Arc;

// Re-export Request and Response for convenience
pub use reinhardt_http::{Request, Response};

/// Handler trait for processing requests
/// This is the core abstraction - all request handlers implement this
#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, request: Request) -> Result<Response>;
}

/// Middleware trait for request/response processing
/// Uses composition pattern instead of inheritance
#[async_trait]
pub trait Middleware: Send + Sync {
    async fn process(&self, request: Request, next: Arc<dyn Handler>) -> Result<Response>;
}

/// Middleware chain - composes multiple middleware
pub struct MiddlewareChain {
    middlewares: Vec<Arc<dyn Middleware>>,
    handler: Arc<dyn Handler>,
}

impl MiddlewareChain {
    /// Creates a new middleware chain with the given handler.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use reinhardt_types::{MiddlewareChain, Handler};
    /// use std::sync::Arc;
    ///
    /// # struct MyHandler;
    /// # #[async_trait::async_trait]
    /// # impl Handler for MyHandler {
    /// #     async fn handle(&self, request: reinhardt_http::Request) -> reinhardt_exception::Result<reinhardt_http::Response> {
    /// #         Ok(reinhardt_http::Response::new())
    /// #     }
    /// # }
    /// let handler = Arc::new(MyHandler);
    /// let chain = MiddlewareChain::new(handler);
    /// ```
    pub fn new(handler: Arc<dyn Handler>) -> Self {
        Self {
            middlewares: Vec::new(),
            handler,
        }
    }
    /// Adds a middleware to the chain using builder pattern.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use reinhardt_types::{MiddlewareChain, Handler, Middleware};
    /// use std::sync::Arc;
    ///
    /// # struct MyHandler;
    /// # struct MyMiddleware;
    /// # #[async_trait::async_trait]
    /// # impl Handler for MyHandler {
    /// #     async fn handle(&self, request: reinhardt_http::Request) -> reinhardt_exception::Result<reinhardt_http::Response> {
    /// #         Ok(reinhardt_http::Response::new())
    /// #     }
    /// # }
    /// # #[async_trait::async_trait]
    /// # impl Middleware for MyMiddleware {
    /// #     async fn process(&self, request: reinhardt_http::Request, next: Arc<dyn Handler>) -> reinhardt_exception::Result<reinhardt_http::Response> {
    /// #         next.handle(request).await
    /// #     }
    /// # }
    /// let handler = Arc::new(MyHandler);
    /// let middleware = Arc::new(MyMiddleware);
    /// let chain = MiddlewareChain::new(handler)
    ///     .with_middleware(middleware);
    /// ```
    pub fn with_middleware(mut self, middleware: Arc<dyn Middleware>) -> Self {
        self.middlewares.push(middleware);
        self
    }
    /// Adds a middleware to the chain.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use reinhardt_types::{MiddlewareChain, Handler, Middleware};
    /// use std::sync::Arc;
    ///
    /// # struct MyHandler;
    /// # struct MyMiddleware;
    /// # #[async_trait::async_trait]
    /// # impl Handler for MyHandler {
    /// #     async fn handle(&self, request: reinhardt_http::Request) -> reinhardt_exception::Result<reinhardt_http::Response> {
    /// #         Ok(reinhardt_http::Response::new())
    /// #     }
    /// # }
    /// # #[async_trait::async_trait]
    /// # impl Middleware for MyMiddleware {
    /// #     async fn process(&self, request: reinhardt_http::Request, next: Arc<dyn Handler>) -> reinhardt_exception::Result<reinhardt_http::Response> {
    /// #         next.handle(request).await
    /// #     }
    /// # }
    /// let handler = Arc::new(MyHandler);
    /// let middleware = Arc::new(MyMiddleware);
    /// let mut chain = MiddlewareChain::new(handler);
    /// chain.add_middleware(middleware);
    /// ```
    pub fn add_middleware(&mut self, middleware: Arc<dyn Middleware>) {
        self.middlewares.push(middleware);
    }
}

#[async_trait]
impl Handler for MiddlewareChain {
    async fn handle(&self, request: Request) -> Result<Response> {
        if self.middlewares.is_empty() {
            return self.handler.handle(request).await;
        }

        // Build nested handler chain using composition
        let mut current_handler = self.handler.clone();

        for middleware in self.middlewares.iter().rev() {
            let mw = middleware.clone();
            let handler = current_handler.clone();

            current_handler = Arc::new(ComposedHandler {
                middleware: mw,
                next: handler,
            });
        }

        current_handler.handle(request).await
    }
}

/// Internal handler that composes middleware with next handler
struct ComposedHandler {
    middleware: Arc<dyn Middleware>,
    next: Arc<dyn Handler>,
}

#[async_trait]
impl Handler for ComposedHandler {
    async fn handle(&self, request: Request) -> Result<Response> {
        self.middleware.process(request, self.next.clone()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use hyper::{HeaderMap, Method, Uri, Version};

    // Mock handler for testing
    struct MockHandler {
        response_body: String,
    }

    #[async_trait]
    impl Handler for MockHandler {
        async fn handle(&self, _request: Request) -> Result<Response> {
            Ok(Response::ok().with_body(self.response_body.clone()))
        }
    }

    // Mock middleware for testing
    struct MockMiddleware {
        prefix: String,
    }

    #[async_trait]
    impl Middleware for MockMiddleware {
        async fn process(&self, request: Request, next: Arc<dyn Handler>) -> Result<Response> {
            // Call the next handler
            let response = next.handle(request).await?;

            // Modify the response
            let current_body = String::from_utf8(response.body.to_vec()).unwrap_or_default();
            let new_body = format!("{}{}", self.prefix, current_body);

            Ok(Response::ok().with_body(new_body))
        }
    }

    fn create_test_request() -> Request {
        Request::new(
            Method::GET,
            "/".parse::<Uri>().unwrap(),
            Version::HTTP_11,
            HeaderMap::new(),
            Bytes::new(),
        )
    }

    #[tokio::test]
    async fn test_handler_basic() {
        let handler = MockHandler {
            response_body: "Hello".to_string(),
        };

        let request = create_test_request();
        let response = handler.handle(request).await.unwrap();

        let body = String::from_utf8(response.body.to_vec()).unwrap();
        assert_eq!(body, "Hello");
    }

    #[tokio::test]
    async fn test_middleware_basic() {
        let handler = Arc::new(MockHandler {
            response_body: "World".to_string(),
        });

        let middleware = MockMiddleware {
            prefix: "Hello, ".to_string(),
        };

        let request = create_test_request();
        let response = middleware.process(request, handler).await.unwrap();

        let body = String::from_utf8(response.body.to_vec()).unwrap();
        assert_eq!(body, "Hello, World");
    }

    #[tokio::test]
    async fn test_middleware_chain_empty() {
        let handler = Arc::new(MockHandler {
            response_body: "Test".to_string(),
        });

        let chain = MiddlewareChain::new(handler);

        let request = create_test_request();
        let response = chain.handle(request).await.unwrap();

        let body = String::from_utf8(response.body.to_vec()).unwrap();
        assert_eq!(body, "Test");
    }

    #[tokio::test]
    async fn test_middleware_chain_single() {
        let handler = Arc::new(MockHandler {
            response_body: "Handler".to_string(),
        });

        let middleware1 = Arc::new(MockMiddleware {
            prefix: "MW1:".to_string(),
        });

        let chain = MiddlewareChain::new(handler).with_middleware(middleware1);

        let request = create_test_request();
        let response = chain.handle(request).await.unwrap();

        let body = String::from_utf8(response.body.to_vec()).unwrap();
        assert_eq!(body, "MW1:Handler");
    }

    #[tokio::test]
    async fn test_middleware_chain_multiple() {
        let handler = Arc::new(MockHandler {
            response_body: "Data".to_string(),
        });

        let middleware1 = Arc::new(MockMiddleware {
            prefix: "M1:".to_string(),
        });

        let middleware2 = Arc::new(MockMiddleware {
            prefix: "M2:".to_string(),
        });

        let chain = MiddlewareChain::new(handler)
            .with_middleware(middleware1)
            .with_middleware(middleware2);

        let request = create_test_request();
        let response = chain.handle(request).await.unwrap();

        let body = String::from_utf8(response.body.to_vec()).unwrap();
        // Middleware are applied in the order they were added
        assert_eq!(body, "M1:M2:Data");
    }

    #[tokio::test]
    async fn test_middleware_chain_add_middleware() {
        let handler = Arc::new(MockHandler {
            response_body: "Result".to_string(),
        });

        let middleware = Arc::new(MockMiddleware {
            prefix: "Prefix:".to_string(),
        });

        let mut chain = MiddlewareChain::new(handler);
        chain.add_middleware(middleware);

        let request = create_test_request();
        let response = chain.handle(request).await.unwrap();

        let body = String::from_utf8(response.body.to_vec()).unwrap();
        assert_eq!(body, "Prefix:Result");
    }
}
