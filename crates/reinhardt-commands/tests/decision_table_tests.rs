//! Decision table tests
//!
//! Tests that verify behavior based on decision tables with multiple conditions.

use reinhardt_commands::CommandContext;
use rstest::rstest;

// =============================================================================
// Skip Checks Decision Table
// =============================================================================

/// Decision table for should_skip_checks()
///
/// **Category**: Decision Table
/// **Verifies**: Skip checks behavior based on option combinations
///
/// | skip_checks | skip-checks | Result |
/// |-------------|-------------|--------|
/// | false       | false       | false  |
/// | true        | false       | true   |
/// | false       | true        | true   |
/// | true        | true        | true   |
#[rstest]
#[case(false, false, false, "no options set")]
#[case(true, false, true, "skip_checks only")]
#[case(false, true, true, "skip-checks only")]
#[case(true, true, true, "both options set")]
fn test_should_skip_checks_decision_table(
	#[case] skip_underscore: bool,
	#[case] skip_hyphen: bool,
	#[case] expected: bool,
	#[case] _description: &str,
) {
	let mut ctx = CommandContext::new(vec![]);

	if skip_underscore {
		ctx.set_option("skip_checks".to_string(), "true".to_string());
	}
	if skip_hyphen {
		ctx.set_option("skip-checks".to_string(), "true".to_string());
	}

	assert_eq!(
		ctx.should_skip_checks(),
		expected,
		"skip_underscore={}, skip_hyphen={} should return {}",
		skip_underscore,
		skip_hyphen,
		expected
	);
}

// =============================================================================
// Verbosity and Output Decision Table
// =============================================================================

/// Decision table for verbosity level classification
///
/// **Category**: Decision Table
/// **Verifies**: Verbosity levels map to correct output behavior
///
/// | Level   | Class   | Show Errors | Show Info | Show Debug |
/// |---------|---------|-------------|-----------|------------|
/// | 0       | quiet   | true        | false     | false      |
/// | 1       | normal  | true        | true      | false      |
/// | 2       | normal  | true        | true      | false      |
/// | 3+      | verbose | true        | true      | true       |
#[rstest]
#[case(0, true, false, false, "quiet")]
#[case(1, true, true, false, "normal")]
#[case(2, true, true, false, "normal")]
#[case(3, true, true, true, "verbose")]
#[case(10, true, true, true, "very verbose")]
#[case(255, true, true, true, "maximum")]
fn test_verbosity_output_decision_table(
	#[case] level: u8,
	#[case] show_errors: bool,
	#[case] show_info: bool,
	#[case] show_debug: bool,
	#[case] _class: &str,
) {
	let mut ctx = CommandContext::new(vec![]);
	ctx.set_verbosity(level);

	// Simulate output behavior decisions
	// Errors are always shown (any verbosity level)
	let actual_show_errors = true;
	let actual_show_info = ctx.verbosity() >= 1;
	let actual_show_debug = ctx.verbosity() >= 3;

	assert_eq!(
		actual_show_errors, show_errors,
		"Level {} show_errors",
		level
	);
	assert_eq!(actual_show_info, show_info, "Level {} show_info", level);
	assert_eq!(actual_show_debug, show_debug, "Level {} show_debug", level);
}

// =============================================================================
// Argument and Option Presence Decision Table
// =============================================================================

/// Decision table for argument/option presence scenarios
///
/// **Category**: Decision Table
/// **Verifies**: Different combinations of args and options
///
/// | Has Args | Has Options | Verbosity | Scenario        |
/// |----------|-------------|-----------|-----------------|
/// | false    | false       | 0         | minimal context |
/// | true     | false       | 0         | args only       |
/// | false    | true        | 0         | options only    |
/// | true     | true        | 0         | both present    |
/// | false    | false       | 3         | verbose minimal |
/// | true     | true        | 3         | full context    |
#[rstest]
#[case(false, false, 0, "minimal context")]
#[case(true, false, 0, "args only")]
#[case(false, true, 0, "options only")]
#[case(true, true, 0, "both present")]
#[case(false, false, 3, "verbose minimal")]
#[case(true, true, 3, "full context")]
fn test_context_presence_decision_table(
	#[case] has_args: bool,
	#[case] has_options: bool,
	#[case] verbosity: u8,
	#[case] _scenario: &str,
) {
	let args = if has_args {
		vec!["arg1".to_string(), "arg2".to_string()]
	} else {
		vec![]
	};

	let mut ctx = CommandContext::new(args);
	ctx.set_verbosity(verbosity);

	if has_options {
		ctx.set_option("opt1".to_string(), "val1".to_string());
		ctx.set_option("opt2".to_string(), "val2".to_string());
	}

	// Verify state
	assert_eq!(!ctx.args.is_empty(), has_args);
	assert_eq!(!ctx.options.is_empty(), has_options);
	assert_eq!(ctx.verbosity(), verbosity);

	// Verify specific counts when present
	if has_args {
		assert_eq!(ctx.args.len(), 2);
	}
	if has_options {
		assert_eq!(ctx.options.len(), 2);
	}
}

