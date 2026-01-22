//! CommandArgument and CommandOption builder tests
//!
//! Tests for argument and option definition builders.

use reinhardt_commands::{CommandArgument, CommandOption};
use rstest::rstest;

// =============================================================================
// CommandArgument Happy Path Tests
// =============================================================================

/// Test creating a required argument
///
/// **Category**: Happy Path
/// **Verifies**: CommandArgument::required() creates correct argument
#[rstest]
fn test_argument_required_creation() {
	let arg = CommandArgument::required("name", "The name argument");

	assert_eq!(arg.name, "name", "Name should match");
	assert_eq!(
		arg.description, "The name argument",
		"Description should match"
	);
	assert!(arg.required, "Should be marked as required");
	assert!(
		arg.default.is_none(),
		"Required argument should have no default"
	);
}

/// Test creating an optional argument
///
/// **Category**: Happy Path
/// **Verifies**: CommandArgument::optional() creates correct argument
#[rstest]
fn test_argument_optional_creation() {
	let arg = CommandArgument::optional("extra", "Optional extra argument");

	assert_eq!(arg.name, "extra", "Name should match");
	assert_eq!(
		arg.description, "Optional extra argument",
		"Description should match"
	);
	assert!(!arg.required, "Should be marked as optional");
	assert!(
		arg.default.is_none(),
		"Optional argument without with_default has no default"
	);
}

/// Test adding a default value to argument
///
/// **Category**: Happy Path
/// **Verifies**: CommandArgument::with_default() adds default value
#[rstest]
fn test_argument_with_default() {
	let arg = CommandArgument::optional("format", "Output format").with_default("json");

	assert_eq!(arg.name, "format");
	assert!(!arg.required);
	assert_eq!(
		arg.default,
		Some("json".to_string()),
		"Default value should be set"
	);
}

/// Test required argument with default (allowed but unusual)
///
/// **Category**: Happy Path
/// **Verifies**: Required argument can have a default value
#[rstest]
fn test_argument_required_with_default() {
	let arg = CommandArgument::required("mode", "Operation mode").with_default("auto");

	assert!(arg.required, "Should still be marked as required");
	assert_eq!(
		arg.default,
		Some("auto".to_string()),
		"Default should be set"
	);
}

// =============================================================================
// CommandOption Happy Path Tests
// =============================================================================

/// Test creating a flag option (boolean)
///
/// **Category**: Happy Path
/// **Verifies**: CommandOption::flag() creates boolean flag
#[rstest]
fn test_option_flag_creation() {
	let opt = CommandOption::flag(Some('v'), "verbose", "Enable verbose output");

	assert_eq!(opt.short, Some('v'), "Short flag should match");
	assert_eq!(opt.long, "verbose", "Long flag should match");
	assert_eq!(opt.description, "Enable verbose output");
	assert!(!opt.takes_value, "Flag should not take value");
	assert!(!opt.required, "Flags are not required by default");
	assert!(opt.default.is_none(), "Flags have no default");
	assert!(!opt.multiple, "Flags are not multiple by default");
}

/// Test creating a value-taking option
///
/// **Category**: Happy Path
/// **Verifies**: CommandOption::option() creates value-taking option
#[rstest]
fn test_option_value_creation() {
	let opt = CommandOption::option(Some('f'), "format", "Output format");

	assert_eq!(opt.short, Some('f'));
	assert_eq!(opt.long, "format");
	assert!(opt.takes_value, "Option should take value");
	assert!(!opt.required);
	assert!(opt.default.is_none());
	assert!(!opt.multiple);
}

/// Test making an option required
///
/// **Category**: Happy Path
/// **Verifies**: CommandOption::required() makes option required
#[rstest]
fn test_option_required() {
	let opt = CommandOption::option(Some('o'), "output", "Output file").required();

	assert!(opt.required, "Option should be required");
	assert!(opt.takes_value, "Should still take value");
}

/// Test adding default to option
///
/// **Category**: Happy Path
/// **Verifies**: CommandOption::with_default() adds default value
#[rstest]
fn test_option_with_default() {
	let opt = CommandOption::option(Some('p'), "port", "Server port").with_default("8080");

	assert_eq!(
		opt.default,
		Some("8080".to_string()),
		"Default should be set"
	);
	assert!(opt.takes_value);
}

