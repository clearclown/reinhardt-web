//! Combination tests
//!
//! Tests that verify behavior with multiple parameter combinations.

use reinhardt_commands::{CommandArgument, CommandContext, CommandOption};
use rstest::rstest;

// =============================================================================
// CommandContext Combination Tests
// =============================================================================

/// Test CommandContext with various argument and option combinations
///
/// **Category**: Combination
/// **Verifies**: Context works with different arg/option configurations
#[rstest]
#[case(vec![], 0, false, "empty args, no verbosity, no options")]
#[case(vec!["arg1".to_string()], 0, false, "single arg, no verbosity")]
#[case(vec!["arg1".to_string(), "arg2".to_string()], 0, false, "multiple args")]
#[case(vec![], 1, false, "empty args with verbosity")]
#[case(vec![], 0, true, "empty args with options")]
#[case(vec!["cmd".to_string()], 2, true, "full combination")]
fn test_context_combinations(
	#[case] args: Vec<String>,
	#[case] verbosity: u8,
	#[case] has_options: bool,
	#[case] _description: &str,
) {
	let mut ctx = CommandContext::new(args.clone());
	ctx.set_verbosity(verbosity);

	if has_options {
		ctx.set_option("verbose".to_string(), "true".to_string());
		ctx.set_option("format".to_string(), "json".to_string());
	}

	// Verify args
	assert_eq!(ctx.args.len(), args.len());
	for (i, arg) in args.iter().enumerate() {
		assert_eq!(ctx.arg(i), Some(arg));
	}

	// Verify verbosity
	assert_eq!(ctx.verbosity(), verbosity);

	// Verify options
	if has_options {
		assert!(ctx.has_option("verbose"));
		assert!(ctx.has_option("format"));
	} else {
		assert!(!ctx.has_option("verbose"));
	}
}

/// Test option value types combinations
///
/// **Category**: Combination
/// **Verifies**: Different value types for options
#[rstest]
#[case("flag_only", vec!["true".to_string()])]
#[case("single_value", vec!["value".to_string()])]
#[case("empty_value", vec!["".to_string()])]
#[case("multi_values", vec!["v1".to_string(), "v2".to_string(), "v3".to_string()])]
fn test_option_value_combinations(#[case] key: &str, #[case] values: Vec<String>) {
	let mut ctx = CommandContext::new(vec![]);
	ctx.set_option_multi(key.to_string(), values.clone());

	assert!(ctx.has_option(key));
	assert_eq!(ctx.option_values(key), Some(values.clone()));
	assert_eq!(ctx.option(key), values.first());
}

// =============================================================================
// CommandArgument Combination Tests
// =============================================================================

/// Test all CommandArgument builder combinations
///
/// **Category**: Combination
/// **Verifies**: All argument modifier combinations work
#[rstest]
#[case(true, None, "required_no_default")]
#[case(true, Some("default"), "required_with_default")]
#[case(false, None, "optional_no_default")]
#[case(false, Some("default"), "optional_with_default")]
fn test_argument_builder_combinations(
	#[case] required: bool,
	#[case] default: Option<&str>,
	#[case] description: &str,
) {
	let mut arg = if required {
		CommandArgument::required("test", description)
	} else {
		CommandArgument::optional("test", description)
	};

	if let Some(def) = default {
		arg = arg.with_default(def);
	}

	assert_eq!(arg.required, required);
	assert_eq!(arg.default, default.map(|s| s.to_string()));
	assert_eq!(arg.name, "test");
}

// =============================================================================
// CommandOption Combination Tests
// =============================================================================

/// Test all CommandOption builder combinations
///
/// **Category**: Combination
/// **Verifies**: All option modifier combinations work
#[rstest]
#[case(false, false, None, false, "flag_basic")]
#[case(false, true, None, false, "flag_required")]
#[case(true, false, None, false, "value_basic")]
#[case(true, true, None, false, "value_required")]
#[case(true, false, Some("def"), false, "value_default")]
#[case(true, false, None, true, "value_multi")]
#[case(true, true, Some("def"), false, "value_required_default")]
#[case(true, true, Some("def"), true, "value_all_modifiers")]
fn test_option_builder_combinations(
	#[case] takes_value: bool,
	#[case] required: bool,
	#[case] default: Option<&str>,
	#[case] multiple: bool,
	#[case] _description: &str,
) {
	let mut opt = if takes_value {
		CommandOption::option(Some('t'), "test", "Test option")
	} else {
		CommandOption::flag(Some('t'), "test", "Test flag")
	};

	if required {
		opt = opt.required();
	}
	if let Some(def) = default {
		opt = opt.with_default(def);
	}
	if multiple {
		opt = opt.multi();
	}

	assert_eq!(opt.takes_value, takes_value);
	assert_eq!(opt.required, required);
	assert_eq!(opt.default, default.map(|s| s.to_string()));
	assert_eq!(opt.multiple, multiple);
}

