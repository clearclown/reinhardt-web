//! Database Integration Tests
//!
//! Compilation and execution control:
//! - Cargo.toml: [[test]] name = "database_tests" required-features = ["with-reinhardt"]
//! - build.rs: Sets 'with-reinhardt' feature when reinhardt is available
//! - When feature is disabled, this entire test file is excluded from compilation
//!
//! Uses standard fixtures from reinhardt-test for TestContainers management.

use example_test_macros::example_test;
use reinhardt_test::fixtures::postgres_container;
use rstest::*;
use sqlx::{PgPool, Row};
use std::sync::Arc;
use testcontainers::{ContainerAsync, GenericImage};

// ============================================================================
// Basic Database Connection Tests
// ============================================================================

/// Test basic database connection using standard fixture
#[rstest]
#[tokio::test]
#[example_test("*")]
async fn test_database_connection(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, port, database_url) = postgres_container.await;

	// Verify connection URL format
	assert!(database_url.contains("postgres"), "Should use PostgreSQL");
	assert!(database_url.contains(&port.to_string()), "URL should contain port");

	// Verify pool connection
	let result = sqlx::query("SELECT 1").fetch_one(pool.as_ref()).await;
	assert!(result.is_ok(), "Failed to execute simple query");

	println!("✅ Database connection successful (port: {})", port);
}

/// Test that database is ready to accept connections
#[rstest]
#[tokio::test]
#[example_test("*")]
async fn test_database_ready(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _database_url) = postgres_container.await;

	// Check database version
	let row: (String,) = sqlx::query_as("SELECT version()")
		.fetch_one(pool.as_ref())
		.await
		.expect("Failed to query database version");

	assert!(row.0.contains("PostgreSQL"), "Should be PostgreSQL database");

	println!("✅ Database ready: {}", row.0);
}

// ============================================================================
// Table Creation and Schema Tests
// ============================================================================

/// Test creating and verifying users table
#[rstest]
#[tokio::test]
#[example_test("*")]
async fn test_users_table_creation(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _database_url) = postgres_container.await;

	// Create users table
	let create_table_sql = r#"
		CREATE TABLE IF NOT EXISTS users (
			id SERIAL PRIMARY KEY,
			name VARCHAR(255) NOT NULL,
			email VARCHAR(255) NOT NULL UNIQUE,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	"#;

	sqlx::query(create_table_sql)
		.execute(pool.as_ref())
		.await
		.expect("Failed to create users table");

	// Verify table exists by querying it
	let result = sqlx::query("SELECT * FROM users LIMIT 1")
		.fetch_optional(pool.as_ref())
		.await;
	assert!(result.is_ok(), "users table should exist");

	// Verify table structure
	let columns: Vec<(String,)> = sqlx::query_as(
		r#"
		SELECT column_name
		FROM information_schema.columns
		WHERE table_name = 'users'
		ORDER BY ordinal_position
	"#,
	)
	.fetch_all(pool.as_ref())
	.await
	.expect("Failed to query table structure");

	let column_names: Vec<String> = columns.iter().map(|(name,)| name.clone()).collect();
	assert_eq!(column_names, vec!["id", "name", "email", "created_at"]);

	println!("✅ Users table created with correct schema");
}

// ============================================================================
// CRUD Operations Tests
// ============================================================================

/// Test CREATE operation - inserting a user
#[rstest]
#[tokio::test]
#[example_test("*")]
async fn test_create_user(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _database_url) = postgres_container.await;

	// Create users table
	setup_users_table(pool.as_ref()).await;

	// Insert a user
	let result = sqlx::query(
		r#"
		INSERT INTO users (name, email)
		VALUES ($1, $2)
		RETURNING id, name, email
	"#,
	)
	.bind("Alice Smith")
	.bind("alice@example.com")
	.fetch_one(pool.as_ref())
	.await;

	assert!(result.is_ok(), "Failed to insert user");

	let row = result.unwrap();
	let id: i32 = row.get("id");
	let name: String = row.get("name");
	let email: String = row.get("email");

	assert!(id > 0, "User ID should be positive");
	assert_eq!(name, "Alice Smith");
	assert_eq!(email, "alice@example.com");

	println!("✅ User created successfully (id: {})", id);
}

/// Test READ operation - querying users
#[rstest]
#[tokio::test]
#[example_test("*")]
async fn test_read_users(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _database_url) = postgres_container.await;

	// Setup table and insert test data
	setup_users_table(pool.as_ref()).await;
	insert_test_users(pool.as_ref()).await;

	// Query all users
	let users: Vec<(i32, String, String)> = sqlx::query_as(
		r#"
		SELECT id, name, email
		FROM users
		ORDER BY id
	"#,
	)
	.fetch_all(pool.as_ref())
	.await
	.expect("Failed to query users");

	assert_eq!(users.len(), 3, "Should have 3 test users");
	assert_eq!(users[0].1, "Alice");
	assert_eq!(users[1].1, "Bob");
	assert_eq!(users[2].1, "Charlie");

	println!("✅ Read {} users successfully", users.len());
}

