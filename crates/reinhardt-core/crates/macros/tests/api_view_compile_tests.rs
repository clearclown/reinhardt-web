//! Compile-time tests for api_view macro using trybuild

#[test]
fn test_api_view_macro_pass() {
	let t = trybuild::TestCases::new();
	t.pass("tests/ui/api_view/pass/*.rs");
}

#[test]
fn test_api_view_macro_fail() {
	let t = trybuild::TestCases::new();
	t.compile_fail("tests/ui/api_view/fail/*.rs");
}
