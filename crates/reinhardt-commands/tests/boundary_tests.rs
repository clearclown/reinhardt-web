//! Boundary value analysis tests
//!
//! Tests that verify behavior at boundary conditions.

use reinhardt_commands::{CommandContext, generate_secret_key};
use rstest::rstest;

// =============================================================================
// Argument Index Boundary Tests
// =============================================================================

/// Test argument index boundaries
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Edge indices behave correctly
///
/// Boundary points for 3-element array:
/// - -1 (invalid, not applicable for usize)
/// - 0 (first valid)
/// - 2 (last valid = len-1)
/// - 3 (first invalid = len)
/// - 4+ (well beyond)
#[rstest]
#[case(0, Some("first"))] // first valid index
#[case(1, Some("second"))] // middle
#[case(2, Some("third"))] // last valid (len - 1)
#[case(3, None)] // first invalid (len)
#[case(4, None)] // beyond bounds
#[case(100, None)] // well beyond
#[case(usize::MAX, None)] // maximum usize
fn test_args_index_boundaries(#[case] index: usize, #[case] expected: Option<&str>) {
	let ctx = CommandContext::new(vec![
		"first".to_string(),
		"second".to_string(),
		"third".to_string(),
	]);

	assert_eq!(
		ctx.arg(index).map(|s| s.as_str()),
		expected,
		"Index {} should return {:?}",
		index,
		expected
	);
}

/// Test empty args index boundaries
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: All indices are invalid for empty args
#[rstest]
#[case(0)]
#[case(1)]
#[case(100)]
fn test_empty_args_index_boundaries(#[case] index: usize) {
	let ctx = CommandContext::new(vec![]);

	assert_eq!(
		ctx.arg(index),
		None,
		"Index {} should return None for empty args",
		index
	);
}

/// Test single-element args boundaries
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Only index 0 is valid for single-element args
#[rstest]
#[case(0, Some("only"))]
#[case(1, None)]
fn test_single_args_boundaries(#[case] index: usize, #[case] expected: Option<&str>) {
	let ctx = CommandContext::new(vec!["only".to_string()]);

	assert_eq!(ctx.arg(index).map(|s| s.as_str()), expected);
}

// =============================================================================
// Verbosity Boundary Tests
// =============================================================================

/// Test verbosity value boundaries
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: u8 boundary values work correctly
///
/// Boundary points for u8:
/// - 0 (minimum)
/// - 1 (minimum + 1)
/// - 127 (middle - 1)
/// - 128 (middle)
/// - 254 (maximum - 1)
/// - 255 (maximum)
#[rstest]
#[case(0)] // minimum u8
#[case(1)] // minimum + 1
#[case(127)] // middle - 1
#[case(128)] // middle
#[case(254)] // maximum - 1
#[case(255)] // maximum u8
fn test_verbosity_boundaries(#[case] level: u8) {
	let mut ctx = CommandContext::new(vec![]);
	ctx.set_verbosity(level);

	assert_eq!(
		ctx.verbosity(),
		level,
		"Verbosity {} should be stored correctly",
		level
	);
}

// =============================================================================
// Option Values Count Boundary Tests
// =============================================================================

/// Test option values count boundaries
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Different value counts are handled correctly
#[rstest]
#[case(0)] // empty
#[case(1)] // single
#[case(2)] // double
#[case(10)] // moderate
#[case(100)] // large
fn test_option_values_count_boundaries(#[case] count: usize) {
	let mut ctx = CommandContext::new(vec![]);
	let values: Vec<String> = (0..count).map(|i| format!("v{}", i)).collect();
	ctx.set_option_multi("key".to_string(), values.clone());

	let result = ctx.option_values("key").unwrap();
	assert_eq!(result.len(), count);

	if count > 0 {
		assert_eq!(ctx.option("key"), Some(&values[0]));
	}
}

// =============================================================================
// String Length Boundary Tests
// =============================================================================