/// Test short flag presence combinations
///
/// **Category**: Combination
/// **Verifies**: Options work with and without short flags
#[rstest]
#[case(Some('v'), "verbose", "with_short")]
#[case(None, "verbose", "without_short")]
#[case(Some('x'), "x-option", "short_with_hyphen_long")]
fn test_option_short_flag_combinations(
	#[case] short: Option<char>,
	#[case] long: &str,
	#[case] _description: &str,
) {
	let opt = CommandOption::flag(short, long, "Description");

	assert_eq!(opt.short, short);
	assert_eq!(opt.long, long);
}

// =============================================================================
// Context Builder Chain Combinations
// =============================================================================

/// Test builder method order combinations
///
/// **Category**: Combination
/// **Verifies**: Builder methods work in any order
#[rstest]
fn test_context_builder_order_1() {
	use std::collections::HashMap;

	let mut options = HashMap::new();
	options.insert("key".to_string(), vec!["val".to_string()]);

	let ctx = CommandContext::new(vec![])
		.with_args(vec!["a".to_string()])
		.with_options(options);

	assert_eq!(ctx.arg(0), Some(&"a".to_string()));
	assert!(ctx.has_option("key"));
}

#[rstest]
fn test_context_builder_order_2() {
	use std::collections::HashMap;

	let mut options = HashMap::new();
	options.insert("key".to_string(), vec!["val".to_string()]);

	let ctx = CommandContext::new(vec!["initial".to_string()])
		.with_options(options)
		.with_args(vec!["replaced".to_string()]);

	// with_args replaces args
	assert_eq!(ctx.arg(0), Some(&"replaced".to_string()));
	assert!(ctx.has_option("key"));
}

// =============================================================================
// Multiple Operations Combination Tests
// =============================================================================

/// Test multiple add_arg operations
///
/// **Category**: Combination
/// **Verifies**: Sequential add_arg calls accumulate
#[rstest]
#[case(0, 0)]
#[case(0, 5)]
#[case(3, 0)]
#[case(3, 7)]
fn test_add_arg_combinations(#[case] initial_count: usize, #[case] add_count: usize) {
	let initial: Vec<String> = (0..initial_count).map(|i| format!("init{}", i)).collect();
	let mut ctx = CommandContext::new(initial);

	for i in 0..add_count {
		ctx.add_arg(format!("added{}", i));
	}

	assert_eq!(ctx.args.len(), initial_count + add_count);
}

/// Test multiple set_option operations
///
/// **Category**: Combination
/// **Verifies**: Multiple set_option calls work correctly
#[rstest]
fn test_set_option_multiple_keys() {
	let mut ctx = CommandContext::new(vec![]);

	ctx.set_option("a".to_string(), "1".to_string());
	ctx.set_option("b".to_string(), "2".to_string());
	ctx.set_option("c".to_string(), "3".to_string());

	assert_eq!(ctx.option("a"), Some(&"1".to_string()));
	assert_eq!(ctx.option("b"), Some(&"2".to_string()));
	assert_eq!(ctx.option("c"), Some(&"3".to_string()));
}

/// Test overwriting options multiple times
///
/// **Category**: Combination
/// **Verifies**: Multiple overwrites preserve last value
#[rstest]
fn test_set_option_multiple_overwrites() {
	let mut ctx = CommandContext::new(vec![]);

	ctx.set_option("key".to_string(), "first".to_string());
	ctx.set_option("key".to_string(), "second".to_string());
	ctx.set_option("key".to_string(), "third".to_string());

	assert_eq!(ctx.option("key"), Some(&"third".to_string()));
}
