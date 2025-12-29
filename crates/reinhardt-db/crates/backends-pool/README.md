# backends-pool

Database connection pool backend abstractions

## Overview

`backends-pool` provides backend abstractions for database connection pooling in the Reinhardt framework. It defines traits and utilities for managing database connection pools with dependency injection support.

## Implemented Features ✓

- **Connection Pool Abstractions**: Backend traits for database connection pooling
- **Async Connection Management**: Asynchronous connection acquisition and release
- **SQLx Integration**: Seamless integration with sqlx connection pools
- **Dependency Injection Support**: Optional DI integration for automatic pool injection
- **Thread-Safe Handling**: Concurrent connection access with Arc-based sharing
- **Lifecycle Management**: Automatic connection cleanup and pool maintenance

## Installation

Add `reinhardt` to your `Cargo.toml`:

```toml
[dependencies]
reinhardt = { version = "0.1.0-alpha.1", features = ["db-pool"] }

# Or use a preset:
# reinhardt = { version = "0.1.0-alpha.1", features = ["full"] }  # All features
```

Then import pool features:

```rust
use reinhardt::db::pool::{ConnectionPool, PoolConfig};
use reinhardt::db::pool::PoolEventListener;
```

**Note:** Pool features are included in the `full` feature preset.

### Available Features

- `reinhardt-di`: Dependency injection integration

## Usage Examples

### Basic Pool Creation

```rust
use reinhardt::db::pool::{ConnectionPool, PoolConfig};
use std::time::Duration;

let pool_config = PoolConfig::default()
    .with_max_connections(5)
    .with_acquire_timeout(Duration::from_secs(10));

let pool = ConnectionPool::new_postgres(
    "postgres://user:password@localhost/database",
    pool_config
).await?;

// Acquire connection from pool
let mut conn = pool.inner().acquire().await?;
```

### DI Integration (with `reinhardt-di` feature)

```rust
use reinhardt::db::pool::{ConnectionPool, PoolConfig};
use reinhardt_di::{Injectable, Container};

// Pool is automatically injectable
async fn my_handler(pool: Injectable<ConnectionPool>) {
    let conn = pool.acquire().await.unwrap();
    // Use connection...
}

// Register in DI container
let container = Container::new();
container.register(pool);
```

### Custom Configuration

```rust
use reinhardt::db::pool::PoolConfig;
use std::time::Duration;

let config = PoolConfig::new()
    .with_min_connections(2)
    .with_max_connections(10)
    .with_acquire_timeout(Duration::from_secs(30))
    .with_idle_timeout(Duration::from_secs(600))
    .with_test_before_acquire(true);
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.