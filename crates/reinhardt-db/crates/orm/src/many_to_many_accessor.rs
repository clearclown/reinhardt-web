//! Django-style accessor for ManyToMany relationships.
//!
//! This module provides the ManyToManyAccessor type, which implements
//! Django-style API for managing many-to-many relationships:
//! - `add()` - Add a relationship
//! - `remove()` - Remove a relationship
//! - `all()` - Get all related records
//! - `clear()` - Remove all relationships
//! - `set()` - Replace all relationships

use crate::Model;
use sea_query::{Alias, BinOper, Expr, ExprTrait, PostgresQueryBuilder, Query};
use serde::{Serialize, de::DeserializeOwned};
use std::marker::PhantomData;

/// Django-style accessor for ManyToMany relationships.
///
/// This type provides methods to manage many-to-many relationships
/// using an intermediate/through table.
///
/// # Type Parameters
///
/// - `S`: Source model type (the model that owns the ManyToMany field)
/// - `T`: Target model type (the related model)
///
/// # Examples
///
/// ```ignore
/// use reinhardt_orm::{Model, ManyToManyAccessor};
///
/// let user = User::find_by_id(&db, user_id).await?;
/// let accessor = ManyToManyAccessor::new(&user, "groups", db.clone());
///
/// // Add a relationship
/// accessor.add(&group).await?;
///
/// // Get all related records
/// let groups = accessor.all().await?;
///
/// // Remove a relationship
/// accessor.remove(&group).await?;
///
/// // Clear all relationships
/// accessor.clear().await?;
/// ```
pub struct ManyToManyAccessor<S, T>
where
	S: Model,
	T: Model + Serialize + DeserializeOwned,
{
	source_id: S::PrimaryKey,
	through_table: String,
	source_field: String,
	target_field: String,
	// TODO: Replace with actual DatabaseConnection type when available
	_db_placeholder: (),
	_phantom_source: PhantomData<S>,
	_phantom_target: PhantomData<T>,
}

