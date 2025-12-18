//! Macro expansion tests for #[server_fn] attribute (Week 4 Day 1-2)
//!
//! These tests verify that the #[server_fn] macro expands correctly,
//! particularly for DI parameter detection (use_inject = true).
//!
//! Test Strategy:
//! - Compile-time verification using trybuild
//! - Tests in tests/ui/server_fn/ directory
//! - Pass: Files should compile successfully
//! - Fail: Files should produce expected compilation errors

#[test]
fn test_server_fn_macro_ui() {
	let t = trybuild::TestCases::new();

	// Week 4 Day 1-2: DI Parameter Detection tests
	// These should compile successfully
	t.pass("tests/ui/server_fn/with_inject.rs");

	// Week 4 Day 3-4: Codec tests
	t.pass("tests/ui/server_fn/codec_json.rs");
	t.pass("tests/ui/server_fn/codec_url.rs");

	// Future tests:
	// t.compile_fail("tests/ui/server_fn/invalid_inject.rs");
	// t.compile_fail("tests/ui/server_fn/invalid_codec.rs");
}
