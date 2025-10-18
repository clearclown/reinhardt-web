//! WebSocket support for Reinhardt framework

pub mod connection;
pub mod handler;

pub use connection::{Message, WebSocketConnection, WebSocketError, WebSocketResult};
pub use handler::{RoomManager, WebSocketHandler};

#[cfg(test)]
mod tests;
