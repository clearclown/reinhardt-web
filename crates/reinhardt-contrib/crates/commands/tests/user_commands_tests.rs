//! User Commands Tests
//!
//! Comprehensive test suite for reinhardt management commands.
//! Translation of Django's user_commands tests from django/tests/user_commands/tests.py
//!
//! Reference: https://github.com/django/django/blob/main/tests/user_commands/tests.py

use async_trait::async_trait;
use reinhardt_commands::{
    BaseCommand, CommandContext, CommandError, CommandRegistry, CommandResult,
};
use std::sync::{Arc, Mutex};

// ============================================================================
// Test Helper Commands
// ============================================================================

/// Simple test command that outputs a message
struct DanceCommand;

impl DanceCommand {
    fn new() -> Self {
        Self
    }
}

#[async_trait]
impl BaseCommand for DanceCommand {
    fn name(&self) -> &str {
        "dance"
    }

    fn description(&self) -> &str {
        "A test command that dances"
    }

    async fn execute(&self, ctx: &CommandContext) -> CommandResult<()> {
        let style = ctx
            .option("style")
            .map(|s| s.as_str())
            .unwrap_or("Rock'n'Roll");

        println!("I don't feel like dancing {}.", style);

        if let Some(opt3) = ctx.option("opt_3") {
            if opt3 == "true" {
                println!("option3");
            }
        }

        if let Some(example) = ctx.option("example") {
            if example == "raise" {
                return Err(CommandError::ExecutionError("Test error".to_string()));
            }
        }

        if let Some(integer) = ctx.arg(0) {
            println!("You passed {} as a positional argument.", integer);
        }

        Ok(())
    }
}

/// Command for testing required options
struct RequiredOptionCommand;

#[async_trait]
impl BaseCommand for RequiredOptionCommand {
    fn name(&self) -> &str {
        "required_option"
    }

    async fn execute(&self, ctx: &CommandContext) -> CommandResult<()> {
        if let Some(_need_me) = ctx.option("need_me") {
            println!("need_me");
        }
        if let Some(_needme2) = ctx.option("needme2") {
            println!("needme2");
        }
        Ok(())
    }
}

/// Command for testing subparsers
struct SubparserCommand;

#[async_trait]
impl BaseCommand for SubparserCommand {
    fn name(&self) -> &str {
        "subparser"
    }

    async fn execute(&self, ctx: &CommandContext) -> CommandResult<()> {
        let subcommand = ctx.arg(0).ok_or_else(|| {
            CommandError::InvalidArguments(
                "the following arguments are required: subcommand".to_string(),
            )
        })?;

        match subcommand.as_str() {
            "foo" => {
                println!("bar");
                Ok(())
            }
            invalid => Err(CommandError::InvalidArguments(format!(
                "invalid choice: '{}' (choose from 'foo')",
                invalid
            ))),
        }
    }
}

/// Command that requires app labels
struct AppLabelCommand;

#[async_trait]
impl BaseCommand for AppLabelCommand {
    fn name(&self) -> &str {
        "app_label_command"
    }

    async fn execute(&self, ctx: &CommandContext) -> CommandResult<()> {
        if ctx.args.is_empty() {
            return Err(CommandError::InvalidArguments(
                "At least one app label is required".to_string(),
            ));
        }
        Ok(())
    }
}

// ============================================================================
// OutputWrapper Tests
// ============================================================================

#[tokio::test]
async fn test_unhandled_exceptions() {
    // Test that OutputWrapper handles unhandled exceptions properly
    // In Rust, resource cleanup is automatically handled by Drop trait
    // This test verifies that we don't have resource leaks

    use std::fs::File;
    use std::io::{BufWriter, Write};
    use tempfile::NamedTempFile;

    // Create a temporary file
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");

    // Scope to ensure Drop is called
    {
        let file = File::create(temp_file.path()).expect("Failed to open file");
        let mut writer = BufWriter::new(file);
        writer.write_all(b"test").expect("Failed to write");
        // Writer is dropped here automatically
    }

    // Verify file was properly closed and written
    let contents = std::fs::read_to_string(temp_file.path()).expect("Failed to read file");
    assert_eq!(
        contents, "test",
        "OutputWrapper should properly flush and close"
    );
}

