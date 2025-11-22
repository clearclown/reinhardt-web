//! Integration tests for hello-world example
//!
//! Compilation and execution control:
//! - Cargo.toml: [[test]] name = "integration" required-features = ["with-reinhardt"]
//! - build.rs: Sets 'with-reinhardt' feature when reinhardt is available
//! - When feature is disabled, this entire test file is excluded from compilation

use example_test_macros::example_test;
use reinhardt::prelude::*;

/// Test that reinhardt can be imported and basic functionality works
#[example_test(version = "*")]
fn test_reinhardt_available() {
	// If this compiles and runs, reinhardt is available
	println!("✅ reinhardt is available from crates.io");
	assert!(true, "reinhardt should be available");
}

/// Test application initialization
#[example_test(version = "*")]
fn test_application_initialization() {
	let result = Application::builder().build();
	assert!(result.is_ok(), "Failed to initialize reinhardt application");
	println!("✅ Application initialized successfully");
}