/// Test allowing multiple values
///
/// **Category**: Happy Path
/// **Verifies**: CommandOption::multi() allows multiple values
#[rstest]
fn test_option_multi() {
	let opt = CommandOption::option(Some('i'), "include", "Include paths").multi();

	assert!(opt.multiple, "Option should allow multiple values");
	assert!(opt.takes_value);
}

/// Test chaining multiple modifiers
///
/// **Category**: Happy Path
/// **Verifies**: Multiple modifiers can be chained
#[rstest]
fn test_option_chained_modifiers() {
	let opt = CommandOption::option(Some('c'), "config", "Config file")
		.required()
		.with_default("default.toml")
		.multi();

	assert!(opt.required, "Should be required");
	assert_eq!(
		opt.default,
		Some("default.toml".to_string()),
		"Should have default"
	);
	assert!(opt.multiple, "Should allow multiple");
	assert!(opt.takes_value);
}

// =============================================================================
// Edge Case Tests
// =============================================================================

/// Test argument with empty name
///
/// **Category**: Edge Case
/// **Verifies**: Empty argument name is handled
#[rstest]
fn test_argument_empty_name() {
	let arg = CommandArgument::required("", "Empty name argument");

	assert_eq!(arg.name, "", "Empty name should be preserved");
	assert!(arg.required);
}

/// Test argument with Unicode name
///
/// **Category**: Edge Case
/// **Verifies**: Unicode argument names work
#[rstest]
fn test_argument_unicode_name() {
	let arg = CommandArgument::required("パス", "日本語の引数");

	assert_eq!(arg.name, "パス");
	assert_eq!(arg.description, "日本語の引数");
}

/// Test option without short flag
///
/// **Category**: Edge Case
/// **Verifies**: Options work with only long flag
#[rstest]
fn test_option_no_short() {
	let opt = CommandOption::flag(None, "very-verbose", "Very verbose output");

	assert_eq!(opt.short, None, "Short should be None");
	assert_eq!(opt.long, "very-verbose");
}

/// Test option with special characters in long name
///
/// **Category**: Edge Case
/// **Verifies**: Long names with special characters work
#[rstest]
fn test_option_special_long_name() {
	let opt = CommandOption::flag(None, "dry-run", "Perform dry run");

	assert_eq!(opt.long, "dry-run", "Long name with hyphen should work");
}

/// Test option with empty default
///
/// **Category**: Edge Case
/// **Verifies**: Empty string as default works
#[rstest]
fn test_option_empty_default() {
	let opt = CommandOption::option(None, "prefix", "Prefix string").with_default("");

	assert_eq!(
		opt.default,
		Some(String::new()),
		"Empty default should be set"
	);
}

/// Test argument with very long description
///
/// **Category**: Edge Case
/// **Verifies**: Long descriptions are handled
#[rstest]
fn test_argument_long_description() {
	let long_desc = "x".repeat(1000);
	let arg = CommandArgument::required("arg", &long_desc);

	assert_eq!(
		arg.description, long_desc,
		"Long description should be preserved"
	);
}

// =============================================================================
// Combination Tests
// =============================================================================

/// Test all CommandArgument combinations
///
/// **Category**: Combination
/// **Verifies**: All argument builder combinations work
#[rstest]
#[case(true, None, "required, no default")]
#[case(true, Some("default"), "required, with default")]
#[case(false, None, "optional, no default")]
#[case(false, Some("default"), "optional, with default")]
fn test_argument_combinations(
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

	assert_eq!(
		arg.required, required,
		"Required flag mismatch for {}",
		description
	);
	assert_eq!(
		arg.default,
		default.map(|s| s.to_string()),
		"Default mismatch for {}",
		description
	);
}