/// Test argument string length boundaries
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Different string lengths are handled correctly
#[rstest]
#[case(0)] // empty
#[case(1)] // single char
#[case(10)] // short
#[case(100)] // medium
#[case(1000)] // long
#[case(10000)] // very long
fn test_arg_string_length_boundaries(#[case] length: usize) {
	let value = "x".repeat(length);
	let ctx = CommandContext::new(vec![value.clone()]);

	let result = ctx.arg(0).unwrap();
	assert_eq!(result.len(), length);
	assert_eq!(result, &value);
}

/// Test option key length boundaries
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Different key lengths work correctly
#[rstest]
#[case(1)] // single char
#[case(10)] // short
#[case(100)] // medium
fn test_option_key_length_boundaries(#[case] length: usize) {
	let key = "k".repeat(length);
	let mut ctx = CommandContext::new(vec![]);
	ctx.set_option(key.clone(), "value".to_string());

	assert!(ctx.has_option(&key));
	assert_eq!(ctx.option(&key), Some(&"value".to_string()));
}

// =============================================================================
// Arguments Count Boundary Tests
// =============================================================================

/// Test arguments count boundaries
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Different argument counts are handled correctly
#[rstest]
#[case(0)] // empty
#[case(1)] // single
#[case(10)] // moderate
#[case(100)] // large
#[case(1000)] // very large
fn test_args_count_boundaries(#[case] count: usize) {
	let args: Vec<String> = (0..count).map(|i| format!("arg{}", i)).collect();
	let ctx = CommandContext::new(args.clone());

	assert_eq!(ctx.args.len(), count);

	// First and last should be accessible
	if count > 0 {
		assert_eq!(ctx.arg(0), Some(&args[0]));
		assert_eq!(ctx.arg(count - 1), Some(&args[count - 1]));
	}
	assert_eq!(ctx.arg(count), None);
}

// =============================================================================
// Secret Key Length Boundary Tests
// =============================================================================

/// Test secret key is exactly 50 characters
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Key length is exactly 50
#[rstest]
fn test_secret_key_length_boundary() {
	let key = generate_secret_key();

	assert_eq!(key.len(), 50, "Key must be exactly 50 characters");
	assert!(key.len() >= 50, "Key must be at least 50 characters");
	assert!(key.len() <= 50, "Key must be at most 50 characters");
}

// =============================================================================
// Options Count Boundary Tests
// =============================================================================

/// Test options count boundaries
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Different option counts are handled correctly
#[rstest]
#[case(0)] // none
#[case(1)] // single
#[case(10)] // moderate
#[case(100)] // large
fn test_options_count_boundaries(#[case] count: usize) {
	let mut ctx = CommandContext::new(vec![]);

	for i in 0..count {
		ctx.set_option(format!("key{}", i), format!("val{}", i));
	}

	for i in 0..count {
		assert!(ctx.has_option(&format!("key{}", i)));
	}
	assert!(!ctx.has_option(&format!("key{}", count)));
}

// =============================================================================
// Add/Set Operation Count Boundary Tests
// =============================================================================

/// Test many add_arg operations
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Many add_arg calls accumulate correctly
#[rstest]
#[case(0)]
#[case(1)]
#[case(100)]
#[case(500)]
fn test_add_arg_count_boundaries(#[case] count: usize) {
	let mut ctx = CommandContext::new(vec![]);

	for i in 0..count {
		ctx.add_arg(format!("arg{}", i));
	}

	assert_eq!(ctx.args.len(), count);
}

/// Test many set_option operations on same key
///
/// **Category**: Boundary Value Analysis
/// **Verifies**: Many overwrites preserve last value
#[rstest]
#[case(1)]
#[case(10)]
#[case(100)]
fn test_set_option_overwrite_count_boundaries(#[case] count: usize) {
	let mut ctx = CommandContext::new(vec![]);

	for i in 0..count {
		ctx.set_option("key".to_string(), format!("val{}", i));
	}

	// Should have last value
	let expected = format!("val{}", count - 1);
	assert_eq!(ctx.option("key"), Some(&expected));
}
