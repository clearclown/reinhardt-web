#![cfg(all(feature = "templates", feature = "database"))]

use reinhardt_shortcuts::get_object_or_404;
use reinhardt_shortcuts::template_inheritance::render_string_with_inheritance;
use reinhardt_test::fixtures::testcontainers::postgres_container;
use rstest::*;
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, prelude::FromRow};
use std::collections::HashMap;
use std::sync::Arc;
use testcontainers::{ContainerAsync, GenericImage};

/// Test model for user data
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
struct User {
	id: i64,
	username: String,
	email: String,
	is_active: bool,
}

impl reinhardt_db::prelude::Model for User {
	type PrimaryKey = i64;

	fn table_name() -> &'static str {
		"users"
	}

	fn primary_key_field() -> &'static str {
		"id"
	}

	fn primary_key(&self) -> Option<&Self::PrimaryKey> {
		Some(&self.id)
	}

	fn set_primary_key(&mut self, pk: Self::PrimaryKey) {
		self.id = pk;
	}
}

/// Fixture to setup test database with users table and sample data
#[fixture]
async fn test_database(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) -> (ContainerAsync<GenericImage>, String, Arc<PgPool>) {
	let (container, pool, _port, url) = postgres_container.await;

	// Create users table
	sqlx::query(
		r#"
		CREATE TABLE users (
			id BIGSERIAL PRIMARY KEY,
			username VARCHAR(255) NOT NULL UNIQUE,
			email VARCHAR(255) NOT NULL,
			is_active BOOLEAN NOT NULL DEFAULT true
		)
		"#,
	)
	.execute(pool.as_ref())
	.await
	.expect("Failed to create users table");

	// Insert test data
	sqlx::query(
		r#"
		INSERT INTO users (username, email, is_active)
		VALUES
			('alice', 'alice@example.com', true),
			('bob', 'bob@example.com', false),
			('charlie', 'charlie@example.com', true)
		"#,
	)
	.execute(pool.as_ref())
	.await
	.expect("Failed to insert test data");

	(container, url, pool)
}

/// Test: get_object_or_404 retrieves existing user
#[rstest]
#[tokio::test]
async fn test_get_object_or_404_success(
	#[future] test_database: (ContainerAsync<GenericImage>, String, Arc<PgPool>),
) {
	let (_container, url, _pool) = test_database.await;

	// Setup database connection for ORM
	reinhardt_db::orm::manager::init_database(&url)
		.await
		.unwrap();

	// Query user with id=1 (alice)
	let result = get_object_or_404::<User>(1).await;

	assert!(result.is_ok(), "Expected Ok but got Err");
	let user = result.unwrap();
	assert_eq!(user.id, 1);
	assert_eq!(user.username, "alice");
	assert_eq!(user.email, "alice@example.com");
	assert!(user.is_active);
}

/// Test: get_object_or_404 returns 404 for non-existent user
#[rstest]
#[tokio::test]
async fn test_get_object_or_404_not_found(
	#[future] test_database: (ContainerAsync<GenericImage>, String, Arc<PgPool>),
) {
	let (_container, url, _pool) = test_database.await;

	// Setup database connection for ORM
	reinhardt_db::orm::manager::init_database(&url)
		.await
		.unwrap();

	// Query non-existent user with id=999
	let result = get_object_or_404::<User>(999).await;

	assert!(result.is_err(), "Expected Err(404) but got Ok");
	let response = result.unwrap_err();
	assert_eq!(
		response.status,
		hyper::StatusCode::NOT_FOUND,
		"Expected 404 status code"
	);
}

