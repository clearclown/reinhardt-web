//! E2E integration tests for examples-twitter
//!
//! Server Function integration tests using rstest fixtures and testcontainers.
//!
//! NOTE: These tests are currently disabled pending fixture refactoring.
//! The fixture signatures need to be updated to match the actual fixture definitions:
//! - `test_database` returns `TestDatabase = (ContainerAsync<GenericImage>, Arc<DatabaseConnection>)`
//! - `test_user` returns `(User, TestDatabase)`
//!
//! To re-enable: Change `e2e-tests` feature gate below and fix fixtures.

// Disabled: Tests require fixture refactoring
// Re-enable with: #![cfg(all(not(target_arch = "wasm32"), feature = "e2e-tests"))]
#![cfg(any())]

use examples_twitter::apps::auth::models::User;
use examples_twitter::apps::profile::models::Profile;
use examples_twitter::apps::tweet::models::Tweet;
use examples_twitter::server::server_fn::auth::{current_user, login, logout, register};
use examples_twitter::server::server_fn::tweet::{create_tweet, delete_tweet, list_tweets};
use examples_twitter::shared::types::{
	CreateTweetRequest, LoginRequest, RegisterRequest, TweetInfo, UserInfo,
};
use examples_twitter::test_utils::fixtures::test_database;
use reinhardt::db::orm::{FilterOperator, FilterValue, Model};
use reinhardt::middleware::session::{SessionData, SessionStore, SessionStoreRef};
use reinhardt::test::testcontainers::{ContainerAsync, GenericImage};
use reinhardt::{BaseUser, DatabaseConnection};
use rstest::*;
use std::sync::Arc;
use uuid::Uuid;

// ============================================================================
// Fixtures
// ============================================================================
// Note: test_database fixture is imported from examples_twitter::test_utils::fixtures

// ============================================================================
// Test Utilities
// ============================================================================

/// Create a test user with given email and password
async fn create_test_user(db: &DatabaseConnection, email: &str, password: &str) -> User {
	let username = email.split('@').next().unwrap().to_string();

	let mut user = User::new(username, email.to_string(), None, true, None);

	user.set_password(password).expect("Failed to set password");

	User::objects()
		.create_with_conn(db, &user)
		.await
		.expect("Failed to create test user")
}

/// Create a test tweet
async fn create_test_tweet(db: &DatabaseConnection, user_id: Uuid, content: &str) -> Tweet {
	use chrono::Utc;

	let tweet = Tweet {
		id: Uuid::new_v4(),
		user_id,
		content: content.to_string(),
		like_count: 0,
		retweet_count: 0,
		created_at: Utc::now(),
		updated_at: Utc::now(),
	};

	Tweet::objects()
		.create_with_conn(db, &tweet)
		.await
		.expect("Failed to create test tweet");

	tweet
}

/// Create a test profile
async fn create_test_profile(db: &DatabaseConnection, user_id: Uuid) -> Profile {
	let profile = Profile {
		id: Uuid::new_v4(),
		user_id,
		bio: None,
		avatar_url: None,
		location: None,
		website: None,
	};

	Profile::objects()
		.create_with_conn(db, &profile)
		.await
		.expect("Failed to create test profile");

	profile
}

// ============================================================================
// Authentication Tests (High Priority)
// ============================================================================

#[cfg(test)]
mod auth_e2e {
	use super::*;

	#[rstest]
	#[tokio::test]
	async fn test_user_registration_flow(
		#[future] db: DatabaseConnection,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Test: User registration
		let register_req = RegisterRequest {
			username: "newuser".to_string(),
			email: "newuser@example.com".to_string(),
			password: "password123".to_string(),
			password_confirmation: "password123".to_string(),
		};

		let result = register(register_req, db.clone()).await;
		assert!(result.is_ok(), "Registration should succeed");

		// Verify: User was created in database
		let user = User::objects()
			.filter(
				User::field_email(),
				FilterOperator::Eq,
				FilterValue::String("newuser@example.com".to_string()),
			)
			.first_with_db(&db)
			.await?;

		assert!(user.is_some(), "User should exist in database");
		let user = user.unwrap();
		assert_eq!(user.username, "newuser");
		assert_eq!(user.email, "newuser@example.com");

		Ok(())
	}

