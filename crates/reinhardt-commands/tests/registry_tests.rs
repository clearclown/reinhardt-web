//! CommandRegistry unit tests
//!
//! Tests for the command registry functionality including registration,
//! retrieval, and listing of commands.

use async_trait::async_trait;
use reinhardt_commands::{BaseCommand, CommandContext, CommandRegistry, CommandResult};
use rstest::{fixture, rstest};

// =============================================================================
// Mock Command Implementation
// =============================================================================

/// Mock command for testing registry operations
struct MockCommand {
	name: String,
	description: String,
}

impl MockCommand {
	fn new(name: impl Into<String>) -> Self {
		let name = name.into();
		Self {
			description: format!("Mock command: {}", name),
			name,
		}
	}

	fn with_description(name: impl Into<String>, description: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			description: description.into(),
		}
	}
}

#[async_trait]
impl BaseCommand for MockCommand {
	fn name(&self) -> &str {
		&self.name
	}

	fn description(&self) -> &str {
		&self.description
	}

	async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		Ok(())
	}
}

// =============================================================================
// Fixtures
// =============================================================================

#[fixture]
fn empty_registry() -> CommandRegistry {
	CommandRegistry::new()
}

#[fixture]
fn mock_command() -> MockCommand {
	MockCommand::new("test")
}

#[fixture]
fn populated_registry() -> CommandRegistry {
	let mut registry = CommandRegistry::new();
	registry.register(Box::new(MockCommand::new("cmd1")));
	registry.register(Box::new(MockCommand::new("cmd2")));
	registry.register(Box::new(MockCommand::new("cmd3")));
	registry
}

// =============================================================================
// Happy Path Tests
// =============================================================================

/// Test that a new registry is empty
///
/// **Category**: Happy Path
/// **Verifies**: CommandRegistry::new() creates an empty registry
#[rstest]
fn test_registry_new_creates_empty(empty_registry: CommandRegistry) {
	let names = empty_registry.list();

	assert!(names.is_empty(), "New registry should have no commands");
}

/// Test that Default trait creates an empty registry
///
/// **Category**: Happy Path
/// **Verifies**: CommandRegistry::default() creates an empty registry
#[rstest]
fn test_registry_default_creates_empty() {
	let registry = CommandRegistry::default();
	let names = registry.list();

	assert!(names.is_empty(), "Default registry should have no commands");
}

/// Test registering and retrieving a command
///
/// **Category**: Happy Path
/// **Verifies**: register() stores command, get() retrieves by name
#[rstest]
fn test_registry_register_and_get(mut empty_registry: CommandRegistry) {
	let cmd = MockCommand::with_description("mycommand", "Test command description");
	let expected_name = "mycommand";
	let expected_description = "Test command description";

	empty_registry.register(Box::new(cmd));
	let retrieved = empty_registry.get(expected_name);

	assert!(
		retrieved.is_some(),
		"Command should be retrievable after registration"
	);
	let retrieved_cmd = retrieved.unwrap();
	assert_eq!(
		retrieved_cmd.name(),
		expected_name,
		"Retrieved command name should match"
	);
	assert_eq!(
		retrieved_cmd.description(),
		expected_description,
		"Retrieved command description should match"
	);
}

/// Test listing all registered command names
///
/// **Category**: Happy Path
/// **Verifies**: list() returns all registered command names
#[rstest]
fn test_registry_list_returns_all_names(populated_registry: CommandRegistry) {
	let names = populated_registry.list();

	assert_eq!(names.len(), 3, "Should have exactly 3 registered commands");

	// Check that all expected names are present (order may vary due to HashMap)
	assert!(names.contains(&"cmd1"), "Should contain 'cmd1'");
	assert!(names.contains(&"cmd2"), "Should contain 'cmd2'");
	assert!(names.contains(&"cmd3"), "Should contain 'cmd3'");
}

