//! Type-safe JOIN Operation Integration Tests
//!
//! This test file covers normal cases and use cases for various JOIN operations
//! using SeaQuery v1.0.0-rc.
//!
//! # Table Structure
//! - Users(id, name)
//! - Posts(id, user_id, title)
//! - Comments(id, post_id, content)

use reinhardt_orm;
use reinhardt_test::fixtures::postgres_container;
use rstest::*;
use sea_query::{
	Alias, ColumnDef, Expr, ExprTrait, Iden, JoinType, PostgresQueryBuilder, Query, Table,
};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use testcontainers::{ContainerAsync, GenericImage};

/// Users table definition
#[derive(Iden)]
enum Users {
	Table,
	Id,
	Name,
}

/// Posts table definition
#[derive(Iden)]
enum Posts {
	Table,
	Id,
	UserId,
	Title,
}

/// Comments table definition
#[derive(Iden)]
enum Comments {
	Table,
	Id,
	PostId,
	Content,
}

/// Initialize test tables and data
async fn setup_tables(pool: &PgPool) {
	// Create users table
	let create_users = Table::create()
		.table(Users::Table)
		.if_not_exists()
		.col(
			ColumnDef::new(Users::Id)
				.integer()
				.not_null()
				.auto_increment()
				.primary_key(),
		)
		.col(ColumnDef::new(Users::Name).string().not_null())
		.to_string(PostgresQueryBuilder);

	sqlx::query(&create_users).execute(pool).await.unwrap();

	// Create posts table
	let create_posts = Table::create()
		.table(Posts::Table)
		.if_not_exists()
		.col(
			ColumnDef::new(Posts::Id)
				.integer()
				.not_null()
				.auto_increment()
				.primary_key(),
		)
		.col(ColumnDef::new(Posts::UserId).integer().not_null())
		.col(ColumnDef::new(Posts::Title).string().not_null())
		.to_string(PostgresQueryBuilder);

	sqlx::query(&create_posts).execute(pool).await.unwrap();

	// Create comments table
	let create_comments = Table::create()
		.table(Comments::Table)
		.if_not_exists()
		.col(
			ColumnDef::new(Comments::Id)
				.integer()
				.not_null()
				.auto_increment()
				.primary_key(),
		)
		.col(ColumnDef::new(Comments::PostId).integer().not_null())
		.col(ColumnDef::new(Comments::Content).string().not_null())
		.to_string(PostgresQueryBuilder);

	sqlx::query(&create_comments).execute(pool).await.unwrap();

	// Insert test data
	sqlx::query("INSERT INTO users (name) VALUES ('Alice'), ('Bob'), ('Charlie')")
		.execute(pool)
		.await
		.unwrap();

	sqlx::query("INSERT INTO posts (user_id, title) VALUES (1, 'First Post'), (1, 'Second Post'), (2, 'Bob Post')")
		.execute(pool)
		.await
		.unwrap();

	sqlx::query(
		"INSERT INTO comments (post_id, content) VALUES (1, 'Great!'), (1, 'Thanks'), (2, 'Nice')",
	)
	.execute(pool)
	.await
	.unwrap();
}

/// Test INNER JOIN of two tables
///
/// INNER JOIN Users and Posts to retrieve only users who have posts
#[rstest]
#[tokio::test]
async fn test_inner_join_two_tables(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_tables(pool.as_ref()).await;

	let query = Query::select()
		.column((Users::Table, Users::Name))
		.column((Posts::Table, Posts::Title))
		.from(Users::Table)
		.join(
			JoinType::InnerJoin,
			Posts::Table,
			Expr::col((Posts::Table, Posts::UserId)).equals((Users::Table, Users::Id)),
		)
		.order_by((Users::Table, Users::Id), sea_query::Order::Asc)
		.order_by((Posts::Table, Posts::Id), sea_query::Order::Asc)
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&query).fetch_all(pool.as_ref()).await.unwrap();

	// Alice and Bob's posts are retrieved (Charlie excluded due to no posts)
	assert_eq!(rows.len(), 3);
	assert_eq!(rows[0].get::<String, _>("name"), "Alice");
	assert_eq!(rows[0].get::<String, _>("title"), "First Post");
	assert_eq!(rows[1].get::<String, _>("name"), "Alice");
	assert_eq!(rows[1].get::<String, _>("title"), "Second Post");
	assert_eq!(rows[2].get::<String, _>("name"), "Bob");
	assert_eq!(rows[2].get::<String, _>("title"), "Bob Post");
}

