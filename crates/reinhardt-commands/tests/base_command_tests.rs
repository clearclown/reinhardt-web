//! BaseCommand lifecycle and system check integration tests
//!
//! Tests for command lifecycle (before_execute, execute, after_execute)
//! and system check integration.

use async_trait::async_trait;
use reinhardt_commands::{
	BaseCommand, CommandArgument, CommandContext, CommandError, CommandOption, CommandResult,
};
use rstest::{fixture, rstest};
use serial_test::serial;
use std::sync::{
	Arc,
	atomic::{AtomicBool, AtomicU8, Ordering},
};

// =============================================================================
// Mock Command Implementations
// =============================================================================

/// State tracking command for verifying lifecycle execution order
struct StateTrackingCommand {
	before_called: Arc<AtomicBool>,
	execute_called: Arc<AtomicBool>,
	after_called: Arc<AtomicBool>,
	call_order: Arc<std::sync::Mutex<Vec<&'static str>>>,
}

impl StateTrackingCommand {
	fn new() -> Self {
		Self {
			before_called: Arc::new(AtomicBool::new(false)),
			execute_called: Arc::new(AtomicBool::new(false)),
			after_called: Arc::new(AtomicBool::new(false)),
			call_order: Arc::new(std::sync::Mutex::new(Vec::new())),
		}
	}
}

#[async_trait]
impl BaseCommand for StateTrackingCommand {
	fn name(&self) -> &str {
		"state_tracking"
	}

	fn requires_system_checks(&self) -> bool {
		false // Disable system checks for lifecycle tests
	}

	async fn before_execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.before_called.store(true, Ordering::SeqCst);
		self.call_order.lock().unwrap().push("before");
		Ok(())
	}

	async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.execute_called.store(true, Ordering::SeqCst);
		self.call_order.lock().unwrap().push("execute");
		Ok(())
	}

	async fn after_execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.after_called.store(true, Ordering::SeqCst);
		self.call_order.lock().unwrap().push("after");
		Ok(())
	}
}

/// Command that fails at before_execute
struct FailingBeforeCommand {
	before_called: Arc<AtomicBool>,
	execute_called: Arc<AtomicBool>,
	after_called: Arc<AtomicBool>,
}

impl FailingBeforeCommand {
	fn new() -> Self {
		Self {
			before_called: Arc::new(AtomicBool::new(false)),
			execute_called: Arc::new(AtomicBool::new(false)),
			after_called: Arc::new(AtomicBool::new(false)),
		}
	}
}

#[async_trait]
impl BaseCommand for FailingBeforeCommand {
	fn name(&self) -> &str {
		"failing_before"
	}

	fn requires_system_checks(&self) -> bool {
		false
	}

	async fn before_execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.before_called.store(true, Ordering::SeqCst);
		Err(CommandError::ExecutionError(
			"before_execute failed".to_string(),
		))
	}

	async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.execute_called.store(true, Ordering::SeqCst);
		Ok(())
	}

	async fn after_execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.after_called.store(true, Ordering::SeqCst);
		Ok(())
	}
}

/// Command that fails at execute
struct FailingExecuteCommand {
	before_called: Arc<AtomicBool>,
	execute_called: Arc<AtomicBool>,
	after_called: Arc<AtomicBool>,
}

impl FailingExecuteCommand {
	fn new() -> Self {
		Self {
			before_called: Arc::new(AtomicBool::new(false)),
			execute_called: Arc::new(AtomicBool::new(false)),
			after_called: Arc::new(AtomicBool::new(false)),
		}
	}
}

#[async_trait]
impl BaseCommand for FailingExecuteCommand {
	fn name(&self) -> &str {
		"failing_execute"
	}

	fn requires_system_checks(&self) -> bool {
		false
	}

	async fn before_execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.before_called.store(true, Ordering::SeqCst);
		Ok(())
	}

	async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.execute_called.store(true, Ordering::SeqCst);
		Err(CommandError::ExecutionError("execute failed".to_string()))
	}

	async fn after_execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.after_called.store(true, Ordering::SeqCst);
		Ok(())
	}
}

