//! NoSQL backend traits
//!
//! This module provides trait definitions for different NoSQL paradigms:
//! - `NoSQLBackend`: Base trait for all NoSQL backends
//! - `DocumentBackend`: Trait for document-oriented databases (MongoDB, CouchDB)
//! - `KeyValueBackend`: Trait for key-value stores (Redis, DynamoDB) - TODO
//! - `ColumnBackend`: Trait for column-family stores (Cassandra) - TODO
//! - `GraphBackend`: Trait for graph databases (Neo4j) - TODO

mod base;
mod document;

pub use base::NoSQLBackend;
pub use document::DocumentBackend;

// TODO: Implement in future phases
// mod key_value;
// mod column;
// mod graph;
// pub use key_value::KeyValueBackend;
// pub use column::ColumnBackend;
// pub use graph::GraphBackend;
