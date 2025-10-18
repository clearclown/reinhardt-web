use bytes::Bytes;
use http_body_util::{BodyExt, Full};
use hyper::body::Incoming;
use hyper::server::conn::http1;
use hyper::service::Service;
use hyper_util::rt::TokioIo;
use reinhardt_http::{Request, Response};
use reinhardt_types::Handler;
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

/// HTTP Server
pub struct HttpServer {
    pub handler: Arc<dyn Handler>,
}

impl HttpServer {
    /// Create a new server with the given handler
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use reinhardt_server::HttpServer;
    /// use reinhardt_types::Handler;
    /// use reinhardt_http::{Request, Response};
    ///
    /// struct MyHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl Handler for MyHandler {
    ///     async fn handle(&self, _req: Request) -> reinhardt_exception::Result<Response> {
    ///         Ok(Response::ok().with_body("Hello"))
    ///     }
    /// }
    ///
    /// let handler = Arc::new(MyHandler);
    /// let server = HttpServer::new(handler);
    /// ```
    pub fn new(handler: Arc<dyn Handler>) -> Self {
        Self { handler }
    }
    /// Start the server and listen on the given address
    ///
    /// This method starts the server and begins accepting connections.
    /// It runs indefinitely until an error occurs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use std::net::SocketAddr;
    /// use reinhardt_server::HttpServer;
    /// use reinhardt_types::Handler;
    /// use reinhardt_http::{Request, Response};
    ///
    /// struct MyHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl Handler for MyHandler {
    ///     async fn handle(&self, _req: Request) -> reinhardt_exception::Result<Response> {
    ///         Ok(Response::ok())
    ///     }
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let handler = Arc::new(MyHandler);
    /// let server = HttpServer::new(handler);
    /// let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    /// server.listen(addr).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn listen(self, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("Server listening on http://{}", addr);

        loop {
            let (stream, _) = listener.accept().await?;
            let handler = self.handler.clone();

            tokio::task::spawn(async move {
                if let Err(err) = Self::handle_connection(stream, handler).await {
                    eprintln!("Error handling connection: {:?}", err);
                }
            });
        }
    }
    /// Handle a single TCP connection by processing HTTP requests
    ///
    /// This is an internal method used by the server to process individual connections.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use std::net::SocketAddr;
    /// use tokio::net::TcpStream;
    /// use reinhardt_server::HttpServer;
    /// use reinhardt_types::Handler;
    /// use reinhardt_http::{Request, Response};
    ///
    /// struct MyHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl Handler for MyHandler {
    ///     async fn handle(&self, _req: Request) -> reinhardt_exception::Result<Response> {
    ///         Ok(Response::ok())
    ///     }
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let handler = Arc::new(MyHandler);
    /// let addr: SocketAddr = "127.0.0.1:8080".parse()?;
    /// let stream = TcpStream::connect(addr).await?;
    /// HttpServer::handle_connection(stream, handler).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn handle_connection(
        stream: TcpStream,
        handler: Arc<dyn Handler>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let io = TokioIo::new(stream);
        let service = RequestService { handler };

        http1::Builder::new().serve_connection(io, service).await?;

        Ok(())
    }
}

/// Service implementation for hyper
struct RequestService {
    handler: Arc<dyn Handler>,
}

impl Service<hyper::Request<Incoming>> for RequestService {
    type Response = hyper::Response<Full<Bytes>>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: hyper::Request<Incoming>) -> Self::Future {
        let handler = self.handler.clone();

        Box::pin(async move {
            // Extract request parts
            let (parts, body) = req.into_parts();

            // Read body
            let body_bytes = body.collect().await?.to_bytes();

            // Create reinhardt Request
            let request = Request::new(
                parts.method,
                parts.uri,
                parts.version,
                parts.headers,
                Bytes::from(body_bytes),
            );

            // Handle request
            let response = handler
                .handle(request)
                .await
                .unwrap_or_else(|_| Response::internal_server_error());

            // Convert to hyper response
            let mut hyper_response = hyper::Response::builder().status(response.status);

            // Add headers
            for (key, value) in response.headers.iter() {
                hyper_response = hyper_response.header(key, value);
            }

            Ok(hyper_response.body(Full::new(response.body))?)
        })
    }
}
/// Helper function to create and run a server
///
/// This is a convenience function that creates an `HttpServer` and starts listening.
///
/// # Examples
///
/// ```no_run
/// use std::sync::Arc;
/// use std::net::SocketAddr;
/// use reinhardt_server::serve;
/// use reinhardt_types::Handler;
/// use reinhardt_http::{Request, Response};
///
/// struct MyHandler;
///
/// #[async_trait::async_trait]
/// impl Handler for MyHandler {
///     async fn handle(&self, _req: Request) -> reinhardt_exception::Result<Response> {
///         Ok(Response::ok().with_body("Hello, World!"))
///     }
/// }
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let handler = Arc::new(MyHandler);
/// let addr: SocketAddr = "127.0.0.1:3000".parse()?;
/// serve(addr, handler).await?;
/// # Ok(())
/// # }
/// ```
pub async fn serve(
    addr: SocketAddr,
    handler: Arc<dyn Handler>,
) -> Result<(), Box<dyn std::error::Error>> {
    let server = HttpServer::new(handler);
    server.listen(addr).await
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestHandler;

    #[async_trait::async_trait]
    impl Handler for TestHandler {
        async fn handle(&self, _request: Request) -> reinhardt_exception::Result<Response> {
            Ok(Response::ok().with_body("Hello, World!"))
        }
    }

    #[tokio::test]
    async fn test_http_server_creation() {
        let _server = HttpServer::new(Arc::new(TestHandler));
        // Just verify server can be created without panicking
    }
}
