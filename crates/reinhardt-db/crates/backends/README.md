# backends

Database backend implementations for Reinhardt ORM

## Overview

`backends` provides database backend implementations for the Reinhardt ORM layer. It includes support for PostgreSQL, MySQL, and SQLite databases with unified abstractions for query building and execution.

## Features

- PostgreSQL backend implementation
- MySQL backend implementation
- SQLite backend implementation
- Unified database abstraction layer
- Query builder integration with sea-query
- Type-safe parameter binding with sqlx

## Installation

```toml
[dependencies]
backends = { workspace = true }

# Or specify version explicitly if outside workspace
backends = "0.1.0-alpha.1"
```

### Features

- `postgres` (default): PostgreSQL support
- `mysql`: MySQL support
- `sqlite`: SQLite support
- `all-databases`: All database backends

## Usage Examples

### Basic Connection

```rust
use reinhardt_backends::{DatabaseBackend, DatabaseConnection};

// PostgreSQL
let conn = DatabaseConnection::new("postgres://user:password@localhost/database")
    .await?;

// MySQL
let conn = DatabaseConnection::new("mysql://user:password@localhost/database")
    .await?;

// SQLite
let conn = DatabaseConnection::new("sqlite::memory:")
    .await?;
```

### Query Building with SeaQuery v1.0.0-rc

```rust
use reinhardt_backends::DatabaseConnection;
use sea_query::{PostgresQueryBuilder, Query, Expr};
use sea_query::Iden;

#[derive(Iden)]
enum Users {
    Table,
    Id,
    Name,
    Email,
}

// Build query using SeaQuery
let query = Query::select()
    .column(Users::Id)
    .column(Users::Name)
    .column(Users::Email)
    .from(Users::Table)
    .and_where(Expr::col(Users::Name).like("%alice%"))
    .to_owned();

// Convert to SQL
let (sql, values) = query.build(PostgresQueryBuilder);

// Execute with connection
let result = conn.execute(&sql, &values).await?;
```

### Schema Operations

```rust
use reinhardt_backends::schema::{SchemaEditor, CreateTable};

let mut editor = SchemaEditor::new(&conn);

// Create table
editor.create_table("users", |t| {
    t.column("id", "SERIAL PRIMARY KEY");
    t.column("name", "VARCHAR(100) NOT NULL");
    t.column("email", "VARCHAR(255) UNIQUE");
}).await?;

// Add column
editor.add_column("users", "created_at", "TIMESTAMP DEFAULT NOW()")
    .await?;
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.