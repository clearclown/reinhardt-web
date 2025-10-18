//! # Reinhardt Dependency Injection
//!
//! FastAPI-inspired dependency injection system for Reinhardt.
//!
//! ## Features
//!
//! - **Type-safe**: Full compile-time type checking
//! - **Async-first**: Built for async/await
//! - **Scoped**: Request-scoped and singleton dependencies
//! - **Composable**: Dependencies can depend on other dependencies
//! - **Cache**: Automatic caching within request scope
//!
//! ## Example
//!
//! ```rust,ignore
//! use reinhardt_di::{Depends, Injectable};
//!
//! // Define a dependency
//! struct Database {
//!     pool: DbPool,
//! }
//!
//! #[async_trait]
//! impl Injectable for Database {
//!     async fn inject(ctx: &InjectionContext) -> Result<Self> {
//!         Ok(Database {
//!             pool: get_pool().await?,
//!         })
//!     }
//! }
//!
//! // Use in endpoint
//! #[endpoint(GET "/users")]
//! async fn list_users(
//!     db: Depends<Database>,
//! ) -> Result<Vec<User>> {
//!     db.query("SELECT * FROM users").await
//! }
//! ```

// Re-export DI core
pub use di::*;

#[cfg(feature = "params")]
pub use reinhardt_params as params;
