//! System check integration tests
//!
//! Tests for system check configuration in commands.
//! Note: This tests the system check configuration methods, not actual check execution,
//! as the SystemCheck trait is not part of the public API.

use async_trait::async_trait;
use reinhardt_commands::{BaseCommand, CommandContext, CommandResult};
use rstest::rstest;

// =============================================================================
// Test Command Implementations
// =============================================================================

/// A command that requires system checks
struct CheckRequiringCommand {
	requires_checks: bool,
	check_tags: Vec<String>,
}

impl CheckRequiringCommand {
	fn new(requires_checks: bool) -> Self {
		Self {
			requires_checks,
			check_tags: vec![],
		}
	}

	fn with_tags(mut self, tags: Vec<&str>) -> Self {
		self.check_tags = tags.into_iter().map(|s| s.to_string()).collect();
		self
	}
}

#[async_trait]
impl BaseCommand for CheckRequiringCommand {
	fn name(&self) -> &str {
		"check_requiring"
	}

	fn description(&self) -> &str {
		"A command that may require system checks"
	}

	fn requires_system_checks(&self) -> bool {
		self.requires_checks
	}

	fn check_tags(&self) -> Vec<String> {
		self.check_tags.clone()
	}

	async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		Ok(())
	}
}

// =============================================================================
// Use Case Tests
// =============================================================================

/// Test command that requires system checks
///
/// **Category**: Use Case
/// **Verifies**: requires_system_checks returns correct value
#[rstest]
#[tokio::test]
async fn test_command_requires_checks() {
	let cmd = CheckRequiringCommand::new(true);

	assert!(
		cmd.requires_system_checks(),
		"Command should require system checks"
	);
}

/// Test command that does not require system checks
///
/// **Category**: Use Case
/// **Verifies**: requires_system_checks returns false when not required
#[rstest]
#[tokio::test]
async fn test_command_no_checks_required() {
	let cmd = CheckRequiringCommand::new(false);

	assert!(
		!cmd.requires_system_checks(),
		"Command should not require system checks"
	);
}

/// Test command with check tags
///
/// **Category**: Use Case
/// **Verifies**: check_tags returns configured tags
#[rstest]
#[tokio::test]
async fn test_command_with_check_tags() {
	let cmd = CheckRequiringCommand::new(true).with_tags(vec!["database", "cache"]);

	let tags = cmd.check_tags();
	assert_eq!(tags.len(), 2, "Should have 2 tags");
	assert!(
		tags.contains(&"database".to_string()),
		"Should contain database tag"
	);
	assert!(
		tags.contains(&"cache".to_string()),
		"Should contain cache tag"
	);
}

/// Test command with skip_checks option
///
/// **Category**: Use Case
/// **Verifies**: --skip-checks option is detected
#[rstest]
#[tokio::test]
async fn test_skip_checks_option() {
	let cmd = CheckRequiringCommand::new(true);
	let mut ctx = CommandContext::new(vec![]);
	ctx.set_option("skip-checks".to_string(), "true".to_string());

	assert!(
		ctx.should_skip_checks(),
		"Context should detect skip-checks option"
	);

	// Command should still execute
	let result = cmd.execute(&ctx).await;
	assert!(result.is_ok());
}

/// Test command with skip_checks underscore variant
///
/// **Category**: Use Case
/// **Verifies**: skip_checks option variant is detected
#[rstest]
#[tokio::test]
async fn test_skip_checks_underscore_option() {
	let mut ctx = CommandContext::new(vec![]);
	ctx.set_option("skip_checks".to_string(), "true".to_string());

	assert!(
		ctx.should_skip_checks(),
		"Context should detect skip_checks option"
	);
}

// =============================================================================
// Decision Table Tests
// =============================================================================

