//! REST API Tests
//!
//! Compilation and execution control:
//! - Cargo.toml: [[test]] name = "api_tests" required-features = ["with-reinhardt"]
//! - build.rs: Sets 'with-reinhardt' feature when reinhardt is available
//! - When feature is disabled, this entire test file is excluded from compilation

use example_test_macros::example_test;
use reinhardt::prelude::*;

/// REST API functionality test (0.1.x)
#[example_test(version = "^0.1")]
async fn test_rest_api_initialization() {
	let app = Application::builder().build();
	assert!(app.is_ok(), "Failed to initialize REST API application");

	println!("✅ REST API initialized successfully");
}

/// JSON serialization test
#[example_test(version = ">=0.1.0, <0.2.0")]
async fn test_json_serialization() {
	use serde_json::json;

	let user = json!({
		"id": 1,
		"name": "Alice",
		"email": "alice@example.com"
	});

	assert_eq!(user["name"], "Alice");
	println!("✅ JSON serialization works correctly");
}
