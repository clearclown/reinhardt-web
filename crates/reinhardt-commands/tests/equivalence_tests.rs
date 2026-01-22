//! Equivalence partitioning tests
//!
//! Tests that verify behavior for different equivalence classes.

use reinhardt_commands::{CommandContext, CommandError};
use rstest::rstest;

// =============================================================================
// Verbosity Level Equivalence Classes
// =============================================================================

/// Test verbosity level equivalence classes
///
/// **Category**: Equivalence Partitioning
/// **Verifies**: Different verbosity levels behave correctly
///
/// | Level | Class    | Description |
/// |-------|----------|-------------|
/// | 0     | quiet    | No output   |
/// | 1-2   | normal   | Normal output |
/// | 3+    | verbose  | Detailed output |
#[rstest]
#[case(0, "quiet")]
#[case(1, "normal")]
#[case(2, "normal")]
#[case(3, "verbose")]
#[case(10, "verbose")]
#[case(100, "verbose")]
#[case(255, "verbose")]
fn test_verbosity_equivalence_classes(#[case] level: u8, #[case] expected_class: &str) {
	let mut ctx = CommandContext::new(vec![]);
	ctx.set_verbosity(level);

	let actual_class = match ctx.verbosity() {
		0 => "quiet",
		1..=2 => "normal",
		_ => "verbose",
	};

	assert_eq!(
		actual_class, expected_class,
		"Verbosity {} should be in class '{}'",
		level, expected_class
	);
}

// =============================================================================
// Argument Count Equivalence Classes
// =============================================================================

/// Test argument count equivalence classes
///
/// **Category**: Equivalence Partitioning
/// **Verifies**: Different argument counts are handled correctly
///
/// | Count | Class    | Description |
/// |-------|----------|-------------|
/// | 0     | empty    | No args     |
/// | 1     | single   | One arg     |
/// | 2+    | multiple | Many args   |
#[rstest]
#[case(0, "empty")]
#[case(1, "single")]
#[case(2, "multiple")]
#[case(5, "multiple")]
#[case(100, "multiple")]
fn test_args_count_equivalence_classes(#[case] count: usize, #[case] expected_class: &str) {
	let args: Vec<String> = (0..count).map(|i| format!("arg{}", i)).collect();
	let ctx = CommandContext::new(args);

	let actual_class = match ctx.args.len() {
		0 => "empty",
		1 => "single",
		_ => "multiple",
	};

	assert_eq!(actual_class, expected_class);
}

// =============================================================================
// Option State Equivalence Classes
// =============================================================================

/// Test option state equivalence classes
///
/// **Category**: Equivalence Partitioning
/// **Verifies**: Option presence states are distinguished
///
/// | State   | Class   |
/// |---------|---------|
/// | Missing | absent  |
/// | Set     | present |
#[rstest]
#[case(false, "absent")]
#[case(true, "present")]
fn test_option_state_equivalence_classes(#[case] set_option: bool, #[case] expected_class: &str) {
	let mut ctx = CommandContext::new(vec![]);

	if set_option {
		ctx.set_option("test".to_string(), "value".to_string());
	}

	let actual_class = if ctx.has_option("test") {
		"present"
	} else {
		"absent"
	};

	assert_eq!(actual_class, expected_class);
}

// =============================================================================
// Option Value Count Equivalence Classes
// =============================================================================

/// Test option value count equivalence classes
///
/// **Category**: Equivalence Partitioning
/// **Verifies**: Different value counts are handled correctly
///
/// | Count | Class    |
/// |-------|----------|
/// | 0     | empty    |
/// | 1     | single   |
/// | 2+    | multiple |
#[rstest]
#[case(0, "empty")]
#[case(1, "single")]
#[case(2, "multiple")]
#[case(10, "multiple")]
fn test_option_value_count_equivalence_classes(#[case] count: usize, #[case] expected_class: &str) {
	let mut ctx = CommandContext::new(vec![]);
	let values: Vec<String> = (0..count).map(|i| format!("v{}", i)).collect();
	ctx.set_option_multi("key".to_string(), values);

	let actual_class = match ctx.option_values("key").map(|v| v.len()).unwrap_or(0) {
		0 => "empty",
		1 => "single",
		_ => "multiple",
	};

	assert_eq!(actual_class, expected_class);
}

// =============================================================================
// Error Type Equivalence Classes
// =============================================================================

/// Test error type equivalence classes
///
/// **Category**: Equivalence Partitioning
/// **Verifies**: Each error type has distinct display behavior
#[rstest]
#[case(CommandError::NotFound("x".to_string()), "not_found", "Command not found")]
#[case(CommandError::InvalidArguments("x".to_string()), "invalid_args", "Invalid arguments")]
#[case(CommandError::ExecutionError("x".to_string()), "execution", "Execution error")]
#[case(CommandError::ParseError("x".to_string()), "parse", "Parse error")]
#[case(CommandError::TemplateError("x".to_string()), "template", "Template error")]
fn test_error_type_equivalence_classes(
	#[case] error: CommandError,
	#[case] _class: &str,
	#[case] expected_prefix: &str,
) {
	let display = format!("{}", error);
	assert!(
		display.contains(expected_prefix),
		"Error display '{}' should contain '{}'",
		display,
		expected_prefix
	);
}

// =============================================================================
// Skip Checks State Equivalence Classes
// =============================================================================

/// Test skip_checks state equivalence classes
///
/// **Category**: Equivalence Partitioning
/// **Verifies**: Skip checks detection works correctly
///
/// | skip_checks | skip-checks | Result |
/// |-------------|-------------|--------|
/// | No          | No          | false  |
/// | Yes         | *           | true   |
/// | *           | Yes         | true   |
#[rstest]
#[case(false, false, "not_skipped")]
#[case(true, false, "skipped")]
#[case(false, true, "skipped")]
#[case(true, true, "skipped")]
fn test_skip_checks_equivalence_classes(
	#[case] skip_underscore: bool,
	#[case] skip_hyphen: bool,
	#[case] expected_class: &str,
) {
	let mut ctx = CommandContext::new(vec![]);

	if skip_underscore {
		ctx.set_option("skip_checks".to_string(), "true".to_string());
	}
	if skip_hyphen {
		ctx.set_option("skip-checks".to_string(), "true".to_string());
	}

	let actual_class = if ctx.should_skip_checks() {
		"skipped"
	} else {
		"not_skipped"
	};

	assert_eq!(actual_class, expected_class);
}

// =============================================================================
// String Content Equivalence Classes
// =============================================================================

/// Test string content equivalence classes for arguments
///
/// **Category**: Equivalence Partitioning
/// **Verifies**: Different string types are handled correctly
#[rstest]
#[case("", "empty")]
#[case("a", "single_char")]
#[case("hello", "ascii")]
#[case("hello world", "with_space")]
#[case("日本語", "unicode")]
#[case("hello\nworld", "with_newline")]
fn test_string_content_equivalence_classes(#[case] input: &str, #[case] _class: &str) {
	let ctx = CommandContext::new(vec![input.to_string()]);

	assert_eq!(ctx.arg(0), Some(&input.to_string()));
}
