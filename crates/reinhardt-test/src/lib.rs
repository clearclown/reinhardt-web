//! # Reinhardt Test
//!
//! Testing utilities for the Reinhardt framework.
//!
//! ## Overview
//!
//! This crate provides comprehensive testing tools inspired by Django REST Framework,
//! including API clients, request factories, assertions, and TestContainers integration
//! for database testing.
//!
//! ## Features
//!
//! - **[`APIClient`]**: HTTP client for making test API requests
//! - **[`APIRequestFactory`]**: Factory for creating mock HTTP requests
//! - **[`APITestCase`]**: Base test case with common assertions
//! - **Response Assertions**: Status, header, and body assertions
//! - **[`Factory`]**: Model factory for generating test data
//! - **[`DebugToolbar`]**: Debug panel for inspecting queries and timing
//! - **[`WebSocketTestClient`]**: WebSocket connection testing
//! - **TestContainers**: Database containers (PostgreSQL, MySQL, Redis) integration
//!
//! ## Quick Start
//!
//! ### API Client
//!
//! ```rust,ignore
//! use reinhardt_test::{APIClient, assert_status};
//! use hyper::StatusCode;
//!
//! #[tokio::test]
//! async fn test_user_list() {
//!     let client = APIClient::new("http://localhost:8000");
//!
//!     let response = client.get("/api/users/").await.unwrap();
//!     assert_status(&response, StatusCode::OK);
//!
//!     let users: Vec<User> = response.json().await.unwrap();
//!     assert!(!users.is_empty());
//! }
//! ```
//!
//! ### Request Factory
//!
//! ```rust,ignore
//! use reinhardt_test::{APIRequestFactory, create_test_request};
//!
//! #[tokio::test]
//! async fn test_view_directly() {
//!     let factory = APIRequestFactory::new();
//!
//!     // Create a GET request
//!     let request = factory.get("/api/users/").build();
//!
//!     // Create a POST request with JSON body
//!     let request = factory.post("/api/users/")
//!         .json(&json!({"name": "Alice"}))
//!         .build();
//!
//!     // Pass to view handler directly
//!     let response = my_view(request).await;
//! }
//! ```
//!
//! ### Assertions
//!
//! ```rust,ignore
//! use reinhardt_test::{assert_status, assert_has_header, assert_header_equals, extract_json};
//! use hyper::StatusCode;
//!
//! // Status assertions
//! assert_status(&response, StatusCode::OK);
//! assert_status(&response, StatusCode::CREATED);
//!
//! // Header assertions
//! assert_has_header(&response, "Content-Type");
//! assert_header_equals(&response, "Content-Type", "application/json");
//!
//! // Body extraction
//! let data: MyStruct = extract_json(&response).await.unwrap();
//! ```
//!
//! ### TestContainers (Database Testing)
//!
//! Requires the `testcontainers` feature:
//!
//! ```rust,ignore
//! use reinhardt_test::{with_postgres, PostgresContainer};
//!
//! #[tokio::test]
//! async fn test_with_database() {
//!     with_postgres(|db: PostgresContainer| async move {
//!         let connection_url = db.connection_url();
//!
//!         // Run tests against the database
//!         let pool = create_pool(&connection_url).await;
//!         // ...
//!     }).await;
//! }
//! ```
//!
//! ### Model Factory
//!
//! ```rust,ignore
//! use reinhardt_test::{Factory, FactoryBuilder};
//!
//! let user = FactoryBuilder::<User>::new()
//!     .with("name", "Test User")
//!     .with("email", "test@example.com")
//!     .build();
//! ```
//!
//! ## Modules
//!
//! - [`assertions`]: Response assertion utilities
//! - [`client`]: [`APIClient`] for HTTP testing
//! - [`factory`]: [`APIRequestFactory`] for request creation
//! - [`fixtures`]: Test data generation and fixtures
//! - [`http`]: HTTP helper functions
//! - [`mock`]: Mock objects and spies
//! - [`server`]: Test server utilities
//! - [`testcase`]: [`APITestCase`] base class
//! - [`containers`]: TestContainers integration (requires feature)
//!
//! ## Feature Flags
//!
//! - **`testcontainers`**: Enable TestContainers for database testing
//! - **`static`**: Enable static file testing utilities

pub mod assertions;
pub mod client;
pub mod debug;
pub mod factory;
pub mod fixtures;
pub mod http;
pub mod logging;
pub mod messages;
pub mod mock;
pub mod resource;
pub mod response;
pub mod server;
pub mod testcase;
pub mod views;
pub mod viewsets;

#[cfg(feature = "testcontainers")]
pub mod containers;

pub mod websocket;

// Re-export testcontainers crates for convenient access via reinhardt::test::testcontainers
#[cfg(feature = "testcontainers")]
pub use testcontainers;

#[cfg(feature = "testcontainers")]
pub use testcontainers_modules;

#[cfg(feature = "static")]
pub mod static_files;

pub use assertions::*;
pub use client::{APIClient, ClientError};
pub use debug::{DebugEntry, DebugPanel, DebugToolbar, SqlQuery, TimingInfo};
pub use factory::{APIRequestFactory, RequestBuilder};
pub use fixtures::{
	Factory, FactoryBuilder, FixtureError, FixtureLoader, FixtureResult, random_test_key,
	test_config_value,
};

