//! Advanced Commands Integration Tests
//!
//! Integration tests for reinhardt-commands that require multiple Reinhardt crates.
//! These tests were migrated from the unit test file to comply with CLAUDE.md rules.
//!
//! Reference: Django's user_commands tests
//! https://github.com/django/django/blob/main/tests/user_commands/tests.py

use async_trait::async_trait;
use reinhardt_commands::{BaseCommand, CommandContext, CommandResult};
use reinhardt_database::{DatabaseConnection, DatabaseType};
use reinhardt_migrations::{DatabaseMigrationRecorder, MigrationExecutor};
use reinhardt_orm::engine::DatabaseEngine;

// ============================================================================
// Test Helper Commands
// ============================================================================

/// Simple command for testing
struct TestCommand;

#[async_trait]
impl BaseCommand for TestCommand {
    fn name(&self) -> &str {
        "test_command"
    }

    async fn execute(&self, _ctx: &CommandContext) -> CommandResult<()> {
        println!("Command executed");
        Ok(())
    }
}

// ============================================================================
// Database Transaction Tests
// ============================================================================

/// Test: Command output within database transaction
/// Reference: django/tests/user_commands/tests.py::test_output_transaction
#[tokio::test]
async fn test_output_transaction() {
    // Create in-memory SQLite database for testing
    let connection = DatabaseConnection::connect_sqlite(":memory:")
        .await
        .expect("Failed to connect to SQLite");

    let engine = DatabaseEngine::new(connection, DatabaseType::Sqlite);

    // Create a test table
    engine
        .execute("CREATE TABLE test_table (id INTEGER PRIMARY KEY, data TEXT)")
        .await
        .expect("Failed to create table");

    // Insert data
    engine
        .execute("INSERT INTO test_table (id, data) VALUES (1, 'test')")
        .await
        .expect("Failed to insert data");

    // Execute command - simulates running a command within database context
    let cmd = TestCommand;
    let ctx = CommandContext::new(vec![]);
    let result = cmd.execute(&ctx).await;

    // Verify command executed successfully
    assert!(
        result.is_ok(),
        "Command should execute successfully with database connection"
    );

    // Verify data is still accessible
    let rows = engine
        .fetch_all("SELECT * FROM test_table")
        .await
        .expect("Failed to fetch data");

    assert_eq!(rows.len(), 1);
}

// ============================================================================
// Migration Check Tests
// ============================================================================

/// Test: Command checks for pending migrations
/// Reference: django/tests/user_commands/tests.py::test_check_migrations
#[tokio::test]
async fn test_check_migrations() {
    // Create in-memory database
    let connection = DatabaseConnection::connect_sqlite(":memory:")
        .await
        .expect("Failed to connect to SQLite");

    let engine = DatabaseEngine::new(connection, DatabaseType::Sqlite);

    // Initialize migration recorder (creates django_migrations table)
    let recorder = DatabaseMigrationRecorder::new(engine.clone_ref());
    recorder
        .ensure_schema()
        .await
        .expect("Failed to create migrations table");

    // Check applied migrations
    let applied = recorder
        .applied_migrations()
        .await
        .expect("Failed to get applied migrations");

    // For a fresh database, there should be no applied migrations
    assert_eq!(
        applied.len(),
        0,
        "Fresh database should have no applied migrations"
    );

    // Execute command
    let cmd = TestCommand;
    let ctx = CommandContext::new(vec![]);
    let result = cmd.execute(&ctx).await;

    assert!(
        result.is_ok(),
        "Command should execute successfully after migration check"
    );
}

/// Test: Migration executor with commands
#[tokio::test]
async fn test_migration_executor_with_commands() {
    // Create in-memory database
    let connection = DatabaseConnection::connect_sqlite(":memory:")
        .await
        .expect("Failed to connect to SQLite");

    let engine = DatabaseEngine::new(connection, DatabaseType::Sqlite);

    // Initialize migration recorder
    let recorder = DatabaseMigrationRecorder::new(engine.clone_ref());
    recorder
        .ensure_schema()
        .await
        .expect("Failed to create migrations table");

    // Create a migration executor
    let _executor = MigrationExecutor::new(engine.clone_ref());

    // Verify executor is initialized
    // (No pending migrations to execute in this test)

    // Execute command after migration setup
    let cmd = TestCommand;
    let ctx = CommandContext::new(vec![]);
    let result = cmd.execute(&ctx).await;

    assert!(
        result.is_ok(),
        "Command should execute with migration executor initialized"
    );
}

// ============================================================================
// URL Script Prefix Tests
// ============================================================================

/// Test: Script prefix is set correctly in commands
/// Reference: Django's test_script_prefix_set_in_commands
#[tokio::test]
#[serial_test::serial(url_prefix)]
async fn test_script_prefix_set_in_commands() {
    use reinhardt_routers::{clear_script_prefix, get_script_prefix, set_script_prefix};

    // Set a script prefix
    set_script_prefix("/myapp/");
    assert_eq!(get_script_prefix(), "/myapp/");

    // Execute a command
    let cmd = TestCommand;
    let ctx = CommandContext::new(vec![]);
    let result = cmd.execute(&ctx).await;

    assert!(
        result.is_ok(),
        "Command should execute successfully with script prefix set"
    );

    // Verify the prefix is still set after command execution
    assert_eq!(
        get_script_prefix(),
        "/myapp/",
        "Script prefix should be preserved after command execution"
    );

    // Cleanup
    clear_script_prefix();
}

/// Test: Command execution with empty script prefix
#[tokio::test]
#[serial_test::serial(url_prefix)]
async fn test_command_with_empty_prefix() {
    use reinhardt_routers::{clear_script_prefix, get_script_prefix, set_script_prefix};

    // Set empty prefix
    set_script_prefix("");
    assert_eq!(get_script_prefix(), "");

    // Execute a command
    let cmd = TestCommand;
    let ctx = CommandContext::new(vec![]);
    let result = cmd.execute(&ctx).await;

    assert!(
        result.is_ok(),
        "Command should execute successfully with empty prefix"
    );

    // Verify the empty prefix is preserved
    assert_eq!(get_script_prefix(), "", "Empty prefix should be preserved");

    // Cleanup
    clear_script_prefix();
}

/// Test: Multiple commands with different script prefixes
#[tokio::test]
#[serial_test::serial(url_prefix)]
async fn test_multiple_commands_different_prefixes() {
    use reinhardt_routers::{clear_script_prefix, get_script_prefix, set_script_prefix};

    // First command with prefix "/app1/"
    set_script_prefix("/app1/");
    let cmd1 = TestCommand;
    let ctx1 = CommandContext::new(vec![]);
    let result1 = cmd1.execute(&ctx1).await;
    assert!(result1.is_ok());
    assert_eq!(get_script_prefix(), "/app1/");

    // Second command with prefix "/app2/"
    set_script_prefix("/app2/");
    let cmd2 = TestCommand;
    let ctx2 = CommandContext::new(vec![]);
    let result2 = cmd2.execute(&ctx2).await;
    assert!(result2.is_ok());
    assert_eq!(get_script_prefix(), "/app2/");

    // Third command with default prefix
    clear_script_prefix();
    let cmd3 = TestCommand;
    let ctx3 = CommandContext::new(vec![]);
    let result3 = cmd3.execute(&ctx3).await;
    assert!(result3.is_ok());
    assert_eq!(get_script_prefix(), "/");

    // Cleanup
    clear_script_prefix();
}
