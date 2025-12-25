//! Migration Squashing Integration Tests
//!
//! Tests that verify the migration squashing (compression) feature. Squashing combines
//! multiple migrations into a single migration to reduce the number of migration files
//! and improve performance when applying migrations from scratch.
//!
//! **Test Coverage:**
//! - Squashing multiple migrations into one
//! - Preserving dependencies after squashing
//! - replaces attribute verification
//! - Skipping already-applied migrations (--fake-initial equivalent)
//! - Detecting non-squashable operations (RunSQL, RunCode)
//!
//! **Fixtures Used:**
//! - postgres_container: PostgreSQL database container
//!
//! **Key Concepts:**
//! - **Squashing**: Combining multiple migrations into one optimized migration
//! - **replaces**: Attribute listing which migrations the squashed migration replaces
//! - **--fake-initial**: Django feature to skip migrations if tables already exist
//! - **Non-squashable**: Operations that can't be optimized (e.g., RunSQL with data changes)
//!
//! **Django Equivalent**: `python manage.py squashmigrations app 0001 0010`
//!
//! **Note**: Most tests are marked as `#[ignore]` because squashing is not yet
//! implemented in reinhardt-db. These serve as specifications for future implementation.

use reinhardt_backends::types::DatabaseType;
use reinhardt_backends::DatabaseConnection;
use reinhardt_migrations::{
	executor::DatabaseMigrationExecutor, ColumnDefinition, FieldType, Migration, Operation,
};
use reinhardt_test::fixtures::postgres_container;
use rstest::*;
use sqlx::PgPool;
use std::sync::Arc;
use testcontainers::{ContainerAsync, GenericImage};

// ============================================================================
// Test Helper Functions
// ============================================================================

fn leak_str(s: impl Into<String>) -> &'static str {
	Box::leak(s.into().into_boxed_str())
}

/// Create a migration with dependencies
fn create_migration_with_deps(
	app: &'static str,
	name: &'static str,
	operations: Vec<Operation>,
	dependencies: Vec<(&'static str, &'static str)>,
	replaces: Vec<(&'static str, &'static str)>,
) -> Migration {
	Migration {
		app_label: app,
		name,
		operations,
		dependencies,
		replaces,
		atomic: true,
		initial: None,
	}
}

/// Create a basic column definition
fn create_basic_column(name: &'static str, type_def: FieldType) -> ColumnDefinition {
	ColumnDefinition {
		name,
		type_definition: type_def,
		not_null: false,
		unique: false,
		primary_key: false,
		auto_increment: false,
		default: None,
	}
}

/// Create an auto-increment primary key column
fn create_auto_pk_column(name: &'static str, type_def: FieldType) -> ColumnDefinition {
	ColumnDefinition {
		name,
		type_definition: type_def,
		not_null: true,
		unique: false,
		primary_key: true,
		auto_increment: true,
		default: None,
	}
}

// ============================================================================
// Normal Case Tests - Migration Squashing
// ============================================================================