/// Decision table for requires_system_checks and skip_checks combinations
///
/// **Category**: Decision Table
/// **Verifies**: Different check requirement and skip combinations
///
/// | requires_checks | skip_checks | should_run_checks |
/// |-----------------|-------------|-------------------|
/// | false           | false       | false             |
/// | false           | true        | false             |
/// | true            | false       | true              |
/// | true            | true        | false             |
#[rstest]
#[case(false, false, false, "no requirement, no skip")]
#[case(false, true, false, "no requirement, skip enabled")]
#[case(true, false, true, "requires, no skip")]
#[case(true, true, false, "requires but skipped")]
#[tokio::test]
async fn test_check_decision_table(
	#[case] requires_checks: bool,
	#[case] skip_checks: bool,
	#[case] should_run_checks: bool,
	#[case] _description: &str,
) {
	let cmd = CheckRequiringCommand::new(requires_checks);
	let mut ctx = CommandContext::new(vec![]);

	if skip_checks {
		ctx.set_option("skip-checks".to_string(), "true".to_string());
	}

	// Determine if checks should run
	let actual_should_run = cmd.requires_system_checks() && !ctx.should_skip_checks();

	assert_eq!(
		actual_should_run, should_run_checks,
		"requires={}, skip={} should result in run_checks={}",
		requires_checks, skip_checks, should_run_checks
	);
}

// =============================================================================
// Edge Case Tests
// =============================================================================

/// Test command with empty check tags
///
/// **Category**: Edge Case
/// **Verifies**: Empty tags list is handled correctly
#[rstest]
#[tokio::test]
async fn test_command_empty_check_tags() {
	let cmd = CheckRequiringCommand::new(true);

	let tags = cmd.check_tags();
	assert!(tags.is_empty(), "Should have no tags");
}

/// Test command with many check tags
///
/// **Category**: Edge Case
/// **Verifies**: Many tags are handled correctly
#[rstest]
#[tokio::test]
async fn test_command_many_check_tags() {
	let tags: Vec<&str> = (0..10).map(|_| "tag").collect();
	let cmd = CheckRequiringCommand::new(true).with_tags(tags);

	assert_eq!(cmd.check_tags().len(), 10, "Should have 10 tags");
}

/// Test command with duplicate check tags
///
/// **Category**: Edge Case
/// **Verifies**: Duplicate tags are preserved
#[rstest]
#[tokio::test]
async fn test_command_duplicate_check_tags() {
	let cmd = CheckRequiringCommand::new(true).with_tags(vec!["database", "database", "cache"]);

	let tags = cmd.check_tags();
	assert_eq!(tags.len(), 3, "Should preserve duplicate tags");
}

// =============================================================================
// State Transition Tests
// =============================================================================

/// Test skip_checks state changes
///
/// **Category**: State Transition
/// **Verifies**: Skip checks state can be toggled
#[rstest]
#[tokio::test]
async fn test_skip_checks_state_transition() {
	let mut ctx = CommandContext::new(vec![]);

	// Initial state: not skipping
	assert!(!ctx.should_skip_checks(), "Initial state should not skip");

	// Transition: enable skip
	ctx.set_option("skip-checks".to_string(), "true".to_string());
	assert!(ctx.should_skip_checks(), "After setting, should skip");
}

// =============================================================================
// Combination Tests
// =============================================================================

/// Test various tag combinations
///
/// **Category**: Combination
/// **Verifies**: Different tag configurations work correctly
#[rstest]
#[case(vec![], 0, "empty tags")]
#[case(vec!["db"], 1, "single tag")]
#[case(vec!["db", "cache"], 2, "two tags")]
#[case(vec!["db", "cache", "migrations"], 3, "three tags")]
#[tokio::test]
async fn test_check_tag_combinations(
	#[case] tags: Vec<&str>,
	#[case] expected_count: usize,
	#[case] _description: &str,
) {
	let cmd = CheckRequiringCommand::new(true).with_tags(tags);

	assert_eq!(
		cmd.check_tags().len(),
		expected_count,
		"Tag count should match"
	);
}

/// Test requires_checks and execute interaction
///
/// **Category**: Combination
/// **Verifies**: Check requirement doesn't affect execution
#[rstest]
#[case(true)]
#[case(false)]
#[tokio::test]
async fn test_requires_checks_execute_combination(#[case] requires_checks: bool) {
	let cmd = CheckRequiringCommand::new(requires_checks);
	let ctx = CommandContext::new(vec![]);

	// Execute should succeed regardless of check requirement
	let result = cmd.execute(&ctx).await;
	assert!(
		result.is_ok(),
		"Execute should succeed with requires_checks={}",
		requires_checks
	);
}
