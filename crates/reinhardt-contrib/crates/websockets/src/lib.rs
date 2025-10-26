//! WebSocket support for Reinhardt framework
//!
//! This crate provides comprehensive WebSocket support for the Reinhardt framework,
//! including connection management, room-based messaging, authentication, rate limiting,
//! middleware integration, and distributed channel layers.
//!
//! ## Features
//!
//! - **Connection Management**: Robust WebSocket connection handling with lifecycle hooks
//! - **Room-Based Messaging**: Group connections into rooms for targeted broadcasting
//! - **Authentication & Authorization**: Token-based auth and permission-based authorization
//! - **Rate Limiting**: Connection and message rate limiting to prevent abuse
//! - **Middleware Integration**: Pre-processing and post-processing of connections and messages
//! - **WebSocket Routing**: URL-based WebSocket endpoint registration
//! - **Channel Layers**: Distributed messaging for multi-instance deployments
//! - **Consumer Classes**: Django Channels-inspired message handling patterns
//!
//! ## Basic Usage
//!
//! ```
//! use reinhardt_websockets::{WebSocketConnection, Message};
//! use tokio::sync::mpsc;
//! use std::sync::Arc;
//!
//! # tokio_test::block_on(async {
//! let (tx, mut rx) = mpsc::unbounded_channel();
//! let conn = Arc::new(WebSocketConnection::new("user_1".to_string(), tx));
//!
//! conn.send_text("Hello, WebSocket!".to_string()).await.unwrap();
//!
//! let msg = rx.recv().await.unwrap();
//! match msg {
//!     Message::Text { data } => println!("Received: {}", data),
//!     _ => {}
//! }
//! # });
//! ```
//!
//! ## Planned Features
//!
//! - Message compression (gzip, deflate, brotli)
//! - Automatic reconnection support with exponential backoff
//! - Custom protocol support (subprotocols)
//! - Redis-backed channel layer for horizontal scaling
//! - WebSocket metrics and monitoring

pub mod auth;
pub mod channels;
pub mod connection;
pub mod consumers;
pub mod handler;
pub mod middleware;
pub mod room;
pub mod routing;
pub mod throttling;

pub use auth::{
    AuthError, AuthResult, AuthUser, AuthenticatedConnection, AuthorizationPolicy,
    PermissionBasedPolicy, SimpleAuthUser, TokenAuthenticator, WebSocketAuthenticator,
};
pub use channels::{
    ChannelError, ChannelLayer, ChannelLayerWrapper, ChannelMessage, ChannelResult,
    InMemoryChannelLayer,
};
pub use connection::{Message, WebSocketConnection, WebSocketError, WebSocketResult};
pub use consumers::{
    BroadcastConsumer, ConsumerChain, ConsumerContext, EchoConsumer, JsonConsumer,
    WebSocketConsumer,
};
pub use handler::{RoomManager, WebSocketHandler};
pub use middleware::{
    ConnectionContext, ConnectionMiddleware, IpFilterMiddleware, LoggingMiddleware,
    MessageMiddleware, MessageSizeLimitMiddleware, MiddlewareChain, MiddlewareError,
    MiddlewareResult,
};
pub use room::{Room, RoomError, RoomManager as RoomMgr, RoomResult};
pub use routing::{
    clear_websocket_router, get_websocket_router, register_websocket_router,
    reverse_websocket_url, RouteError, RouteResult, WebSocketRoute, WebSocketRouter,
};
pub use throttling::{
    CombinedThrottler, ConnectionThrottler, RateLimitConfig, RateLimiter, ThrottleError,
    ThrottleResult,
};

#[cfg(test)]
mod tests;
