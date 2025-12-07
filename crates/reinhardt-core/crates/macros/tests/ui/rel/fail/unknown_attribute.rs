//! Test error for unknown rel attribute

use reinhardt::db::associations::ForeignKeyField;
use reinhardt::model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[model(app_label = "test", table_name = "users")]
pub struct User {
	#[field(primary_key = true)]
	pub id: i64,
}

#[derive(Serialize, Deserialize)]
#[model(app_label = "test", table_name = "posts")]
pub struct Post {
	#[field(primary_key = true)]
	pub id: i64,
	#[rel(foreign_key, unknown_option = true)]
	pub author: ForeignKeyField<User>,
}

fn main() {}
