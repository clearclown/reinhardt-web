//! # reinhardt-database
//!
//! Database abstraction layer for the Reinhardt framework.
//!
//! This crate provides a schema editor abstraction for database DDL operations,
//! inspired by Django's schema editor system.
//!
//! ## Features
//!
//! - **Schema Editor Abstraction**: Unified [`BaseDatabaseSchemaEditor`] trait
//! - **Database-Specific Implementations**: PostgreSQL, MySQL, SQLite support
//! - **Factory Pattern**: Easy creation of database-specific editors
//! - **DDL Operations**: CREATE TABLE, ALTER TABLE, CREATE INDEX, etc.
//! - **Migration Support**: Designed for use with `reinhardt-migrations`
//!
//! ## Quick Start
//!
//! ```rust
//! use reinhardt_database::schema::factory::{SchemaEditorFactory, DatabaseType};
//!
//! let factory = SchemaEditorFactory::new();
//! let editor = factory.create_for_database(DatabaseType::PostgreSQL);
//!
//! // Generate SQL
//! let sql = editor.create_table_sql("users", &[
//!     ("id", "INTEGER PRIMARY KEY"),
//!     ("name", "VARCHAR(100)"),
//! ]);
//! println!("{}", sql);
//! // Output: CREATE TABLE "users" (id INTEGER PRIMARY KEY, name VARCHAR(100));
//! ```
//!
//! ## Database-Specific Features
//!
//! ### PostgreSQL
//!
//! ```rust
//! use reinhardt_database::backends::postgresql::schema::PostgreSQLSchemaEditor;
//!
//! let editor = PostgreSQLSchemaEditor::new();
//!
//! // PostgreSQL-specific features
//! let sql = editor.create_index_concurrently_sql("users", "idx_email", &["email"], false);
//! // CREATE INDEX CONCURRENTLY "idx_email" ON "users" (email);
//! ```
//!
//! ## Feature Flags
//!
//! - `postgres` (default): PostgreSQL support
//! - `sqlite`: SQLite support
//! - `mysql`: MySQL support
//! - `all-databases`: Enable all database backends
//!
//! ## Integration with Reinhardt Migrations
//!
//! This crate is designed to be used by `reinhardt-migrations`:
//!
//! ```rust
//! use reinhardt_database::schema::BaseDatabaseSchemaEditor;
//! use reinhardt_database::schema::factory::{SchemaEditorFactory, DatabaseType};
//!
//! // Create schema editor
//! let factory = SchemaEditorFactory::new();
//! let editor = factory.create_for_database(DatabaseType::PostgreSQL);
//!
//! // Use in migrations
//! let create_table = editor.create_table_sql("products", &[
//!     ("id", "INTEGER PRIMARY KEY"),
//!     ("name", "VARCHAR(255)"),
//!     ("price", "DECIMAL(10, 2)"),
//! ]);
//! ```

pub mod backends;
pub mod error;
pub mod schema;

// Query abstraction layer modules
pub mod backend;
pub mod connection;
pub mod dialect;
pub mod query_builder;
pub mod types;

// Re-export commonly used types
pub use error::DatabaseError;
pub use schema::{BaseDatabaseSchemaEditor, SchemaEditorError, SchemaEditorResult};

// Re-export query abstraction types
pub use backend::DatabaseBackend;
pub use connection::DatabaseConnection;
pub use query_builder::{InsertBuilder, SelectBuilder, UpdateBuilder};
pub use types::{
    DatabaseError as QueryDatabaseError, DatabaseType, QueryResult, QueryValue, Result, Row,
};

// Re-export database-specific schema editors
#[cfg(feature = "postgres")]
pub use backends::postgresql::schema::PostgreSQLSchemaEditor;

#[cfg(feature = "mysql")]
pub use backends::mysql::schema::MySQLSchemaEditor;

#[cfg(feature = "sqlite")]
pub use backends::sqlite::schema::SQLiteSchemaEditor;

// Re-export dialect backends
#[cfg(feature = "postgres")]
pub use dialect::PostgresBackend;

#[cfg(feature = "sqlite")]
pub use dialect::SqliteBackend;

#[cfg(feature = "mysql")]
pub use dialect::MySqlBackend;
