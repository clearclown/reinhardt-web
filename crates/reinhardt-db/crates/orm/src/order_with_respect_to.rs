//! Order with respect to functionality
//!
//! Django's order_with_respect_to allows automatic ordering of model instances
//! relative to a parent model or set of fields.

use crate::query_types::DbBackend;
use sea_query::{
	Alias, Expr, ExprTrait, Func, MysqlQueryBuilder, PostgresQueryBuilder, Query as SeaQuery,
	SqliteQueryBuilder,
};
use serde::{Deserialize, Serialize};
use sqlx::{AnyPool, Row};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

/// Error type for ordering operations
#[derive(Debug)]
pub enum OrderError {
	InvalidOrder(String),
	OrderFieldNotFound(String),
	UpdateFailed(String),
}

impl fmt::Display for OrderError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			OrderError::InvalidOrder(msg) => write!(f, "Invalid order value: {}", msg),
			OrderError::OrderFieldNotFound(msg) => write!(f, "Order field not found: {}", msg),
			OrderError::UpdateFailed(msg) => write!(f, "Failed to update order: {}", msg),
		}
	}
}

impl std::error::Error for OrderError {}

/// Value type for filter conditions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OrderValue {
	Integer(i64),
	String(String),
	Boolean(bool),
}

/// Manages ordering for models with order_with_respect_to
///
/// # Examples
///
/// ```
/// use reinhardt_orm::order_with_respect_to::OrderedModel;
/// use std::collections::HashMap;
/// use std::sync::Arc;
/// use sqlx::AnyPool;
///
/// # async fn example() {
/// # let pool = Arc::new(AnyPool::connect("sqlite::memory:").await.unwrap());
/// let ordered = OrderedModel::new(
///     "order".to_string(),
///     vec!["parent_id".to_string()],
///     "items".to_string(),
///     pool,
/// );
///
/// assert_eq!(ordered.order_field(), "order");
/// assert_eq!(ordered.order_with_respect_to(), &["parent_id"]);
/// # }
/// ```
pub struct OrderedModel {
	/// Field name for storing the order
	order_field: String,
	/// Fields that define the ordering scope
	order_with_respect_to: Vec<String>,
	/// Table name
	table_name: String,
	/// Database connection pool
	pool: Arc<AnyPool>,
}

impl OrderedModel {
	/// Creates a new OrderedModel
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_orm::order_with_respect_to::OrderedModel;
	/// use std::sync::Arc;
	/// use sqlx::AnyPool;
	///
	/// # async fn example() {
	/// # let pool = Arc::new(AnyPool::connect("sqlite::memory:").await.unwrap());
	/// let ordered = OrderedModel::new(
	///     "_order".to_string(),
	///     vec!["category_id".to_string()],
	///     "products".to_string(),
	///     pool,
	/// );
	///
	/// assert_eq!(ordered.order_field(), "_order");
	/// # }
	/// ```
	pub fn new(
		order_field: String,
		order_with_respect_to: Vec<String>,
		table_name: String,
		pool: Arc<AnyPool>,
	) -> Self {
		Self {
			order_field,
			order_with_respect_to,
			table_name,
			pool,
		}
	}

	/// Gets the order field name
	pub fn order_field(&self) -> &str {
		&self.order_field
	}

	/// Gets the fields that define the ordering scope
	pub fn order_with_respect_to(&self) -> &[String] {
		&self.order_with_respect_to
	}