/// Command that fails at after_execute
struct FailingAfterCommand {
	before_called: Arc<AtomicBool>,
	execute_called: Arc<AtomicBool>,
	after_called: Arc<AtomicBool>,
}

impl FailingAfterCommand {
	fn new() -> Self {
		Self {
			before_called: Arc::new(AtomicBool::new(false)),
			execute_called: Arc::new(AtomicBool::new(false)),
			after_called: Arc::new(AtomicBool::new(false)),
		}
	}
}

#[async_trait]
impl BaseCommand for FailingAfterCommand {
	fn name(&self) -> &str {
		"failing_after"
	}

	fn requires_system_checks(&self) -> bool {
		false
	}

	async fn before_execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.before_called.store(true, Ordering::SeqCst);
		Ok(())
	}

	async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.execute_called.store(true, Ordering::SeqCst);
		Ok(())
	}

	async fn after_execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.after_called.store(true, Ordering::SeqCst);
		Err(CommandError::ExecutionError(
			"after_execute failed".to_string(),
		))
	}
}

/// Command with configurable system check settings
struct ConfigurableCommand {
	requires_checks: bool,
	check_tags: Vec<String>,
	execution_count: Arc<AtomicU8>,
}

impl ConfigurableCommand {
	fn new(requires_checks: bool, check_tags: Vec<String>) -> Self {
		Self {
			requires_checks,
			check_tags,
			execution_count: Arc::new(AtomicU8::new(0)),
		}
	}

	fn execution_count(&self) -> u8 {
		self.execution_count.load(Ordering::SeqCst)
	}
}

#[async_trait]
impl BaseCommand for ConfigurableCommand {
	fn name(&self) -> &str {
		"configurable"
	}

	fn requires_system_checks(&self) -> bool {
		self.requires_checks
	}

	fn check_tags(&self) -> Vec<String> {
		self.check_tags.clone()
	}

	async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		self.execution_count.fetch_add(1, Ordering::SeqCst);
		Ok(())
	}
}

/// Simple command for testing default implementations
struct SimpleCommand;

#[async_trait]
impl BaseCommand for SimpleCommand {
	fn name(&self) -> &str {
		"simple"
	}

	async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		Ok(())
	}
}

/// Command with custom arguments and options
struct CommandWithMetadata;

#[async_trait]
impl BaseCommand for CommandWithMetadata {
	fn name(&self) -> &str {
		"metadata"
	}

	fn description(&self) -> &str {
		"Command with metadata"
	}

	fn help(&self) -> &str {
		"Detailed help text for the command"
	}

	fn arguments(&self) -> Vec<CommandArgument> {
		vec![
			CommandArgument::required("name", "The name argument"),
			CommandArgument::optional("extra", "Optional extra argument").with_default("default"),
		]
	}

	fn options(&self) -> Vec<CommandOption> {
		vec![
			CommandOption::flag(Some('v'), "verbose", "Enable verbose output"),
			CommandOption::option(Some('f'), "format", "Output format").with_default("json"),
		]
	}

	async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
		Ok(())
	}
}

// =============================================================================
// Fixtures
// =============================================================================

#[fixture]
fn context() -> CommandContext {
	CommandContext::new(vec![])
}

#[fixture]
fn state_tracking_command() -> StateTrackingCommand {
	StateTrackingCommand::new()
}

#[fixture]
fn failing_before_command() -> FailingBeforeCommand {
	FailingBeforeCommand::new()
}

#[fixture]
fn failing_execute_command() -> FailingExecuteCommand {
	FailingExecuteCommand::new()
}

#[fixture]
fn failing_after_command() -> FailingAfterCommand {
	FailingAfterCommand::new()
}

// =============================================================================
// Happy Path Tests
// =============================================================================