/// Test LEFT JOIN with NULL values
///
/// LEFT JOIN Posts on Users to retrieve users without posts as well
#[rstest]
#[tokio::test]
async fn test_left_join_with_nulls(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_tables(pool.as_ref()).await;

	let query = Query::select()
		.columns([(Users::Table, Users::Id), (Users::Table, Users::Name)])
		.expr_as(
			Expr::col((Posts::Table, Posts::Title)),
			Alias::new("post_title"),
		)
		.from(Users::Table)
		.join(
			JoinType::LeftJoin,
			Posts::Table,
			Expr::col((Posts::Table, Posts::UserId)).equals((Users::Table, Users::Id)),
		)
		.order_by((Users::Table, Users::Id), sea_query::Order::Asc)
		.order_by((Posts::Table, Posts::Id), sea_query::Order::Asc)
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&query).fetch_all(pool.as_ref()).await.unwrap();

	// All users are retrieved (Charlie's posts are NULL)
	assert_eq!(rows.len(), 4);

	// Alice (2 posts)
	assert_eq!(rows[0].get::<i32, _>("id"), 1);
	assert_eq!(rows[0].get::<String, _>("name"), "Alice");
	assert_eq!(rows[0].get::<String, _>("post_title"), "First Post");

	assert_eq!(rows[1].get::<i32, _>("id"), 1);
	assert_eq!(rows[1].get::<String, _>("name"), "Alice");
	assert_eq!(rows[1].get::<String, _>("post_title"), "Second Post");

	// Bob (1 post)
	assert_eq!(rows[2].get::<i32, _>("id"), 2);
	assert_eq!(rows[2].get::<String, _>("name"), "Bob");
	assert_eq!(rows[2].get::<String, _>("post_title"), "Bob Post");

	// Charlie (no posts - NULL)
	assert_eq!(rows[3].get::<i32, _>("id"), 3);
	assert_eq!(rows[3].get::<String, _>("name"), "Charlie");
	assert_eq!(rows[3].get::<Option<String>, _>("post_title"), None);
}

/// Test RIGHT JOIN
///
/// RIGHT JOIN Users on Posts to retrieve posts without users as well
/// (In this test case, all posts actually have users)
#[rstest]
#[tokio::test]
async fn test_right_join(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_tables(pool.as_ref()).await;

	// RIGHT JOIN: Join Users based on Posts
	let query = Query::select()
		.expr_as(
			Expr::col((Users::Table, Users::Name)),
			Alias::new("user_name"),
		)
		.columns([(Posts::Table, Posts::Id), (Posts::Table, Posts::Title)])
		.from(Users::Table)
		.join(
			JoinType::RightJoin,
			Posts::Table,
			Expr::col((Posts::Table, Posts::UserId)).equals((Users::Table, Users::Id)),
		)
		.order_by((Posts::Table, Posts::Id), sea_query::Order::Asc)
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&query).fetch_all(pool.as_ref()).await.unwrap();

	// All posts are retrieved
	assert_eq!(rows.len(), 3);
	assert_eq!(rows[0].get::<String, _>("user_name"), "Alice");
	assert_eq!(rows[0].get::<String, _>("title"), "First Post");
	assert_eq!(rows[1].get::<String, _>("user_name"), "Alice");
	assert_eq!(rows[1].get::<String, _>("title"), "Second Post");
	assert_eq!(rows[2].get::<String, _>("user_name"), "Bob");
	assert_eq!(rows[2].get::<String, _>("title"), "Bob Post");
}

