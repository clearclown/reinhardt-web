//! Custom assertion helpers for tests.
//!
//! Provides trait extensions for common test assertions.

use crate::apps::auth::models::User;
use reinhardt::db::DatabaseConnection;
use serde_json::Value;
use uuid::Uuid;

/// Database query helpers for assertions.
pub struct DbAssertions;

impl DbAssertions {
	/// Check if a user exists in the database by email.
	pub async fn user_exists_by_email(db: &DatabaseConnection, email: &str) -> bool {
		let sql = format!(
			"SELECT COUNT(*) as count FROM auth_user WHERE email = '{}'",
			email
		);
		let result = db.query_one(&sql, vec![]).await;
		match result {
			Ok(row) => {
				let count: i64 = row.get("count").unwrap_or(0);
				count > 0
			}
			Err(_) => false,
		}
	}

	/// Get a user from the database by ID.
	pub async fn get_user_by_id(db: &DatabaseConnection, id: Uuid) -> Option<User> {
		let sql = format!(
			r#"SELECT id, username, email, password_hash, is_active, last_login, created_at
			FROM auth_user WHERE id = '{}'"#,
			id
		);

		match db.query_one(&sql, vec![]).await {
			Ok(row) => Some(User {
				id: row.get("id").expect("id"),
				username: row.get("username").expect("username"),
				email: row.get("email").expect("email"),
				password_hash: row.get("password_hash"),
				is_active: row.get("is_active").expect("is_active"),
				last_login: row.get("last_login"),
				created_at: row.get("created_at").expect("created_at"),
				following: Default::default(),
				blocked_users: Default::default(),
			}),
			Err(_) => None,
		}
	}

	/// Count users in the database.
	pub async fn count_users(db: &DatabaseConnection) -> i64 {
		let sql = "SELECT COUNT(*) as count FROM auth_user";
		match db.query_one(sql, vec![]).await {
			Ok(row) => row.get("count").unwrap_or(0),
			Err(_) => 0,
		}
	}

	/// Delete all users from the database (for cleanup).
	pub async fn cleanup_users(db: &DatabaseConnection) {
		let _ = db.execute("DELETE FROM auth_user_following", vec![]).await;
		let _ = db.execute("DELETE FROM auth_user_blocked_users", vec![]).await;
		let _ = db.execute("DELETE FROM auth_user", vec![]).await;
	}

	/// Delete a specific user by ID.
	pub async fn delete_user(db: &DatabaseConnection, id: Uuid) {
		let sql = format!("DELETE FROM auth_user WHERE id = '{}'", id);
		let _ = db.execute(&sql, vec![]).await;
	}
}

/// JSON assertion helpers.
pub trait JsonAssertions {
	/// Assert that JSON contains a specific key with a value.
	fn assert_json_contains(&self, key: &str, expected_value: &str);

	/// Assert that JSON has a specific key.
	fn assert_json_has_key(&self, key: &str);
}

impl JsonAssertions for Value {
	fn assert_json_contains(&self, key: &str, expected_value: &str) {
		let actual = self.get(key).and_then(|v| v.as_str());
		assert_eq!(
			actual,
			Some(expected_value),
			"Expected key '{}' to have value '{}', but got {:?}",
			key,
			expected_value,
			actual
		);
	}

	fn assert_json_has_key(&self, key: &str) {
		assert!(
			self.get(key).is_some(),
			"Expected JSON to have key '{}', but it was missing",
			key
		);
	}
}
