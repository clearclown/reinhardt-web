//! Subquery Integration Tests
//!
//! Tests comprehensive subquery functionality covering:
//! - Subqueries in WHERE clause
//! - Subqueries in SELECT clause
//! - Subqueries in FROM clause
//! - EXISTS and NOT EXISTS predicates
//! - IN predicates with subqueries
//! - Correlated subqueries
//! - Nested subqueries
//!
//! **Fixtures Used:**
//! - postgres_container: PostgreSQL database container
//!
//! **Test Data Schema:**
//! - authors(id SERIAL PRIMARY KEY, name TEXT NOT NULL)
//! - books(id SERIAL PRIMARY KEY, author_id INT NOT NULL, title TEXT NOT NULL, price BIGINT NOT NULL)

use reinhardt_orm;
use reinhardt_test::fixtures::postgres_container;
use rstest::*;
use sea_query::{Alias, Expr, ExprTrait, Iden, PostgresQueryBuilder, Query};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use testcontainers::{ContainerAsync, GenericImage};

// ============================================================================
// Table Identifiers
// ============================================================================

#[derive(Iden)]
enum Authors {
	Table,
	Id,
	Name,
}

#[derive(Iden)]
enum Books {
	Table,
	Id,
	AuthorId,
	Title,
	Price,
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Create test table and insert test data
async fn setup_test_data(pool: &PgPool) {
	// Create tables
	sqlx::query(
		"CREATE TABLE IF NOT EXISTS authors (
			id SERIAL PRIMARY KEY,
			name TEXT NOT NULL
		)",
	)
	.execute(pool)
	.await
	.expect("Failed to create authors table");

	sqlx::query(
		"CREATE TABLE IF NOT EXISTS books (
			id SERIAL PRIMARY KEY,
			author_id INT NOT NULL,
			title TEXT NOT NULL,
			price BIGINT NOT NULL
		)",
	)
	.execute(pool)
	.await
	.expect("Failed to create books table");

	// Insert authors
	sqlx::query("INSERT INTO authors (name) VALUES ($1), ($2), ($3)")
		.bind("Author A")
		.bind("Author B")
		.bind("Author C")
		.execute(pool)
		.await
		.expect("Failed to insert authors");

	// Insert books
	// Author A: 2 books (1000, 2000)
	sqlx::query("INSERT INTO books (author_id, title, price) VALUES ($1, $2, $3)")
		.bind(1)
		.bind("Book A1")
		.bind(1000_i64)
		.execute(pool)
		.await
		.expect("Failed to insert book A1");

	sqlx::query("INSERT INTO books (author_id, title, price) VALUES ($1, $2, $3)")
		.bind(1)
		.bind("Book A2")
		.bind(2000_i64)
		.execute(pool)
		.await
		.expect("Failed to insert book A2");

	// Author B: 1 book (1500)
	sqlx::query("INSERT INTO books (author_id, title, price) VALUES ($1, $2, $3)")
		.bind(2)
		.bind("Book B1")
		.bind(1500_i64)
		.execute(pool)
		.await
		.expect("Failed to insert book B1");

	// Author C: No books
}

// ============================================================================
// Subquery in WHERE Clause Tests
// ============================================================================

/// Test subquery in WHERE clause
///
/// **Test Intent**: Use subquery in WHERE clause and filter results with comparison operators
///
/// **Integration Point**: SeaQuery SubqueryExpr → PostgreSQL WHERE clause
///
/// **Not Intent**: EXISTS, IN predicate usage
#[rstest]
#[tokio::test]
async fn test_subquery_in_where_clause(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_test_data(pool.as_ref()).await;

	// Find authors whose average book price > 1500
	// SELECT name FROM authors WHERE id IN (
	//   SELECT author_id FROM books GROUP BY author_id HAVING AVG(price) > 1500
	// )
	let subquery = Query::select()
		.column(Books::AuthorId)
		.from(Books::Table)
		.group_by_col(Books::AuthorId)
		.and_having(Expr::col(Books::Price).avg().gt(1500))
		.to_owned();

	let sql = Query::select()
		.column(Authors::Name)
		.from(Authors::Table)
		.and_where(Expr::col(Authors::Id).in_subquery(subquery))
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&sql)
		.fetch_all(pool.as_ref())
		.await
		.expect("Failed to execute subquery in WHERE");

	assert_eq!(rows.len(), 1, "Expected 1 author with avg price > 1500");
	let name: String = rows[0].get("name");
	assert_eq!(name, "Author A", "Expected Author A");
}