	#[rstest]
	#[tokio::test]
	async fn test_login_logout_flow(
		#[future] db: DatabaseConnection,
		#[future] test_user: User,
		session: SessionData,
		session_store: Arc<SessionStore>,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Test: Login
		let login_req = LoginRequest {
			email: test_user.email.clone(),
			password: "password123".to_string(),
		};

		let result = login(
			login_req,
			db.clone(),
			session.clone(),
			session_store.clone(),
		)
		.await;
		assert!(result.is_ok(), "Login should succeed");

		let user_info = result.unwrap();
		assert_eq!(user_info.email, test_user.email);
		assert_eq!(user_info.username, test_user.username);

		// Verify: Session contains user_id (session was updated by login function)
		// Note: We need to retrieve the updated session from the store
		// The login function saves the updated session to the store
		// For now, we'll verify by calling current_user
		let current_result = current_user(db.clone(), session.clone()).await;
		assert!(
			current_result.is_ok(),
			"current_user should succeed after login"
		);

		// Test: Logout
		let logout_result = logout(session.clone(), session_store.clone()).await;
		assert!(logout_result.is_ok(), "Logout should succeed");

		// Note: After logout, the session is deleted from the store
		// So current_user should return None or fail

		Ok(())
	}

	#[rstest]
	#[tokio::test]
	async fn test_login_with_invalid_credentials(
		#[future] db: DatabaseConnection,
		#[future] test_user: User,
		session: SessionData,
		session_store: Arc<SessionStore>,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Test: Login with wrong password
		let login_req = LoginRequest {
			email: test_user.email.clone(),
			password: "wrong_password".to_string(),
		};

		let result = login(login_req, db.clone(), session, session_store).await;
		assert!(result.is_err(), "Login should fail with wrong password");

		Ok(())
	}

	#[rstest]
	#[tokio::test]
	async fn test_login_with_nonexistent_user(
		#[future] db: DatabaseConnection,
		session: SessionData,
		session_store: Arc<SessionStore>,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Test: Login with nonexistent email
		let login_req = LoginRequest {
			email: "nonexistent@example.com".to_string(),
			password: "password123".to_string(),
		};

		let result = login(login_req, db.clone(), session, session_store).await;
		assert!(result.is_err(), "Login should fail with nonexistent user");

		Ok(())
	}

	#[rstest]
	#[tokio::test]
	async fn test_register_with_duplicate_email(
		#[future] db: DatabaseConnection,
		#[future] test_user: User,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Test: Register with existing email
		let register_req = RegisterRequest {
			username: "anotheruser".to_string(),
			email: test_user.email.clone(), // Duplicate email
			password: "password123".to_string(),
			password_confirmation: "password123".to_string(),
		};

		let result = register(register_req, db.clone()).await;
		assert!(
			result.is_err(),
			"Registration should fail with duplicate email"
		);

		Ok(())
	}
}

// ============================================================================
// Tweet CRUD Tests (High Priority)
// ============================================================================

#[cfg(test)]
mod tweet_e2e {
	use super::*;

	#[rstest]
	#[tokio::test]
	async fn test_create_and_list_tweets(
		#[future] db: DatabaseConnection,
		#[future] test_user: User,
		#[future] authenticated_session: SessionData,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Test: Create tweet
		let create_req = CreateTweetRequest {
			content: "Hello, Twitter!".to_string(),
		};

		let result = create_tweet(create_req, db.clone(), authenticated_session).await;
		assert!(result.is_ok(), "Tweet creation should succeed");

		let tweet_info = result.unwrap();
		assert_eq!(tweet_info.content, "Hello, Twitter!");
		assert_eq!(tweet_info.user_id, test_user.id);

		// Test: List tweets
		let result = list_tweets(Some(test_user.id), 0, db.clone()).await;
		assert!(result.is_ok(), "Tweet listing should succeed");

		let tweets = result.unwrap();
		assert_eq!(tweets.len(), 1, "Should have 1 tweet");
		assert_eq!(tweets[0].content, "Hello, Twitter!");
		assert_eq!(tweets[0].username, test_user.username);

		Ok(())
	}

