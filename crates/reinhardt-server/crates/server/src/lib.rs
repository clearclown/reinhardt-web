pub mod http;

#[cfg(feature = "graphql")]
pub mod graphql;

#[cfg(feature = "websocket")]
pub mod websocket;

pub use http::{serve, HttpServer};

#[cfg(feature = "graphql")]
pub use graphql::{graphql_handler, GraphQLHandler};

#[cfg(feature = "websocket")]
pub use websocket::{serve_websocket, WebSocketHandler, WebSocketServer};

// Re-export types needed for server trait
pub use reinhardt_http::{Request, Response};

/// Common server trait that all server types implement
pub trait ServerHandler: Send + Sync {
    type Error;
    fn handle(
        &self,
        request: Request,
    ) -> impl std::future::Future<Output = Result<Response, Self::Error>> + Send;
}