// =============================================================================
// Option Access Methods Decision Table
// =============================================================================

/// Decision table for option access method behavior
///
/// **Category**: Decision Table
/// **Verifies**: Different option access methods return correct values
///
/// | Option Set | has_option | option()   | option_values()  |
/// |------------|------------|------------|------------------|
/// | not set    | false      | None       | None             |
/// | empty vec  | true       | None       | Some([])         |
/// | single val | true       | Some(val)  | Some([val])      |
/// | multi vals | true       | Some(first)| Some([all])      |
#[rstest]
#[case("not_set", false, false, None, None, "not set")]
#[case("empty", true, true, None, Some(0), "empty values")]
#[case("single", true, true, Some("val1"), Some(1), "single value")]
#[case("multi", true, true, Some("val1"), Some(3), "multiple values")]
fn test_option_access_decision_table(
	#[case] setup: &str,
	#[case] set_option: bool,
	#[case] expected_has: bool,
	#[case] expected_option: Option<&str>,
	#[case] expected_values_len: Option<usize>,
	#[case] _description: &str,
) {
	let mut ctx = CommandContext::new(vec![]);
	let key = "test_key";

	if set_option {
		let values = match setup {
			"empty" => vec![],
			"single" => vec!["val1".to_string()],
			"multi" => vec!["val1".to_string(), "val2".to_string(), "val3".to_string()],
			_ => vec![],
		};
		ctx.set_option_multi(key.to_string(), values);
	}

	// Test has_option
	assert_eq!(
		ctx.has_option(key),
		expected_has,
		"has_option mismatch for {}",
		setup
	);

	// Test option() - returns first value or None
	let actual_option = ctx.option(key).map(|s| s.as_str());
	assert_eq!(
		actual_option, expected_option,
		"option() mismatch for {}",
		setup
	);

	// Test option_values()
	let actual_values = ctx.option_values(key);
	match expected_values_len {
		Some(len) => {
			assert!(
				actual_values.is_some(),
				"option_values should be Some for {}",
				setup
			);
			assert_eq!(
				actual_values.unwrap().len(),
				len,
				"option_values len mismatch for {}",
				setup
			);
		}
		None => {
			assert!(
				actual_values.is_none(),
				"option_values should be None for {}",
				setup
			);
		}
	}
}

// =============================================================================
// Argument Index Access Decision Table
// =============================================================================

/// Decision table for argument index access
///
/// **Category**: Decision Table
/// **Verifies**: arg() returns correct values for different indices
///
/// | Args Count | Index | Result   |
/// |------------|-------|----------|
/// | 0          | 0     | None     |
/// | 0          | 1     | None     |
/// | 1          | 0     | Some     |
/// | 1          | 1     | None     |
/// | 3          | 0     | Some     |
/// | 3          | 2     | Some     |
/// | 3          | 3     | None     |
#[rstest]
#[case(0, 0, false, "empty at 0")]
#[case(0, 1, false, "empty at 1")]
#[case(1, 0, true, "single at 0")]
#[case(1, 1, false, "single at 1")]
#[case(3, 0, true, "triple at 0")]
#[case(3, 2, true, "triple at 2")]
#[case(3, 3, false, "triple at 3")]
#[case(3, 100, false, "triple at 100")]
fn test_arg_index_decision_table(
	#[case] args_count: usize,
	#[case] index: usize,
	#[case] should_exist: bool,
	#[case] _description: &str,
) {
	let args: Vec<String> = (0..args_count).map(|i| format!("arg{}", i)).collect();
	let ctx = CommandContext::new(args);

	let result = ctx.arg(index);

	if should_exist {
		assert!(
			result.is_some(),
			"Index {} should exist for {} args",
			index,
			args_count
		);
		assert_eq!(result.unwrap(), &format!("arg{}", index));
	} else {
		assert!(
			result.is_none(),
			"Index {} should not exist for {} args",
			index,
			args_count
		);
	}
}

// =============================================================================
// Option Override Decision Table
// =============================================================================

/// Decision table for option override behavior
///
/// **Category**: Decision Table
/// **Verifies**: set_option correctly overrides previous values
///
/// | First Set | Second Set | Final Value |
/// |-----------|------------|-------------|
/// | "a"       | -          | "a"         |
/// | "a"       | "b"        | "b"         |
/// | "a"       | ""         | ""          |
/// | ""        | "b"        | "b"         |
#[rstest]
#[case(Some("a"), None, "a", "single set")]
#[case(Some("a"), Some("b"), "b", "override")]
#[case(Some("a"), Some(""), "", "override with empty")]
#[case(Some(""), Some("b"), "b", "empty then value")]
fn test_option_override_decision_table(
	#[case] first: Option<&str>,
	#[case] second: Option<&str>,
	#[case] expected: &str,
	#[case] _description: &str,
) {
	let mut ctx = CommandContext::new(vec![]);
	let key = "key";

	if let Some(v) = first {
		ctx.set_option(key.to_string(), v.to_string());
	}
	if let Some(v) = second {
		ctx.set_option(key.to_string(), v.to_string());
	}

	let result = ctx.option(key).map(|s| s.as_str()).unwrap_or("");
	assert_eq!(result, expected);
}