#[cfg(feature = "testcontainers")]
pub use fixtures::{postgres_container, redis_container};
pub use http::{
	assert_has_header, assert_header_contains, assert_header_equals, assert_no_header,
	assert_status, create_insecure_request, create_request, create_response_with_headers,
	create_response_with_status, create_secure_request, create_test_request, create_test_response,
	extract_json, get_header, has_header, header_contains, header_equals,
};
pub use logging::init_test_logging;
pub use messages::{
	MessagesTestMixin, assert_message_count, assert_message_exists, assert_message_level,
	assert_message_tags, assert_messages,
};
pub use mock::{CallRecord, MockFunction, MockSchemaEditor, SimpleHandler, Spy};
pub use resource::{
	AsyncTeardownGuard, AsyncTestResource, SuiteGuard, SuiteResource, TeardownGuard, TestResource,
	acquire_suite,
};
pub use response::{ResponseExt, TestResponse};
pub use server::{
	BodyEchoHandler, DelayedHandler, EchoPathHandler, LargeResponseHandler, MethodEchoHandler,
	RouterHandler, StatusCodeHandler, shutdown_test_server, spawn_test_server,
};
pub use testcase::APITestCase;
pub use views::{
	ApiTestModel, ErrorKind, ErrorTestView, SimpleTestView, TestModel, create_api_test_objects,
	create_json_request, create_large_test_objects, create_request as create_view_request,
	create_request_with_headers, create_request_with_path_params, create_test_objects,
};
pub use viewsets::{SimpleViewSet, TestViewSet};

#[cfg(feature = "testcontainers")]
pub use containers::{
	MailHogContainer, MySqlContainer, PostgresContainer, RabbitMQContainer, RedisContainer,
	TestDatabase, with_mailhog, with_mysql, with_postgres, with_rabbitmq, with_redis,
};

#[cfg(feature = "static")]
pub use static_files::*;

pub use websocket::WebSocketTestClient;

/// Re-export commonly used testing types
pub mod prelude {
	pub use super::assertions::*;
	pub use super::client::APIClient;
	pub use super::debug::DebugToolbar;
	pub use super::factory::APIRequestFactory;
	pub use super::fixtures::{
		Factory, FactoryBuilder, FixtureLoader, random_test_key, test_config_value,
	};

	#[cfg(feature = "testcontainers")]
	pub use super::fixtures::{postgres_container, redis_container};
	pub use super::http::{
		assert_has_header, assert_header_contains, assert_header_equals, assert_no_header,
		assert_status, create_insecure_request, create_request, create_response_with_headers,
		create_response_with_status, create_secure_request, create_test_request,
		create_test_response, extract_json, get_header, has_header, header_contains, header_equals,
	};
	pub use super::logging::init_test_logging;
	pub use super::messages::{
		MessagesTestMixin, assert_message_count, assert_message_exists, assert_messages,
	};
	pub use super::mock::{MockFunction, SimpleHandler, Spy};
	pub use super::poll_until;
	pub use super::resource::{
		AsyncTeardownGuard, AsyncTestResource, SuiteGuard, SuiteResource, TeardownGuard,
		TestResource, acquire_suite,
	};
	pub use super::response::TestResponse;
	pub use super::server::{
		BodyEchoHandler, DelayedHandler, EchoPathHandler, LargeResponseHandler, MethodEchoHandler,
		RouterHandler, StatusCodeHandler, shutdown_test_server, spawn_test_server,
	};
	pub use super::testcase::APITestCase;
	pub use super::views::{
		ApiTestModel, ErrorTestView, SimpleTestView, TestModel, create_api_test_objects,
		create_test_objects,
	};
	pub use super::viewsets::{SimpleViewSet, TestViewSet};

	#[cfg(feature = "testcontainers")]
	pub use super::containers::{
		MySqlContainer, PostgresContainer, RedisContainer, TestDatabase, with_mysql, with_postgres,
		with_redis,
	};

	#[cfg(feature = "static")]
	pub use super::static_files::*;
}

/// Poll a condition until it becomes true or timeout is reached.
///
/// This is useful for testing asynchronous operations that may take some time to complete,
/// such as cache expiration, rate limit window resets, or background task completion.
///
/// # Arguments
///
/// * `timeout` - Maximum duration to wait for the condition to become true
/// * `interval` - Duration to wait between each poll attempt
/// * `condition` - Async closure that returns `true` when the desired state is reached
///
/// # Returns
///
/// * `Ok(())` if the condition becomes true within the timeout
/// * `Err(String)` if the timeout is reached before the condition becomes true
///
/// # Examples
///
/// ```no_run
/// use reinhardt_test::poll_until;
/// use std::time::Duration;
///
/// # async fn example() {
/// // Poll until a cache entry expires
/// poll_until(
///     Duration::from_millis(200),
///     Duration::from_millis(10),
///     || async {
///         // Check if cache entry has expired
///         // cache.get("key").await.is_none()
///         true
///     }
/// ).await.expect("Condition should be met");
/// # }
/// ```
pub async fn poll_until<F, Fut>(
	timeout: std::time::Duration,
	interval: std::time::Duration,
	mut condition: F,
) -> Result<(), String>
where
	F: FnMut() -> Fut,
	Fut: std::future::Future<Output = bool>,
{
	let start = std::time::Instant::now();
	while start.elapsed() < timeout {
		if condition().await {
			return Ok(());
		}
		tokio::time::sleep(interval).await;
	}
	Err(format!("Timeout after {:?} waiting for condition", timeout))
}