/// Test subquery in SELECT clause
///
/// **Test Intent**: Use subquery in SELECT clause to retrieve scalar values
///
/// **Integration Point**: SeaQuery SubqueryExpr → PostgreSQL SELECT clause
///
/// **Not Intent**: Subqueries returning multiple rows, subqueries without aggregate functions
#[rstest]
#[tokio::test]
async fn test_subquery_in_select_clause(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_test_data(pool.as_ref()).await;

	// SELECT name, (SELECT COUNT(*) FROM books WHERE author_id = authors.id) as book_count
	// FROM authors
	let subquery = Query::select()
		.expr(Expr::col(Books::Id).count())
		.from(Books::Table)
		.and_where(Expr::col(Books::AuthorId).equals((Authors::Table, Authors::Id)))
		.to_owned();

	let sql = Query::select()
		.column(Authors::Name)
		.expr_as(subquery, Alias::new("book_count"))
		.from(Authors::Table)
		.order_by(Authors::Id, sea_query::Order::Asc)
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&sql)
		.fetch_all(pool.as_ref())
		.await
		.expect("Failed to execute subquery in SELECT");

	assert_eq!(rows.len(), 3, "Expected 3 authors");

	// Author A: 2 books
	let name: String = rows[0].get("name");
	let count: i64 = rows[0].get("book_count");
	assert_eq!(name, "Author A");
	assert_eq!(count, 2);

	// Author B: 1 book
	let name: String = rows[1].get("name");
	let count: i64 = rows[1].get("book_count");
	assert_eq!(name, "Author B");
	assert_eq!(count, 1);

	// Author C: 0 books
	let name: String = rows[2].get("name");
	let count: i64 = rows[2].get("book_count");
	assert_eq!(name, "Author C");
	assert_eq!(count, 0);
}

// ============================================================================
// EXISTS Predicate Tests
// ============================================================================

/// Test EXISTS predicate
///
/// **Test Intent**: Use EXISTS predicate to check if subquery results exist
///
/// **Integration Point**: SeaQuery EXISTS → PostgreSQL EXISTS predicate
///
/// **Not Intent**: NOT EXISTS, no correlated subquery
#[rstest]
#[tokio::test]
async fn test_exists_subquery(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_test_data(pool.as_ref()).await;

	// Find authors who have at least one book
	// SELECT name FROM authors WHERE EXISTS (
	//   SELECT 1 FROM books WHERE books.author_id = authors.id
	// )
	let subquery = Query::select()
		.expr(Expr::value(1))
		.from(Books::Table)
		.and_where(Expr::col(Books::AuthorId).equals((Authors::Table, Authors::Id)))
		.to_owned();

	let sql = Query::select()
		.column(Authors::Name)
		.from(Authors::Table)
		.cond_where(Expr::exists(subquery))
		.order_by(Authors::Id, sea_query::Order::Asc)
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&sql)
		.fetch_all(pool.as_ref())
		.await
		.expect("Failed to execute EXISTS subquery");

	assert_eq!(rows.len(), 2, "Expected 2 authors with books");

	let names: Vec<String> = rows.iter().map(|row| row.get("name")).collect();
	assert_eq!(names, vec!["Author A", "Author B"]);
}