	#[rstest]
	#[tokio::test]
	async fn test_delete_tweet_by_owner(
		#[future] db: DatabaseConnection,
		#[future] test_user: User,
		#[future] authenticated_session: SessionData,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Setup: Create a tweet
		let tweet = create_test_tweet(&db, test_user.id, "Test tweet").await;

		// Test: Delete tweet
		let result = delete_tweet(tweet.id, db.clone(), authenticated_session).await;
		assert!(result.is_ok(), "Tweet deletion should succeed");

		// Verify: Tweet was deleted
		let deleted_tweet = Tweet::objects()
			.filter(
				Tweet::field_id(),
				FilterOperator::Eq,
				FilterValue::Uuid(tweet.id),
			)
			.first_with_db(&db)
			.await?;

		assert!(deleted_tweet.is_none(), "Tweet should be deleted");

		Ok(())
	}

	#[rstest]
	#[tokio::test]
	async fn test_delete_tweet_by_non_owner(
		#[future] db: DatabaseConnection,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Setup: Create two users
		let owner = create_test_user(&db, "owner@example.com", "password123").await;
		let other_user = create_test_user(&db, "other@example.com", "password123").await;

		// Setup: Create a tweet owned by the first user
		let tweet = create_test_tweet(&db, owner.id, "Owner's tweet").await;

		// Setup: Create session for the other user
		let session_store = Arc::new(SessionStore::new());
		let mut session = session_store.create();
		session
			.set("user_id".to_string(), other_user.id)
			.expect("Failed to set user_id");

		// Test: Try to delete tweet as non-owner
		let result = delete_tweet(tweet.id, db.clone(), session).await;
		assert!(result.is_err(), "Tweet deletion should fail for non-owner");

		// Verify: Tweet still exists
		let existing_tweet = Tweet::objects()
			.filter(
				Tweet::field_id(),
				FilterOperator::Eq,
				FilterValue::Uuid(tweet.id),
			)
			.first_with_db(&db)
			.await?;

		assert!(
			existing_tweet.is_some(),
			"Tweet should still exist after failed deletion"
		);

		Ok(())
	}

	#[rstest]
	#[tokio::test]
	async fn test_create_tweet_unauthenticated(
		#[future] db: DatabaseConnection,
		session: SessionData, // Empty session without user_id
	) -> Result<(), Box<dyn std::error::Error>> {
		// Test: Try to create tweet without authentication
		let create_req = CreateTweetRequest {
			content: "Unauthenticated tweet".to_string(),
		};

		let result = create_tweet(create_req, db.clone(), session).await;
		assert!(
			result.is_err(),
			"Tweet creation should fail without authentication"
		);

		Ok(())
	}
}

// ============================================================================
// Profile Management Tests (Medium Priority)
// ============================================================================

#[cfg(test)]
mod profile_e2e {
	use super::*;
	use examples_twitter::server::server_fn::profile::{fetch_profile, update_profile};
	use examples_twitter::shared::types::{ProfileResponse, UpdateProfileRequest};

	#[rstest]
	#[tokio::test]
	async fn test_fetch_profile(
		#[future] db: DatabaseConnection,
		#[future] test_user: User,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Setup: Create profile for test user
		let _profile = create_test_profile(&db, test_user.id).await;

		// Test: Fetch profile
		let result = fetch_profile(test_user.id, db.clone()).await;
		assert!(result.is_ok(), "Profile fetch should succeed");

		let profile_response = result.unwrap();
		assert_eq!(
			profile_response.user_id, test_user.id,
			"Profile should belong to test user"
		);

		Ok(())
	}

	#[rstest]
	#[tokio::test]
	async fn test_fetch_nonexistent_profile(
		#[future] db: DatabaseConnection,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Test: Fetch profile for nonexistent user
		let nonexistent_id = Uuid::new_v4();
		let result = fetch_profile(nonexistent_id, db.clone()).await;
		assert!(
			result.is_err(),
			"Profile fetch should fail for nonexistent user"
		);

		Ok(())
	}