/// Test squashing 3 migrations into 1
///
/// **Test Intent**: Verify that multiple sequential migrations can be combined
///
/// **Example**:
/// - Before: 0001_create (CREATE TABLE), 0002_add_col1 (ADD COLUMN), 0003_add_col2 (ADD COLUMN)
/// - After: 0001_squashed (CREATE TABLE with both columns)
///
/// **Note**: Currently marked as ignore - squashing not yet implemented
#[rstest]
#[ignore = "Migration squashing not yet implemented in reinhardt-db"]
#[tokio::test]
async fn test_squash_three_migrations(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	// TODO: Implement squashing functionality
	//
	// Original migrations:
	// let migration_1 = Migration {
	// 	app_label: "app",
	// 	name: "0001_create_users",
	// 	operations: vec![Operation::CreateTable {
	// 		name: leak_str("users"),
	// 		columns: vec![create_auto_pk_column("id", FieldType::Integer)],
	// 		constraints: vec![],
	// 	// 	}],
	// 	...
	// };
	//
	// let migration_2 = Migration {
	// 	app_label: "app",
	// 	name: "0002_add_name",
	// 	operations: vec![Operation::AddColumn {
	// 		table: leak_str("users"),
	// 		column: create_basic_column("name", FieldType::VarChar(100)),
	// 	}],
	// 	dependencies: vec![("app", "0001_create_users")],
	// 	...
	// };
	//
	// let migration_3 = Migration {
	// 	app_label: "app",
	// 	name: "0003_add_email",
	// 	operations: vec![Operation::AddColumn {
	// 		table: leak_str("users"),
	// 		column: create_basic_column("email", FieldType::VarChar(255)),
	// 	}],
	// 	dependencies: vec![("app", "0002_add_name")],
	// 	...
	// };
	//
	// Squashed migration:
	// let squashed = Migration {
	// 	app_label: "app",
	// 	name: "0001_squashed_0003",
	// 	operations: vec![Operation::CreateTable {
	// 		name: leak_str("users"),
	// 		columns: vec![
	// 			create_auto_pk_column("id", FieldType::Integer),
	// 			create_basic_column("name", FieldType::VarChar(100)),
	// 			create_basic_column("email", FieldType::VarChar(255)),
	// 		],
	// 		constraints: vec![],
	// 	// 	}],
	// 	replaces: vec![
	// 		("app", "0001_create_users"),
	// 		("app", "0002_add_name"),
	// 		("app", "0003_add_email"),
	// 	],
	// 	...
	// };
	//
	// Both approaches should produce identical schema:
	// 1. Apply migration_1, migration_2, migration_3 sequentially
	// 2. Apply squashed migration
	//
	// The squashed version is faster and cleaner
}

/// Test dependency preservation after squashing
///
/// **Test Intent**: Verify that external dependencies are preserved in squashed migration
///
/// **Example**:
/// - migration_1 depends on external app: ("other_app", "0001_initial")
/// - migration_2 adds column
/// - migration_3 adds another column
/// - Squashed migration must still depend on ("other_app", "0001_initial")
///
/// **Note**: Currently marked as ignore - squashing not yet implemented
#[rstest]
#[ignore = "Migration squashing not yet implemented in reinhardt-db"]
#[tokio::test]
async fn test_squashing_preserves_external_dependencies(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	// TODO: When squashing migrations, ensure:
	// 1. Internal dependencies (within squashed set) are removed
	// 2. External dependencies (to other apps or prior migrations) are preserved
	//
	// Example:
	// Original:
	// - 0001: depends on ("other_app", "0001_initial")
	// - 0002: depends on ("app", "0001") <- internal dependency
	// - 0003: depends on ("app", "0002") <- internal dependency
	//
	// Squashed:
	// - 0001_squashed_0003: depends on ("other_app", "0001_initial") <- preserved
	//
	// Internal dependencies are collapsed, external ones remain
}

/// Test replaces attribute verification
///
/// **Test Intent**: Verify that squashed migration correctly lists replaced migrations
///
/// **Note**: Currently marked as ignore - squashing not yet implemented
#[rstest]
#[ignore = "Migration squashing not yet implemented in reinhardt-db"]
#[tokio::test]
async fn test_replaces_attribute_verification(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	// TODO: The `replaces` attribute is critical for squashing
	//
	// let squashed = Migration {
	// 	app_label: "app",
	// 	name: "0001_squashed_0005",
	// 	operations: vec![...],
	// 	dependencies: vec![...],
	// 	replaces: vec![
	// 		("app", "0001_create_users"),
	// 		("app", "0002_add_name"),
	// 		("app", "0003_add_email"),
	// 		("app", "0004_add_phone"),
	// 		("app", "0005_add_address"),
	// 	],
	// 	...
	// };
	//
	// The migration executor uses `replaces` to:
	// 1. Skip applying individual migrations if squashed version is applied
	// 2. Track which migrations have been "virtually" applied
	// 3. Maintain migration history consistency
	//
	// When a new database is created:
	// - Apply squashed migration → marks all replaced migrations as applied
	//
	// When an existing database is migrated:
	// - If old migrations already applied, skip squashed version
	// - If squashed version applied, skip old migrations
}