/// Test all CommandOption flag/value × required × default × multi combinations
///
/// **Category**: Combination
/// **Verifies**: All option builder combinations work
#[rstest]
#[case(false, false, None, false, "flag, not required")]
#[case(false, true, None, false, "flag, required")]
#[case(true, false, None, false, "value, not required")]
#[case(true, true, None, false, "value, required")]
#[case(true, false, Some("def"), false, "value with default")]
#[case(true, false, None, true, "value, multiple")]
#[case(true, true, Some("def"), true, "value, required, default, multi")]
fn test_option_combinations(
	#[case] takes_value: bool,
	#[case] required: bool,
	#[case] default: Option<&str>,
	#[case] multiple: bool,
	#[case] description: &str,
) {
	let mut opt = if takes_value {
		CommandOption::option(Some('t'), "test", description)
	} else {
		CommandOption::flag(Some('t'), "test", description)
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

	assert_eq!(
		opt.takes_value, takes_value,
		"takes_value mismatch for {}",
		description
	);
	assert_eq!(
		opt.required, required,
		"required mismatch for {}",
		description
	);
	assert_eq!(
		opt.default,
		default.map(|s| s.to_string()),
		"default mismatch for {}",
		description
	);
	assert_eq!(
		opt.multiple, multiple,
		"multiple mismatch for {}",
		description
	);
}

// =============================================================================
// Clone and Debug Tests
// =============================================================================

/// Test CommandArgument Clone
///
/// **Category**: Happy Path
/// **Verifies**: CommandArgument implements Clone correctly
#[rstest]
fn test_argument_clone() {
	let arg = CommandArgument::required("name", "Description").with_default("default");
	let cloned = arg.clone();

	assert_eq!(cloned.name, arg.name);
	assert_eq!(cloned.description, arg.description);
	assert_eq!(cloned.required, arg.required);
	assert_eq!(cloned.default, arg.default);
}

/// Test CommandOption Clone
///
/// **Category**: Happy Path
/// **Verifies**: CommandOption implements Clone correctly
#[rstest]
fn test_option_clone() {
	let opt = CommandOption::option(Some('o'), "output", "Output file")
		.required()
		.with_default("out.txt")
		.multi();
	let cloned = opt.clone();

	assert_eq!(cloned.short, opt.short);
	assert_eq!(cloned.long, opt.long);
	assert_eq!(cloned.description, opt.description);
	assert_eq!(cloned.takes_value, opt.takes_value);
	assert_eq!(cloned.required, opt.required);
	assert_eq!(cloned.default, opt.default);
	assert_eq!(cloned.multiple, opt.multiple);
}

/// Test CommandArgument Debug
///
/// **Category**: Happy Path
/// **Verifies**: CommandArgument implements Debug
#[rstest]
fn test_argument_debug() {
	let arg = CommandArgument::required("test", "Test argument");
	let debug = format!("{:?}", arg);

	assert!(
		debug.contains("CommandArgument"),
		"Debug should contain type name"
	);
	assert!(debug.contains("test"), "Debug should contain name");
}

/// Test CommandOption Debug
///
/// **Category**: Happy Path
/// **Verifies**: CommandOption implements Debug
#[rstest]
fn test_option_debug() {
	let opt = CommandOption::flag(Some('t'), "test", "Test option");
	let debug = format!("{:?}", opt);

	assert!(
		debug.contains("CommandOption"),
		"Debug should contain type name"
	);
	assert!(debug.contains("test"), "Debug should contain long name");
}

// =============================================================================
// Sanity Tests
// =============================================================================

/// Sanity test for basic argument/option workflow
///
/// **Category**: Sanity
/// **Verifies**: Basic creation and access works
#[rstest]
fn test_argument_option_basic_sanity() {
	// Create argument
	let arg = CommandArgument::required("file", "Input file");
	assert_eq!(arg.name, "file");
	assert!(arg.required);

	// Create option
	let opt = CommandOption::option(Some('o'), "output", "Output file").with_default("out.txt");
	assert_eq!(opt.long, "output");
	assert!(opt.takes_value);
	assert_eq!(opt.default, Some("out.txt".to_string()));

	// Create flag
	let flag = CommandOption::flag(Some('v'), "verbose", "Verbose mode");
	assert!(!flag.takes_value);
	assert!(!flag.required);
}
