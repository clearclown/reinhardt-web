//! WebSocket handler and room management

use crate::connection::{Message, WebSocketConnection, WebSocketResult};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Manages WebSocket rooms and client connections
pub struct RoomManager {
    rooms: Arc<RwLock<HashMap<String, Vec<Arc<WebSocketConnection>>>>>,
}

impl RoomManager {
    /// Create a new RoomManager
    pub fn new() -> Self {
        Self {
            rooms: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a connection to a room
    pub async fn join_room(&self, room_name: String, connection: Arc<WebSocketConnection>) {
        let mut rooms = self.rooms.write().await;
        rooms
            .entry(room_name)
            .or_insert_with(Vec::new)
            .push(connection);
    }

    /// Remove a connection from a room
    pub async fn leave_room(&self, room_name: &str, client_id: &str) {
        let mut rooms = self.rooms.write().await;
        if let Some(room) = rooms.get_mut(room_name) {
            room.retain(|conn| conn.id() != client_id);
            if room.is_empty() {
                rooms.remove(room_name);
            }
        }
    }

    /// Broadcast a message to all connections in a room
    pub async fn broadcast_to_room(
        &self,
        room_name: &str,
        message: Message,
    ) -> WebSocketResult<()> {
        let rooms = self.rooms.read().await;
        if let Some(room) = rooms.get(room_name) {
            for connection in room {
                connection.send(message.clone()).await?;
            }
        }
        Ok(())
    }

    /// Broadcast a message to all rooms
    pub async fn broadcast_to_all(&self, message: Message) -> WebSocketResult<()> {
        let rooms = self.rooms.read().await;
        for room in rooms.values() {
            for connection in room {
                connection.send(message.clone()).await?;
            }
        }
        Ok(())
    }

    /// Get the number of connections in a room
    pub async fn get_room_size(&self, room_name: &str) -> usize {
        let rooms = self.rooms.read().await;
        rooms.get(room_name).map(|r| r.len()).unwrap_or(0)
    }

    /// Get all room names
    pub async fn get_all_rooms(&self) -> Vec<String> {
        let rooms = self.rooms.read().await;
        rooms.keys().cloned().collect()
    }
}

impl Default for RoomManager {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket handler trait
pub trait WebSocketHandler: Send + Sync {
    /// Handle incoming message
    fn on_message(
        &self,
        message: Message,
    ) -> impl std::future::Future<Output = WebSocketResult<()>> + Send;

    /// Handle connection open
    fn on_connect(&self) -> impl std::future::Future<Output = WebSocketResult<()>> + Send;

    /// Handle connection close
    fn on_disconnect(&self) -> impl std::future::Future<Output = WebSocketResult<()>> + Send;
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_room_manager_basic() {
        let manager = RoomManager::new();
        let (tx, _rx) = mpsc::unbounded_channel();
        let conn = Arc::new(WebSocketConnection::new("test1".to_string(), tx));

        manager.join_room("chat".to_string(), conn.clone()).await;
        assert_eq!(manager.get_room_size("chat").await, 1);

        manager.leave_room("chat", "test1").await;
        assert_eq!(manager.get_room_size("chat").await, 0);
    }

    #[tokio::test]
    async fn test_room_manager_broadcast() {
        let manager = RoomManager::new();
        let (tx1, mut rx1) = mpsc::unbounded_channel();
        let (tx2, mut rx2) = mpsc::unbounded_channel();

        let conn1 = Arc::new(WebSocketConnection::new("user1".to_string(), tx1));
        let conn2 = Arc::new(WebSocketConnection::new("user2".to_string(), tx2));

        manager.join_room("chat".to_string(), conn1).await;
        manager.join_room("chat".to_string(), conn2).await;

        let msg = Message::text("Hello everyone".to_string());
        manager.broadcast_to_room("chat", msg).await.unwrap();

        assert!(matches!(rx1.try_recv(), Ok(Message::Text { .. })));
        assert!(matches!(rx2.try_recv(), Ok(Message::Text { .. })));
    }
}