/// Test registering multiple commands with different names
///
/// **Category**: Happy Path
/// **Verifies**: Multiple commands can be registered and retrieved
#[rstest]
fn test_registry_multiple_commands(mut empty_registry: CommandRegistry) {
	let commands = vec![
		MockCommand::with_description("alpha", "Alpha command"),
		MockCommand::with_description("beta", "Beta command"),
		MockCommand::with_description("gamma", "Gamma command"),
	];

	for cmd in commands {
		empty_registry.register(Box::new(cmd));
	}

	// Verify each command is retrievable with correct data
	let alpha = empty_registry.get("alpha").expect("alpha should exist");
	assert_eq!(alpha.name(), "alpha");
	assert_eq!(alpha.description(), "Alpha command");

	let beta = empty_registry.get("beta").expect("beta should exist");
	assert_eq!(beta.name(), "beta");
	assert_eq!(beta.description(), "Beta command");

	let gamma = empty_registry.get("gamma").expect("gamma should exist");
	assert_eq!(gamma.name(), "gamma");
	assert_eq!(gamma.description(), "Gamma command");
}

// =============================================================================
// Error Path Tests
// =============================================================================

/// Test getting a nonexistent command returns None
///
/// **Category**: Error Path
/// **Verifies**: get() returns None for unknown command names
#[rstest]
fn test_registry_get_nonexistent(empty_registry: CommandRegistry) {
	let result = empty_registry.get("nonexistent");

	assert!(
		result.is_none(),
		"Getting nonexistent command should return None"
	);
}

/// Test getting a nonexistent command from populated registry
///
/// **Category**: Error Path
/// **Verifies**: get() returns None even when other commands exist
#[rstest]
fn test_registry_get_nonexistent_from_populated(populated_registry: CommandRegistry) {
	let result = populated_registry.get("unknown_command");

	assert!(
		result.is_none(),
		"Getting nonexistent command from populated registry should return None"
	);
}

// =============================================================================
// Edge Case Tests
// =============================================================================

/// Test that duplicate registration overwrites the previous command
///
/// **Category**: Edge Case
/// **Verifies**: Registering same name twice overwrites the first command
#[rstest]
fn test_registry_duplicate_registration(mut empty_registry: CommandRegistry) {
	let cmd1 = MockCommand::with_description("duplicate", "First version");
	let cmd2 = MockCommand::with_description("duplicate", "Second version");

	empty_registry.register(Box::new(cmd1));
	empty_registry.register(Box::new(cmd2));

	let names = empty_registry.list();
	assert_eq!(
		names.len(),
		1,
		"Should have only one command after duplicate registration"
	);

	let retrieved = empty_registry
		.get("duplicate")
		.expect("Command should exist");
	assert_eq!(
		retrieved.description(),
		"Second version",
		"Second registration should overwrite the first"
	);
}

/// Test listing an empty registry
///
/// **Category**: Edge Case
/// **Verifies**: list() returns empty Vec for empty registry
#[rstest]
fn test_registry_empty_list(empty_registry: CommandRegistry) {
	let names = empty_registry.list();

	assert!(names.is_empty(), "Empty registry should return empty list");
	assert_eq!(names.len(), 0, "Empty registry list length should be 0");
}

/// Test command with empty name
///
/// **Category**: Edge Case
/// **Verifies**: Commands with empty names can be registered and retrieved
#[rstest]
fn test_registry_empty_name_command(mut empty_registry: CommandRegistry) {
	let cmd = MockCommand::new("");

	empty_registry.register(Box::new(cmd));
	let retrieved = empty_registry.get("");

	assert!(
		retrieved.is_some(),
		"Empty name command should be retrievable"
	);
	assert_eq!(
		retrieved.unwrap().name(),
		"",
		"Empty name should be preserved"
	);
}

/// Test command with special characters in name
///
/// **Category**: Edge Case
/// **Verifies**: Commands with special characters in names work correctly
#[rstest]
fn test_registry_special_characters_in_name(mut empty_registry: CommandRegistry) {
	let special_names = vec![
		"cmd-with-dashes",
		"cmd_with_underscores",
		"cmd:with:colons",
		"cmd.with.dots",
	];

	for name in &special_names {
		empty_registry.register(Box::new(MockCommand::new(*name)));
	}

	for name in &special_names {
		let retrieved = empty_registry.get(name);
		assert!(
			retrieved.is_some(),
			"Command '{}' should be retrievable",
			name
		);
		assert_eq!(
			retrieved.unwrap().name(),
			*name,
			"Command name should match for '{}'",
			name
		);
	}
}

