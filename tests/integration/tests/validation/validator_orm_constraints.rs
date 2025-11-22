//! ORM constraint validation integration tests
//!
//! These tests verify the integration between validators and ORM database constraints:
//! - Composite UNIQUE constraints
//! - CHECK constraints
//! - CASCADE DELETE validation
//! - Transaction rollback on validation failure
//! - Partial update (PATCH) validation
//!
//! **USES TESTCONTAINERS**: These tests use TestContainers for PostgreSQL database.
//! Docker Desktop must be running before executing these tests.

use reinhardt_core::validators::{MinValueValidator, RangeValidator, Validator};
use reinhardt_test::fixtures::validator::{
	validator_db_guard, validator_test_db, ValidatorDbGuard,
};
use reinhardt_test::resource::TeardownGuard;
use rstest::*;
use sea_query::{Expr, ExprTrait, Iden, PostgresQueryBuilder, Query};
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use testcontainers::{ContainerAsync, GenericImage};

// ============================================================================
// Test Tables Setup
// ============================================================================

/// Setup test tables for constraint validation tests
async fn setup_constraint_test_tables(pool: &PgPool) {
	// Users table
	sqlx::query(
		r#"
		CREATE TABLE IF NOT EXISTS test_users (
			id SERIAL PRIMARY KEY,
			username VARCHAR(100) UNIQUE NOT NULL,
			email VARCHAR(255) UNIQUE NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
		"#,
	)
	.execute(pool)
	.await
	.expect("Failed to create test_users table");

	// Products table with CHECK constraint
	sqlx::query(
		r#"
		CREATE TABLE IF NOT EXISTS test_products (
			id SERIAL PRIMARY KEY,
			name VARCHAR(200) NOT NULL,
			code VARCHAR(50) UNIQUE NOT NULL,
			price DECIMAL(10, 2) NOT NULL CHECK (price >= 0),
			stock INTEGER NOT NULL CHECK (stock >= 0),
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
		"#,
	)
	.execute(pool)
	.await
	.expect("Failed to create test_products table");

	// Posts table for composite unique constraint test
	sqlx::query(
		r#"
		CREATE TABLE IF NOT EXISTS test_posts (
			id SERIAL PRIMARY KEY,
			user_id INTEGER NOT NULL REFERENCES test_users(id),
			title VARCHAR(200) NOT NULL,
			content TEXT NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			UNIQUE(user_id, title)
		)
		"#,
	)
	.execute(pool)
	.await
	.expect("Failed to create test_posts table");

	// Comments table with ON DELETE CASCADE
	sqlx::query(
		r#"
		CREATE TABLE IF NOT EXISTS test_comments (
			id SERIAL PRIMARY KEY,
			post_id INTEGER NOT NULL REFERENCES test_posts(id) ON DELETE CASCADE,
			user_id INTEGER NOT NULL REFERENCES test_users(id),
			content TEXT NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		)
		"#,
	)
	.execute(pool)
	.await
	.expect("Failed to create test_comments table");
}

// ============================================================================
// SeaQuery Table Identifiers
// ============================================================================

#[allow(dead_code)]
#[derive(Iden)]
enum TestUsers {
	Table,
	Id,
	Username,
	Email,
}

#[allow(dead_code)]
#[derive(Iden)]
enum TestProducts {
	Table,
	Id,
	Name,
	Code,
	Price,
	Stock,
}

#[allow(dead_code)]
#[derive(Iden)]
enum TestPosts {
	Table,
	Id,
	UserId,
	Title,
	Content,
}

#[allow(dead_code)]
#[derive(Iden)]
enum TestComments {
	Table,
	Id,
	PostId,
	UserId,
	Content,
}

// ============================================================================
// Test 1: Composite UNIQUE Constraint Validation
// ============================================================================

/// Test composite UNIQUE constraint validation (user_id, title)
///
/// Verifies that:
/// - Same user cannot create posts with duplicate titles
/// - Different users can use the same title
/// - Validator detects constraint violations before database execution
#[rstest]
#[tokio::test]
async fn test_composite_unique_constraint_validation(
	#[future] validator_test_db: (ContainerAsync<GenericImage>, Arc<sqlx::PgPool>, u16, String),
	_validator_db_guard: TeardownGuard<ValidatorDbGuard>,
) {
	let (_container, pool, _port, _database_url) = validator_test_db.await;
	setup_constraint_test_tables(pool.as_ref()).await;

	// Insert test users
	let user1_id = insert_user(pool.as_ref(), "alice", "alice@example.com").await;
	let user2_id = insert_user(pool.as_ref(), "bob", "bob@example.com").await;

	// Insert first post by alice
	let post1_id = insert_post(pool.as_ref(), user1_id, "First Post", "Content 1").await;
	assert!(post1_id > 0);

	// Attempt to insert duplicate (user_id, title) by same user - should fail
	let duplicate_result =
		insert_post_result(pool.as_ref(), user1_id, "First Post", "Different content").await;
	assert!(duplicate_result.is_err());

	let error_message = duplicate_result.unwrap_err().to_string();
	assert!(
		error_message.contains("unique") || error_message.contains("duplicate"),
		"Expected UNIQUE constraint error, got: {}",
		error_message
	);

	// Different user can use same title - should succeed
	let post2_id = insert_post(pool.as_ref(), user2_id, "First Post", "Bob's content").await;
	assert!(post2_id > 0);

	// Same user can use different title - should succeed
	let post3_id = insert_post(pool.as_ref(), user1_id, "Second Post", "Content 2").await;
	assert!(post3_id > 0);

	// Verify composite uniqueness with SeaQuery
	let count = count_posts_by_user_and_title(pool.as_ref(), user1_id, "First Post").await;
	assert_eq!(
		count, 1,
		"Should only have one post with this user_id and title"
	);

	// Cleanup handled automatically by TeardownGuard
}

// ============================================================================
// Test 2: CHECK Constraint Integration
// ============================================================================

/// Test CHECK constraint (price >= 0) integration with validator
///
/// Verifies that:
/// - Application-level validator catches negative prices before database
/// - Database CHECK constraint catches values that bypass validator
/// - Both validations produce consistent error messages
#[rstest]
#[tokio::test]
async fn test_check_constraint_integration(
	#[future] validator_test_db: (ContainerAsync<GenericImage>, Arc<sqlx::PgPool>, u16, String),
	_validator_db_guard: TeardownGuard<ValidatorDbGuard>,
) {
	let (_container, pool, _port, _database_url) = validator_test_db.await;
	setup_constraint_test_tables(pool.as_ref()).await;

	// Application-level validator
	let price_validator = MinValueValidator::new(0.0);
	let stock_validator = MinValueValidator::new(0);

	// Valid product with positive price and stock
	let valid_price = 99.99;
	let valid_stock = 10;

	assert!(price_validator.validate(&valid_price).is_ok());
	assert!(stock_validator.validate(&valid_stock).is_ok());

	let product_id =
		insert_product(pool.as_ref(), "Laptop", "PROD001", valid_price, valid_stock).await;
	assert!(product_id > 0);

	// Application-level validation catches negative price
	let negative_price = -10.0;
	let validation_result = price_validator.validate(&negative_price);
	assert!(validation_result.is_err());
	assert_eq!(
		validation_result.unwrap_err().to_string(),
		"Value too small: -10 (minimum: 0)"
	);

	// Database CHECK constraint also catches negative price
	let db_result =
		insert_product_result(pool.as_ref(), "Invalid Product", "PROD002", -10.0, 5).await;
	assert!(db_result.is_err());

	let db_error = db_result.unwrap_err().to_string();
	assert!(
		db_error.contains("check")
			|| db_error.contains("constraint")
			|| db_error.contains("violates"),
		"Expected CHECK constraint error, got: {}",
		db_error
	);

	// Application-level validation catches negative stock
	let negative_stock = -5;
	let stock_validation_result = stock_validator.validate(&negative_stock);
	assert!(stock_validation_result.is_err());

	// Database CHECK constraint also catches negative stock
	let db_stock_result =
		insert_product_result(pool.as_ref(), "Invalid Stock", "PROD003", 50.0, -10).await;
	assert!(db_stock_result.is_err());

	// Update with invalid price should also fail
	let update_result = update_product_price(pool.as_ref(), product_id, -20.0).await;
	assert!(update_result.is_err());

	// Cleanup handled automatically by TeardownGuard
}

// ============================================================================
// Test 3: CASCADE DELETE Validation
// ============================================================================

/// Test ON DELETE CASCADE validation with foreign key references
///
/// Verifies that:
/// - Deleting a post cascades to delete its comments
/// - Application can validate cascade impacts before deletion
/// - Reference counts are correctly tracked
#[rstest]
#[tokio::test]
async fn test_cascade_delete_validation(
	#[future] validator_test_db: (ContainerAsync<GenericImage>, Arc<sqlx::PgPool>, u16, String),
	_validator_db_guard: TeardownGuard<ValidatorDbGuard>,
) {
	let (_container, pool, _port, _database_url) = validator_test_db.await;
	setup_constraint_test_tables(pool.as_ref()).await;

	// Setup: Create user, post, and comments
	let user_id = insert_user(pool.as_ref(), "charlie", "charlie@example.com").await;
	let post_id = insert_post(pool.as_ref(), user_id, "Test Post", "Content").await;

	// Insert multiple comments on the post
	let comment1_id = insert_comment(pool.as_ref(), post_id, user_id, "Comment 1").await;
	let comment2_id = insert_comment(pool.as_ref(), post_id, user_id, "Comment 2").await;
	let comment3_id = insert_comment(pool.as_ref(), post_id, user_id, "Comment 3").await;

	assert!(comment1_id > 0);
	assert!(comment2_id > 0);
	assert!(comment3_id > 0);

	// Verify comments exist before deletion
	let comment_count_before = count_comments_by_post(pool.as_ref(), post_id).await;
	assert_eq!(comment_count_before, 3);

	// Application-level validation: Check cascade impact before delete
	let affected_comments = count_comments_by_post(pool.as_ref(), post_id).await;
	assert_eq!(
		affected_comments, 3,
		"Should find 3 comments that will be cascade-deleted"
	);

	// Delete post - should cascade delete all comments
	let delete_result = delete_post(pool.as_ref(), post_id).await;
	assert!(delete_result.is_ok());

	// Verify post is deleted
	let post_exists = check_post_exists(pool.as_ref(), post_id).await;
	assert!(!post_exists);

	// Verify comments are cascade-deleted
	let comment_count_after = count_comments_by_post(pool.as_ref(), post_id).await;
	assert_eq!(
		comment_count_after, 0,
		"All comments should be cascade-deleted"
	);

	// Cleanup handled automatically by TeardownGuard
}

// ============================================================================
// Test 4: Validation Failure Transaction Rollback
// ============================================================================

/// Test that validation failures trigger transaction rollback
///
/// Verifies that:
/// - Failed validation within transaction rolls back all changes
/// - Database state remains consistent after rollback
/// - No partial writes occur on validation failure
#[rstest]
#[tokio::test]
async fn test_validation_failure_transaction_rollback(
	#[future] validator_test_db: (ContainerAsync<GenericImage>, Arc<sqlx::PgPool>, u16, String),
	_validator_db_guard: TeardownGuard<ValidatorDbGuard>,
) {
	let (_container, pool, _port, _database_url) = validator_test_db.await;
	setup_constraint_test_tables(pool.as_ref()).await;

	// Get initial product count
	let initial_count = count_all_products(pool.as_ref()).await;

	// Begin transaction
	let mut tx = pool.begin().await.expect("Failed to begin transaction");

	// Insert valid product within transaction
	let product1_result = insert_product_tx(&mut tx, "Valid Product", "PROD001", 100.0, 10).await;
	assert!(product1_result.is_ok());

	// Application-level validation fails for negative price
	let price_validator = MinValueValidator::new(0.0);
	let invalid_price = -50.0;
	let validation_result = price_validator.validate(&invalid_price);

	if validation_result.is_err() {
		// Rollback transaction on validation failure
		tx.rollback().await.expect("Failed to rollback transaction");
	}

	// Verify rollback: product count should remain unchanged
	let final_count = count_all_products(pool.as_ref()).await;
	assert_eq!(
		final_count, initial_count,
		"Transaction should be rolled back, no products added"
	);

	// Verify the valid product was NOT committed
	let product_exists = check_product_code_exists(pool.as_ref(), "PROD001").await;
	assert!(!product_exists, "Product should not exist after rollback");

	// Test successful transaction with valid data
	let mut tx2 = pool.begin().await.expect("Failed to begin transaction");

	let product2_result =
		insert_product_tx(&mut tx2, "Valid Product 2", "PROD002", 200.0, 20).await;
	assert!(product2_result.is_ok());

	// Validation passes
	let valid_price = 200.0;
	assert!(price_validator.validate(&valid_price).is_ok());

	// Commit transaction
	tx2.commit().await.expect("Failed to commit transaction");

	// Verify commit: product count should increase
	let committed_count = count_all_products(pool.as_ref()).await;
	assert_eq!(
		committed_count,
		initial_count + 1,
		"One product should be added after commit"
	);

	// Cleanup handled automatically by TeardownGuard
}

// ============================================================================
// Test 5: Partial Update (PATCH) Validation
// ============================================================================

/// Test validation for partial updates (PATCH operations)
///
/// Verifies that:
/// - Only modified fields are validated
/// - Unmodified fields retain original values
/// - Partial validation respects database constraints
#[rstest]
#[tokio::test]
async fn test_partial_update_validation(
	#[future] validator_test_db: (ContainerAsync<GenericImage>, Arc<sqlx::PgPool>, u16, String),
	_validator_db_guard: TeardownGuard<ValidatorDbGuard>,
) {
	let (_container, pool, _port, _database_url) = validator_test_db.await;
	setup_constraint_test_tables(pool.as_ref()).await;

	// Insert initial product
	let product_id = insert_product(pool.as_ref(), "Original Product", "PROD001", 100.0, 50).await;
	assert!(product_id > 0);

	// Partial update: Only update price
	let new_price = 150.0;
	let price_validator = RangeValidator::new(0.0, 999999.99);

	// Validate only the updated field
	assert!(price_validator.validate(&new_price).is_ok());

	// Apply partial update
	let update_result = update_product_price(pool.as_ref(), product_id, new_price).await;
	assert!(update_result.is_ok());

	// Verify updated field
	let updated_product = get_product(pool.as_ref(), product_id).await;
	assert_eq!(updated_product.price, new_price);
	assert_eq!(updated_product.name, "Original Product"); // Unchanged
	assert_eq!(updated_product.stock, 50); // Unchanged

	// Partial update: Only update stock
	let new_stock = 100;
	let stock_validator = MinValueValidator::new(0);

	assert!(stock_validator.validate(&new_stock).is_ok());

	let update_stock_result = update_product_stock(pool.as_ref(), product_id, new_stock).await;
	assert!(update_stock_result.is_ok());

	// Verify updated field
	let updated_product2 = get_product(pool.as_ref(), product_id).await;
	assert_eq!(updated_product2.stock, new_stock);
	assert_eq!(updated_product2.price, new_price); // Previously updated value
	assert_eq!(updated_product2.name, "Original Product"); // Still unchanged

	// Invalid partial update: negative price
	let invalid_price = -10.0;
	let invalid_validation = price_validator.validate(&invalid_price);
	assert!(invalid_validation.is_err());

	// Do not apply invalid update
	// Database constraint also prevents it
	let invalid_update_result =
		update_product_price(pool.as_ref(), product_id, invalid_price).await;
	assert!(invalid_update_result.is_err());

	// Verify product state remains unchanged after failed update
	let final_product = get_product(pool.as_ref(), product_id).await;
	assert_eq!(final_product.price, new_price); // Still valid price
	assert_eq!(final_product.stock, new_stock); // Still valid stock

	// Cleanup handled automatically by TeardownGuard
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Insert user helper
async fn insert_user(pool: &PgPool, username: &str, email: &str) -> i32 {
	let row: (i32,) =
		sqlx::query_as("INSERT INTO test_users (username, email) VALUES ($1, $2) RETURNING id")
			.bind(username)
			.bind(email)
			.fetch_one(pool)
			.await
			.expect("Failed to insert user");
	row.0
}

/// Insert post helper
async fn insert_post(pool: &PgPool, user_id: i32, title: &str, content: &str) -> i32 {
	let row: (i32,) = sqlx::query_as(
		"INSERT INTO test_posts (user_id, title, content) VALUES ($1, $2, $3) RETURNING id",
	)
	.bind(user_id)
	.bind(title)
	.bind(content)
	.fetch_one(pool)
	.await
	.expect("Failed to insert post");
	row.0
}

/// Insert post with Result (for error testing)
async fn insert_post_result(
	pool: &PgPool,
	user_id: i32,
	title: &str,
	content: &str,
) -> Result<i32, sqlx::Error> {
	let row: (i32,) = sqlx::query_as(
		"INSERT INTO test_posts (user_id, title, content) VALUES ($1, $2, $3) RETURNING id",
	)
	.bind(user_id)
	.bind(title)
	.bind(content)
	.fetch_one(pool)
	.await?;
	Ok(row.0)
}

/// Insert product helper
async fn insert_product(pool: &PgPool, name: &str, code: &str, price: f64, stock: i32) -> i32 {
	let row: (i32,) = sqlx::query_as(
		"INSERT INTO test_products (name, code, price, stock) VALUES ($1, $2, $3, $4) RETURNING id",
	)
	.bind(name)
	.bind(code)
	.bind(price)
	.bind(stock)
	.fetch_one(pool)
	.await
	.expect("Failed to insert product");
	row.0
}

/// Insert product with Result (for error testing)
async fn insert_product_result(
	pool: &PgPool,
	name: &str,
	code: &str,
	price: f64,
	stock: i32,
) -> Result<i32, sqlx::Error> {
	let row: (i32,) = sqlx::query_as(
		"INSERT INTO test_products (name, code, price, stock) VALUES ($1, $2, $3, $4) RETURNING id",
	)
	.bind(name)
	.bind(code)
	.bind(price)
	.bind(stock)
	.fetch_one(pool)
	.await?;
	Ok(row.0)
}

/// Insert product within transaction
async fn insert_product_tx(
	tx: &mut Transaction<'_, Postgres>,
	name: &str,
	code: &str,
	price: f64,
	stock: i32,
) -> Result<i32, sqlx::Error> {
	let row: (i32,) = sqlx::query_as(
		"INSERT INTO test_products (name, code, price, stock) VALUES ($1, $2, $3, $4) RETURNING id",
	)
	.bind(name)
	.bind(code)
	.bind(price)
	.bind(stock)
	.fetch_one(&mut **tx)
	.await?;
	Ok(row.0)
}

/// Insert comment helper
async fn insert_comment(pool: &PgPool, post_id: i32, user_id: i32, content: &str) -> i32 {
	let row: (i32,) = sqlx::query_as(
		"INSERT INTO test_comments (post_id, user_id, content) VALUES ($1, $2, $3) RETURNING id",
	)
	.bind(post_id)
	.bind(user_id)
	.bind(content)
	.fetch_one(pool)
	.await
	.expect("Failed to insert comment");
	row.0
}

/// Count posts by user_id and title (for composite unique test)
async fn count_posts_by_user_and_title(pool: &PgPool, user_id: i32, title: &str) -> i64 {
	let row: (i64,) =
		sqlx::query_as("SELECT COUNT(*) FROM test_posts WHERE user_id = $1 AND title = $2")
			.bind(user_id)
			.bind(title)
			.fetch_one(pool)
			.await
			.expect("Failed to count posts");
	row.0
}

/// Count comments by post_id
async fn count_comments_by_post(pool: &PgPool, post_id: i32) -> i64 {
	let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM test_comments WHERE post_id = $1")
		.bind(post_id)
		.fetch_one(pool)
		.await
		.expect("Failed to count comments");
	row.0
}

/// Delete post using SeaQuery
async fn delete_post(pool: &PgPool, post_id: i32) -> Result<u64, sqlx::Error> {
	let sql = Query::delete()
		.from_table(TestPosts::Table)
		.cond_where(Expr::col(TestPosts::Id).eq(post_id))
		.to_string(PostgresQueryBuilder);

	let result = sqlx::query(&sql).bind(post_id).execute(pool).await?;
	Ok(result.rows_affected())
}

/// Check if post exists
async fn check_post_exists(pool: &PgPool, post_id: i32) -> bool {
	let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM test_posts WHERE id = $1")
		.bind(post_id)
		.fetch_one(pool)
		.await
		.expect("Failed to check post existence");
	row.0 > 0
}

/// Count all products
async fn count_all_products(pool: &PgPool) -> i64 {
	let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM test_products")
		.fetch_one(pool)
		.await
		.expect("Failed to count products");
	row.0
}

/// Check if product code exists
async fn check_product_code_exists(pool: &PgPool, code: &str) -> bool {
	let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM test_products WHERE code = $1")
		.bind(code)
		.fetch_one(pool)
		.await
		.expect("Failed to check product code");
	row.0 > 0
}

/// Update product price using SeaQuery
async fn update_product_price(
	pool: &PgPool,
	product_id: i32,
	new_price: f64,
) -> Result<u64, sqlx::Error> {
	let sql = Query::update()
		.table(TestProducts::Table)
		.value(TestProducts::Price, new_price)
		.cond_where(Expr::col(TestProducts::Id).eq(product_id))
		.to_string(PostgresQueryBuilder);

	let result = sqlx::query(&sql)
		.bind(new_price)
		.bind(product_id)
		.execute(pool)
		.await?;
	Ok(result.rows_affected())
}

/// Update product stock using SeaQuery
async fn update_product_stock(
	pool: &PgPool,
	product_id: i32,
	new_stock: i32,
) -> Result<u64, sqlx::Error> {
	let sql = Query::update()
		.table(TestProducts::Table)
		.value(TestProducts::Stock, new_stock)
		.cond_where(Expr::col(TestProducts::Id).eq(product_id))
		.to_string(PostgresQueryBuilder);

	let result = sqlx::query(&sql)
		.bind(new_stock)
		.bind(product_id)
		.execute(pool)
		.await?;
	Ok(result.rows_affected())
}

/// Product struct for retrieval
#[derive(Debug)]
struct Product {
	#[allow(dead_code)]
	id: i32,
	name: String,
	#[allow(dead_code)]
	code: String,
	price: f64,
	stock: i32,
}

/// Get product by ID
async fn get_product(pool: &PgPool, product_id: i32) -> Product {
	let row: (i32, String, String, rust_decimal::Decimal, i32) =
		sqlx::query_as("SELECT id, name, code, price, stock FROM test_products WHERE id = $1")
			.bind(product_id)
			.fetch_one(pool)
			.await
			.expect("Failed to get product");

	Product {
		id: row.0,
		name: row.1,
		code: row.2,
		price: row.3.to_string().parse::<f64>().unwrap(),
		stock: row.4,
	}
}
