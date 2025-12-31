//! Compile-time tests for the `form!` macro using trybuild.
//!
//! This test suite verifies that the `form!` macro:
//! - Compiles successfully with valid input (pass tests)
//! - Produces correct error messages for invalid input (fail tests)
//!
//! ## Running Tests
//!
//! ```bash
//! cargo test -p reinhardt-forms-macros
//! ```
//!
//! ## Updating Expected Errors
//!
//! When error messages change, update `.stderr` files with:
//!
//! ```bash
//! TRYBUILD=overwrite cargo test -p reinhardt-forms-macros
//! ```

/// Runs all compile-time tests for the `form!` macro.
///
/// This test function uses trybuild to verify that:
/// - Valid form definitions compile successfully (tests/ui/pass/*.rs)
/// - Invalid form definitions produce expected error messages (tests/ui/fail/*.rs)
#[test]
fn form_macro_compile_tests() {
	let t = trybuild::TestCases::new();

	// Pass tests: these should compile without errors
	t.pass("tests/ui/pass/*.rs");

	// Fail tests: these should produce specific compile errors
	t.compile_fail("tests/ui/fail/*.rs");
}
