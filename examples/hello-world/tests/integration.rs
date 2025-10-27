//! Integration tests for hello-world example
//!
//! These tests only run when reinhardt is available from crates.io.
//! The conditional compilation is handled by build.rs.

use example_test_macros::example_test;

#[cfg(not(any(reinhardt_unavailable, reinhardt_version_mismatch)))]
mod tests_with_reinhardt {
    use super::*;
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
        assert!(
            result.is_ok(),
            "Failed to initialize reinhardt application"
        );
        println!("✅ Application initialized successfully");
    }
}

#[cfg(any(reinhardt_unavailable, reinhardt_version_mismatch))]
mod tests_without_reinhardt {
    use super::*;

    /// Placeholder test that always passes when reinhardt is unavailable
    #[example_test(version = "*")]
    fn test_placeholder() {
        println!("⚠️  Hello World tests require reinhardt from crates.io");
        println!("   Tests will be enabled once reinhardt is published");
    }
}