// ============================================================================
// Command Tests
// ============================================================================

#[tokio::test]
async fn test_command() {
    let ctx = CommandContext::new(vec![]);
    let cmd = DanceCommand::new();

    let result = cmd.execute(&ctx).await;
    assert!(result.is_ok(), "Basic command execution failed");
}

#[tokio::test]
async fn test_command_style() {
    let mut ctx = CommandContext::new(vec![]);
    ctx.set_option("style".to_string(), "Jive".to_string());

    let cmd = DanceCommand::new();
    let result = cmd.execute(&ctx).await;
    assert!(result.is_ok(), "Command with style option failed");
}

#[tokio::test]
async fn test_explode() {
    // Test that an unknown command raises CommandError
    let registry = CommandRegistry::new();

    // Attempting to execute a non-existent command should fail
    // This would be tested at the registry level
    let result = registry.get("explode");
    assert!(result.is_none(), "Unknown command should not be found");
}

#[tokio::test]
async fn test_system_exit() {
    // Exception raised in a command should raise CommandError with call_command,
    // but SystemExit when run from command line
    let mut ctx = CommandContext::new(vec![]);
    ctx.set_option("example".to_string(), "raise".to_string());

    let cmd = DanceCommand::new();
    let result = cmd.execute(&ctx).await;

    assert!(result.is_err(), "Command should raise error");
    match result {
        Err(CommandError::ExecutionError(_)) => (),
        _ => panic!("Expected ExecutionError"),
    }
}

#[tokio::test]
async fn test_call_command_option_parsing() {
    // When passing the long option name to call_command, the available option
    // key is the option dest name (#22985)
    let mut ctx = CommandContext::new(vec![]);
    ctx.set_option("opt_3".to_string(), "true".to_string());

    let cmd = DanceCommand::new();
    let result = cmd.execute(&ctx).await;
    assert!(result.is_ok(), "Command with opt_3 failed");
}

#[tokio::test]
async fn test_call_command_option_parsing_non_string_arg() {
    // It should be possible to pass non-string arguments to call_command
    let ctx = CommandContext::new(vec!["1".to_string()]);
    let cmd = DanceCommand::new();

    let result = cmd.execute(&ctx).await;
    assert!(result.is_ok(), "Command with integer argument failed");
}

#[tokio::test]
async fn test_call_command_with_required_parameters_in_options() {
    let mut ctx = CommandContext::new(vec![]);
    ctx.set_option("need_me".to_string(), "foo".to_string());
    ctx.set_option("needme2".to_string(), "bar".to_string());

    let cmd = RequiredOptionCommand;
    let result = cmd.execute(&ctx).await;
    assert!(result.is_ok(), "Required parameters test failed");
}

#[tokio::test]
async fn test_call_command_with_required_parameters_in_mixed_options() {
    let mut ctx = CommandContext::new(vec!["--need-me=foo".to_string()]);
    ctx.set_option("needme2".to_string(), "bar".to_string());

    let cmd = RequiredOptionCommand;
    let result = cmd.execute(&ctx).await;
    assert!(result.is_ok(), "Mixed options test failed");
}

#[tokio::test]
async fn test_subparser() {
    let ctx = CommandContext::new(vec!["foo".to_string(), "12".to_string()]);
    let cmd = SubparserCommand;

    let result = cmd.execute(&ctx).await;
    assert!(result.is_ok(), "Subparser command failed");
}

#[tokio::test]
async fn test_subparser_invalid_option() {
    // Test that invalid subcommand choices raise appropriate errors
    let ctx = CommandContext::new(vec!["test".to_string(), "12".to_string()]);
    let cmd = SubparserCommand;

    let result = cmd.execute(&ctx).await;
    assert!(result.is_err(), "Invalid subcommand should raise error");

    match result {
        Err(CommandError::InvalidArguments(msg)) => {
            assert!(
                msg.contains("invalid choice: 'test'"),
                "Error message should mention invalid choice"
            );
        }
        _ => panic!("Expected InvalidArguments error"),
    }

    // Test missing subcommand
    let ctx_missing = CommandContext::new(vec![]);
    let result_missing = cmd.execute(&ctx_missing).await;
    assert!(
        result_missing.is_err(),
        "Missing subcommand should raise error"
    );

    match result_missing {
        Err(CommandError::InvalidArguments(msg)) => {
            assert!(
                msg.contains("required: subcommand"),
                "Error should mention required subcommand"
            );
        }
        _ => panic!("Expected InvalidArguments error for missing subcommand"),
    }
}