/// Test command with Unicode name
///
/// **Category**: Edge Case
/// **Verifies**: Commands with Unicode characters in names work correctly
#[rstest]
fn test_registry_unicode_name(mut empty_registry: CommandRegistry) {
	let unicode_name = "コマンド";
	let cmd = MockCommand::with_description(unicode_name, "Unicode command");

	empty_registry.register(Box::new(cmd));
	let retrieved = empty_registry.get(unicode_name);

	assert!(
		retrieved.is_some(),
		"Unicode name command should be retrievable"
	);
	assert_eq!(
		retrieved.unwrap().name(),
		unicode_name,
		"Unicode name should be preserved"
	);
}

// =============================================================================
// State Transition Tests
// =============================================================================

/// Test registry state after multiple registrations
///
/// **Category**: State Transition
/// **Verifies**: Registry correctly tracks state through multiple operations
#[rstest]
fn test_registry_state_after_multiple_operations(mut empty_registry: CommandRegistry) {
	// Initial state
	assert_eq!(
		empty_registry.list().len(),
		0,
		"Initial state should be empty"
	);

	// After first registration
	empty_registry.register(Box::new(MockCommand::new("first")));
	assert_eq!(
		empty_registry.list().len(),
		1,
		"Should have 1 command after first registration"
	);

	// After second registration
	empty_registry.register(Box::new(MockCommand::new("second")));
	assert_eq!(
		empty_registry.list().len(),
		2,
		"Should have 2 commands after second registration"
	);

	// After duplicate registration (same name as "first")
	empty_registry.register(Box::new(MockCommand::with_description("first", "Updated")));
	assert_eq!(
		empty_registry.list().len(),
		2,
		"Should still have 2 commands after duplicate registration"
	);

	// Verify the updated command
	let first = empty_registry.get("first").expect("first should exist");
	assert_eq!(
		first.description(),
		"Updated",
		"First command should be updated version"
	);

	// After third unique registration
	empty_registry.register(Box::new(MockCommand::new("third")));
	assert_eq!(
		empty_registry.list().len(),
		3,
		"Should have 3 commands after third registration"
	);
}

/// Test that getting a command does not modify registry state
///
/// **Category**: State Transition
/// **Verifies**: get() is a pure read operation
#[rstest]
fn test_registry_get_does_not_modify_state(populated_registry: CommandRegistry) {
	let initial_count = populated_registry.list().len();

	// Perform multiple get operations
	let _ = populated_registry.get("cmd1");
	let _ = populated_registry.get("nonexistent");
	let _ = populated_registry.get("cmd2");
	let _ = populated_registry.get("");

	let final_count = populated_registry.list().len();
	assert_eq!(
		initial_count, final_count,
		"get() operations should not modify registry state"
	);
}

// =============================================================================
// Sanity Tests
// =============================================================================

/// Sanity test for basic registry workflow
///
/// **Category**: Sanity
/// **Verifies**: Basic create-register-get-list workflow works
#[rstest]
fn test_registry_basic_workflow() {
	// Create
	let mut registry = CommandRegistry::new();

	// Register
	registry.register(Box::new(MockCommand::with_description(
		"sanity",
		"Sanity test command",
	)));

	// Get
	let cmd = registry.get("sanity");
	assert!(cmd.is_some(), "Sanity: command should be retrievable");

	let cmd = cmd.unwrap();
	assert_eq!(cmd.name(), "sanity", "Sanity: name should match");
	assert_eq!(
		cmd.description(),
		"Sanity test command",
		"Sanity: description should match"
	);

	// List
	let names = registry.list();
	assert_eq!(names.len(), 1, "Sanity: should have exactly one command");
	assert!(
		names.contains(&"sanity"),
		"Sanity: list should contain command name"
	);
}
