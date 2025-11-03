# reinhardt-grpc

gRPC foundation crate for the Reinhardt framework

## Overview

This crate provides the foundation for gRPC functionality in the Reinhardt framework. It includes only framework-level common types and adapter traits, with domain-specific implementations left to users.

## Features

### 1. Common Protobuf Types

Generic types provided by the framework:

```protobuf
// Empty - Empty response
message Empty {}

// Timestamp - Timestamp representation
message Timestamp {
  int64 seconds = 1;
  int32 nanos = 2;
}

// Error - Error information
message Error {
  string code = 1;
  string message = 2;
  map<string, string> metadata = 3;
}

// PageInfo - Pagination information
message PageInfo {
  int32 page = 1;
  int32 per_page = 2;
  int32 total = 3;
  bool has_next = 4;
  bool has_prev = 5;
}

// BatchResult - Batch operation result
message BatchResult {
  int32 success_count = 1;
  int32 failure_count = 2;
  repeated Error errors = 3;
}
```

### 2. Adapter Traits

Traits for integrating gRPC services with other framework components (such as GraphQL):

```rust
use reinhardt_grpc::{GrpcServiceAdapter, GrpcSubscriptionAdapter};

/// Adapter for Query/Mutation
#[async_trait]
pub trait GrpcServiceAdapter: Send + Sync {
    type Input;
    type Output;
    type Error: std::error::Error + Send + Sync + 'static;

    async fn call(&self, input: Self::Input) -> Result<Self::Output, Self::Error>;
}

/// Adapter for Subscription
pub trait GrpcSubscriptionAdapter: Send + Sync {
    type Proto;
    type GraphQL;
    type Error: std::error::Error + Send + Sync + 'static;

    fn map_event(&self, proto: Self::Proto) -> Option<Self::GraphQL>;
}
```

### 3. Error Handling

gRPC error types and conversions:

```rust
use reinhardt_grpc::{GrpcError, GrpcResult};

pub enum GrpcError {
    Connection(String),
    Service(String),
    NotFound(String),
    InvalidArgument(String),
    Internal(String),
}
```

## Usage

### Using Your Own .proto Files

1. Create a `proto/` directory in your project

```
my-app/
├── proto/
│   ├── user.proto
│   └── product.proto
├── src/
│   └── main.rs
└── Cargo.toml
```

2. Compile .proto files in `build.rs`

```rust
// build.rs
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_descriptors = protox::compile(
        &["proto/user.proto", "proto/product.proto"],
        &["proto"],
    )?;

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .compile_fds(file_descriptors)?;

    Ok(())
}
```

3. Add dependencies to `Cargo.toml`

```toml
[dependencies]
reinhardt-grpc = "0.1.0-alpha.1"
tonic = "0.12"
prost = "0.13"

[build-dependencies]
tonic-build = "0.12"
protox = "0.7"
```

4. Use generated code

```rust
// src/lib.rs
pub mod proto {
    pub mod user {
        tonic::include_proto!("myapp.user");
    }
    pub mod product {
        tonic::include_proto!("myapp.product");
    }
}

// Use common types from reinhardt-grpc
use reinhardt_grpc::proto::common::{Empty, Timestamp, PageInfo};
```

### Integration with GraphQL

When using with the `reinhardt-graphql` crate, refer to the [reinhardt-graphql documentation](../reinhardt-contrib/crates/graphql/README.md).

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.