//! Compile-time tests for page! macro using trybuild
//!
//! This test suite validates that:
//! - Valid page! macro usage compiles successfully (tests/ui/page/pass/*.rs)
//! - Invalid page! macro usage fails to compile (tests/ui/page/fail/*.rs)

#[test]
fn test_page_macro_pass() {
	let t = trybuild::TestCases::new();
	t.pass("tests/ui/page/pass/*.rs");
}

#[test]
fn test_page_macro_fail() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/ui/page/fail/*.rs");
}