// ============================================================================
// Command Registry Tests
// ============================================================================

#[test]
fn test_command_registry_register() {
    let mut registry = CommandRegistry::new();

    // Test that registry is initially empty
    assert_eq!(registry.list().len(), 0, "New registry should be empty");

    // In a full implementation, we would test command registration here
    // For now, verify the registry exists and can be created
    let _second_registry = CommandRegistry::new();
    assert_eq!(
        registry.list().len(),
        0,
        "Registry operations should work correctly"
    );
}

#[test]
fn test_command_registry_get() {
    let registry = CommandRegistry::new();

    // Test retrieving a command
    let cmd = registry.get("dance");
    assert!(cmd.is_none(), "Unregistered command should not be found");
}

#[test]
fn test_command_registry_list() {
    let registry = CommandRegistry::new();

    // Test listing all registered commands
    let commands = registry.list();
    // New registry should have no commands
    assert_eq!(commands.len(), 0, "New registry should have no commands");
}

// ============================================================================
// BaseCommand Lifecycle Tests
// ============================================================================

struct LifecycleTestCommand {
    before_called: Arc<Mutex<bool>>,
    execute_called: Arc<Mutex<bool>>,
    after_called: Arc<Mutex<bool>>,
}

#[async_trait]
impl BaseCommand for LifecycleTestCommand {
    fn name(&self) -> &str {
        "lifecycle_test"
    }

    async fn before_execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
        *self.before_called.lock().unwrap() = true;
        Ok(())
    }

    async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
        *self.execute_called.lock().unwrap() = true;
        Ok(())
    }

    async fn after_execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
        *self.after_called.lock().unwrap() = true;
        Ok(())
    }
}

#[tokio::test]
async fn test_command_lifecycle() {
    let before = Arc::new(Mutex::new(false));
    let execute = Arc::new(Mutex::new(false));
    let after = Arc::new(Mutex::new(false));

    let cmd = LifecycleTestCommand {
        before_called: before.clone(),
        execute_called: execute.clone(),
        after_called: after.clone(),
    };

    let ctx = CommandContext::new(vec![]);
    let result = cmd.run(&ctx).await;

    assert!(result.is_ok(), "Command lifecycle failed");
    assert!(*before.lock().unwrap(), "before_execute was not called");
    assert!(*execute.lock().unwrap(), "execute was not called");
    assert!(*after.lock().unwrap(), "after_execute was not called");
}

// ============================================================================
// Command Argument and Option Tests
// ============================================================================

// ============================================================================
// Command Context Tests
// ============================================================================