	/// Gets the next order value for a given filter scope
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_orm::order_with_respect_to::{OrderedModel, OrderValue};
	/// use std::collections::HashMap;
	/// use std::sync::Arc;
	/// use sqlx::AnyPool;
	///
	/// # async fn example() {
	/// # let pool = Arc::new(AnyPool::connect("sqlite::memory:").await.unwrap());
	/// let ordered = OrderedModel::new(
	///     "order".to_string(),
	///     vec!["parent_id".to_string()],
	///     "items".to_string(),
	///     pool,
	/// );
	///
	/// let mut filters = HashMap::new();
	/// filters.insert("parent_id".to_string(), OrderValue::Integer(1));
	///
	/// let next_order = ordered.get_next_order(filters).await.unwrap();
	/// // Returns the next available order number in the scope
	/// # }
	/// ```
	pub async fn get_next_order(
		&self,
		filters: HashMap<String, OrderValue>,
	) -> Result<i32, OrderError> {
		let backend = self.get_backend();

		// Build SELECT MAX(order_field) FROM table WHERE filters
		let mut select_stmt = SeaQuery::select()
			.from(Alias::new(&self.table_name))
			.expr(Func::max(Expr::col(Alias::new(&self.order_field))))
			.to_owned();

		// Add WHERE clauses for filters
		for (col_name, col_value) in &filters {
			select_stmt.and_where(
				Expr::col(Alias::new(col_name)).eq(Expr::val(order_value_to_sea_value(col_value))),
			);
		}

		// Build SQL based on backend
		let (sql, values) = match backend {
			DbBackend::Postgres => select_stmt.build(PostgresQueryBuilder),
			DbBackend::Mysql => select_stmt.build(MysqlQueryBuilder),
			DbBackend::Sqlite => select_stmt.build(SqliteQueryBuilder),
		};

		// Execute query
		let mut query = sqlx::query(&sql);
		for value in &values.0 {
			query = bind_sea_value(query, value);
		}

		let row = query
			.fetch_optional(&*self.pool)
			.await
			.map_err(|e| OrderError::UpdateFailed(format!("Failed to query max order: {}", e)))?;

		if let Some(row) = row {
			// Extract max value - it could be NULL if no rows exist
			let max_order: Option<i32> = row.try_get(0).map_err(|e| {
				OrderError::UpdateFailed(format!("Failed to get max order value: {}", e))
			})?;

			Ok(max_order.map(|v| v + 1).unwrap_or(0))
		} else {
			// No rows in this scope, start from 0
			Ok(0)
		}
	}

	/// Get database backend type
	fn get_backend(&self) -> DbBackend {
		// TODO: Detect actual backend from connection pool
		// Temporary workaround - defaults to Postgres
		DbBackend::Postgres
	}

	/// Moves an object up in the ordering (decreases order value)
	pub async fn move_up(&self, current_order: i32) -> Result<i32, OrderError> {
		if current_order <= 0 {
			return Err(OrderError::InvalidOrder(
				"Cannot move up from position 0".to_string(),
			));
		}
		Ok(current_order - 1)
	}

	/// Moves an object down in the ordering (increases order value)
	pub async fn move_down(&self, current_order: i32, max_order: i32) -> Result<i32, OrderError> {
		if current_order >= max_order {
			return Err(OrderError::InvalidOrder(format!(
				"Cannot move down from max position {}",
				max_order
			)));
		}
		Ok(current_order + 1)
	}

	/// Moves an object to a specific position in the ordering
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_orm::order_with_respect_to::OrderedModel;
	///
	/// let ordered = OrderedModel::new(
	///     "order".to_string(),
	///     vec!["parent_id".to_string()],
	/// );
	///
	/// # tokio_test::block_on(async {
	/// let new_order = ordered.move_to_position(5, 10, 3).await.unwrap();
	/// assert_eq!(new_order, 3);
	/// # });
	/// ```
	pub async fn move_to_position(
		&self,
		_current_order: i32,
		max_order: i32,
		new_position: i32,
	) -> Result<i32, OrderError> {
		if new_position < 0 || new_position > max_order {
			return Err(OrderError::InvalidOrder(format!(
				"Invalid position: {} (max: {})",
				new_position, max_order
			)));
		}
		Ok(new_position)
	}

	/// Swaps the order of two objects
	pub async fn swap_order(&self, order1: i32, order2: i32) -> Result<(i32, i32), OrderError> {
		Ok((order2, order1))
	}

