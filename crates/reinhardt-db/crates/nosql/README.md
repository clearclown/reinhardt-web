# Reinhardt NoSQL

NoSQL database abstractions for the Reinhardt framework.

This crate provides a unified interface for working with various NoSQL databases, organized by paradigm (Document, Key-Value, Column-Family, Graph).

## Features

- **Document Databases**: MongoDB (✅), CouchDB (planned)
- **Key-Value Stores**: Redis (planned), DynamoDB (planned)
- **Column-Family Stores**: Cassandra (planned)
- **Graph Databases**: Neo4j (planned)
- **Zero-Cost Abstractions**: Uses generics to minimize runtime overhead
- **Type-Safe API**: Compile-time guarantees for database operations
- **Transaction Support**: Multi-document ACID transactions (MongoDB with replica set)

## Architecture

The crate is organized around a trait hierarchy that separates concerns by NoSQL paradigm:

```
NoSQLBackend (base trait)
├── DocumentBackend    → MongoDB, CouchDB
├── KeyValueBackend    → Redis, DynamoDB
├── ColumnBackend      → Cassandra
└── GraphBackend       → Neo4j
```

### Design Principles

1. **SQL vs NoSQL Separation**: Complete separation from SQL-focused `DatabaseBackend` trait
2. **Paradigm-Specific Traits**: Each NoSQL type has optimized traits (not one-size-fits-all)
3. **Hierarchical Design**: Base trait + specialized traits for different paradigms
4. **Zero-Cost Abstraction**: Generics preferred over trait objects

## Installation

Add `reinhardt` to your `Cargo.toml`:

```toml
[dependencies]
reinhardt = { version = "0.1.0-alpha.1", features = ["db-nosql-mongodb"] }

# Or use a preset:
# reinhardt = { version = "0.1.0-alpha.1", features = ["full"] }  # All features
```

Then import NoSQL features:

```rust
use reinhardt::db::nosql::{NoSQLBackend, DocumentBackend};
use reinhardt::db::nosql::mongodb::{MongoDBBackend, MongoDBConfig};
```

**Note:** NoSQL features are included in the `full` feature preset.

## Quick Start

### MongoDB Example

```rust
use reinhardt::db::nosql::{
    mongodb::MongoDBBackend,
    DocumentBackend,
};
use bson::doc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to MongoDB
    let backend = MongoDBBackend::builder()
        .url("mongodb://localhost:27017")
        .database("myapp")
        .max_pool_size(100)
        .build()
        .await?;

    // Insert a document
    let id = backend.insert_one("users", doc! {
        "name": "Alice",
        "email": "alice@example.com",
        "age": 30
    }).await?;

    println!("Inserted document with ID: {}", id);

    // Find the document
    let user = backend.find_one("users", doc! {
        "email": "alice@example.com"
    }).await?;

    if let Some(user) = user {
        println!("Found user: {:?}", user);
    }

    // Update the document
    let result = backend.update_one(
        "users",
        doc! { "email": "alice@example.com" },
        doc! { "$set": { "age": 31 } }
    ).await?;

    println!("Modified {} documents", result.modified_count);

    // Aggregate query
    let results = backend.aggregate("users", vec![
        doc! { "$match": { "age": { "$gte": 18 } } },
        doc! { "$group": {
            "_id": "$status",
            "count": { "$sum": 1 }
        }}
    ]).await?;

    println!("Aggregation results: {:?}", results);

    Ok(())
}
```

### Using with reinhardt-db

```rust
use reinhardt::db::nosql::mongodb::MongoDBBackend;
use reinhardt::db::nosql::DocumentBackend;
use bson::doc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let backend = MongoDBBackend::connect("mongodb://localhost:27017")
        .await?
        .with_database("myapp");

    let id = backend.insert_one("users", doc! {
        "name": "Bob",
        "email": "bob@example.com"
    }).await?;

    Ok(())
}
```

## Feature Flags

### Individual Backend Features

