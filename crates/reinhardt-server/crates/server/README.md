# reinhardt-server

High-performance HTTP server implementation

## Overview

High-performance HTTP server based on Hyper. Provides request routing, WebSocket connections, GraphQL support, and handles concurrent connections with async/await.

## Features

### Implemented ✓

#### Core HTTP Server

- **HTTP/1.1 Server**: High-performance HTTP/1.1 server implementation based on Hyper
- **Async Request Processing**: Full asynchronous processing with Tokio runtime
- **Custom Handler Support**: Add custom logic by implementing the `Handler` trait
- **TCP Connection Management**: Efficient TCP connection management and task spawning
- **Request/Response Conversion**: Automatic conversion between Hyper requests and reinhardt-http Request/Response
- **Error Handling**: Automatically converts handler errors to 500 error responses

#### WebSocket Support (feature = "websocket")

- **WebSocket Server**: WebSocket server implementation based on tokio-tungstenite
- **Custom Message Handlers**: Customize message processing via the `WebSocketHandler` trait
- **Connection Lifecycle Hooks**: Handle connection events with `on_connect` and `on_disconnect`
- **Text/Binary Messages**: Process text messages and echo binary messages
- **Automatic Connection Management**: Automatic handling of WebSocket connection establishment, message loops, and closure
- **Peer Information**: Access to client SocketAddr information

#### GraphQL Support (feature = "graphql")

- **GraphQL Handler**: GraphQL endpoint support with async-graphql integration
- **Schema Builder**: Automatic schema construction from Query and Mutation roots
- **POST Request Processing**: Execute GraphQL queries via POST requests
- **JSON Responses**: Automatic JSON serialization of GraphQL execution results
- **Error Handling**: Proper handling and response of GraphQL errors
- **Empty Subscriptions**: Uses `EmptySubscription` by default

#### Convenience Functions

- **`serve()` function**: Helper function providing easy HTTP server startup
- **`serve_websocket()` function**: Helper function providing easy WebSocket server startup
- **`graphql_handler()` function**: Simplifies Arc wrapping of GraphQL handlers

#### Graceful Shutdown

- **ShutdownCoordinator**: Graceful shutdown coordination mechanism
  - Signal handling (SIGTERM, SIGINT)
  - Wait for existing connections to complete
  - Shutdown with timeout processing
  - Shutdown notification via broadcast channel
- **shutdown_signal()**: Listen for OS shutdown signals
- **listen_with_shutdown()**: Start server with graceful shutdown support
- **serve_with_shutdown()**: Convenience function with graceful shutdown support
- **with_shutdown()**: Add shutdown handling to Futures

#### HTTP/2 Support

- **Http2Server**: HTTP/2 protocol server implementation
  - Uses hyper-util's HTTP/2 builder
  - Full asynchronous request processing
  - Graceful shutdown support
  - Uses same Handler trait as HTTP/1.1
- **serve_http2()**: Easy HTTP/2 server startup
- **serve_http2_with_shutdown()**: HTTP/2 server startup with graceful shutdown support

#### Request Timeouts

- **TimeoutHandler**: Request timeout middleware
  - Configurable timeout duration
  - Returns 408 Request Timeout response on timeout
  - Can wrap any Handler
  - Fully tested

#### Rate Limiting

- **RateLimitHandler**: Rate limiting middleware
  - IP address-based rate limiting
  - Supports Fixed Window and Sliding Window strategies
  - Configurable window period and maximum request count
  - Returns 429 Too Many Requests response when rate limit exceeded
- **RateLimitConfig**: Rate limit configuration
  - `per_minute()`: Per-minute rate limiting
  - `per_hour()`: Per-hour rate limiting
  - Custom configurable

#### Advanced HTTP Features

- **Middleware Pipeline**: Middleware chain for request/response processing
- **Connection Pooling**: Efficient HTTP connection pooling mechanism
- **Request Logging**: Structured request logging

#### WebSocket Advanced Features

- **Broadcast Support**: Message broadcasting to multiple clients
- **Room-Based Management**: Manage clients by rooms
- **Message Compression**: WebSocket message compression support
- **Heartbeat/Ping-Pong**: Connection alive check mechanism
- **Authentication/Authorization**: Authentication and authorization for WebSocket connections

#### GraphQL Advanced Features

- **Subscription Support**: Real-time GraphQL subscriptions
- **DataLoader Integration**: DataLoader for solving N+1 problems
- **GraphQL Playground**: GraphQL IDE integration
- **File Uploads**: File uploads via GraphQL
- **Batch Queries**: Batch execution of multiple queries

#### Testing & Monitoring

- **Metrics**: Server metrics collection and publishing
- **Health Checks**: Server health check endpoints
- **Tracing**: Distributed tracing support

## Usage

### Basic HTTP Server

```rust
use std::sync::Arc;
use std::net::SocketAddr;
use reinhardt_server::{HttpServer, serve};
use reinhardt_types::Handler;
use reinhardt_http::{Request, Response};

struct MyHandler;

#[async_trait::async_trait]
impl Handler for MyHandler {
    async fn handle(&self, _req: Request) -> reinhardt_exception::Result<Response> {
        Ok(Response::ok().with_body("Hello, World!"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = Arc::new(MyHandler);
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;

    // Option 1: Using HttpServer directly
    let server = HttpServer::new(handler.clone());
    server.listen(addr).await?;

    // Option 2: Using convenience function
    serve(addr, handler).await?;

    Ok(())
}
```

### WebSocket Server (feature = "websocket")

```rust
use std::sync::Arc;
use std::net::SocketAddr;
use reinhardt_server::{WebSocketServer, WebSocketHandler, serve_websocket};

struct EchoHandler;

#[async_trait::async_trait]
impl WebSocketHandler for EchoHandler {
    async fn handle_message(&self, message: String) -> Result<String, String> {
        Ok(format!("Echo: {}", message))
    }

    async fn on_connect(&self) {
        println!("Client connected");
    }

    async fn on_disconnect(&self) {
        println!("Client disconnected");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = Arc::new(EchoHandler);
    let addr: SocketAddr = "127.0.0.1:9001".parse()?;
    serve_websocket(addr, handler).await?;
    Ok(())
}
```

### GraphQL Server (feature = "graphql")

```rust
use std::sync::Arc;
use std::net::SocketAddr;
use reinhardt_server::{HttpServer, graphql_handler};
use async_graphql::Object;

struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello(&self) -> &str {
        "Hello, GraphQL!"
    }

    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}

struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn increment(&self, value: i32) -> i32 {
        value + 1
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = graphql_handler(QueryRoot, MutationRoot);
    let addr: SocketAddr = "127.0.0.1:8080".parse()?;

    let server = HttpServer::new(handler);
    server.listen(addr).await?;

    Ok(())
}
```

## Feature Flags

- `websocket`: Enable WebSocket server support
- `graphql`: Enable GraphQL server support

## Dependencies

- `hyper`: HTTP server foundation
- `tokio`: Async runtime
- `tokio-tungstenite`: WebSocket support (optional)
- `async-graphql`: GraphQL support (optional)