	/// Reorders all objects in a scope sequentially (0, 1, 2, ...)
	///
	/// This fetches all records in the scope, orders them by the order field,
	/// and updates each one with sequential order values starting from 0.
	///
	/// Returns the list of new order values assigned.
	pub async fn reorder_all(
		&self,
		filters: HashMap<String, OrderValue>,
	) -> Result<Vec<i32>, OrderError> {
		let backend = self.get_backend();

		// Step 1: Query all IDs in the scope, ordered by current order field
		let mut select_stmt = SeaQuery::select()
			.from(Alias::new(&self.table_name))
			.column(Alias::new("id"))
			.column(Alias::new(&self.order_field))
			.order_by(Alias::new(&self.order_field), sea_query::Order::Asc)
			.to_owned();

		// Add WHERE clauses for filters
		for (col_name, col_value) in &filters {
			select_stmt.and_where(
				Expr::col(Alias::new(col_name)).eq(Expr::val(order_value_to_sea_value(col_value))),
			);
		}

		// Build and execute SELECT query
		let (sql, values) = match backend {
			DbBackend::Postgres => select_stmt.build(PostgresQueryBuilder),
			DbBackend::Mysql => select_stmt.build(MysqlQueryBuilder),
			DbBackend::Sqlite => select_stmt.build(SqliteQueryBuilder),
		};

		let mut query = sqlx::query(&sql);
		for value in &values.0 {
			query = bind_sea_value(query, value);
		}

		let rows = query.fetch_all(&*self.pool).await.map_err(|e| {
			OrderError::UpdateFailed(format!("Failed to fetch records for reordering: {}", e))
		})?;

		let mut new_orders = Vec::new();

		// Step 2: Update each record with sequential order values
		for (idx, row) in rows.iter().enumerate() {
			let id: i64 = row.try_get("id").map_err(|e| {
				OrderError::UpdateFailed(format!("Failed to get id from row: {}", e))
			})?;

			let new_order = idx as i32;
			new_orders.push(new_order);

			// Build UPDATE statement for this record
			let update_stmt = SeaQuery::update()
				.table(Alias::new(&self.table_name))
				.value(Alias::new(&self.order_field), new_order)
				.and_where(Expr::col(Alias::new("id")).eq(Expr::val(id)))
				.to_owned();

			// Build and execute UPDATE query
			let (update_sql, update_values) = match backend {
				DbBackend::Postgres => update_stmt.build(PostgresQueryBuilder),
				DbBackend::Mysql => update_stmt.build(MysqlQueryBuilder),
				DbBackend::Sqlite => update_stmt.build(SqliteQueryBuilder),
			};

			let mut update_query = sqlx::query(&update_sql);
			for value in &update_values.0 {
				update_query = bind_sea_value(update_query, value);
			}

			update_query.execute(&*self.pool).await.map_err(|e| {
				OrderError::UpdateFailed(format!("Failed to update order for id {}: {}", id, e))
			})?;
		}

		Ok(new_orders)
	}

	/// Validates an order value
	pub fn validate_order(&self, order: i32, max_order: i32) -> Result<(), OrderError> {
		if order < 0 {
			return Err(OrderError::InvalidOrder(format!(
				"Order must be non-negative, got {}",
				order
			)));
		}
		if order > max_order {
			return Err(OrderError::InvalidOrder(format!(
				"Order {} exceeds maximum {}",
				order, max_order
			)));
		}
		Ok(())
	}
}

/// Convert OrderValue to sea-query Value
fn order_value_to_sea_value(value: &OrderValue) -> sea_query::Value {
	match value {
		OrderValue::Integer(i) => sea_query::Value::BigInt(Some(*i)),
		OrderValue::String(s) => sea_query::Value::String(Some(s.clone())),
		OrderValue::Boolean(b) => sea_query::Value::Bool(Some(*b)),
	}
}

