//! Test error when 'to' parameter is used with foreign_key (no longer allowed)
//!
//! In the new API, target model is inferred from ForeignKeyField<T> type parameter.
//! Using 'to' attribute is now an error.

use reinhardt::db::associations::ForeignKeyField;
use reinhardt::model;

#[model(app_label = "test", table_name = "users")]
pub struct User {
	#[field(primary_key = true)]
	pub id: i64,
}

#[model(app_label = "test", table_name = "posts")]
pub struct Post {
	#[field(primary_key = true)]
	pub id: i64,
	#[field(max_length = 200)]
	pub title: String,
	// ERROR: 'to' parameter is not allowed with ForeignKeyField
	#[rel(foreign_key, to = User)]
	pub author: ForeignKeyField<User>,
}

fn main() {}
