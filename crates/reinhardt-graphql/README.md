# reinhardt-graphql

GraphQL integration

## Overview

GraphQL API support with schema generation from models, query and mutation resolvers, and integration with the authentication and permission system. Provides a flexible alternative to REST APIs.

## Features

### Implemented ✓

#### Core Type System

- **GraphQL Type Markers**: `GraphQLType` and `GraphQLField` traits for type-safe GraphQL type definitions
- **Error Handling**: Custom `GraphQLError` enum with Schema, Resolver, and NotFound variants
- **Base Resolver Trait**: Async `Resolver` trait with generic output types for flexible resolver implementation

#### Schema & Data Types

- **User Type**: Complete GraphQL object implementation with id, name, email, and active fields
- **User Storage**: Thread-safe in-memory storage using `Arc<RwLock<HashMap>>` for user data
  - `new()`: Create new storage instance
  - `add_user()`: Add or update user in storage
  - `get_user()`: Retrieve user by ID
  - `list_users()`: List all stored users
- **Input Types**: `CreateUserInput` for user creation mutations
- **Schema Builder**: `create_schema()` function to build GraphQL schema with data context

#### Query Operations

- **User Queries**:
  - `user(id: ID)`: Retrieve single user by ID
  - `users()`: List all users
  - `hello(name: Option<String>)`: Simple greeting query for testing
- **Context Integration**: Queries access UserStorage through GraphQL context

#### Mutation Operations

- **User Mutations**:
  - `createUser(input: CreateUserInput)`: Create new user with auto-generated UUID
  - `updateUserStatus(id: ID, active: bool)`: Update user active status
- **State Management**: Mutations persist changes to UserStorage

#### Subscription System

- **Event Types**: `UserEvent` enum supporting Created, Updated, and Deleted events
- **Event Broadcasting**: `EventBroadcaster` with tokio broadcast channel (capacity: 100)
  - `new()`: Create new broadcaster instance
  - `broadcast()`: Send events to all subscribers
  - `subscribe()`: Subscribe to event stream
- **Subscription Root**: `SubscriptionRoot` with filtered subscription streams
  - `userCreated()`: Stream of user creation events
  - `userUpdated()`: Stream of user update events
  - `userDeleted()`: Stream of user deletion events (returns ID only)
- **Async Streams**: Real-time event filtering using async-stream

#### Integration

- **async-graphql Integration**: Built on async-graphql framework for production-ready GraphQL server
- **Type Safety**: Full Rust type system integration with compile-time guarantees
- **Async/Await**: Complete async support with tokio runtime
- **Documentation**: Comprehensive doc comments with examples for all public APIs

#### gRPC Transport (Optional - `graphql-grpc` feature)

- **GraphQL over gRPC Service**: `GraphQLGrpcService` implementing gRPC protocol for GraphQL
  - `execute_query()`: Execute GraphQL queries via unary RPC
  - `execute_mutation()`: Execute GraphQL mutations via unary RPC
  - `execute_subscription()`: Execute GraphQL subscriptions via server streaming RPC
- **Protocol Buffers**: Complete proto definitions in `reinhardt-grpc` crate
  - `GraphQLRequest`: query, variables, operation_name
  - `GraphQLResponse`: data, errors, extensions
  - `SubscriptionEvent`: id, event_type, payload, timestamp
- **Request/Response Conversion**: Automatic conversion between gRPC and async-graphql types
- **Error Handling**: Full error information propagation (message, locations, path, extensions)
- **Performance**: Minimal overhead (5-21%, or 0.2-0.8 µs) compared to direct execution
- **Network Communication**: Full TCP/HTTP2 support via tonic
- **Streaming**: Efficient server-side streaming for real-time subscriptions

## Installation

```toml
[dependencies]
# Basic GraphQL support
reinhardt-graphql = "0.1.0-alpha.1"

# With gRPC transport
reinhardt-graphql = { version = "0.1.0-alpha.1", features = ["graphql-grpc"] }

# All features
reinhardt-graphql = { version = "0.1.0-alpha.1", features = ["full"] }
```

## Examples

### Basic GraphQL Usage

```rust
use async_graphql::{EmptySubscription, Schema};
use reinhardt_graphql::schema::{Mutation, Query, UserStorage};

#[tokio::main]
async fn main() {
    let storage = UserStorage::new();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(storage)
        .finish();

    let query = r#"{ hello(name: "World") }"#;
    let result = schema.execute(query).await;
    println!("{}", result.data);
}
```

### GraphQL over gRPC Server

```rust
use async_graphql::{EmptySubscription, Schema};
use reinhardt_graphql::grpc_service::GraphQLGrpcService;
use reinhardt_graphql::schema::{Mutation, Query, UserStorage};
use reinhardt_grpc::proto::graphql::graph_ql_service_server::GraphQlServiceServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let storage = UserStorage::new();
    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .data(storage)
        .finish();

    let service = GraphQLGrpcService::new(schema);
    let grpc_service = GraphQlServiceServer::new(service);

    Server::builder()
        .add_service(grpc_service)
        .serve("127.0.0.1:50051".parse()?)
        .await?;

    Ok(())
}
```

### GraphQL over gRPC Client

```rust
use reinhardt_grpc::proto::graphql::{
    graph_ql_service_client::GraphQlServiceClient,
    GraphQlRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GraphQlServiceClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(GraphQlRequest {
        query: r#"{ hello(name: "gRPC") }"#.to_string(),
        variables: None,
        operation_name: None,
    });

    let response = client.execute_query(request).await?;
    println!("{:?}", response.into_inner());

    Ok(())
}
```

### Running Examples

```bash
# Start gRPC server
cargo run --package reinhardt-graphql --features graphql-grpc --example grpc_server

# In another terminal, run client
cargo run --package reinhardt-graphql --features graphql-grpc --example grpc_client
```

## Testing

```bash
# All tests
cargo test --package reinhardt-graphql --features graphql-grpc

# Integration tests
cargo test --package reinhardt-graphql --features graphql-grpc --test grpc_integration_tests

# Subscription streaming tests
cargo test --package reinhardt-graphql --features graphql-grpc --test grpc_subscription_tests

# E2E tests with real network
cargo test --package reinhardt-graphql --features graphql-grpc --test grpc_e2e_tests

# Performance benchmarks
cargo bench --package reinhardt-graphql --features graphql-grpc
```

## Performance

See [PERFORMANCE.md](PERFORMANCE.md) for detailed benchmarks.

**Summary:**

- Direct GraphQL: ~3-4 µs per query
- gRPC GraphQL: ~4-5 µs per query
- Overhead: 5-21% (+0.2-0.8 µs) for gRPC serialization
- Both approaches are highly performant for real-world applications
