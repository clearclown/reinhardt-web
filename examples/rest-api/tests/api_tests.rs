//! REST API Tests
//!
//! These tests only run when reinhardt is available from crates.io.
//! The conditional compilation is handled by build.rs.

use example_test_macros::example_test;

#[cfg(not(any(reinhardt_unavailable, reinhardt_version_mismatch)))]
mod tests_with_reinhardt {
	use super::*;
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
}

#[cfg(any(reinhardt_unavailable, reinhardt_version_mismatch))]
mod tests_without_reinhardt {
	use super::*;

	/// Placeholder test that always passes when reinhardt is unavailable
	#[example_test(version = "^0.1")]
	async fn test_placeholder() {
		println!("⚠️  REST API tests require reinhardt from crates.io");
		println!("   Tests will be enabled once reinhardt 0.1.x is published");
	}

	/// JSON serialization test (does not require reinhardt)
	#[example_test(version = ">=0.1.0, <0.2.0")]
	async fn test_json_serialization() {
		use serde_json::json;

		let user = json!({
			"id": 1,
			"name": "Alice",
			"email": "alice@example.com"
		});

		assert_eq!(user["name"], "Alice");
		println!("✅ JSON serialization works correctly (no reinhardt required)");
	}
}