/// Test UPDATE operation - modifying user data
#[rstest]
#[tokio::test]
#[example_test("*")]
async fn test_update_user(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _database_url) = postgres_container.await;

	// Setup table and insert test data
	setup_users_table(pool.as_ref()).await;
	insert_test_users(pool.as_ref()).await;

	// Update user's name
	let result = sqlx::query(
		r#"
		UPDATE users
		SET name = $1
		WHERE email = $2
	"#,
	)
	.bind("Alice Updated")
	.bind("alice@example.com")
	.execute(pool.as_ref())
	.await;

	assert!(result.is_ok(), "Failed to update user");
	assert_eq!(result.unwrap().rows_affected(), 1, "Should update 1 row");

	// Verify update
	let row: (String,) = sqlx::query_as("SELECT name FROM users WHERE email = $1")
		.bind("alice@example.com")
		.fetch_one(pool.as_ref())
		.await
		.expect("Failed to fetch updated user");

	assert_eq!(row.0, "Alice Updated");

	println!("✅ User updated successfully");
}

/// Test DELETE operation - removing user
#[rstest]
#[tokio::test]
#[example_test("*")]
async fn test_delete_user(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _database_url) = postgres_container.await;

	// Setup table and insert test data
	setup_users_table(pool.as_ref()).await;
	insert_test_users(pool.as_ref()).await;

	// Delete user
	let result = sqlx::query("DELETE FROM users WHERE email = $1")
		.bind("alice@example.com")
		.execute(pool.as_ref())
		.await;

	assert!(result.is_ok(), "Failed to delete user");
	assert_eq!(result.unwrap().rows_affected(), 1, "Should delete 1 row");

	// Verify deletion
	let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
		.fetch_one(pool.as_ref())
		.await
		.expect("Failed to count users");

	assert_eq!(count.0, 2, "Should have 2 users left");

	println!("✅ User deleted successfully");
}

// ============================================================================
// Transaction Tests
// ============================================================================

/// Test transaction commit
#[rstest]
#[tokio::test]
#[example_test("*")]
async fn test_transaction_commit(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _database_url) = postgres_container.await;

	setup_users_table(pool.as_ref()).await;

	// Start transaction
	let mut tx = pool.begin().await.expect("Failed to start transaction");

	// Insert users in transaction
	sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
		.bind("Transaction User 1")
		.bind("tx1@example.com")
		.execute(&mut *tx)
		.await
		.expect("Failed to insert in transaction");

	sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
		.bind("Transaction User 2")
		.bind("tx2@example.com")
		.execute(&mut *tx)
		.await
		.expect("Failed to insert in transaction");

	// Commit transaction
	tx.commit().await.expect("Failed to commit transaction");

	// Verify both users exist
	let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
		.fetch_one(pool.as_ref())
		.await
		.expect("Failed to count users");

	assert_eq!(count.0, 2, "Both users should be committed");

	println!("✅ Transaction committed successfully");
}

/// Test transaction rollback
#[rstest]
#[tokio::test]
#[example_test("*")]
async fn test_transaction_rollback(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _database_url) = postgres_container.await;

	setup_users_table(pool.as_ref()).await;

	// Start transaction
	let mut tx = pool.begin().await.expect("Failed to start transaction");

	// Insert users in transaction
	sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
		.bind("Rollback User 1")
		.bind("rollback1@example.com")
		.execute(&mut *tx)
		.await
		.expect("Failed to insert in transaction");

	// Rollback transaction
	tx.rollback().await.expect("Failed to rollback transaction");

	// Verify no users exist
	let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
		.fetch_one(pool.as_ref())
		.await
		.expect("Failed to count users");

	assert_eq!(count.0, 0, "No users should exist after rollback");

	println!("✅ Transaction rolled back successfully");
}

// ============================================================================
// Helper Functions
// ============================================================================

async fn setup_users_table(pool: &PgPool) {
	let create_table_sql = r#"
		CREATE TABLE IF NOT EXISTS users (
			id SERIAL PRIMARY KEY,
			name VARCHAR(255) NOT NULL,
			email VARCHAR(255) NOT NULL UNIQUE,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
	"#;

	sqlx::query(create_table_sql)
		.execute(pool)
		.await
		.expect("Failed to create users table");
}

async fn insert_test_users(pool: &PgPool) {
	let users = vec![
		("Alice", "alice@example.com"),
		("Bob", "bob@example.com"),
		("Charlie", "charlie@example.com"),
	];

	for (name, email) in users {
		sqlx::query("INSERT INTO users (name, email) VALUES ($1, $2)")
			.bind(name)
			.bind(email)
			.execute(pool)
			.await
			.expect(&format!("Failed to insert user {}", name));
	}
}