/// Bind sea-query Value to sqlx query
fn bind_sea_value<'a>(
	query: sqlx::query::Query<'a, sqlx::Any, sqlx::any::AnyArguments<'a>>,
	value: &sea_query::Value,
) -> sqlx::query::Query<'a, sqlx::Any, sqlx::any::AnyArguments<'a>> {
	match value {
		sea_query::Value::Bool(Some(b)) => query.bind(*b),
		sea_query::Value::TinyInt(Some(i)) => query.bind(*i as i32),
		sea_query::Value::SmallInt(Some(i)) => query.bind(*i as i32),
		sea_query::Value::Int(Some(i)) => query.bind(*i),
		sea_query::Value::BigInt(Some(i)) => query.bind(*i),
		sea_query::Value::TinyUnsigned(Some(i)) => query.bind(*i as i32),
		sea_query::Value::SmallUnsigned(Some(i)) => query.bind(*i as i32),
		sea_query::Value::Unsigned(Some(i)) => query.bind(*i as i64),
		sea_query::Value::BigUnsigned(Some(i)) => query.bind(*i as i64),
		sea_query::Value::Float(Some(f)) => query.bind(*f),
		sea_query::Value::Double(Some(f)) => query.bind(*f),
		sea_query::Value::String(Some(s)) => query.bind(s.clone()),
		sea_query::Value::Bytes(Some(b)) => query.bind(b.clone()),
		_ => query.bind(None::<i32>), // NULL values
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_ordered_model_creation() {
		let ordered = OrderedModel::new("_order".to_string(), vec!["category_id".to_string()]);

		assert_eq!(ordered.order_field(), "_order");
		assert_eq!(ordered.order_with_respect_to().len(), 1);
		assert_eq!(ordered.order_with_respect_to()[0], "category_id");
	}

	#[test]
	fn test_ordered_model_with_multiple_fields() {
		let ordered = OrderedModel::new(
			"order".to_string(),
			vec!["parent_id".to_string(), "category_id".to_string()],
		);

		assert_eq!(ordered.order_with_respect_to().len(), 2);
	}

	#[tokio::test]
	async fn test_get_next_order() {
		let ordered = OrderedModel::new("order".to_string(), vec!["parent_id".to_string()]);

		let mut filters = HashMap::new();
		filters.insert("parent_id".to_string(), OrderValue::Integer(1));

		let next_order = ordered.get_next_order(filters).await.unwrap();
		assert_eq!(next_order, 0);
	}

	#[tokio::test]
	async fn test_move_up() {
		let ordered = OrderedModel::new("order".to_string(), vec!["parent_id".to_string()]);

		let new_order = ordered.move_up(5).await.unwrap();
		assert_eq!(new_order, 4);

		let result = ordered.move_up(0).await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_move_down() {
		let ordered = OrderedModel::new("order".to_string(), vec!["parent_id".to_string()]);

		let new_order = ordered.move_down(3, 10).await.unwrap();
		assert_eq!(new_order, 4);

		let result = ordered.move_down(10, 10).await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_move_to_position() {
		let ordered = OrderedModel::new("order".to_string(), vec!["parent_id".to_string()]);

		let new_order = ordered.move_to_position(5, 10, 7).await.unwrap();
		assert_eq!(new_order, 7);

		let result = ordered.move_to_position(5, 10, 15).await;
		assert!(result.is_err());

		let result = ordered.move_to_position(5, 10, -1).await;
		assert!(result.is_err());
	}

	#[tokio::test]
	async fn test_swap_order() {
		let ordered = OrderedModel::new("order".to_string(), vec!["parent_id".to_string()]);

		let (new_order1, new_order2) = ordered.swap_order(3, 7).await.unwrap();
		assert_eq!(new_order1, 7);
		assert_eq!(new_order2, 3);
	}

	#[test]
	fn test_validate_order() {
		let ordered = OrderedModel::new("order".to_string(), vec!["parent_id".to_string()]);

		assert!(ordered.validate_order(5, 10).is_ok());
		assert!(ordered.validate_order(0, 10).is_ok());
		assert!(ordered.validate_order(10, 10).is_ok());

		assert!(ordered.validate_order(-1, 10).is_err());
		assert!(ordered.validate_order(11, 10).is_err());
	}

	#[tokio::test]
	async fn test_reorder_all() {
		let ordered = OrderedModel::new("order".to_string(), vec!["parent_id".to_string()]);

		let mut filters = HashMap::new();
		filters.insert("parent_id".to_string(), OrderValue::Integer(1));

		let result = ordered.reorder_all(filters).await.unwrap();
		assert_eq!(result.len(), 0);
	}

	#[test]
	fn test_order_value_variants() {
		let int_value = OrderValue::Integer(42);
		let str_value = OrderValue::String("test".to_string());
		let bool_value = OrderValue::Boolean(true);

		match int_value {
			OrderValue::Integer(v) => assert_eq!(v, 42),
			_ => panic!("Expected Integer variant"),
		}

		match str_value {
			OrderValue::String(v) => assert_eq!(v, "test"),
			_ => panic!("Expected String variant"),
		}

		match bool_value {
			OrderValue::Boolean(v) => assert!(v),
			_ => panic!("Expected Boolean variant"),
		}
	}
}
