//! Tweet server functions
//!
//! Server functions for tweet management.

use crate::apps::auth::models::User;
use crate::apps::tweet::models::Tweet;
use crate::shared::types::{CreateTweetRequest, TweetInfo};
use reinhardt::CurrentUser;
use reinhardt::DatabaseConnection;
use reinhardt::db::orm::{FilterOperator, FilterValue, Model};
use reinhardt::pages::server_fn::{ServerFnError, server_fn};
use uuid::Uuid;
use validator::Validate;

/// Create a new tweet
#[server_fn(use_inject = true)]
pub async fn create_tweet(
	request: CreateTweetRequest,
	#[inject] db: DatabaseConnection,
	#[inject] current_user: CurrentUser<User>,
) -> std::result::Result<TweetInfo, ServerFnError> {
	// Validate request
	request
		.validate()
		.map_err(|e| ServerFnError::application(format!("Validation failed: {}", e)))?;

	// Get current user (already loaded by CurrentUser<User> Injectable)
	let user = current_user
		.user()
		.map_err(|_| ServerFnError::server(401, "Not authenticated"))?;

	let user_id = current_user
		.id()
		.map_err(|_| ServerFnError::server(401, "Not authenticated"))?;

	// Create Tweet model using new() method
	let tweet = Tweet::new(
		request.content.clone(),
		0,       // like_count
		0,       // retweet_count
		user_id, // ForeignKeyField parameter (Uuid)
	);

	// Save to database
	Tweet::objects()
		.create_with_conn(&db, &tweet)
		.await
		.map_err(|e| ServerFnError::application(format!("Database error: {}", e)))?;

	// Return created tweet info
	Ok(TweetInfo::new(
		tweet.id(),
		user_id,
		user.username().to_string(),
		tweet.content().to_string(),
		tweet.like_count(),
		tweet.retweet_count(),
		tweet.created_at().to_rfc3339(),
	))
}

/// List tweets
#[server_fn(use_inject = true)]
pub async fn list_tweets(
	user_id: Option<Uuid>,
	page: u32,
	#[inject] db: DatabaseConnection,
) -> std::result::Result<Vec<TweetInfo>, ServerFnError> {
	const PAGE_SIZE: u32 = 20;

	// Build query
	let mut query = Tweet::objects().all();

	// Filter by user_id if provided
	if let Some(uid) = user_id {
		use reinhardt::db::orm::Filter;
		query = query.filter(Filter::new(
			"user_id",
			FilterOperator::Eq,
			FilterValue::string(uid),
		));
	}

	// Fetch tweets with pagination
	let tweets = query
		.order_by(&["-created_at"]) // Django-style: "-" prefix for descending order
		.limit(PAGE_SIZE as usize)
		.offset((page * PAGE_SIZE) as usize)
		.all_with_db(&db)
		.await
		.map_err(|e| ServerFnError::application(format!("Database error: {}", e)))?;

	// Convert to TweetInfo (N+1 query issue - fetch user for each tweet)
	// TODO: Optimize with JOIN in future
	let mut tweet_infos = Vec::with_capacity(tweets.len());
	for tweet in tweets {
		use reinhardt::db::orm::Filter;
		let user = User::objects()
			.filter_by(Filter::new(
				"id",
				FilterOperator::Eq,
				FilterValue::string(tweet.user_id()),
			))
			.first_with_db(&db)
			.await
			.map_err(|e| ServerFnError::application(format!("User not found: {}", e)))?
			.ok_or_else(|| ServerFnError::application("User not found".to_string()))?;

		tweet_infos.push(TweetInfo::new(
			tweet.id(),
			*tweet.user_id(),
			user.username().to_string(),
			tweet.content().to_string(),
			tweet.like_count(),
			tweet.retweet_count(),
			tweet.created_at().to_rfc3339(),
		));
	}

	Ok(tweet_infos)
}

/// Delete a tweet
#[server_fn(use_inject = true)]
pub async fn delete_tweet(
	tweet_id: Uuid,
	#[inject] db: DatabaseConnection,
	#[inject] current_user: CurrentUser<User>,
) -> std::result::Result<(), ServerFnError> {
	// Get current user
	let user_id = current_user
		.id()
		.map_err(|_| ServerFnError::server(401, "Not authenticated"))?;

	// Fetch the tweet
	use reinhardt::db::orm::Filter;
	let tweet = Tweet::objects()
		.filter_by(Filter::new(
			"id",
			FilterOperator::Eq,
			FilterValue::string(tweet_id),
		))
		.first_with_db(&db)
		.await
		.map_err(|e| ServerFnError::application(format!("Database error: {}", e)))?
		.ok_or_else(|| ServerFnError::application("Tweet not found".to_string()))?;

	// Verify ownership
	if *tweet.user_id() != user_id {
		return Err(ServerFnError::server(
			403,
			"Permission denied: You can only delete your own tweets",
		));
	}

	// Delete the tweet
	Tweet::objects()
		.delete_with_conn(&db, tweet.id())
		.await
		.map_err(|e| ServerFnError::application(format!("Database error: {}", e)))?;

	Ok(())
}
