//! WebSocket message types

use serde::{Deserialize, Serialize};

/// Message type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
	Text,
	Binary,
	Ping,
	Pong,
	Close,
}

/// WebSocket message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketMessage {
	pub message_type: MessageType,
	pub data: serde_json::Value,
	pub timestamp: Option<i64>,
}

impl WebSocketMessage {
	/// Creates a new text message with the given JSON data.
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_websockets::message::WebSocketMessage;
	/// use serde_json::json;
	///
	/// let msg = WebSocketMessage::text(json!({"hello": "world"}));
	/// assert!(matches!(msg.message_type, reinhardt_websockets::message::MessageType::Text));
	/// assert_eq!(msg.data, json!({"hello": "world"}));
	/// assert!(msg.timestamp.is_none());
	/// ```
	pub fn text(data: serde_json::Value) -> Self {
		Self {
			message_type: MessageType::Text,
			data,
			timestamp: None,
		}
	}
	/// Creates a new binary message with the given JSON data.
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_websockets::message::WebSocketMessage;
	/// use serde_json::json;
	///
	/// let msg = WebSocketMessage::binary(json!([1, 2, 3, 4]));
	/// assert!(matches!(msg.message_type, reinhardt_websockets::message::MessageType::Binary));
	/// assert_eq!(msg.data, json!([1, 2, 3, 4]));
	/// assert!(msg.timestamp.is_none());
	/// ```
	pub fn binary(data: serde_json::Value) -> Self {
		Self {
			message_type: MessageType::Binary,
			data,
			timestamp: None,
		}
	}
	/// Adds a timestamp to the message representing the current time.
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_websockets::message::WebSocketMessage;
	/// use serde_json::json;
	///
	/// let msg = WebSocketMessage::text(json!({"hello": "world"}))
	///     .with_timestamp();
	/// assert!(msg.timestamp.is_some());
	/// assert!(msg.timestamp.unwrap() > 0);
	/// ```
	pub fn with_timestamp(mut self) -> Self {
		self.timestamp = Some(chrono::Utc::now().timestamp());
		self
	}
}
