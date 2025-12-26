//! NoSQL backend implementations
//!
//! This module contains concrete implementations of NoSQL backends
//! for various databases:
//! - MongoDB (document-oriented)
//! - Redis (key-value) - TODO
//! - Cassandra (column-family) - TODO
//! - DynamoDB (key-value) - TODO
//! - Neo4j (graph) - TODO

#[cfg(feature = "mongodb")]
pub mod mongodb;

// TODO: Implement in future phases
// #[cfg(feature = "redis")]
// pub mod redis;
// #[cfg(feature = "cassandra")]
// pub mod cassandra;
// #[cfg(feature = "dynamodb")]
// pub mod dynamodb;
// #[cfg(feature = "neo4j")]
// pub mod neo4j;