// ============================================================================
// Edge Case Tests
// ============================================================================

/// Test --fake-initial equivalent (skip migrations if tables already exist)
///
/// **Test Intent**: Verify that migrations can be marked as applied without executing SQL
///
/// **Django Feature**: `python manage.py migrate --fake-initial`
/// - If initial migration creates tables that already exist, mark as applied without error
///
/// **Use Case**: Migrating legacy database to reinhardt-db migrations
///
/// **Note**: Currently marked as ignore - not yet implemented
#[rstest]
#[ignore = "--fake-initial equivalent not yet implemented in reinhardt-db"]
#[tokio::test]
async fn test_fake_initial_skip_existing_tables(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, url) = postgres_container.await;

	// Manually create a table (simulating legacy database)
	sqlx::query("CREATE TABLE users (id SERIAL PRIMARY KEY, name VARCHAR(100))")
		.execute(pool.as_ref())
		.await
		.expect("Failed to create legacy table");

	let connection = DatabaseConnection::connect_postgres(&url)
		.await
		.expect("Failed to connect to PostgreSQL");

	let mut executor =
		DatabaseMigrationExecutor::new(connection.inner().clone(), DatabaseType::Postgres);

	// Migration that creates the same table
	let migration = create_migration_with_deps(
		"app",
		"0001_initial",
		vec![Operation::CreateTable {
			name: leak_str("users"),
			columns: vec![
				create_auto_pk_column("id", FieldType::Integer),
				create_basic_column("name", FieldType::VarChar(100)),
			],
			constraints: vec![],
		}],
		vec![],
		vec![],
	);

	// TODO: Add fake_initial flag to executor
	// executor.set_fake_initial(true);
	//
	// With fake_initial enabled:
	// - Check if table exists
	// - If yes, mark migration as applied WITHOUT executing SQL
	// - If no, execute SQL normally
	//
	// This allows smooth integration of legacy databases

	// For now, applying this migration would fail with "relation already exists"
	let result = executor.apply_migrations(&[migration]).await;

	// Without fake_initial support, this should fail
	assert!(result.is_err(), "Should fail without fake_initial support");
}

/// Test detection of non-squashable operations
///
/// **Test Intent**: Verify that migrations with RunSQL are marked as non-squashable
///
/// **Rationale**: Some operations can't be safely squashed:
/// - RunSQL with data modifications (can't optimize)
/// - RunCode (Rust closures can't be merged)
/// - Operations with state-dependent logic
///
/// **Note**: Currently marked as ignore - squashing not yet implemented
#[rstest]
#[ignore = "Migration squashing not yet implemented in reinhardt-db"]
#[tokio::test]
async fn test_detect_non_squashable_operations(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	// TODO: Implement squashing with safety checks
	//
	// let migration_1 = Migration {
	// 	app_label: "app",
	// 	name: "0001_create_users",
	// 	operations: vec![Operation::CreateTable { ... }],
	// 	...
	// };
	//
	// let migration_2 = Migration {
	// 	app_label: "app",
	// 	name: "0002_populate_data",
	// 	operations: vec![Operation::RunSQL {
	// 		sql: leak_str("INSERT INTO users (name) VALUES ('Admin')"),
	// 		reverse_sql: None,
	// 	}],
	// 	...
	// };
	//
	// let migration_3 = Migration {
	// 	app_label: "app",
	// 	name: "0003_add_email",
	// 	operations: vec![Operation::AddColumn { ... }],
	// 	...
	// };
	//
	// When trying to squash these:
	// - Squashing 0001 + 0002 + 0003 should:
	//   1. Warn that 0002 contains RunSQL (non-optimizable)
	//   2. Either:
	//      a. Keep RunSQL as separate operation in squashed migration
	//      b. Refuse to squash (safer option)
	//
	// Safe squashing:
	// - CreateTable + AddColumn + AddColumn → CreateTable (with all columns)
	// - AddConstraint + DropConstraint → (cancel out)
	//
	// Unsafe squashing:
	// - RunSQL can't be optimized (data-dependent)
	// - RunCode can't be merged (closure semantics)
}