- `mongodb` - MongoDB support
- `redis` - Redis support (planned)
- `cassandra` - Cassandra support (planned)
- `dynamodb` - DynamoDB support (planned)
- `neo4j` - Neo4j support (planned)

### Convenience Feature Groups

- `nosql-all` - Enable all NoSQL backends
- `nosql-document` - Enable all document-oriented databases
- `nosql-key-value` - Enable all key-value stores
- `nosql-column` - Enable all column-family stores
- `nosql-graph` - Enable all graph databases
- `full` - Enable all features

### Example

```toml
# Enable only MongoDB
reinhardt-nosql = { version = "0.1.0-alpha.1", features = ["mongodb"] }

# Enable all document databases
reinhardt-nosql = { version = "0.1.0-alpha.1", features = ["nosql-document"] }

# Enable all NoSQL databases
reinhardt-nosql = { version = "0.1.0-alpha.1", features = ["nosql-all"] }
```

## API Reference

### NoSQLBackend (Base Trait)

All NoSQL backends implement this trait:

```rust
#[async_trait]
pub trait NoSQLBackend: Send + Sync {
    fn backend_type(&self) -> NoSQLBackendType;
    fn nosql_type(&self) -> NoSQLType;
    async fn health_check(&self) -> Result<()>;
    fn as_any(&self) -> &dyn std::any::Any;
}
```

### DocumentBackend (Document-Oriented Databases)

For document databases like MongoDB:

```rust
#[async_trait]
pub trait DocumentBackend: NoSQLBackend {
    async fn find_one(&self, collection: &str, filter: Document) -> Result<Option<Document>>;
    async fn find_many(&self, collection: &str, filter: Document, options: FindOptions) -> Result<Vec<Document>>;
    async fn insert_one(&self, collection: &str, document: Document) -> Result<String>;
    async fn insert_many(&self, collection: &str, documents: Vec<Document>) -> Result<Vec<String>>;
    async fn update_one(&self, collection: &str, filter: Document, update: Document) -> Result<UpdateResult>;
    async fn update_many(&self, collection: &str, filter: Document, update: Document) -> Result<UpdateResult>;
    async fn delete_one(&self, collection: &str, filter: Document) -> Result<u64>;
    async fn delete_many(&self, collection: &str, filter: Document) -> Result<u64>;
    async fn aggregate(&self, collection: &str, pipeline: Vec<Document>) -> Result<Vec<Document>>;
}
```

## Roadmap

### Phase 1: MongoDB Foundation (Current)
- ✅ NoSQL trait hierarchy design
- ✅ MongoDB connection and CRUD operations
- ✅ Transaction support
- ⏸️ Comprehensive tests
- ⏸️ Documentation

### Phase 2: Redis Integration
- KeyValueBackend trait
- Redis implementation
- Cluster support
- Pub/Sub support

### Phase 3: Cassandra Integration
- ColumnBackend trait
- Cassandra implementation
- Consistency level configuration

### Phase 4: DynamoDB & Neo4j
- DynamoDB implementation (KeyValueBackend)
- GraphBackend trait
- Neo4j implementation

### Phase 5: ODM (Object-Document Mapper)
- High-level API for document databases
- Separate crate: `reinhardt-nosql-odm`

## Migration from Old MongoDB Backend

If you're migrating from the old `mongodb-backend` feature in `reinhardt-backends`:

**Before (deprecated)**:
```rust
use reinhardt_backends::drivers::mongodb::MongoDBBackend;
use reinhardt_backends::backend::DatabaseBackend;
```

**After (recommended)**:
```rust
use reinhardt::db::nosql::mongodb::MongoDBBackend;
use reinhardt::db::nosql::{NoSQLBackend, DocumentBackend};
```

The old path is deprecated and will be removed in v0.3.0. Use the `nosql-mongodb` feature instead:

```toml
# Old (deprecated)
reinhardt-db = { version = "0.1.0-alpha.1", features = ["mongodb-backend"] }

# New (recommended)
reinhardt-db = { version = "0.1.0-alpha.1", features = ["nosql-mongodb"] }
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT OR Apache-2.0 license.