/// Test full lifecycle execution order
///
/// **Category**: Happy Path
/// **Verifies**: before_execute -> execute -> after_execute order
#[rstest]
#[tokio::test]
async fn test_command_full_lifecycle(
	context: CommandContext,
	state_tracking_command: StateTrackingCommand,
) {
	let result = state_tracking_command.run(&context).await;

	assert!(result.is_ok(), "Lifecycle should complete successfully");

	// Verify all methods were called
	assert!(
		state_tracking_command.before_called.load(Ordering::SeqCst),
		"before_execute should be called"
	);
	assert!(
		state_tracking_command.execute_called.load(Ordering::SeqCst),
		"execute should be called"
	);
	assert!(
		state_tracking_command.after_called.load(Ordering::SeqCst),
		"after_execute should be called"
	);

	// Verify execution order
	let call_order = state_tracking_command.call_order.lock().unwrap();
	assert_eq!(
		call_order.len(),
		3,
		"All three lifecycle methods should be called"
	);
	assert_eq!(
		call_order[0], "before",
		"before_execute should be called first"
	);
	assert_eq!(call_order[1], "execute", "execute should be called second");
	assert_eq!(
		call_order[2], "after",
		"after_execute should be called third"
	);
}

/// Test default implementations of BaseCommand trait
///
/// **Category**: Happy Path
/// **Verifies**: Default trait implementations work correctly
#[rstest]
fn test_command_default_implementations() {
	let cmd = SimpleCommand;

	// Default description
	assert_eq!(
		cmd.description(),
		"No description available",
		"Default description should be provided"
	);

	// Default help (falls back to description)
	assert_eq!(
		cmd.help(),
		"No description available",
		"Default help should fall back to description"
	);

	// Default arguments (empty)
	assert!(
		cmd.arguments().is_empty(),
		"Default arguments should be empty"
	);

	// Default options (empty)
	assert!(cmd.options().is_empty(), "Default options should be empty");

	// Default requires_system_checks (true)
	assert!(
		cmd.requires_system_checks(),
		"Default requires_system_checks should be true"
	);

	// Default check_tags (empty)
	assert!(
		cmd.check_tags().is_empty(),
		"Default check_tags should be empty"
	);
}

/// Test command with custom metadata
///
/// **Category**: Happy Path
/// **Verifies**: Custom arguments and options are correctly returned
#[rstest]
fn test_command_with_metadata() {
	let cmd = CommandWithMetadata;

	// Custom description
	assert_eq!(cmd.description(), "Command with metadata");

	// Custom help
	assert_eq!(cmd.help(), "Detailed help text for the command");

	// Arguments
	let args = cmd.arguments();
	assert_eq!(args.len(), 2, "Should have 2 arguments");
	assert_eq!(args[0].name, "name");
	assert!(args[0].required);
	assert_eq!(args[1].name, "extra");
	assert!(!args[1].required);
	assert_eq!(args[1].default, Some("default".to_string()));

	// Options
	let opts = cmd.options();
	assert_eq!(opts.len(), 2, "Should have 2 options");
	assert_eq!(opts[0].short, Some('v'));
	assert_eq!(opts[0].long, "verbose");
	assert!(!opts[0].takes_value);
	assert_eq!(opts[1].short, Some('f'));
	assert_eq!(opts[1].long, "format");
	assert!(opts[1].takes_value);
	assert_eq!(opts[1].default, Some("json".to_string()));
}

/// Test command run with skip_checks option
///
/// **Category**: Happy Path
/// **Verifies**: System checks are skipped when skip_checks option is set
#[rstest]
#[tokio::test]
#[serial(system_checks)]
async fn test_command_run_with_skip_checks() {
	let cmd = ConfigurableCommand::new(true, vec![]);
	let mut ctx = CommandContext::new(vec![]);
	ctx.set_option("skip_checks".to_string(), "true".to_string());

	let result = cmd.run(&ctx).await;

	assert!(result.is_ok(), "Command should succeed with skip_checks");
	assert_eq!(
		cmd.execution_count(),
		1,
		"Command should have executed once"
	);
}

// =============================================================================
// Error Path Tests
// =============================================================================