	#[rstest]
	#[tokio::test]
	async fn test_update_profile(
		#[future] db: DatabaseConnection,
		#[future] test_user: User,
		#[future] authenticated_session: SessionData,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Setup: Create profile for test user
		let _profile = create_test_profile(&db, test_user.id).await;

		// Test: Update profile
		let update_req = UpdateProfileRequest {
			bio: Some("Updated bio".to_string()),
			avatar_url: Some("https://example.com/avatar.jpg".to_string()),
			location: Some("Tokyo".to_string()),
			website: None,
		};

		let result = update_profile(update_req, db.clone(), authenticated_session).await;
		assert!(result.is_ok(), "Profile update should succeed");

		let updated_profile = result.unwrap();
		assert_eq!(
			updated_profile.bio,
			Some("Updated bio".to_string()),
			"Bio should be updated"
		);
		assert_eq!(
			updated_profile.location,
			Some("Tokyo".to_string()),
			"Location should be updated"
		);

		Ok(())
	}

	#[rstest]
	#[tokio::test]
	async fn test_update_profile_partial(
		#[future] db: DatabaseConnection,
		#[future] test_user: User,
		#[future] authenticated_session: SessionData,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Setup: Create profile for test user
		let _profile = create_test_profile(&db, test_user.id).await;

		// Test: Update only bio
		let update_req = UpdateProfileRequest {
			bio: Some("New bio".to_string()),
			avatar_url: None,
			location: None,
			website: None,
		};

		let result = update_profile(update_req, db.clone(), authenticated_session).await;
		assert!(result.is_ok(), "Partial profile update should succeed");

		let updated_profile = result.unwrap();
		assert_eq!(
			updated_profile.bio,
			Some("New bio".to_string()),
			"Bio should be updated"
		);

		Ok(())
	}
}

// ============================================================================
// Relationship Tests (Low Priority - Placeholder)
// ============================================================================

#[cfg(test)]
mod relationship_e2e {
	use super::*;

	// Note: These tests are placeholders. Actual implementation requires
	// follow_user, unfollow_user, block_user, and fetch_followers server functions.

	#[rstest]
	#[tokio::test]
	#[ignore = "Server functions not implemented yet"]
	async fn test_follow_and_unfollow_flow(
		#[future] db: DatabaseConnection,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Setup: Create two users
		let follower = create_test_user(&db, "follower@example.com", "password123").await;
		let followee = create_test_user(&db, "followee@example.com", "password123").await;

		// Test: Follow user
		// let result = follow_user(followee.id, db.clone(), current_user).await;
		// assert!(result.is_ok(), "Follow should succeed");

		// Verify: Follower appears in followers list
		// let followers = fetch_followers(followee.id, db.clone()).await?;
		// assert_eq!(followers.len(), 1, "Should have 1 follower");
		// assert_eq!(followers[0].id, follower.id);

		// Test: Unfollow user
		// let result = unfollow_user(followee.id, db.clone(), current_user).await;
		// assert!(result.is_ok(), "Unfollow should succeed");

		// Verify: Follower removed from followers list
		// let followers = fetch_followers(followee.id, db.clone()).await?;
		// assert_eq!(followers.len(), 0, "Should have 0 followers");

		Ok(())
	}

	#[rstest]
	#[tokio::test]
	#[ignore = "Server functions not implemented yet"]
	async fn test_block_user_flow(
		#[future] db: DatabaseConnection,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Setup: Create two users
		let _blocker = create_test_user(&db, "blocker@example.com", "password123").await;
		let _blockee = create_test_user(&db, "blockee@example.com", "password123").await;

		// Test: Block user
		// let result = block_user(blockee.id, db.clone(), current_user).await;
		// assert!(result.is_ok(), "Block should succeed");

		// Verify: Blocked user cannot follow
		// let result = follow_user(blocker.id, db.clone(), blockee_current_user).await;
		// assert!(result.is_err(), "Blocked user should not be able to follow");

		Ok(())
	}

	#[rstest]
	#[tokio::test]
	#[ignore = "Server functions not implemented yet"]
	async fn test_cannot_follow_blocked_user(
		#[future] db: DatabaseConnection,
	) -> Result<(), Box<dyn std::error::Error>> {
		// Setup: Create two users
		let _follower = create_test_user(&db, "follower@example.com", "password123").await;
		let _blocker = create_test_user(&db, "blocker@example.com", "password123").await;

		// Setup: Block follower
		// block_user(follower.id, db.clone(), blocker_current_user).await?;

		// Test: Follower tries to follow blocker
		// let result = follow_user(blocker.id, db.clone(), follower_current_user).await;
		// assert!(result.is_err(), "Should not be able to follow blocker");

		Ok(())
	}
}
