# reinhardt-db

Django-style database layer for Reinhardt framework

## Overview

`reinhardt-db` provides a comprehensive database layer for Reinhardt applications, inspired by Django's ORM with powerful features for database abstraction, object-relational mapping, migrations, and connection pooling.

This crate serves as a parent crate that integrates multiple database-related sub-crates to provide a unified database experience.

## Features

### Implemented ✓

This parent crate re-exports functionality from the following sub-crates:

- **Database** (`reinhardt-database`): Low-level database abstraction layer
  - Unified DatabaseBackend trait for SQL databases
  - Async database operations (execute, fetch_one, fetch_all)
  - Query builders (SelectBuilder, InsertBuilder, UpdateBuilder, DeleteBuilder)
  - Type-safe parameter binding
  - Connection pooling support

- **ORM** (`reinhardt-orm`): Object-Relational Mapping system
  - Django-inspired Model trait
  - QuerySet API for chainable queries
  - Field types (AutoField, CharField, IntegerField, DateTimeField, etc.)
  - Timestamped and SoftDeletable traits
  - Relationship management
  - Validators and choices

- **Migrations** (`reinhardt-migrations`): Schema migration system
  - Automatic migration generation from model changes
  - Forward and backward migrations
  - Schema versioning and dependency management
  - Migration operations (CreateModel, AddField, AlterField, etc.)
  - State management and autodetection

- **Pool** (`reinhardt-pool`): Connection pool management
  - Database connection pooling
  - Connection lifecycle management
  - Pool configuration and sizing

- **Hybrid** (`reinhardt-hybrid`): Hybrid database support
  - Multi-database routing
  - Read/write splitting
  - Database sharding support

- **Associations** (`reinhardt-associations`): Relationship management
  - Foreign key relationships
  - Many-to-many relationships
  - One-to-one relationships
  - Lazy loading and eager loading

### Implemented ✓ (Additional Features)

- **Advanced Query Optimization**
  - Query result caching with cache hit/miss tracking
  - Query plan analysis and optimization
  - SELECT DISTINCT optimization
  - EXISTS vs IN subquery optimization
  - Cursor-based pagination (more efficient than OFFSET)
  - Bulk operations (bulk create, bulk update)
  - N+1 query prevention with select_related and prefetch_related
  - Lazy query evaluation
  - Only/Defer field optimization for reduced data transfer
  - Aggregate pushdown optimization

- **Enhanced Transaction Management**
  - Nested transactions with savepoint support
  - Isolation level control (ReadUncommitted, ReadCommitted, RepeatableRead, Serializable)
  - Named savepoints (create, release, rollback to savepoint)
  - Transaction state tracking (NotStarted, Active, Committed, RolledBack)
  - Two-phase commit (2PC) for distributed transactions
  - Atomic transaction wrapper (Django-style transaction.atomic)
  - Database-level transaction execution methods

- **Database Replication and Routing**
  - Read/write splitting via DatabaseRouter
  - Model-based database routing rules
  - Configurable default database
  - Per-model read and write database configuration
  - Multi-database support through hybrid module

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
reinhardt-db = "0.1.0-alpha.1"
```

### Optional Features

Enable specific features based on your needs:

```toml
[dependencies]
reinhardt-db = { version = "0.1.0-alpha.1", features = ["postgres", "orm", "migrations"] }
```

Available features:

- `database` (default): Low-level database layer
- `backends` (default): Backend implementations
- `pool` (default): Connection pooling
- `orm` (default): ORM functionality
- `migrations` (default): Migration system
- `hybrid` (default): Multi-database support
- `associations` (default): Relationship management
- `postgres`: PostgreSQL support
- `sqlite`: SQLite support
- `mysql`: MySQL support
- `all-databases`: All database backends

## Usage

### Define Models

```rust
use reinhardt::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
#[model(app_label = "myapp", table_name = "users")]
pub struct User {
    /// Primary key
    #[field(primary_key = true)]
    pub id: i64,

    /// Username (max 50 characters, unique)
    #[field(max_length = 50, unique = true)]
    pub username: String,

    /// Email address (max 254 characters)
    #[field(max_length = 254)]
    pub email: String,

    /// User's age
    pub age: i32,

    /// Account creation timestamp (auto-populated on insert)
    #[field(auto_now_add = true)]
    pub created_at: DateTime<Utc>,

    /// Last update timestamp (auto-updated on save)
    #[field(auto_now = true)]
    pub updated_at: DateTime<Utc>,
}
```

**Field Attributes:**
- `#[field(primary_key = true)]` - Primary key
- `#[field(max_length = N)]` - Maximum length for strings
- `#[field(unique = true)]` - Unique constraint
- `#[field(auto_now_add = true)]` - Auto-populate on creation
- `#[field(auto_now = true)]` - Auto-update on save
- `#[field(null = true)]` - Allow NULL values
- `#[field(default = value)]` - Default value
- `#[field(foreign_key = "ModelType")]` - Foreign key relationship

For a complete list of field attributes, see the [Field Attributes Guide](../../docs/field_attributes.md).

**Note**: The `#[model(...)]` attribute macro automatically generates:
- `Model` trait implementation
- Type-safe field accessors (`User::field_username()`, `User::field_email()`, etc.)
- Global model registry registration
- Support for composite primary keys

### Query with QuerySet

```rust
use reinhardt_db::{QuerySet, Model};

// Get all users
let users = User::objects().all().await?;

// Filter users
let adults = User::objects()
    .filter("age__gte", 18)
    .order_by("-created_at")
    .all()
    .await?;

// Get a single user
let user = User::objects()
    .filter("username", "john")
    .first()
    .await?;
```

### Create Migrations

```rust
use reinhardt_db::{Migration, CreateModel, AddField};

// Create a new migration
let migration = Migration::new("0001_initial")
    .add_operation(CreateModel {
        name: "User",
        fields: vec![
            ("id", "AutoField"),
            ("username", "CharField(max_length=50)"),
            ("email", "EmailField"),
        ],
    });

// Apply migration
migration.apply(db).await?;
```

### Connection Pooling

```rust
use reinhardt_db::Pool;

// Create a connection pool
let pool = Pool::new("postgres://user:pass@localhost/db")
    .max_connections(10)
    .build()
    .await?;

// Get a connection
let conn = pool.get().await?;
```

## Sub-crates

This parent crate contains the following sub-crates:

```
reinhardt-db/
├── Cargo.toml          # Parent crate definition
├── src/
│   └── lib.rs          # Re-exports from sub-crates
└── crates/
    ├── backends/       # Backend implementations
    ├── backends-pool/  # Pool backend abstractions
    ├── database/       # Low-level database layer
    ├── pool/           # Connection pooling
    ├── orm/            # ORM system
    ├── migrations/     # Migration system
    ├── hybrid/         # Multi-database support
    └── associations/   # Relationship management
```

## Supported Databases

- PostgreSQL
- MySQL
- SQLite

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