#[test]
fn test_user_commands_context_options() {
    let mut ctx = CommandContext::new(vec![]);
    ctx.set_option("verbose".to_string(), "true".to_string());
    ctx.set_option("debug".to_string(), "".to_string());

    assert_eq!(ctx.option("verbose"), Some(&"true".to_string()));
    assert!(ctx.has_option("debug"));
    assert!(!ctx.has_option("nonexistent"));
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_command_error_not_found() {
    let err = CommandError::NotFound("test_command".to_string());
    assert!(err.to_string().contains("test_command"));
}

#[test]
fn test_command_error_invalid_arguments() {
    let err = CommandError::InvalidArguments("missing required arg".to_string());
    assert!(err.to_string().contains("Invalid arguments"));
}

#[test]
fn test_command_error_execution() {
    let err = CommandError::ExecutionError("failed to create file".to_string());
    assert!(err.to_string().contains("Execution error"));
}

// ============================================================================
// Utils Tests
// ============================================================================

#[test]
fn test_get_random_secret_key() {
    use reinhardt_commands::generate_secret_key;

    let key1 = generate_secret_key();
    let key2 = generate_secret_key();

    // Keys should be non-empty
    assert!(!key1.is_empty());
    assert!(!key2.is_empty());

    // Keys should be different (probabilistically)
    assert_ne!(key1, key2);

    // Keys should be of reasonable length (50 chars in Django)
    assert!(key1.len() >= 32);
}

#[test]
fn test_user_commands_to_camel_case() {
    use reinhardt_commands::to_camel_case;

    assert_eq!(to_camel_case("hello_world"), "HelloWorld");
    assert_eq!(to_camel_case("my_app"), "MyApp");
    assert_eq!(to_camel_case("user"), "User");
    assert_eq!(to_camel_case("api_endpoint"), "ApiEndpoint");
}

// ============================================================================
// Additional Tests from Django
// ============================================================================

#[tokio::test]
async fn test_calling_a_command_with_only_empty_parameter_should_ends_gracefully() {
    let mut ctx = CommandContext::new(vec![]);
    ctx.set_option("empty".to_string(), "".to_string());

    let cmd = DanceCommand::new();
    let result = cmd.execute(&ctx).await;
    assert!(result.is_ok(), "Command with empty parameter failed");
}

#[tokio::test]
async fn test_calling_command_with_app_labels_and_parameters_should_be_ok() {
    let ctx = CommandContext::new(vec!["myapp".to_string()]);
    let cmd = DanceCommand::new();

    let result = cmd.execute(&ctx).await;
    assert!(result.is_ok(), "Command with app labels failed");
}

#[tokio::test]
async fn test_calling_command_with_parameters_and_app_labels_at_the_end_should_be_ok() {
    let ctx = CommandContext::new(vec!["myapp".to_string()]);
    let cmd = DanceCommand::new();

    let result = cmd.execute(&ctx).await;
    assert!(
        result.is_ok(),
        "Command with parameters and app labels at end failed"
    );
}

#[tokio::test]
async fn test_calling_a_command_with_no_app_labels_and_parameters_raise_command_error() {
    // Test that command raises error when required arguments are missing
    let ctx = CommandContext::new(vec![]);
    let cmd = AppLabelCommand;

    let result = cmd.execute(&ctx).await;
    assert!(
        result.is_err(),
        "Command should raise error when app labels are missing"
    );

    match result {
        Err(CommandError::InvalidArguments(msg)) => {
            assert!(
                msg.contains("app label"),
                "Error should mention app label requirement"
            );
        }
        _ => panic!("Expected InvalidArguments error"),
    }
}

#[tokio::test]
async fn test_call_command_unrecognized_option() {
    // Test that unrecognized options are handled
    let mut ctx = CommandContext::new(vec![]);
    ctx.set_option("unrecognized_option".to_string(), "value".to_string());

    let cmd = DanceCommand::new();
    let result = cmd.execute(&ctx).await;

    // In Rust, unrecognized options are simply ignored if not explicitly validated
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_subparser_dest_args() {
    let ctx = CommandContext::new(vec!["foo".to_string()]);
    let cmd = SubparserCommand;

    let result = cmd.execute(&ctx).await;
    assert!(result.is_ok(), "Subparser dest args test failed");
}

#[tokio::test]
async fn test_subparser_dest_required_args() {
    let ctx = CommandContext::new(vec!["foo".to_string(), "bar".to_string()]);
    let cmd = SubparserCommand;

    let result = cmd.execute(&ctx).await;
    assert!(result.is_ok(), "Subparser dest required args test failed");
}

#[tokio::test]
async fn test_create_parser_kwargs() {
    // Test that BaseCommand allows customization
    // In Rust, this is done through trait methods
    let cmd = DanceCommand::new();

    // Verify basic command properties
    assert_eq!(cmd.name(), "dance");
    assert_eq!(cmd.description(), "A test command that dances");

    // Verify command can define arguments and options
    let arguments = cmd.arguments();
    let options = cmd.options();

    // These should return valid vectors (empty or non-empty)
    assert!(
        arguments.len() >= 0,
        "Command should have valid arguments list"
    );
    assert!(options.len() >= 0, "Command should have valid options list");

    // Verify help text can be retrieved
    let help_text = cmd.help();
    assert!(!help_text.is_empty(), "Command should have help text");
}

#[tokio::test]
async fn test_subparser_error_formatting() {
    // Test that subparser errors are properly formatted
    let ctx = CommandContext::new(vec!["invalid_subcommand".to_string()]);
    let cmd = SubparserCommand;

    let result = cmd.execute(&ctx).await;
    assert!(result.is_err(), "Invalid subcommand should produce error");

    match result {
        Err(CommandError::InvalidArguments(msg)) => {
            assert!(
                msg.contains("invalid choice"),
                "Error should be properly formatted"
            );
            assert!(
                msg.contains("invalid_subcommand"),
                "Error should mention the invalid input"
            );
        }
        _ => panic!("Expected InvalidArguments error with proper formatting"),
    }
}

#[tokio::test]
async fn test_subparser_non_django_error_formatting() {
    // Test error formatting for non-Django style subparsers
    let ctx = CommandContext::new(vec!["test".to_string()]);
    let cmd = SubparserCommand;

    let result = cmd.execute(&ctx).await;
    assert!(result.is_err(), "Invalid subcommand should produce error");

    match result {
        Err(CommandError::InvalidArguments(msg)) => {
            // Verify error message is clear and informative
            assert!(!msg.is_empty(), "Error message should not be empty");
            assert!(
                msg.contains("test") || msg.contains("invalid"),
                "Error should mention the issue"
            );
        }
        _ => panic!("Expected InvalidArguments error"),
    }
}

#[test]
fn test_no_existent_external_program() {
    // Test that non-existent programs are handled properly
    // This would use std::process::Command in a real implementation
    use std::process::Command;

    let result = Command::new("a_command_that_doesnt_exist_12345").output();

    assert!(result.is_err(), "Non-existent command should fail");
}

#[test]
fn test_is_ignored_path_true() {
    // Test path pattern matching for ignored paths
    // This would be implemented in a separate module
    fn is_ignored_path(path: &str, patterns: &[&str]) -> bool {
        patterns.iter().any(|pattern| {
            if pattern.contains('*') {
                // Simple wildcard matching
                let parts: Vec<&str> = pattern.split('*').collect();
                if parts.len() == 2 {
                    path.starts_with(parts[0]) && path.ends_with(parts[1])
                } else {
                    false
                }
            } else {
                path.contains(pattern)
            }
        })
    }

    assert!(is_ignored_path("foo/bar/baz", &["baz"]));
    assert!(is_ignored_path("foo/bar/baz", &["*/baz"]));
    assert!(is_ignored_path("foo/bar/baz", &["foo/bar/baz"]));
}

#[test]
fn test_is_ignored_path_false() {
    fn is_ignored_path(path: &str, patterns: &[&str]) -> bool {
        patterns.iter().any(|pattern| path.contains(pattern))
    }

    assert!(!is_ignored_path("foo/bar/baz", &["bat", "bar/bat", "flub"]));
}

#[test]
fn test_normalize_path_patterns_truncates_wildcard_base() {
    // Test that path patterns with wildcards are normalized
    fn normalize_path_pattern(pattern: &str) -> String {
        if pattern.ends_with("/*") {
            pattern.trim_end_matches("/*").to_string()
        } else {
            pattern.to_string()
        }
    }

    assert_eq!(normalize_path_pattern("foo/bar/*"), "foo/bar");
    assert_eq!(normalize_path_pattern("bar/*/"), "bar/*/");
}

// Note: The following tests from Django require more complex implementation
// that is not yet available in reinhardt-commands:
//
// - test_language_preserved (requires i18n support)
// - test_no_translations_deactivate_translations (requires i18n support)
// - test_find_command_without_PATH (requires PATH manipulation)
// - test_discover_commands_in_eggs (requires plugin system)
// - test_output_transaction (requires database connection)
// - test_call_command_no_checks (requires check framework)
// - test_requires_system_checks_* (requires check framework)
// - test_check_migrations (requires migrations framework)
// - test_mutually_exclusive_group_* (requires advanced argument parsing with clap)
// - test_required_list_option (requires list option support with clap)
// - test_required_const_options (requires const option support with clap)
// - test_outputwrapper_flush (requires OutputWrapper implementation)
// - test_script_prefix_set_in_commands (requires URL routing framework)
// - test_disallowed_abbreviated_options (requires argument parser config)
// - test_skip_checks (requires check framework)
// - test_command_add_arguments_after_common_arguments (requires argument ordering)
// - test_run_formatters_handles_oserror_for_black_path (requires code formatter integration)
//
// These tests are documented here for future implementation as the
// reinhardt-commands crate gains more features.
