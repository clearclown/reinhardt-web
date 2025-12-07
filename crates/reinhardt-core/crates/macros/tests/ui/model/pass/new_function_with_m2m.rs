//! Test that #[model] generates new() function correctly with ManyToMany fields
//!
//! ManyToManyField should be automatically excluded from new() function arguments
//! and initialized with Default::default()

use reinhardt::db::associations::ManyToManyField;
use reinhardt::model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[model(app_label = "test", table_name = "categories")]
pub struct Category {
	#[field(primary_key = true)]
	pub id: i64,
	#[field(max_length = 50)]
	pub name: String,
}

#[derive(Serialize, Deserialize)]
#[model(app_label = "test", table_name = "posts")]
pub struct Post {
	#[field(primary_key = true)]
	pub id: i64,

	#[field(max_length = 255)]
	pub title: String,

	// ManyToManyField should be auto-generated (excluded from new())
	#[rel(many_to_many, related_name = "posts")]
	pub categories: ManyToManyField<Post, Category>,
}

fn main() {
	// new() should only require id and title
	// categories (ManyToManyField) should be auto-generated (excluded from new())
	let post = Post::new(1, "Test Post".to_string());

	// Verify post fields are set correctly
	assert_eq!(post.id, 1);
	assert_eq!(post.title, "Test Post");
	// categories should be default (empty ManyToManyField)
}
