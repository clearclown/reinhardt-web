#[cfg(feature = "websocket")]
use futures_util::{SinkExt, StreamExt};
#[cfg(feature = "websocket")]
use std::net::SocketAddr;
#[cfg(feature = "websocket")]
use std::sync::Arc;
#[cfg(feature = "websocket")]
use tokio::net::{TcpListener, TcpStream};
#[cfg(feature = "websocket")]
use tokio_tungstenite::{accept_async, tungstenite::Message};

/// Trait for handling WebSocket messages
#[cfg(feature = "websocket")]
#[async_trait::async_trait]
pub trait WebSocketHandler: Send + Sync {
    /// Handle an incoming WebSocket message
    async fn handle_message(&self, message: String) -> Result<String, String>;

    /// Called when a WebSocket connection is established
    async fn on_connect(&self) {}

    /// Called when a WebSocket connection is closed
    async fn on_disconnect(&self) {}
}

/// WebSocket server
#[cfg(feature = "websocket")]
pub struct WebSocketServer {
    pub handler: Arc<dyn WebSocketHandler>,
}

#[cfg(feature = "websocket")]
impl WebSocketServer {
    /// Create a new WebSocket server with the given handler
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use reinhardt_server::WebSocketServer;
    /// use reinhardt_server::WebSocketHandler;
    ///
    /// struct EchoHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl WebSocketHandler for EchoHandler {
    ///     async fn handle_message(&self, message: String) -> Result<String, String> {
    ///         Ok(format!("Echo: {}", message))
    ///     }
    /// }
    ///
    /// let handler = Arc::new(EchoHandler);
    /// let server = WebSocketServer::new(handler);
    /// ```
    pub fn new(handler: Arc<dyn WebSocketHandler>) -> Self {
        Self { handler }
    }
    /// Start the WebSocket server and listen on the given address
    ///
    /// This method starts the server and begins accepting WebSocket connections.
    /// It runs indefinitely until an error occurs.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use std::net::SocketAddr;
    /// use reinhardt_server::WebSocketServer;
    /// use reinhardt_server::WebSocketHandler;
    ///
    /// struct EchoHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl WebSocketHandler for EchoHandler {
    ///     async fn handle_message(&self, message: String) -> Result<String, String> {
    ///         Ok(message)
    ///     }
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let handler = Arc::new(EchoHandler);
    /// let server = WebSocketServer::new(handler);
    /// let addr: SocketAddr = "127.0.0.1:9001".parse()?;
    /// server.listen(addr).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn listen(self, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(addr).await?;
        println!("WebSocket server listening on ws://{}", addr);

        loop {
            let (stream, peer_addr) = listener.accept().await?;
            let handler = self.handler.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_connection(stream, handler, peer_addr).await {
                    eprintln!("Error handling WebSocket connection: {:?}", e);
                }
            });
        }
    }
    /// Handle a single WebSocket connection
    ///
    /// This is an internal method used by the server to process individual WebSocket connections.
    /// It manages the WebSocket handshake, message handling, and connection lifecycle.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use std::net::SocketAddr;
    /// use tokio::net::TcpStream;
    /// use reinhardt_server::WebSocketServer;
    /// use reinhardt_server::WebSocketHandler;
    ///
    /// struct EchoHandler;
    ///
    /// #[async_trait::async_trait]
    /// impl WebSocketHandler for EchoHandler {
    ///     async fn handle_message(&self, message: String) -> Result<String, String> {
    ///         Ok(message)
    ///     }
    /// }
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let handler = Arc::new(EchoHandler);
    /// let addr: SocketAddr = "127.0.0.1:9001".parse()?;
    /// let stream = TcpStream::connect(addr).await?;
    /// WebSocketServer::handle_connection(stream, handler, addr).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn handle_connection(
        stream: TcpStream,
        handler: Arc<dyn WebSocketHandler>,
        peer_addr: SocketAddr,
    ) -> Result<(), Box<dyn std::error::Error>> {
        println!("WebSocket connection from: {}", peer_addr);

        let ws_stream = accept_async(stream).await?;
        let (mut write, mut read) = ws_stream.split();

        // Notify handler of connection
        handler.on_connect().await;

        // Handle messages
        while let Some(message) = read.next().await {
            match message {
                Ok(msg) => {
                    if msg.is_text() {
                        let text = msg.to_text()?;
                        println!("Received: {}", text);

                        // Process message through handler
                        match handler.handle_message(text.to_string()).await {
                            Ok(response) => {
                                write.send(Message::Text(response)).await?;
                            }
                            Err(error) => {
                                write.send(Message::Text(error)).await?;
                            }
                        }
                    } else if msg.is_binary() {
                        // Echo binary messages
                        write.send(msg).await?;
                    } else if msg.is_close() {
                        println!("Connection closing: {}", peer_addr);
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("WebSocket error: {}", e);
                    break;
                }
            }
        }

        // Notify handler of disconnection
        handler.on_disconnect().await;

        println!("WebSocket connection closed: {}", peer_addr);
        Ok(())
    }
}

/// Helper function to create and run a WebSocket server
///
/// This is a convenience function that creates a `WebSocketServer` and starts listening.
///
/// # Examples
///
/// ```no_run
/// use std::sync::Arc;
/// use std::net::SocketAddr;
/// use reinhardt_server::serve_websocket;
/// use reinhardt_server::WebSocketHandler;
///
/// struct ChatHandler;
///
/// #[async_trait::async_trait]
/// impl WebSocketHandler for ChatHandler {
///     async fn handle_message(&self, message: String) -> Result<String, String> {
///         Ok(format!("Received: {}", message))
///     }
///
///     async fn on_connect(&self) {
///         println!("Client connected");
///     }
///
///     async fn on_disconnect(&self) {
///         println!("Client disconnected");
///     }
/// }
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let handler = Arc::new(ChatHandler);
/// let addr: SocketAddr = "127.0.0.1:9001".parse()?;
/// serve_websocket(addr, handler).await?;
/// # Ok(())
/// # }
/// ```
#[cfg(feature = "websocket")]
pub async fn serve_websocket(
    addr: SocketAddr,
    handler: Arc<dyn WebSocketHandler>,
) -> Result<(), Box<dyn std::error::Error>> {
    let server = WebSocketServer::new(handler);
    server.listen(addr).await
}

#[cfg(all(test, feature = "websocket"))]
mod tests {
    use super::*;

    struct EchoHandler;

    #[async_trait::async_trait]
    impl WebSocketHandler for EchoHandler {
        async fn handle_message(&self, message: String) -> Result<String, String> {
            Ok(format!("Echo: {}", message))
        }
    }

    #[tokio::test]
    async fn test_websocket_server_creation() {
        let handler = Arc::new(EchoHandler);
        let _server = WebSocketServer::new(handler);
    }
}
