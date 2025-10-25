//! WebSocket support for Reinhardt framework
//!
//! ## Planned Features
//! TODO: Implement WebSocket routing integration
//! TODO: Add authentication and authorization hooks
//! TODO: Implement rate limiting and throttling
//! TODO: Add automatic reconnection support
//! TODO: Implement message compression
//! TODO: Add custom protocol support
//! TODO: Implement channel layers for distributed systems
//! TODO: Add consumer classes for advanced patterns
//! TODO: Integrate with Reinhardt middleware system

pub mod connection;
pub mod handler;
pub mod room;

pub use connection::{Message, WebSocketConnection, WebSocketError, WebSocketResult};
pub use handler::{RoomManager, WebSocketHandler};
pub use room::{Room, RoomError, RoomResult};

#[cfg(test)]
mod tests;