/// Test NOT EXISTS predicate
///
/// **Test Intent**: Use NOT EXISTS predicate to check that subquery results do not exist
///
/// **Integration Point**: SeaQuery NOT EXISTS → PostgreSQL NOT EXISTS predicate
///
/// **Not Intent**: EXISTS, complex conditions
#[rstest]
#[tokio::test]
async fn test_not_exists_subquery(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_test_data(pool.as_ref()).await;

	// Find authors who have no books
	// SELECT name FROM authors WHERE NOT EXISTS (
	//   SELECT 1 FROM books WHERE books.author_id = authors.id
	// )
	let subquery = Query::select()
		.expr(Expr::value(1))
		.from(Books::Table)
		.and_where(Expr::col(Books::AuthorId).equals((Authors::Table, Authors::Id)))
		.to_owned();

	let sql = Query::select()
		.column(Authors::Name)
		.from(Authors::Table)
		.cond_where(Expr::not_exists(subquery))
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&sql)
		.fetch_all(pool.as_ref())
		.await
		.expect("Failed to execute NOT EXISTS subquery");

	assert_eq!(rows.len(), 1, "Expected 1 author without books");

	let name: String = rows[0].get("name");
	assert_eq!(name, "Author C");
}

// ============================================================================
// IN Predicate with Subquery Tests
// ============================================================================

/// Test IN predicate with subquery
///
/// **Test Intent**: Use IN predicate to check if value is in subquery result list
///
/// **Integration Point**: SeaQuery IN subquery → PostgreSQL IN predicate
///
/// **Not Intent**: NOT IN, multi-column IN
#[rstest]
#[tokio::test]
async fn test_in_subquery(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_test_data(pool.as_ref()).await;

	// Find authors who have books priced over 1500
	// SELECT name FROM authors WHERE id IN (
	//   SELECT DISTINCT author_id FROM books WHERE price > 1500
	// )
	let subquery = Query::select()
		.distinct()
		.column(Books::AuthorId)
		.from(Books::Table)
		.and_where(Expr::col(Books::Price).gt(1500))
		.to_owned();

	let sql = Query::select()
		.column(Authors::Name)
		.from(Authors::Table)
		.and_where(Expr::col(Authors::Id).in_subquery(subquery))
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&sql)
		.fetch_all(pool.as_ref())
		.await
		.expect("Failed to execute IN subquery");

	assert_eq!(rows.len(), 1, "Expected 1 author with books > 1500");

	let name: String = rows[0].get("name");
	assert_eq!(name, "Author A");
}

// ============================================================================
// Correlated Subquery Tests
// ============================================================================

/// Test correlated subquery
///
/// **Test Intent**: Use correlated subquery that references columns from outer query
///
/// **Integration Point**: SeaQuery correlated subquery → PostgreSQL correlated subquery
///
/// **Not Intent**: Non-correlated subquery, complex nesting
#[rstest]
#[tokio::test]
async fn test_correlated_subquery(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_test_data(pool.as_ref()).await;

	// Find books with price above their author's average
	// SELECT title, price FROM books b1
	// WHERE price > (
	//   SELECT AVG(price) FROM books b2 WHERE b2.author_id = b1.author_id
	// )
	let b1 = Alias::new("b1");
	let b2 = Alias::new("b2");

	let subquery = Query::select()
		.expr(Expr::col((b2.clone(), Books::Price)).avg())
		.from_as(Books::Table, b2.clone())
		.and_where(Expr::col((b2.clone(), Books::AuthorId)).equals((b1.clone(), Books::AuthorId)))
		.to_owned();

	let sql = Query::select()
		.columns([Books::Title, Books::Price])
		.from_as(Books::Table, b1.clone())
		.and_where(Expr::col((b1.clone(), Books::Price)).gt(subquery))
		.order_by((b1.clone(), Books::Price), sea_query::Order::Asc)
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&sql)
		.fetch_all(pool.as_ref())
		.await
		.expect("Failed to execute correlated subquery");

	// Author A's books: 1000, 2000 (avg=1500)
	// Book A2 (2000) is above average
	assert_eq!(rows.len(), 1, "Expected 1 book above author's average");

	let title: String = rows[0].get("title");
	let price: i64 = rows[0].get("price");
	assert_eq!(title, "Book A2");
	assert_eq!(price, 2000);
}

