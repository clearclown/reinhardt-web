//! # Migrations (Deprecated)
//!
//! **This file is deprecated.** Migrations are now registered using the `collect_migrations!` macro
//! in each app's `migrations.rs` file (e.g., `apps/todos/migrations.rs`, `apps/users/migrations.rs`).
//!
//! The migrations are automatically registered in the global registry via linkme's distributed_slice
//! pattern at compile time.
//!
//! ## Migration Pattern
//!
//! ```rust,ignore
//! // apps/todos/migrations.rs
//! pub mod _0001_initial;
//!
//! reinhardt::collect_migrations!(
//!     app_label = "todos",
//!     _0001_initial,
//! );
//! ```
//!
//! ## Accessing Migrations
//!
//! ```rust,ignore
//! use reinhardt_migrations::registry::{global_registry, MigrationRegistry};
//!
//! // Get all migrations from global registry
//! let all_migrations = global_registry().all_migrations();
//!
//! // Get migrations for a specific app
//! let todos_migrations = global_registry().migrations_for_app("todos");
//! ```
//!
//! ## Why This Change?
//!
//! - **Automatic Registration**: No need to manually list migrations in a central file
//! - **Type Safety**: Compile-time registration via linkme
//! - **Scalability**: Each app manages its own migrations independently
//! - **Consistency**: Aligns with modern Rust patterns and Reinhardt's architecture