// =============================================================================
// Builder Method Chain Decision Table
// =============================================================================

/// Decision table for builder method chaining
///
/// **Category**: Decision Table
/// **Verifies**: Builder methods combine correctly
///
/// | with_args | with_options | Final Args | Final Options |
/// |-----------|--------------|------------|---------------|
/// | no        | no           | initial    | empty         |
/// | yes       | no           | replaced   | empty         |
/// | no        | yes          | initial    | set           |
/// | yes       | yes          | replaced   | set           |
#[rstest]
#[case(false, false, 2, 0, "no modifications")]
#[case(true, false, 3, 0, "args replaced")]
#[case(false, true, 2, 2, "options added")]
#[case(true, true, 3, 2, "both modified")]
fn test_builder_chain_decision_table(
	#[case] use_with_args: bool,
	#[case] use_with_options: bool,
	#[case] expected_args_len: usize,
	#[case] expected_options_len: usize,
	#[case] _description: &str,
) {
	use std::collections::HashMap;

	let initial = vec!["init1".to_string(), "init2".to_string()];
	let mut ctx = CommandContext::new(initial);

	if use_with_args {
		ctx = ctx.with_args(vec![
			"new1".to_string(),
			"new2".to_string(),
			"new3".to_string(),
		]);
	}

	if use_with_options {
		let mut options = HashMap::new();
		options.insert("opt1".to_string(), vec!["v1".to_string()]);
		options.insert("opt2".to_string(), vec!["v2".to_string()]);
		ctx = ctx.with_options(options);
	}

	assert_eq!(ctx.args.len(), expected_args_len);
	assert_eq!(ctx.options.len(), expected_options_len);
}

// =============================================================================
// add_arg Operation Decision Table
// =============================================================================

/// Decision table for add_arg operations
///
/// **Category**: Decision Table
/// **Verifies**: add_arg accumulates correctly
///
/// | Initial | Add Count | Final Count |
/// |---------|-----------|-------------|
/// | 0       | 0         | 0           |
/// | 0       | 1         | 1           |
/// | 0       | 5         | 5           |
/// | 3       | 0         | 3           |
/// | 3       | 2         | 5           |
/// | 3       | 10        | 13          |
#[rstest]
#[case(0, 0, 0, "empty + none")]
#[case(0, 1, 1, "empty + one")]
#[case(0, 5, 5, "empty + five")]
#[case(3, 0, 3, "three + none")]
#[case(3, 2, 5, "three + two")]
#[case(3, 10, 13, "three + ten")]
fn test_add_arg_decision_table(
	#[case] initial_count: usize,
	#[case] add_count: usize,
	#[case] expected_total: usize,
	#[case] _description: &str,
) {
	let initial: Vec<String> = (0..initial_count).map(|i| format!("init{}", i)).collect();
	let mut ctx = CommandContext::new(initial);

	for i in 0..add_count {
		ctx.add_arg(format!("added{}", i));
	}

	assert_eq!(ctx.args.len(), expected_total);

	// Verify initial args are still at the beginning
	for i in 0..initial_count {
		assert_eq!(ctx.arg(i), Some(&format!("init{}", i)));
	}

	// Verify added args are at the end
	for i in 0..add_count {
		assert_eq!(ctx.arg(initial_count + i), Some(&format!("added{}", i)));
	}
}

// =============================================================================
// Multi-value Option Decision Table
// =============================================================================

/// Decision table for multi-value option behavior
///
/// **Category**: Decision Table
/// **Verifies**: set_option_multi handles various value counts
///
/// | Values Count | has_option | option()  | option_values.len() |
/// |--------------|------------|-----------|---------------------|
/// | 0            | true       | None      | 0                   |
/// | 1            | true       | Some(v0)  | 1                   |
/// | 3            | true       | Some(v0)  | 3                   |
/// | 10           | true       | Some(v0)  | 10                  |
#[rstest]
#[case(0, true, None, 0, "empty values")]
#[case(1, true, Some("val0"), 1, "single value")]
#[case(3, true, Some("val0"), 3, "three values")]
#[case(10, true, Some("val0"), 10, "ten values")]
fn test_multi_value_option_decision_table(
	#[case] values_count: usize,
	#[case] expected_has: bool,
	#[case] expected_first: Option<&str>,
	#[case] expected_len: usize,
	#[case] _description: &str,
) {
	let mut ctx = CommandContext::new(vec![]);
	let key = "multi_key";
	let values: Vec<String> = (0..values_count).map(|i| format!("val{}", i)).collect();

	ctx.set_option_multi(key.to_string(), values);

	assert_eq!(ctx.has_option(key), expected_has);
	assert_eq!(ctx.option(key).map(|s| s.as_str()), expected_first);

	let actual_values = ctx.option_values(key).unwrap();
	assert_eq!(actual_values.len(), expected_len);
}
