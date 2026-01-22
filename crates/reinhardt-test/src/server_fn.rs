//! Server function testing utilities.
//!
//! This module provides comprehensive testing utilities for server functions,
//! including context management, authentication mocking, and assertions.
//!
//! # Features
//!
//! - **Enhanced Test Context**: DI-based test context with authentication support
//! - **Mock HTTP**: Request/response mocking for server function testing
//! - **Authentication Mocking**: Test user and session simulation
//! - **Assertions**: Server function result assertions
//! - **Transaction Management**: Database transaction utilities for test isolation
//!
//! # Example
//!
//! ```rust,ignore
//! use reinhardt_test::server_fn::{ServerFnTestContext, TestUser};
//! use reinhardt_di::SingletonScope;
//! use std::sync::Arc;
//! use rstest::*;
//!
//! #[fixture]
//! fn singleton_scope() -> Arc<SingletonScope> {
//!     Arc::new(SingletonScope::new())
//! }
//!
//! #[rstest]
//! #[tokio::test]
//! async fn test_protected_endpoint(singleton_scope: Arc<SingletonScope>) {
//!     let ctx = ServerFnTestContext::new(singleton_scope)
//!         .with_authenticated_user(TestUser::admin())
//!         .build();
//!
//!     let result = my_server_fn::test_call(input, &ctx).await;
//!     assert!(result.is_ok());
//! }
//! ```

mod assertions;
mod auth;
mod context;
mod mock_request;
mod transaction;

// Re-export all public items
pub use assertions::*;
pub use auth::*;
pub use context::*;
pub use mock_request::*;
pub use transaction::*;

// Re-export commonly used types from reinhardt-pages for convenience
#[cfg(not(target_arch = "wasm32"))]
pub use reinhardt_pages::testing::{ServerFnTestable, TestSessionData};