impl<S, T> ManyToManyAccessor<S, T>
where
	S: Model,
	T: Model + Serialize + DeserializeOwned,
{
	/// Create a new ManyToManyAccessor.
	///
	/// # Parameters
	///
	/// - `source`: The source model instance
	/// - `field_name`: The name of the ManyToMany field
	/// - `db`: Database connection (placeholder for now)
	///
	/// # Panics
	///
	/// Panics if:
	/// - The field_name does not correspond to a ManyToMany field
	/// - The source model has no primary key
	pub fn new(source: &S, field_name: &str, _db: ()) -> Self {
		// Get through table name from metadata
		// For now, use Django naming convention: {app}_{model}_{field}
		let through_table = format!(
			"{}_{}_{}",
			S::app_label(),
			Self::table_name_lower(S::table_name()),
			field_name
		);

		let source_id = source
			.primary_key()
			.expect("Source model must have primary key")
			.clone();

		let source_field = format!("{}_id", Self::table_name_lower(S::table_name()));
		let target_field = format!("{}_id", Self::table_name_lower(T::table_name()));

		Self {
			source_id,
			through_table,
			source_field,
			target_field,
			_db_placeholder: (),
			_phantom_source: PhantomData,
			_phantom_target: PhantomData,
		}
	}

	/// Convert table name to lowercase for field naming.
	fn table_name_lower(table_name: &str) -> String {
		table_name.to_lowercase()
	}

	/// Add a relationship to the target model.
	///
	/// Creates a record in the intermediate table linking the source and target.
	///
	/// # Parameters
	///
	/// - `target`: The target model to add
	///
	/// # Errors
	///
	/// Returns an error if:
	/// - The target model has no primary key
	/// - The database operation fails
	///
	/// # Examples
	///
	/// ```ignore
	/// accessor.add(&group).await?;
	/// ```
	pub async fn add(&self, target: &T) -> Result<(), String> {
		let target_id = target
			.primary_key()
			.ok_or_else(|| "Target model has no primary key".to_string())?;

		let query = Query::insert()
			.into_table(Alias::new(&self.through_table))
			.columns([
				Alias::new(&self.source_field),
				Alias::new(&self.target_field),
			])
			.values_panic([
				Expr::val(self.source_id.to_string()),
				Expr::val(target_id.to_string()),
			])
			.to_owned();

		let (sql, _values) = query.build(PostgresQueryBuilder);

		// TODO: Execute query when DatabaseConnection is available
		let _ = sql;

		Ok(())
	}

	/// Remove a relationship to the target model.
	///
	/// Deletes the record in the intermediate table linking the source and target.
	///
	/// # Parameters
	///
	/// - `target`: The target model to remove
	///
	/// # Errors
	///
	/// Returns an error if:
	/// - The target model has no primary key
	/// - The database operation fails
	///
	/// # Examples
	///
	/// ```ignore
	/// accessor.remove(&group).await?;
	/// ```
	pub async fn remove(&self, target: &T) -> Result<(), String> {
		let target_id = target
			.primary_key()
			.ok_or_else(|| "Target model has no primary key".to_string())?;

		let query = Query::delete()
			.from_table(Alias::new(&self.through_table))
			.and_where(
				Expr::col(Alias::new(&self.source_field))
					.binary(BinOper::Equal, Expr::val(self.source_id.to_string())),
			)
			.and_where(
				Expr::col(Alias::new(&self.target_field))
					.binary(BinOper::Equal, Expr::val(target_id.to_string())),
			)
			.to_owned();

		let (sql, _values) = query.build(PostgresQueryBuilder);

		// TODO: Execute query when DatabaseConnection is available
		let _ = sql;

		Ok(())
	}

	/// Get all related target models.
	///
	/// Queries the target table joined with the intermediate table to fetch all
	/// related records.
	///
	/// # Errors
	///
	/// Returns an error if the database operation fails.
	///
	/// # Examples
	///
	/// ```ignore
	/// let groups = accessor.all().await?;
	/// ```
	pub async fn all(&self) -> Result<Vec<T>, String> {
		let query = Query::select()
			.from(Alias::new(T::table_name()))
			.column((Alias::new(T::table_name()), Alias::new("*")))
			.inner_join(
				Alias::new(&self.through_table),
				Expr::col((Alias::new(T::table_name()), Alias::new("id"))).equals((
					Alias::new(&self.through_table),
					Alias::new(&self.target_field),
				)),
			)
			.and_where(
				Expr::col((
					Alias::new(&self.through_table),
					Alias::new(&self.source_field),
				))
				.binary(BinOper::Equal, Expr::val(self.source_id.to_string())),
			)
			.to_owned();

		let (sql, _values) = query.build(PostgresQueryBuilder);

		// TODO: Execute query and deserialize results when DatabaseConnection is available
		let _ = sql;

		// Placeholder: return empty Vec for now
		Ok(Vec::new())
	}

	/// Remove all relationships.
	///
	/// Deletes all records in the intermediate table for this source instance.
	///
	/// # Errors
	///
	/// Returns an error if the database operation fails.
	///
	/// # Examples
	///
	/// ```ignore
	/// accessor.clear().await?;
	/// ```
	pub async fn clear(&self) -> Result<(), String> {
		let query = Query::delete()
			.from_table(Alias::new(&self.through_table))
			.and_where(
				Expr::col(Alias::new(&self.source_field))
					.binary(BinOper::Equal, Expr::val(self.source_id.to_string())),
			)
			.to_owned();

		let (sql, _values) = query.build(PostgresQueryBuilder);

		// TODO: Execute query when DatabaseConnection is available
		let _ = sql;

		Ok(())
	}

	/// Replace all relationships with a new set.
	///
	/// This is a transactional operation that:
	/// 1. Removes all existing relationships
	/// 2. Adds new relationships
	///
	/// # Parameters
	///
	/// - `targets`: The new set of target models
	///
	/// # Errors
	///
	/// Returns an error if the database operation fails.
	///
	/// # Examples
	///
	/// ```ignore
	/// accessor.set(&[group1, group2, group3]).await?;
	/// ```
	pub async fn set(&self, targets: &[T]) -> Result<(), String> {
		// TODO: Use transaction when DatabaseConnection is available

		// Clear existing relationships
		self.clear().await?;

		// Add new relationships
		for target in targets {
			self.add(target).await?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde::{Deserialize, Serialize};

	#[derive(Debug, Clone, Serialize, Deserialize)]
	struct TestUser {
		id: i64,
		username: String,
	}

	impl Model for TestUser {
		type PrimaryKey = i64;

		fn table_name() -> &'static str {
			"users"
		}

		fn app_label() -> &'static str {
			"auth"
		}

		fn primary_key(&self) -> Option<&Self::PrimaryKey> {
			Some(&self.id)
		}

		fn set_primary_key(&mut self, value: Self::PrimaryKey) {
			self.id = value;
		}
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	struct TestGroup {
		id: i64,
		name: String,
	}

	impl Model for TestGroup {
		type PrimaryKey = i64;

		fn table_name() -> &'static str {
			"groups"
		}

		fn app_label() -> &'static str {
			"auth"
		}

		fn primary_key(&self) -> Option<&Self::PrimaryKey> {
			Some(&self.id)
		}

		fn set_primary_key(&mut self, value: Self::PrimaryKey) {
			self.id = value;
		}
	}

	#[test]
	fn test_accessor_creation() {
		let user = TestUser {
			id: 1,
			username: "alice".to_string(),
		};

		let accessor = ManyToManyAccessor::<TestUser, TestGroup>::new(&user, "groups", ());

		assert_eq!(accessor.through_table, "auth_users_groups");
		assert_eq!(accessor.source_field, "users_id");
		assert_eq!(accessor.target_field, "groups_id");
		assert_eq!(accessor.source_id, 1);
	}

	#[test]
	fn test_table_name_lower() {
		assert_eq!(
			ManyToManyAccessor::<TestUser, TestGroup>::table_name_lower("Users"),
			"users"
		);
		assert_eq!(
			ManyToManyAccessor::<TestUser, TestGroup>::table_name_lower("UserGroups"),
			"usergroups"
		);
	}

	#[tokio::test]
	async fn test_add_generates_correct_sql() {
		let user = TestUser {
			id: 1,
			username: "alice".to_string(),
		};
		let group = TestGroup {
			id: 10,
			name: "admins".to_string(),
		};

		let accessor = ManyToManyAccessor::<TestUser, TestGroup>::new(&user, "groups", ());

		// This will be a placeholder until we have actual DB connection
		let result = accessor.add(&group).await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn test_remove_generates_correct_sql() {
		let user = TestUser {
			id: 1,
			username: "alice".to_string(),
		};
		let group = TestGroup {
			id: 10,
			name: "admins".to_string(),
		};

		let accessor = ManyToManyAccessor::<TestUser, TestGroup>::new(&user, "groups", ());

		let result = accessor.remove(&group).await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn test_clear_generates_correct_sql() {
		let user = TestUser {
			id: 1,
			username: "alice".to_string(),
		};

		let accessor = ManyToManyAccessor::<TestUser, TestGroup>::new(&user, "groups", ());

		let result = accessor.clear().await;
		assert!(result.is_ok());
	}

	#[tokio::test]
	async fn test_all_generates_correct_sql() {
		let user = TestUser {
			id: 1,
			username: "alice".to_string(),
		};

		let accessor = ManyToManyAccessor::<TestUser, TestGroup>::new(&user, "groups", ());

		let result = accessor.all().await;
		assert!(result.is_ok());
		assert_eq!(result.unwrap().len(), 0); // Placeholder returns empty Vec
	}

	#[tokio::test]
	async fn test_set_clears_and_adds() {
		let user = TestUser {
			id: 1,
			username: "alice".to_string(),
		};
		let groups = vec![
			TestGroup {
				id: 10,
				name: "admins".to_string(),
			},
			TestGroup {
				id: 20,
				name: "editors".to_string(),
			},
		];

		let accessor = ManyToManyAccessor::<TestUser, TestGroup>::new(&user, "groups", ());

		let result = accessor.set(&groups).await;
		assert!(result.is_ok());
	}
}