/// Test failure in before_execute stops lifecycle
///
/// **Category**: Error Path
/// **Verifies**: Failure in before_execute prevents execute and after_execute
#[rstest]
#[tokio::test]
async fn test_command_before_execute_failure(
	context: CommandContext,
	failing_before_command: FailingBeforeCommand,
) {
	let result = failing_before_command.run(&context).await;

	// Should return error
	assert!(
		result.is_err(),
		"run() should return error when before_execute fails"
	);

	let error = result.unwrap_err();
	match error {
		CommandError::ExecutionError(msg) => {
			assert!(
				msg.contains("before_execute failed"),
				"Error message should mention before_execute"
			);
		}
		_ => panic!("Expected ExecutionError, got {:?}", error),
	}

	// Verify state
	assert!(
		failing_before_command.before_called.load(Ordering::SeqCst),
		"before_execute should have been called"
	);
	assert!(
		!failing_before_command.execute_called.load(Ordering::SeqCst),
		"execute should NOT have been called"
	);
	assert!(
		!failing_before_command.after_called.load(Ordering::SeqCst),
		"after_execute should NOT have been called"
	);
}

/// Test failure in execute stops lifecycle
///
/// **Category**: Error Path
/// **Verifies**: Failure in execute prevents after_execute
#[rstest]
#[tokio::test]
async fn test_command_execute_failure(
	context: CommandContext,
	failing_execute_command: FailingExecuteCommand,
) {
	let result = failing_execute_command.run(&context).await;

	// Should return error
	assert!(
		result.is_err(),
		"run() should return error when execute fails"
	);

	let error = result.unwrap_err();
	match error {
		CommandError::ExecutionError(msg) => {
			assert!(
				msg.contains("execute failed"),
				"Error message should mention execute"
			);
		}
		_ => panic!("Expected ExecutionError, got {:?}", error),
	}

	// Verify state
	assert!(
		failing_execute_command.before_called.load(Ordering::SeqCst),
		"before_execute should have been called"
	);
	assert!(
		failing_execute_command
			.execute_called
			.load(Ordering::SeqCst),
		"execute should have been called"
	);
	assert!(
		!failing_execute_command.after_called.load(Ordering::SeqCst),
		"after_execute should NOT have been called"
	);
}

/// Test failure in after_execute returns error but execute was successful
///
/// **Category**: Error Path
/// **Verifies**: Failure in after_execute still returns error
#[rstest]
#[tokio::test]
async fn test_command_after_execute_failure(
	context: CommandContext,
	failing_after_command: FailingAfterCommand,
) {
	let result = failing_after_command.run(&context).await;

	// Should return error
	assert!(
		result.is_err(),
		"run() should return error when after_execute fails"
	);

	let error = result.unwrap_err();
	match error {
		CommandError::ExecutionError(msg) => {
			assert!(
				msg.contains("after_execute failed"),
				"Error message should mention after_execute"
			);
		}
		_ => panic!("Expected ExecutionError, got {:?}", error),
	}

	// Verify all methods were called
	assert!(
		failing_after_command.before_called.load(Ordering::SeqCst),
		"before_execute should have been called"
	);
	assert!(
		failing_after_command.execute_called.load(Ordering::SeqCst),
		"execute should have been called"
	);
	assert!(
		failing_after_command.after_called.load(Ordering::SeqCst),
		"after_execute should have been called"
	);
}

// =============================================================================
// State Transition Tests
// =============================================================================

/// Test command lifecycle state tracking
///
/// **Category**: State Transition
/// **Verifies**: State flags are correctly set through lifecycle
#[rstest]
#[tokio::test]
async fn test_command_lifecycle_state_tracking(
	context: CommandContext,
	state_tracking_command: StateTrackingCommand,
) {
	// Before run, all flags should be false
	assert!(
		!state_tracking_command.before_called.load(Ordering::SeqCst),
		"before_called should be false initially"
	);
	assert!(
		!state_tracking_command.execute_called.load(Ordering::SeqCst),
		"execute_called should be false initially"
	);
	assert!(
		!state_tracking_command.after_called.load(Ordering::SeqCst),
		"after_called should be false initially"
	);

	// Run the command
	let _ = state_tracking_command.run(&context).await;

	// After run, all flags should be true
	assert!(
		state_tracking_command.before_called.load(Ordering::SeqCst),
		"before_called should be true after run"
	);
	assert!(
		state_tracking_command.execute_called.load(Ordering::SeqCst),
		"execute_called should be true after run"
	);
	assert!(
		state_tracking_command.after_called.load(Ordering::SeqCst),
		"after_called should be true after run"
	);
}

