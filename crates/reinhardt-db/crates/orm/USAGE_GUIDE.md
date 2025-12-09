# reinhardt-orm Usage Guide

A comprehensive guide for using reinhardt-orm, Reinhardt's Object-Relational Mapping system.

## Table of Contents

1. [Basic Model Definition](#1-basic-model-definition)
2. [CRUD Operations](#2-crud-operations)
3. [Query Building](#3-query-building)
4. [Relationships](#4-relationships)
5. [Transaction Management](#5-transaction-management)
6. [Advanced Features](#6-advanced-features)
7. [Best Practices](#7-best-practices)

---

## 1. Basic Model Definition

### Creating a Model

Define models using the `#[model(...)]` macro:

```rust
use reinhardt_macros::model;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[model(app_label = "users", table_name = "users")]
#[derive(Serialize, Deserialize)]
pub struct User {
	#[field(primary_key = true)]
	pub id: Uuid,

	#[field(max_length = 100)]
	pub username: String,

	#[field(max_length = 255, email = true)]
	pub email: String,

	#[field(default = true)]
	pub is_active: bool,

	#[field(auto_now_add = true)]
	pub created_at: DateTime<Utc>,

	#[field(auto_now = true)]
	pub updated_at: DateTime<Utc>,
}
```

**Important**: The `#[model(...)]` macro automatically derives `Model` trait implementation, so you don't need to explicitly write `#[derive(Model)]`.

### Available Field Attributes

| Attribute | Purpose | Example |
|-----------|---------|---------|
| `primary_key = true` | Mark as primary key | `#[field(primary_key = true)]` |
| `max_length = N` | Maximum string length | `#[field(max_length = 100)]` |
| `default = value` | Default value | `#[field(default = true)]` |
| `auto_now_add = true` | Auto-populate timestamp on creation | `#[field(auto_now_add = true)]` |
| `auto_now = true` | Auto-update timestamp on save | `#[field(auto_now = true)]` |
| `null = true` | Allow NULL values | `#[field(null = true)]` |
| `unique = true` | Enforce uniqueness constraint | `#[field(unique = true)]` |
| `index = true` | Create database index | `#[field(index = true)]` |
| `email = true` | Email validation | `#[field(email = true)]` |
| `url = true` | URL validation | `#[field(url = true)]` |
| `min_length = N` | Minimum string length | `#[field(min_length = 3)]` |
| `min_value = N` | Minimum numeric value | `#[field(min_value = 0)]` |
| `max_value = N` | Maximum numeric value | `#[field(max_value = 100)]` |
| `check = "expr"` | CHECK constraint | `#[field(check = "price > 0")]` |

### Auto-Generated Features

The `#[model(...)]` macro automatically generates:

1. **Model trait implementation**
2. **Type-safe field accessors**: `User::field_username()`, `User::field_email()`, etc.
3. **Global model registry registration**
4. **Composite Primary Key support**

```rust
// Auto-generated field accessor example
impl User {
	pub const fn field_id() -> FieldRef<User, Uuid> { FieldRef::new("id") }
	pub const fn field_username() -> FieldRef<User, String> { FieldRef::new("username") }
	pub const fn field_email() -> FieldRef<User, String> { FieldRef::new("email") }
	// ...
}
```

---

## 2. CRUD Operations

### Manager API (Django-style)

```rust
use reinhardt_orm::Manager;

// Create
async fn create_user() -> Result<User, Box<dyn std::error::Error>> {
	// Create user using generated new() function
	// Auto-generates: id (UUID), created_at, updated_at
	// Auto-sets defaults: is_active = true
	let user = User::new(
		"alice".to_string(),
		"alice@example.com".to_string(),
	);

	User::objects().create(&user).await
}

// Read - get all
async fn get_all_users() -> Result<Vec<User>, Box<dyn std::error::Error>> {
	User::objects().all().all().await
}

// Read - get single record
async fn get_user_by_id(id: Uuid) -> Result<User, Box<dyn std::error::Error>> {
	User::objects().get(id).get().await
}

// Update
async fn update_user(user: &mut User) -> Result<User, Box<dyn std::error::Error>> {
	user.email = "newemail@example.com".to_string();
	User::objects().update(user).await
}

// Delete
async fn delete_user(id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
	User::objects().delete(id).await
}

// Count
async fn count_users() -> Result<i64, Box<dyn std::error::Error>> {
	User::objects().count().await
}
```

### Bulk Operations

```rust
// Bulk Create
async fn bulk_create_users(users: Vec<User>) -> Result<Vec<User>, Box<dyn std::error::Error>> {
	User::objects().bulk_create(
		users,
		Some(100),  // batch_size
		false       // ignore_conflicts
	).await
}

// Bulk Update
async fn bulk_update_users(users: Vec<User>) -> Result<usize, Box<dyn std::error::Error>> {
	User::objects().bulk_update(
		users,
		vec!["email".to_string(), "is_active".to_string()],  // fields
		Some(100)  // batch_size
	).await
}

// Get or Create (atomic operation)
async fn get_or_create_user() -> Result<(User, bool), Box<dyn std::error::Error>> {
	let mut lookup = HashMap::new();
	lookup.insert("username".to_string(), "alice".to_string());

	let mut defaults = HashMap::new();
	defaults.insert("email".to_string(), "alice@example.com".to_string());

	User::objects().get_or_create(lookup, Some(defaults)).await
}
```

---

## 3. Query Building

### QuerySet API (Chainable)

```rust
use reinhardt_orm::{QuerySet, Filter, FilterOperator, FilterValue};

// Basic filtering
async fn filter_active_users() -> Result<Vec<User>, Box<dyn std::error::Error>> {
	User::objects()
		.filter(Filter {
			field: "is_active".to_string(),
			operator: FilterOperator::Eq,
			value: FilterValue::Boolean(true),
		})
		.all()
		.await
}

// Chaining multiple filters
async fn complex_query() -> Result<Vec<User>, Box<dyn std::error::Error>> {
	User::objects()
		.filter(Filter {
			field: "is_active".to_string(),
			operator: FilterOperator::Eq,
			value: FilterValue::Boolean(true),
		})
		.filter(Filter {
			field: "created_at".to_string(),
			operator: FilterOperator::Gte,
			value: FilterValue::String(Utc::now().to_rfc3339()),
		})
		.order_by(&["username", "-created_at"])
		.all()
		.await
}

// Distinct, Values, Limit
async fn advanced_query() -> Result<Vec<User>, Box<dyn std::error::Error>> {
	User::objects()
		.distinct()
		.values(&["username", "email"])
		.all()
		.await
}
```

### Type-Safe Field References (Recommended)

```rust
// Use type-safe field references (compile-time type checking)
async fn type_safe_filter() -> Result<Vec<User>, Box<dyn std::error::Error>> {
	let username_ref = User::field_username();  // FieldRef<User, String>
	let email_ref = User::field_email();        // FieldRef<User, String>

	// Generate filter
	let filter = username_ref.eq("alice");

	User::objects()
		.filter(filter)
		.all()
		.await
}
```

### F Expressions (Field References)

```rust
use reinhardt_orm::F;

// F expression for field comparisons
let f_username = F { field: "username".to_string() };
let f_email = F { field: "email".to_string() };

// Or using type-safe field references
let f_from_ref: F = User::field_username().into();
```

### Q Expressions (Complex Logic)

```rust
use reinhardt_orm::{Q, QOperator};

// Q expressions for complex AND/OR logic
let query = Q::new()
	.field("is_active").eq(true)
	.and(Q::new()
		.field("username").contains("alice")
		.or(Q::new().field("email").contains("alice"))
	);
```

### Eager Loading

```rust
// select_related (uses JOIN)
async fn with_related() -> Result<Vec<User>, Box<dyn std::error::Error>> {
	User::objects()
		.select_related(&["profile", "address"])
		.all()
		.await
}

// prefetch_related (uses separate queries)
async fn with_prefetch() -> Result<Vec<User>, Box<dyn std::error::Error>> {
	User::objects()
		.prefetch_related(&["groups", "permissions"])
		.all()
		.await
}
```

---

## 4. Relationships

### ForeignKey (Many-to-One)

```rust
#[model(app_label = "blog", table_name = "posts")]
pub struct Post {
	#[field(primary_key = true)]
	pub id: i64,

	#[field(max_length = 200)]
	pub title: String,

	#[rel(foreign_key, related_name = "posts")]
	pub author: ForeignKeyField<Post, User>,
}
```

### OneToOne

```rust
#[model(app_label = "users", table_name = "profiles")]
pub struct Profile {
	#[field(primary_key = true)]
	pub id: i64,

	#[rel(one_to_one)]
	pub user: OneToOneField<Profile, User>,

	#[field(max_length = 500)]
	pub bio: String,
}
```

### ManyToMany

```rust
use reinhardt_orm::ManyToManyField;

#[model(app_label = "dm", table_name = "dm_room")]
pub struct DMRoom {
	#[field(primary_key = true)]
	pub id: Uuid,

	#[field(max_length = 100)]
	pub name: Option<String>,

	#[rel(many_to_many, related_name = "rooms")]
	pub members: ManyToManyField<DMRoom, User>,
}
```

---

## 5. Transaction Management

### Recommended: Closure-based API

```rust
use reinhardt_orm::transaction::transaction;
use reinhardt_orm::connection::DatabaseConnection;

// Basic transaction (automatic commit/rollback)
async fn create_user_with_transaction(
	conn: &DatabaseConnection,
	name: &str
) -> Result<i64, anyhow::Error> {
	transaction(conn, |_tx| async move {
		let id = insert_user(name).await?;
		update_user_count().await?;
		Ok(id)  // Auto-commit on success
	}).await  // Auto-rollback on error
}

// With isolation level
async fn with_isolation_level(
	conn: &DatabaseConnection
) -> Result<(), anyhow::Error> {
	use reinhardt_orm::transaction::{transaction_with_isolation, IsolationLevel};

	transaction_with_isolation(conn, IsolationLevel::Serializable, |_tx| async move {
		let stock = get_current_stock().await?;
		if stock > 0 {
			decrement_stock().await?;
		}
		Ok(())
	}).await
}

// Error handling
async fn transfer_money(
	conn: &DatabaseConnection,
	from: &str,
	to: &str,
	amount: i64,
) -> Result<(), anyhow::Error> {
	transaction(conn, |_tx| async move {
		debit_account(from, amount).await?;   // Error → auto-rollback
		credit_account(to, amount).await?;    // Error → auto-rollback
		Ok(())  // Success → auto-commit
	}).await
}
```

### Low-level API (Advanced)

```rust
use reinhardt_orm::transaction::TransactionScope;

async fn advanced_transaction(
	conn: &DatabaseConnection
) -> Result<(), anyhow::Error> {
	let tx = TransactionScope::begin(conn).await?;

	// Perform operations
	insert_record().await?;

	// Conditional commit/rollback
	if some_condition {
		tx.commit().await?;
	} else {
		tx.rollback().await?;
	}

	Ok(())
}
```

**Best Practices**:
- ✅ **Recommended**: `transaction()` - Closure-based, automatic management
- ✅ **Recommended**: `transaction_with_isolation()` - When isolation level is needed
- ⚠️ **Advanced**: `TransactionScope` - Only when manual control is necessary

---

## 6. Advanced Features

### Aggregate Functions

```rust
use reinhardt_orm::aggregation::{Aggregate, AggregateFunc};

// Count, Sum, Avg, Max, Min
let user_count = Aggregate::count(User::field_id().into());
let avg_age = Aggregate::avg(User::field_age().into());
let max_created = Aggregate::max(User::field_created_at().into());
```

### Database Functions

```rust
use reinhardt_orm::functions::{Concat, Upper, Lower, Now, CurrentDate};

// String functions
let email_lower = Lower::new(User::field_email().into());
let username_upper = Upper::new(User::field_username().into());

// Date/Time functions
let current_time = Now::new();
let today = CurrentDate::new();
```

### Window Functions

```rust
use reinhardt_orm::window::{Window, RowNumber, Rank, DenseRank};

// Ranking by join date
let rank_by_join_date = Window::new()
	.partition_by(vec![User::field_is_active().into()])
	.order_by(vec![(User::field_created_at().into(), "DESC")])
	.function(RowNumber::new());
```

### Constraints

```rust
// CHECK constraints
#[model(app_label = "products", table_name = "products")]
pub struct Product {
	#[field(primary_key = true)]
	pub id: i64,

	#[field(check = "price > 0")]
	pub price: f64,

	#[field(check = "quantity >= 0")]
	pub quantity: i32,
}

// Access metadata
let constraints = Product::constraint_metadata();
```

### Composite Primary Keys

```rust
#[model(app_label = "test_app", table_name = "post_tags")]
pub struct PostTag {
	#[field(primary_key = true)]
	pub post_id: i64,

	#[field(primary_key = true)]
	pub tag_id: i64,

	#[field(max_length = 200)]
	pub description: String,
}

// Access composite PK
let composite_pk = PostTag::composite_primary_key();
assert!(composite_pk.is_some());

// Get composite PK values
let post_tag = PostTag { post_id: 1, tag_id: 5, description: "Tech".to_string() };
let pk_values = post_tag.get_composite_pk_values();
```

---

## 7. Best Practices

### ✅ Recommended Patterns

1. **Use type-safe field references**:
   ```rust
   // ✅ Good: Compile-time type checking
   let filter = User::field_username().eq("alice");

   // ❌ Bad: String-based, prone to typos
   let filter = Filter { field: "username".to_string(), ... };
   ```

2. **Closure-based transactions**:
   ```rust
   // ✅ Good: Automatic management
   transaction(conn, |_tx| async move { ... }).await

   // ⚠️ Manual: Requires manual management
   let tx = TransactionScope::begin(conn).await?;
   ```

3. **Leverage SeaQuery integration benefits**:
   - SQL injection protection
   - Parameter binding
   - Multi-database support (PostgreSQL, MySQL, SQLite)

4. **Avoid N+1 queries with eager loading**:
   ```rust
   // ✅ Good: Single JOIN
   User::objects().select_related(&["profile"]).all().await

   // ❌ Bad: N additional queries
   let users = User::objects().all().await;
   for user in users {
	   let profile = Profile::get_by_user_id(user.id).await;
   }
   ```

### ⚠️ Important Notes

- **Model trait auto-implementation**: When using `#[model(...)]` macro, `#[derive(Model)]` is unnecessary
- **Transactions**: Automatic rollback occurs even on panic
- **Field attributes**: `auto_now` and `auto_now_add` are managed by Rust code, not DB DEFAULT

### 📚 Additional Resources

- README.md: `/Users/kent8192/Projects/reinhardt/crates/reinhardt-db/crates/orm/README.md` - 70+ feature list
- lib.rs: `/Users/kent8192/Projects/reinhardt/crates/reinhardt-db/crates/orm/src/lib.rs` - API definitions and docstrings
- Examples: `/Users/kent8192/Projects/reinhardt/examples/local/examples-twitter/` - Real-world usage examples

---

We hope this guide helps you get started with reinhardt-orm!
