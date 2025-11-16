//! # reinhardt-graphql
//!
//! GraphQL API support for Reinhardt framework.
//!
//! This is a facade crate that re-exports functionality from:
//! - `reinhardt-graphql-core`: Core GraphQL implementation
//! - `reinhardt-graphql-macros`: Procedural macros for GraphQL (when features enabled)
//!
//! ## Features
//!
//! - `graphql-grpc`: GraphQL facade over gRPC for Query/Mutation
//! - `subscription`: gRPC-based Subscriptions (Rust 2024 compatible)
//! - `full`: All features enabled
//!
//! ## Example
//!
//! ```rust,ignore
//! use reinhardt_graphql::{Schema, Query, Mutation};
//!
//! struct QueryRoot;
//!
//! #[async_graphql::Object]
//! impl QueryRoot {
//!     async fn hello(&self) -> &str {
//!         "Hello, World!"
//!     }
//! }
//! ```

// Re-export all core functionality
pub use reinhardt_graphql_core::*;

// Re-export macros when graphql-grpc or subscription features are enabled
#[cfg(any(feature = "graphql-grpc", feature = "subscription"))]
pub use reinhardt_graphql_macros::{GrpcGraphQLConvert, GrpcSubscription};

// Provide a convenient macros module for explicit imports
#[cfg(any(feature = "graphql-grpc", feature = "subscription"))]
pub mod macros {
	pub use reinhardt_graphql_macros::*;
}
