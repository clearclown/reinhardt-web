//! Common test utilities for integration tests
//!
//! This module provides shared functionality for all integration tests including:
//! - Test database setup/teardown
//! - Model definitions (User, Post, Tag)
//! - Test data seeding
//! - SQL execution helpers

use reinhardt_database::Database;
use reinhardt_filters::SearchableModel;
use reinhardt_orm::{DatabaseConnection, Field, Model};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashMap;
use tempfile::NamedTempFile;

/// Test database wrapper
pub struct TestDb {
    pub pool: SqlitePool,
    pub db: DatabaseConnection,
    _temp_file: NamedTempFile,
}

/// Setup a fresh SQLite test database
pub async fn setup_test_db() -> TestDb {
    let temp_file = NamedTempFile::new().expect("Failed to create temp file");
    let db_url = format!("sqlite:{}", temp_file.path().display());

    let pool = SqlitePool::connect(&db_url)
        .await
        .expect("Failed to connect to test database");

    let db = DatabaseConnection::from_sqlite_pool(pool.clone());

    // Create schema
    create_schema(&db).await;

    TestDb {
        pool,
        db,
        _temp_file: temp_file,
    }
}

/// Create database schema
async fn create_schema(db: &DatabaseConnection) {
    // Use raw SQL for DDL operations (CREATE TABLE) as these should be handled by migrations
    db.execute(
        r#"
        CREATE TABLE users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL,
            email TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
        vec![],
    )
    .await
    .expect("Failed to create users table");

    db.execute(
        r#"
        CREATE TABLE posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            author_id INTEGER NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (author_id) REFERENCES users(id)
        )
        "#,
        vec![],
    )
    .await
    .expect("Failed to create posts table");

    db.execute(
        r#"
        CREATE TABLE tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )
        "#,
        vec![],
    )
    .await
    .expect("Failed to create tags table");

    db.execute(
        r#"
        CREATE TABLE post_tags (
            post_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (post_id, tag_id),
            FOREIGN KEY (post_id) REFERENCES posts(id),
            FOREIGN KEY (tag_id) REFERENCES tags(id)
        )
        "#,
        vec![],
    )
    .await
    .expect("Failed to create post_tags table");
}

/// Seed test data
pub async fn seed_test_data(db: &DatabaseConnection) {
    // Insert users using From trait implementation for automatic conversion
    db.insert("users")
        .value("username", "alice")
        .value("email", "alice@example.com")
        .execute()
        .await
        .expect("Failed to insert user");

    db.insert("users")
        .value("username", "bob")
        .value("email", "bob@example.com")
        .execute()
        .await
        .expect("Failed to insert user");

    db.insert("users")
        .value("username", "charlie")
        .value("email", "charlie@example.com")
        .execute()
        .await
        .expect("Failed to insert user");

    // Insert posts
    db.insert("posts")
        .value("title", "Hello World")
        .value("content", "First post")
        .value("author_id", 1_i64)
        .execute()
        .await
        .expect("Failed to insert post");

    db.insert("posts")
        .value("title", "Rust Tutorial")
        .value("content", "Learning Rust")
        .value("author_id", 2_i64)
        .execute()
        .await
        .expect("Failed to insert post");

    db.insert("posts")
        .value("title", "Hello Rust")
        .value("content", "Rust is great")
        .value("author_id", 1_i64)
        .execute()
        .await
        .expect("Failed to insert post");

    db.insert("posts")
        .value("title", "Advanced Rust")
        .value("content", "Lifetimes and traits")
        .value("author_id", 3_i64)
        .execute()
        .await
        .expect("Failed to insert post");

    // Insert tags
    db.insert("tags")
        .value("name", "programming")
        .execute()
        .await
        .expect("Failed to insert tag");

    db.insert("tags")
        .value("name", "tutorial")
        .execute()
        .await
        .expect("Failed to insert tag");

    db.insert("tags")
        .value("name", "rust")
        .execute()
        .await
        .expect("Failed to insert tag");

    // Link posts to tags
    db.insert("post_tags")
        .value("post_id", 1_i64)
        .value("tag_id", 1_i64)
        .execute()
        .await
        .expect("Failed to insert post_tag");

    db.insert("post_tags")
        .value("post_id", 2_i64)
        .value("tag_id", 2_i64)
        .execute()
        .await
        .expect("Failed to insert post_tag");

    db.insert("post_tags")
        .value("post_id", 2_i64)
        .value("tag_id", 3_i64)
        .execute()
        .await
        .expect("Failed to insert post_tag");

    db.insert("post_tags")
        .value("post_id", 3_i64)
        .value("tag_id", 3_i64)
        .execute()
        .await
        .expect("Failed to insert post_tag");
}

/// Execute SQL query and return row count
pub async fn execute_and_count(db: &DatabaseConnection, sql: &str) -> usize {
    let rows = db
        .fetch_all(sql, vec![])
        .await
        .expect("Failed to execute query");
    rows.len()
}

/// Execute SQL query and return rows as HashMaps
pub async fn execute_and_fetch(db: &DatabaseConnection, sql: &str) -> Vec<HashMap<String, String>> {
    use reinhardt_database::QueryValue;

    let rows = db
        .fetch_all(sql, vec![])
        .await
        .expect("Failed to execute query");

    rows.iter()
        .map(|row| {
            let mut map = HashMap::new();
            for (key, value) in row.columns() {
                let string_value = match value {
                    QueryValue::String(s) => s.clone(),
                    QueryValue::Int(i) => i.to_string(),
                    QueryValue::Float(f) => f.to_string(),
                    QueryValue::Bool(b) => b.to_string(),
                    QueryValue::Null => "NULL".to_string(),
                    QueryValue::Bytes(b) => format!("{:?}", b),
                    QueryValue::Timestamp(t) => t.to_string(),
                };
                map.insert(key.clone(), string_value);
            }
            map
        })
        .collect()
}

// Test model definitions for type-safe filtering

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub created_at: String,
}

impl Model for User {
    type PrimaryKey = i64;

    fn table_name() -> &'static str {
        "users"
    }

    fn primary_key(&self) -> Option<&Self::PrimaryKey> {
        Some(&self.id)
    }

    fn set_primary_key(&mut self, value: Self::PrimaryKey) {
        self.id = value;
    }
}

impl SearchableModel for User {
    fn searchable_fields() -> Vec<Field<Self, String>> {
        vec![
            Field::<User, String>::new(vec!["username"]),
            Field::<User, String>::new(vec!["email"]),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
    pub created_at: String,
}

impl Model for Post {
    type PrimaryKey = i64;

    fn table_name() -> &'static str {
        "posts"
    }

    fn primary_key(&self) -> Option<&Self::PrimaryKey> {
        Some(&self.id)
    }

    fn set_primary_key(&mut self, value: Self::PrimaryKey) {
        self.id = value;
    }
}

impl SearchableModel for Post {
    fn searchable_fields() -> Vec<Field<Self, String>> {
        vec![
            Field::<Post, String>::new(vec!["title"]),
            Field::<Post, String>::new(vec!["content"]),
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

impl Model for Tag {
    type PrimaryKey = i64;

    fn table_name() -> &'static str {
        "tags"
    }

    fn primary_key(&self) -> Option<&Self::PrimaryKey> {
        Some(&self.id)
    }

    fn set_primary_key(&mut self, value: Self::PrimaryKey) {
        self.id = value;
    }
}

impl SearchableModel for Tag {
    fn searchable_fields() -> Vec<Field<Self, String>> {
        vec![Field::<Tag, String>::new(vec!["name"])]
    }
}