/// Test multiple JOINs across three tables
///
/// Chained JOINs: Users -> Posts -> Comments
#[rstest]
#[tokio::test]
async fn test_multiple_joins_three_tables(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_tables(pool.as_ref()).await;

	let query = Query::select()
		.column((Users::Table, Users::Name))
		.column((Posts::Table, Posts::Title))
		.column((Comments::Table, Comments::Content))
		.from(Users::Table)
		.join(
			JoinType::InnerJoin,
			Posts::Table,
			Expr::col((Posts::Table, Posts::UserId)).equals((Users::Table, Users::Id)),
		)
		.join(
			JoinType::InnerJoin,
			Comments::Table,
			Expr::col((Comments::Table, Comments::PostId)).equals((Posts::Table, Posts::Id)),
		)
		.order_by((Users::Table, Users::Id), sea_query::Order::Asc)
		.order_by((Posts::Table, Posts::Id), sea_query::Order::Asc)
		.order_by((Comments::Table, Comments::Id), sea_query::Order::Asc)
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&query).fetch_all(pool.as_ref()).await.unwrap();

	// Only posts with comments are retrieved
	assert_eq!(rows.len(), 3);

	// First Post (2 comments)
	assert_eq!(rows[0].get::<String, _>("name"), "Alice");
	assert_eq!(rows[0].get::<String, _>("title"), "First Post");
	assert_eq!(rows[0].get::<String, _>("content"), "Great!");

	assert_eq!(rows[1].get::<String, _>("name"), "Alice");
	assert_eq!(rows[1].get::<String, _>("title"), "First Post");
	assert_eq!(rows[1].get::<String, _>("content"), "Thanks");

	// Second Post (1 comment)
	assert_eq!(rows[2].get::<String, _>("name"), "Alice");
	assert_eq!(rows[2].get::<String, _>("title"), "Second Post");
	assert_eq!(rows[2].get::<String, _>("content"), "Nice");
}

/// Test self-JOIN
///
/// JOIN Users table with itself to create pairs in lexicographical order by name
#[rstest]
#[tokio::test]
async fn test_self_join(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_tables(pool.as_ref()).await;

	// Self-JOIN using aliases
	let u1 = Alias::new("u1");
	let u2 = Alias::new("u2");

	let query = Query::select()
		.expr_as(Expr::col((u1.clone(), Users::Name)), Alias::new("user1"))
		.expr_as(Expr::col((u2.clone(), Users::Name)), Alias::new("user2"))
		.from_as(Users::Table, u1.clone())
		.join(
			JoinType::InnerJoin,
			(Users::Table, u2.clone()),
			Expr::col((u1.clone(), Users::Id)).lt(Expr::col((u2.clone(), Users::Id))),
		)
		.order_by((u1.clone(), Users::Name), sea_query::Order::Asc)
		.order_by((u2.clone(), Users::Name), sea_query::Order::Asc)
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&query).fetch_all(pool.as_ref()).await.unwrap();

	// Combinations of choosing 2 users from 3 (3C2 = 3 combinations)
	assert_eq!(rows.len(), 3);

	assert_eq!(rows[0].get::<String, _>("user1"), "Alice");
	assert_eq!(rows[0].get::<String, _>("user2"), "Bob");

	assert_eq!(rows[1].get::<String, _>("user1"), "Alice");
	assert_eq!(rows[1].get::<String, _>("user2"), "Charlie");

	assert_eq!(rows[2].get::<String, _>("user1"), "Bob");
	assert_eq!(rows[2].get::<String, _>("user2"), "Charlie");
}

/// Test JOIN with complex conditions
///
/// JOIN conditions combining multiple conditions with AND
#[rstest]
#[tokio::test]
async fn test_join_with_complex_conditions(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_tables(pool.as_ref()).await;

	// Additional test data: posts with specific title pattern
	sqlx::query("INSERT INTO posts (user_id, title) VALUES (2, 'First Post')")
		.execute(pool.as_ref())
		.await
		.unwrap();

	// Join only posts where user_id matches and title starts with 'First Post'
	let query = Query::select()
		.column((Users::Table, Users::Name))
		.column((Posts::Table, Posts::Title))
		.from(Users::Table)
		.join(
			JoinType::InnerJoin,
			Posts::Table,
			Expr::col((Posts::Table, Posts::UserId))
				.equals((Users::Table, Users::Id))
				.and(Expr::col((Posts::Table, Posts::Title)).like("First%")),
		)
		.order_by((Users::Table, Users::Id), sea_query::Order::Asc)
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&query).fetch_all(pool.as_ref()).await.unwrap();

	// Only Alice and Bob's posts with 'First Post' title
	assert_eq!(rows.len(), 2);
	assert_eq!(rows[0].get::<String, _>("name"), "Alice");
	assert_eq!(rows[0].get::<String, _>("title"), "First Post");
	assert_eq!(rows[1].get::<String, _>("name"), "Bob");
	assert_eq!(rows[1].get::<String, _>("title"), "First Post");
}

