//! Tweet model

use chrono::{DateTime, Utc};
use reinhardt::db::associations::ForeignKeyField;
use reinhardt::model;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[allow(unused_imports)]
use crate::apps::auth::models::User;

#[model(app_label = "tweet", table_name = "tweet_tweet")]
#[derive(Serialize, Deserialize)]
pub struct Tweet {
	#[field(primary_key = true)]
	id: Uuid,

	#[rel(foreign_key, related_name = "tweets")]
	user: ForeignKeyField<User>,

	#[field(max_length = 280)]
	content: String,

	#[field(default = 0)]
	like_count: i32,

	#[field(default = 0)]
	retweet_count: i32,

	#[field(auto_now_add = true)]
	created_at: DateTime<Utc>,

	#[field(auto_now = true)]
	updated_at: DateTime<Utc>,
}
