//! MongoDB backend module (DEPRECATED)
//!
//! **DEPRECATED**: MongoDB has been moved to the `reinhardt-nosql` crate.
//! Please use `reinhardt_nosql::backends::mongodb` instead.
//!
//! This module provides backward compatibility re-exports and will be removed in v0.3.0.
//!
//! # Migration Guide
//!
//! **Before (deprecated)**:
//! ```rust,ignore
//! use reinhardt_backends::drivers::mongodb::MongoDBBackend;
//! use reinhardt_backends::DatabaseBackend;
//! ```
//!
//! **After (recommended)**:
//! ```rust,ignore
//! use reinhardt_nosql::backends::mongodb::MongoDBBackend;
//! use reinhardt_nosql::traits::{NoSQLBackend, DocumentBackend};
//! ```
//!
//! # Why This Change?
//!
//! MongoDB is a NoSQL database and does not fit the SQL-focused `DatabaseBackend` trait.
//! The new `reinhardt-nosql` crate provides a proper trait hierarchy for different NoSQL paradigms:
//! - `NoSQLBackend` (base trait)
//! - `DocumentBackend` (for MongoDB, CouchDB, etc.)
//! - `KeyValueBackend` (for Redis, DynamoDB, etc.)
//! - `ColumnBackend` (for Cassandra, etc.)
//! - `GraphBackend` (for Neo4j, etc.)

#![deprecated(
	since = "0.1.0",
	note = "MongoDB has been moved to reinhardt-nosql. Use reinhardt_nosql::backends::mongodb instead."
)]

#[cfg(feature = "mongodb-backend")]
pub use reinhardt_nosql::backends::mongodb::*;