/// Test command multiple executions
///
/// **Category**: State Transition
/// **Verifies**: Command can be executed multiple times
#[rstest]
#[tokio::test]
async fn test_command_multiple_executions(context: CommandContext) {
	let cmd = ConfigurableCommand::new(false, vec![]);

	// Execute multiple times
	for i in 1..=5 {
		let result = cmd.run(&context).await;
		assert!(result.is_ok(), "Run {} should succeed", i);
		assert_eq!(
			cmd.execution_count(),
			i,
			"Execution count should be {} after {} runs",
			i,
			i
		);
	}
}

// =============================================================================
// Decision Table Tests - System Check Combinations
// =============================================================================

/// Decision table test for requires_checks × skip_checks combinations
///
/// **Category**: Decision Table
/// **Verifies**: System check execution based on configuration
///
/// | requires_checks | skip_checks | Expected Behavior |
/// |-----------------|-------------|-------------------|
/// | false           | false       | Execute without checks |
/// | false           | true        | Execute without checks |
/// | true            | false       | Attempt checks, then execute |
/// | true            | true        | Skip checks, execute |
#[rstest]
#[case(false, false, true)] // No checks required, no skip -> execute
#[case(false, true, true)] // No checks required, skip set -> execute
#[case(true, true, true)] // Checks required but skipped -> execute
#[tokio::test]
#[serial(system_checks)]
async fn test_system_check_decision_table(
	#[case] requires_checks: bool,
	#[case] skip_checks: bool,
	#[case] expected_success: bool,
) {
	let cmd = ConfigurableCommand::new(requires_checks, vec![]);
	let mut ctx = CommandContext::new(vec![]);

	if skip_checks {
		ctx.set_option("skip_checks".to_string(), "true".to_string());
	}

	let result = cmd.run(&ctx).await;

	if expected_success {
		assert!(
			result.is_ok(),
			"Command should succeed with requires_checks={}, skip_checks={}",
			requires_checks,
			skip_checks
		);
		assert_eq!(cmd.execution_count(), 1, "Command should have executed");
	} else {
		assert!(
			result.is_err(),
			"Command should fail with requires_checks={}, skip_checks={}",
			requires_checks,
			skip_checks
		);
		assert_eq!(cmd.execution_count(), 0, "Command should not have executed");
	}
}

// =============================================================================
// Edge Case Tests
// =============================================================================

/// Test command with empty check_tags
///
/// **Category**: Edge Case
/// **Verifies**: Empty check_tags means run all checks
#[rstest]
#[tokio::test]
#[serial(system_checks)]
async fn test_command_empty_check_tags() {
	let cmd = ConfigurableCommand::new(false, vec![]);
	let ctx = CommandContext::new(vec![]);

	let result = cmd.run(&ctx).await;

	assert!(
		result.is_ok(),
		"Command with empty check_tags should succeed"
	);
	assert!(cmd.check_tags().is_empty(), "check_tags should be empty");
}

/// Test command with specific check_tags
///
/// **Category**: Edge Case
/// **Verifies**: Specific check_tags are correctly set
#[rstest]
fn test_command_specific_check_tags() {
	let tags = vec![
		"db".to_string(),
		"models".to_string(),
		"security".to_string(),
	];
	let cmd = ConfigurableCommand::new(true, tags.clone());

	let returned_tags = cmd.check_tags();
	assert_eq!(returned_tags.len(), 3, "Should have 3 check tags");
	assert_eq!(returned_tags, tags, "Check tags should match");
}

// =============================================================================
// Sanity Tests
// =============================================================================

/// Sanity test for basic command workflow
///
/// **Category**: Sanity
/// **Verifies**: Basic command creation and execution works
#[rstest]
#[tokio::test]
async fn test_command_basic_sanity(context: CommandContext) {
	let cmd = SimpleCommand;

	// Basic properties
	assert_eq!(cmd.name(), "simple");

	// Execute
	let result = cmd.run(&context).await;
	assert!(result.is_ok(), "Simple command should execute successfully");
}