/// Test CROSS JOIN
///
/// Retrieve the Cartesian product of Users and Posts
#[rstest]
#[tokio::test]
async fn test_cross_join(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_tables(pool.as_ref()).await;

	let query = Query::select()
		.column((Users::Table, Users::Name))
		.column((Posts::Table, Posts::Title))
		.from(Users::Table)
		.join(JoinType::CrossJoin, Posts::Table, Expr::cust(""))
		.order_by((Users::Table, Users::Id), sea_query::Order::Asc)
		.order_by((Posts::Table, Posts::Id), sea_query::Order::Asc)
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&query).fetch_all(pool.as_ref()).await.unwrap();

	// 3 users × 3 posts = 9 rows
	assert_eq!(rows.len(), 9);

	// First 3 rows are combinations of Alice with each post
	assert_eq!(rows[0].get::<String, _>("name"), "Alice");
	assert_eq!(rows[0].get::<String, _>("title"), "First Post");
	assert_eq!(rows[1].get::<String, _>("name"), "Alice");
	assert_eq!(rows[1].get::<String, _>("title"), "Second Post");
	assert_eq!(rows[2].get::<String, _>("name"), "Alice");
	assert_eq!(rows[2].get::<String, _>("title"), "Bob Post");

	// Next 3 rows are combinations of Bob with each post
	assert_eq!(rows[3].get::<String, _>("name"), "Bob");
	assert_eq!(rows[4].get::<String, _>("name"), "Bob");
	assert_eq!(rows[5].get::<String, _>("name"), "Bob");

	// Last 3 rows are combinations of Charlie with each post
	assert_eq!(rows[6].get::<String, _>("name"), "Charlie");
	assert_eq!(rows[7].get::<String, _>("name"), "Charlie");
	assert_eq!(rows[8].get::<String, _>("name"), "Charlie");
}

/// Test aggregation after JOIN
///
/// Retrieve post count per user
#[rstest]
#[tokio::test]
async fn test_join_with_aggregation(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_tables(pool.as_ref()).await;

	let query = Query::select()
		.column((Users::Table, Users::Name))
		.expr_as(
			Expr::col((Posts::Table, Posts::Id)).count(),
			Alias::new("post_count"),
		)
		.from(Users::Table)
		.join(
			JoinType::LeftJoin,
			Posts::Table,
			Expr::col((Posts::Table, Posts::UserId)).equals((Users::Table, Users::Id)),
		)
		.group_by_col((Users::Table, Users::Name))
		.order_by((Users::Table, Users::Name), sea_query::Order::Asc)
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&query).fetch_all(pool.as_ref()).await.unwrap();

	assert_eq!(rows.len(), 3);

	assert_eq!(rows[0].get::<String, _>("name"), "Alice");
	assert_eq!(rows[0].get::<i64, _>("post_count"), 2);

	assert_eq!(rows[1].get::<String, _>("name"), "Bob");
	assert_eq!(rows[1].get::<i64, _>("post_count"), 1);

	assert_eq!(rows[2].get::<String, _>("name"), "Charlie");
	assert_eq!(rows[2].get::<i64, _>("post_count"), 0);
}

/// Test combining subquery with JOIN
///
/// Retrieve users who have posts with 2 or more comments
#[rstest]
#[tokio::test]
async fn test_join_with_subquery(
	#[future] postgres_container: (ContainerAsync<GenericImage>, Arc<PgPool>, u16, String),
) {
	let (_container, pool, _port, _url) = postgres_container.await;
	setup_tables(pool.as_ref()).await;

	// Subquery: Retrieve IDs of posts with 2 or more comments
	let subquery = Query::select()
		.column(Comments::PostId)
		.from(Comments::Table)
		.group_by_col(Comments::PostId)
		.cond_having(Expr::col(Comments::Id).count().gte(2))
		.to_owned();

	let query = Query::select()
		.column((Users::Table, Users::Name))
		.column((Posts::Table, Posts::Title))
		.from(Users::Table)
		.join(
			JoinType::InnerJoin,
			Posts::Table,
			Expr::col((Posts::Table, Posts::UserId)).equals((Users::Table, Users::Id)),
		)
		.and_where(Expr::col((Posts::Table, Posts::Id)).in_subquery(subquery))
		.to_string(PostgresQueryBuilder);

	let rows = sqlx::query(&query).fetch_all(pool.as_ref()).await.unwrap();

	// Only First Post has 2 comments
	assert_eq!(rows.len(), 1);
	assert_eq!(rows[0].get::<String, _>("name"), "Alice");
	assert_eq!(rows[0].get::<String, _>("title"), "First Post");
}