// ============================================================================
// Nested Subquery Tests
// ============================================================================

/// Test nested subqueries
///
/// **Test Intent**: Use multi-level nested subqueries
///
/// **Integration Point**: SeaQuery nested subqueries → PostgreSQL nested subqueries
///
/// **Not Intent**: Single-level subquery, simple queries
#[rstest]
#[tokio::test]
async fn test_nested_subqueries(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_test_data(pool.as_ref()).await;

	// Find authors who have books with price above the overall average
	// SELECT name FROM authors WHERE id IN (
	//   SELECT author_id FROM books WHERE price > (
	//     SELECT AVG(price) FROM books
	//   )
	// )

	// Innermost: Overall average price
	let avg_price_query = Query::select()
		.expr(Expr::col(Books::Price).avg())
		.from(Books::Table)
		.to_owned();

	// Middle: Books above average
	let books_above_avg = Query::select()
		.distinct()
		.column(Books::AuthorId)
		.from(Books::Table)
		.and_where(Expr::col(Books::Price).gt(avg_price_query))
		.to_owned();

	// Outer: Authors with those books
	let sql = Query::select()
		.column(Authors::Name)
		.from(Authors::Table)
		.and_where(Expr::col(Authors::Id).in_subquery(books_above_avg))
		.order_by(Authors::Id, sea_query::Order::Asc)
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&sql)
		.fetch_all(pool.as_ref())
		.await
		.expect("Failed to execute nested subqueries");

	// Overall average: (1000 + 2000 + 1500) / 3 = 1500
	// Books above 1500: Book A2 (2000)
	// Authors: Author A
	assert_eq!(
		rows.len(),
		1,
		"Expected 1 author with books above overall avg"
	);

	let name: String = rows[0].get("name");
	assert_eq!(name, "Author A");
}

// ============================================================================
// Subquery in FROM Clause Tests
// ============================================================================

/// Test subquery in FROM clause (derived table)
///
/// **Test Intent**: Use subquery in FROM clause to create derived table
///
/// **Integration Point**: SeaQuery subquery in FROM → PostgreSQL derived table
///
/// **Not Intent**: Regular table joins, view usage
#[rstest]
#[tokio::test]
async fn test_subquery_in_from_clause(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_test_data(pool.as_ref()).await;

	// Select from derived table showing author book counts
	// SELECT * FROM (
	//   SELECT author_id, COUNT(*) as book_count
	//   FROM books
	//   GROUP BY author_id
	// ) AS book_stats
	// WHERE book_count > 1

	let derived_alias = Alias::new("book_stats");
	let book_count_alias = Alias::new("book_count");

	let subquery = Query::select()
		.column(Books::AuthorId)
		.expr_as(Expr::col(Books::Id).count(), book_count_alias.clone())
		.from(Books::Table)
		.group_by_col(Books::AuthorId)
		.to_owned();

	let sql = Query::select()
		.column(sea_query::Asterisk)
		.from_subquery(subquery, derived_alias.clone())
		.and_where(Expr::col(book_count_alias.clone()).gt(1))
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&sql)
		.fetch_all(pool.as_ref())
		.await
		.expect("Failed to execute subquery in FROM");

	// Only Author A has more than 1 book
	assert_eq!(rows.len(), 1, "Expected 1 author with > 1 book");

	let author_id: i32 = rows[0].get("author_id");
	let book_count: i64 = rows[0].get("book_count");
	assert_eq!(author_id, 1);
	assert_eq!(book_count, 2);
}