/// Test: Template rendering with model instance from database
#[rstest]
#[tokio::test]
async fn test_template_with_model_instance(
	#[future] test_database: (ContainerAsync<GenericImage>, String, Arc<PgPool>),
) {
	let (_container, url, _pool) = test_database.await;

	// Setup database connection for ORM
	reinhardt_db::orm::manager::init_database(&url)
		.await
		.unwrap();

	// Retrieve user from database
	let user_result = get_object_or_404::<User>(1).await;
	assert!(user_result.is_ok());
	let user = user_result.unwrap();

	// Prepare template context with user data
	let mut context = HashMap::new();
	context.insert("user", serde_json::to_value(&user).unwrap());

	// Render template with user data
	let template = r#"
		<div class="user-profile">
			<h1>{{ user.username }}</h1>
			<p>Email: {{ user.email }}</p>
			{% if user.is_active %}
				<span class="badge">Active</span>
			{% else %}
				<span class="badge">Inactive</span>
			{% endif %}
		</div>
	"#;

	let result = render_string_with_inheritance(template, &context);
	assert!(result.is_ok(), "Template rendering failed");

	let rendered = result.unwrap();
	assert!(rendered.contains("<h1>alice</h1>"));
	assert!(rendered.contains("Email: alice@example.com"));
	assert!(rendered.contains("Active"));
	assert!(!rendered.contains("Inactive"));
}

/// Test: Template rendering with multiple model instances
#[rstest]
#[tokio::test]
async fn test_template_with_multiple_models(
	#[future] test_database: (ContainerAsync<GenericImage>, String, Arc<PgPool>),
) {
	let (_container, _url, pool) = test_database.await;

	// Retrieve all users from database
	let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY id")
		.fetch_all(pool.as_ref())
		.await
		.expect("Failed to fetch users");

	assert_eq!(users.len(), 3, "Expected 3 users in database");

	// Prepare template context with all users
	let mut context = HashMap::new();
	context.insert("users", serde_json::to_value(&users).unwrap());

	// Render template with user list
	let template = r#"
		<ul class="user-list">
		{% for user in users %}
			<li>
				{{ user.username }} ({{ user.email }})
				{% if user.is_active %}✓{% else %}✗{% endif %}
			</li>
		{% endfor %}
		</ul>
	"#;

	let result = render_string_with_inheritance(template, &context);
	assert!(result.is_ok(), "Template rendering failed");

	let rendered = result.unwrap();
	assert!(rendered.contains("alice (alice@example.com)"));
	assert!(rendered.contains("bob (bob@example.com)"));
	assert!(rendered.contains("charlie (charlie@example.com)"));

	// Check active/inactive markers
	let alice_index = rendered.find("alice").unwrap();
	let bob_index = rendered.find("bob").unwrap();
	let charlie_index = rendered.find("charlie").unwrap();

	// alice is active (✓)
	assert!(
		rendered[alice_index..].contains("✓"),
		"Alice should be marked active"
	);
	// bob is inactive (✗)
	assert!(
		rendered[bob_index..bob_index + 100].contains("✗"),
		"Bob should be marked inactive"
	);
	// charlie is active (✓)
	assert!(
		rendered[charlie_index..].contains("✓"),
		"Charlie should be marked active"
	);
}

/// Test: Database query error handling in template context
#[rstest]
#[tokio::test]
async fn test_database_error_handling(
	#[future] test_database: (ContainerAsync<GenericImage>, String, Arc<PgPool>),
) {
	let (_container, url, _pool) = test_database.await;

	// Setup database connection for ORM
	reinhardt_db::orm::manager::init_database(&url)
		.await
		.unwrap();

	// Query non-existent user
	let result = get_object_or_404::<User>(999).await;
	assert!(result.is_err());

	// Verify error response
	let response = result.unwrap_err();
	assert_eq!(response.status, hyper::StatusCode::NOT_FOUND);

	// Template should handle missing data gracefully
	let mut context = HashMap::new();
	context.insert("user", serde_json::Value::Null);

	let template = r#"
		{% if user %}
			<p>User: {{ user.username }}</p>
		{% else %}
			<p>No user found</p>
		{% endif %}
	"#;

	let rendered = render_string_with_inheritance(template, &context).unwrap();
	assert!(rendered.contains("No user found"));
	assert!(!rendered.contains("User:"));
}
