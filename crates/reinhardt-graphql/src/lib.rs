//! GraphQL support for Reinhardt framework
//!
//! This crate provides GraphQL API support for the Reinhardt framework.
//!
//! # Features
//!
//! - **graphql-grpc**: GraphQL facade over gRPC for Query/Mutation
//! - **subscription**: gRPC-based Subscriptions (Rust 2024 compatible)
//! - **di**: Dependency injection support for GraphQL resolvers
//! - **full**: All features enabled
//!
//! # Dependency Injection
//!
//! Enable the `di` feature to use dependency injection in GraphQL resolvers:
//!
//! ```toml
//! [dependencies]
//! reinhardt-graphql = { version = "0.1", features = ["di"] }
//! ```
//!
//! Then use the `#[graphql_handler]` macro:
//!
//! ```rust,ignore
//! use async_graphql::{Context, Object, Result, ID};
//! use reinhardt_graphql::graphql_handler;
//!
//! #[Object]
//! impl Query {
//!     async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<User> {
//!         user_impl(ctx, id).await
//!     }
//! }
//!
//! #[graphql_handler]
//! async fn user_impl(
//!     ctx: &Context<'_>,
//!     id: ID,
//!     #[inject] db: DatabaseConnection,
//! ) -> Result<User> {
//!     // db is automatically resolved
//! }
//! ```

pub mod context;
pub mod schema;
pub mod subscription;
pub mod types;

#[cfg(feature = "di")]
pub mod di;

#[cfg(feature = "graphql-grpc")]
pub mod grpc_service;

pub use context::{DataLoader, GraphQLContext, LoaderError};
pub use schema::{AppSchema, CreateUserInput, Mutation, Query, User, UserStorage, create_schema};
pub use subscription::{EventBroadcaster, SubscriptionRoot, UserEvent};

#[cfg(feature = "graphql-grpc")]
pub use grpc_service::GraphQLGrpcService;

// gRPC integration: re-export of adapter traits and derive macros
#[cfg(any(feature = "graphql-grpc", feature = "subscription"))]
pub use reinhardt_grpc::{GrpcServiceAdapter, GrpcSubscriptionAdapter};

#[cfg(any(feature = "graphql-grpc", feature = "subscription"))]
pub use reinhardt_graphql_macros::{GrpcGraphQLConvert, GrpcSubscription};

// DI support: re-export extension traits and macro
#[cfg(feature = "di")]
pub use di::{GraphQLContextExt, SchemaBuilderExt};

#[cfg(feature = "di")]
pub use reinhardt_graphql_macros::graphql_handler;
