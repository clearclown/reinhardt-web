//! Database Integration Tests
//!
//! Compilation and execution control:
//! - Cargo.toml: [[test]] name = "database_tests" required-features = ["with-reinhardt"]
//! - build.rs: Sets 'with-reinhardt' feature when reinhardt is available
//! - When feature is disabled, this entire test file is excluded from compilation
//!
//! TestContainers is used to automatically start PostgreSQL for testing.

use example_test_macros::example_test;
use reinhardt::prelude::*;
use std::env;
use testcontainers::{clients::Cli, core::WaitFor, GenericImage, RunnableImage};

/// Database connection test using TestContainers
#[example_test(version = "^0.1")]
async fn test_database_connection() {
	let docker = Cli::default();
	let postgres_image = RunnableImage::from(
		GenericImage::new("postgres", "16-alpine")
			.with_wait_for(WaitFor::message_on_stderr("database system is ready"))
			.with_env_var("POSTGRES_PASSWORD", "test")
			.with_env_var("POSTGRES_DB", "testdb"),
	);

	let container = docker.run(postgres_image);
	let port = container.get_host_port_ipv4(5432);
	let database_url = format!("postgres://postgres:test@localhost:{}/testdb", port);

	assert!(database_url.contains("postgres"), "Should use PostgreSQL");

	// Actual connection test (using reinhardt's API)
	let db = reinhardt::Database::connect(&database_url).await;
	assert!(db.is_ok(), "Failed to connect to database");

	println!("✅ Database connection successful");
}

/// Table existence test using TestContainers
#[example_test(version = "^0.1")]
async fn test_users_table_exists() {
	let (container, db) = setup_test_database().await;

	// Create users table
	let create_table_sql = r#"
		CREATE TABLE IF NOT EXISTS users (
			id SERIAL PRIMARY KEY,
			name VARCHAR(255) NOT NULL,
			email VARCHAR(255) NOT NULL
		)
	"#;
	db.execute(create_table_sql, &[]).await.expect("Failed to create users table");

	// Check if users table exists
	let result = db.query("SELECT * FROM users LIMIT 1").await;
	assert!(result.is_ok(), "users table should exist");

	println!("✅ Users table exists");
	drop(container); // Cleanup
}

/// CRUD operations test using TestContainers
#[example_test(version = ">=0.1.0, <0.2.0")]
async fn test_crud_operations() {
	let (container, db) = setup_test_database().await;

	// Create users table
	let create_table_sql = r#"
		CREATE TABLE IF NOT EXISTS users (
			id SERIAL PRIMARY KEY,
			name VARCHAR(255) NOT NULL,
			email VARCHAR(255) NOT NULL
		)
	"#;
	db.execute(create_table_sql, &[]).await.expect("Failed to create users table");

	// Create
	let user = User {
		id: None,
		name: "Test User".into(),
		email: "test@example.com".into(),
	};
	let created = db.insert(&user).await;
	assert!(created.is_ok(), "Failed to create user");

	// Read
	let users = db
		.query(
			"SELECT * FROM users WHERE email = $1",
			&["test@example.com"],
		)
		.await;
	assert!(users.is_ok(), "Failed to read users");

	// Update
	let updated = db
		.execute(
			"UPDATE users SET name = $1 WHERE email = $2",
			&["Updated User", "test@example.com"],
		)
		.await;
	assert!(updated.is_ok(), "Failed to update user");

	// Delete
	let deleted = db
		.execute("DELETE FROM users WHERE email = $1", &["test@example.com"])
		.await;
	assert!(deleted.is_ok(), "Failed to delete user");

	println!("✅ CRUD operations successful");
	drop(container); // Cleanup
}

async fn setup_test_database() -> (testcontainers::Container<'static, GenericImage>, reinhardt::Database) {
	let docker = Cli::default();
	let postgres_image = RunnableImage::from(
		GenericImage::new("postgres", "16-alpine")
			.with_wait_for(WaitFor::message_on_stderr("database system is ready"))
			.with_env_var("POSTGRES_PASSWORD", "test")
			.with_env_var("POSTGRES_DB", "testdb"),
	);

	let container = docker.run(postgres_image);
	let port = container.get_host_port_ipv4(5432);
	let database_url = format!("postgres://postgres:test@localhost:{}/testdb", port);

	let db = reinhardt::Database::connect(&database_url)
		.await
		.expect("Failed to connect to test database");

	(container, db)
}

#[derive(Debug)]
struct User {
	id: Option<i64>,
	name: String,
	email: String,
}
