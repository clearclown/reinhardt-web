# reinhardt-graphql-core

Core GraphQL implementation for Reinhardt framework.

## Overview

This crate provides the core GraphQL functionality for the Reinhardt framework, including:
- GraphQL type system
- Schema and resolver infrastructure
- Query, Mutation, and Subscription support
- gRPC transport integration (optional)

## Important

**This is an internal subcrate.** Users should depend on `reinhardt-graphql` (the parent facade crate) instead of this crate directly.

```toml
# ✅ Correct - use the facade crate
[dependencies]
reinhardt-graphql = "0.1.0-alpha.1"

# ❌ Incorrect - don't depend on subcrates directly
[dependencies]
reinhardt-graphql-core = "0.1.0-alpha.1"
```

## Features

See the main [reinhardt-graphql README](../../README.md) for comprehensive feature documentation.

## Internal Architecture

This crate is organized into the following modules:

- `types`: GraphQL type markers and base traits
- `schema`: Schema definitions and user types
- `resolvers`: Resolver trait and implementations
- `context`: GraphQL execution context
- `subscription`: Subscription and event system
- `grpc_service`: gRPC transport layer (feature-gated)
