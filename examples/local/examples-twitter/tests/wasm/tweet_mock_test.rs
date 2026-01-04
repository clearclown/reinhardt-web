//! Tweet Component WASM Tests with Mocking
//!
//! Layer 2 tests for tweet components that interact with server functions.
//! These tests verify that actual tweet components from `src/client/components/features/tweet.rs`
//! render correctly and can interact with mocked server function responses.
//!
//! **Test Categories:**
//! - Pure rendering tests (no server_fn interaction)
//! - Structure validation tests (card elements, form elements)
//! - Mock infrastructure integration tests
//!
//! **Run with**: `cargo make wasm-test`

#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// Import actual components from the application
use examples_twitter::client::components::features::tweet::{tweet_card, tweet_form, tweet_list};
use examples_twitter::shared::types::{CreateTweetRequest, TweetInfo};
use reinhardt::pages::component::View;
use reinhardt::pages::testing::{
	assert_server_fn_call_count, assert_server_fn_not_called, clear_mocks, mock_server_fn,
	mock_server_fn_error,
};
use uuid::Uuid;

// ============================================================================
// Test Fixtures
// ============================================================================

/// Create a mock TweetInfo for testing
fn mock_tweet_info() -> TweetInfo {
	TweetInfo::new(
		Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
		Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
		"testuser".to_string(),
		"Hello, this is a test tweet!".to_string(),
		5,
		2,
		"2025-01-01T12:00:00Z".to_string(),
	)
}

/// Create multiple mock tweets for list testing
fn mock_tweet_list() -> Vec<TweetInfo> {
	vec![
		TweetInfo::new(
			Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
			Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
			"user1".to_string(),
			"First tweet content".to_string(),
			10,
			3,
			"2025-01-01T12:00:00Z".to_string(),
		),
		TweetInfo::new(
			Uuid::parse_str("33333333-3333-3333-3333-333333333333").unwrap(),
			Uuid::parse_str("44444444-4444-4444-4444-444444444444").unwrap(),
			"user2".to_string(),
			"Second tweet content".to_string(),
			5,
			1,
			"2025-01-01T11:00:00Z".to_string(),
		),
		TweetInfo::new(
			Uuid::parse_str("55555555-5555-5555-5555-555555555555").unwrap(),
			Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
			"user1".to_string(),
			"Third tweet from user1".to_string(),
			20,
			5,
			"2025-01-01T10:00:00Z".to_string(),
		),
	]
}

/// Create a mock CreateTweetRequest for testing
#[allow(dead_code)] // For future mock tests
fn mock_create_tweet_request() -> CreateTweetRequest {
	CreateTweetRequest {
		content: "This is a new test tweet".to_string(),
	}
}

// ============================================================================
// Tweet Card Rendering Tests
// ============================================================================

/// Test tweet card renders as a View::Element
#[wasm_bindgen_test]
fn test_tweet_card_renders() {
	let tweet = mock_tweet_info();
	let view = tweet_card(&tweet, false);
	assert!(matches!(view, View::Element(_)));
}

/// Test tweet card renders with delete button
#[wasm_bindgen_test]
fn test_tweet_card_with_delete_renders() {
	let tweet = mock_tweet_info();
	let view = tweet_card(&tweet, true);
	assert!(matches!(view, View::Element(_)));
}

/// Test tweet card contains expected structure
#[wasm_bindgen_test]
fn test_tweet_card_structure() {
	let tweet = mock_tweet_info();
	let view = tweet_card(&tweet, false);

	if let View::Element(element) = view {
		let html = element.to_html();

		// Verify card container
		assert!(html.contains("card"));
		assert!(html.contains("card-body"));

		// Verify username display with @ prefix
		assert!(html.contains("@testuser"));

		// Verify tweet content
		assert!(html.contains("Hello, this is a test tweet!"));

		// Verify timestamp
		assert!(html.contains("2025-01-01T12:00:00Z"));
	} else {
		panic!("Expected View::Element, got Fragment or None");
	}
}

/// Test tweet card shows delete button when show_delete is true
#[wasm_bindgen_test]
fn test_tweet_card_shows_delete_button() {
	let tweet = mock_tweet_info();
	let view = tweet_card(&tweet, true);

	if let View::Element(element) = view {
		let html = element.to_html();
		assert!(html.contains("Delete"));
		assert!(html.contains("btn btn-sm btn-danger"));
	} else {
		panic!("Expected View::Element");
	}
}

/// Test tweet card hides delete button when show_delete is false
#[wasm_bindgen_test]
fn test_tweet_card_hides_delete_button() {
	let tweet = mock_tweet_info();
	let view = tweet_card(&tweet, false);

	if let View::Element(element) = view {
		let html = element.to_html();
		// Delete button should not be visible
		assert!(!html.contains("btn-danger"));
	} else {
		panic!("Expected View::Element");
	}
}

/// Test tweet card displays username with @ prefix
#[wasm_bindgen_test]
fn test_tweet_card_username_format() {
	let tweet = mock_tweet_info();
	let view = tweet_card(&tweet, false);

	if let View::Element(element) = view {
		let html = element.to_html();
		// Username should be formatted with @ prefix
		assert!(
			html.contains("@testuser"),
			"Username should be displayed with @ prefix"
		);
	} else {
		panic!("Expected View::Element");
	}
}

/// Test tweet card with long content
#[wasm_bindgen_test]
fn test_tweet_card_long_content() {
	let long_content = "x".repeat(280); // Max tweet length
	let tweet = TweetInfo::new(
		Uuid::parse_str("11111111-1111-1111-1111-111111111111").unwrap(),
		Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap(),
		"testuser".to_string(),
		long_content.clone(),
		0,
		0,
		"2025-01-01T12:00:00Z".to_string(),
	);

	let view = tweet_card(&tweet, false);

	if let View::Element(element) = view {
		let html = element.to_html();
		assert!(
			html.contains(&long_content),
			"Should render full tweet content"
		);
	} else {
		panic!("Expected View::Element");
	}
}

// ============================================================================
// Tweet Form Rendering Tests
// ============================================================================

/// Test tweet form renders as a View::Element
#[wasm_bindgen_test]
fn test_tweet_form_renders() {
	let view = tweet_form();
	assert!(matches!(view, View::Element(_)));
}

/// Test tweet form contains expected structure
#[wasm_bindgen_test]
fn test_tweet_form_structure() {
	let view = tweet_form();

	if let View::Element(element) = view {
		let html = element.to_html();

		// Verify card container
		assert!(html.contains("card"));
		assert!(html.contains("card-body"));

		// Verify title
		assert!(html.contains("What's happening?"));

		// Verify form element
		assert!(html.contains("<form"));

		// Verify textarea
		assert!(html.contains("<textarea"));
		assert!(html.contains("form-control"));
		assert!(html.contains("maxlength=\"280\""));

		// Verify submit button
		assert!(html.contains("<button"));
		assert!(html.contains("Post"));
	} else {
		panic!("Expected View::Element, got Fragment or None");
	}
}

/// Test tweet form has character count display
#[wasm_bindgen_test]
fn test_tweet_form_char_count_display() {
	let view = tweet_form();

	if let View::Element(element) = view {
		let html = element.to_html();
		// Initial character count should be 0/280
		assert!(
			html.contains("/280"),
			"Should show character count with /280 format"
		);
	} else {
		panic!("Expected View::Element");
	}
}

/// Test tweet form has placeholder text
#[wasm_bindgen_test]
fn test_tweet_form_placeholder() {
	let view = tweet_form();

	if let View::Element(element) = view {
		let html = element.to_html();
		assert!(
			html.contains("placeholder"),
			"Should have placeholder attribute"
		);
		assert!(
			html.contains("What's on your mind?"),
			"Should have correct placeholder text"
		);
	} else {
		panic!("Expected View::Element");
	}
}

/// Test tweet form textarea has correct attributes
#[wasm_bindgen_test]
fn test_tweet_form_textarea_attributes() {
	let view = tweet_form();

	if let View::Element(element) = view {
		let html = element.to_html();
		assert!(html.contains("id=\"content\""));
		assert!(html.contains("name=\"content\""));
		assert!(html.contains("rows=\"3\""));
		assert!(html.contains("maxlength=\"280\""));
	} else {
		panic!("Expected View::Element");
	}
}

// ============================================================================
// Tweet List Rendering Tests
// ============================================================================

/// Test tweet list renders as a View::Element
#[wasm_bindgen_test]
fn test_tweet_list_renders() {
	// Note: tweet_list() will attempt to fetch tweets via server_fn
	// In the mock environment, this will show loading state initially
	let view = tweet_list(None);
	assert!(matches!(view, View::Element(_)));
}

/// Test tweet list renders for specific user
#[wasm_bindgen_test]
fn test_tweet_list_for_user_renders() {
	let user_id = Uuid::parse_str("22222222-2222-2222-2222-222222222222").unwrap();
	let view = tweet_list(Some(user_id));
	assert!(matches!(view, View::Element(_)));
}

// ============================================================================
// Mock Infrastructure Tests for Tweets
// ============================================================================

/// Test mock infrastructure for tweet list endpoint
#[wasm_bindgen_test]
fn test_mock_tweet_list_endpoint() {
	clear_mocks();

	let tweets = mock_tweet_list();
	mock_server_fn("/api/server_fn/list_tweets", &tweets);

	// Verify mock was registered (no calls yet)
	assert_server_fn_not_called("/api/server_fn/list_tweets");
	assert_server_fn_call_count("/api/server_fn/list_tweets", 0);

	clear_mocks();
}

/// Test mock infrastructure for create tweet endpoint
#[wasm_bindgen_test]
fn test_mock_create_tweet_endpoint() {
	clear_mocks();

	let tweet = mock_tweet_info();
	mock_server_fn("/api/server_fn/create_tweet", &tweet);

	assert_server_fn_not_called("/api/server_fn/create_tweet");

	clear_mocks();
}

/// Test mock infrastructure for delete tweet endpoint
#[wasm_bindgen_test]
fn test_mock_delete_tweet_endpoint() {
	clear_mocks();

	// Mock successful delete (returns unit type)
	mock_server_fn("/api/server_fn/delete_tweet", &());

	assert_server_fn_not_called("/api/server_fn/delete_tweet");

	clear_mocks();
}

/// Test mock error for tweet endpoints
#[wasm_bindgen_test]
fn test_mock_tweet_error_endpoints() {
	clear_mocks();

	// Mock various error scenarios
	mock_server_fn_error(
		"/api/server_fn/create_tweet",
		401,
		"Unauthorized: Please login",
	);
	mock_server_fn_error("/api/server_fn/delete_tweet", 403, "Permission denied");
	mock_server_fn_error("/api/server_fn/list_tweets", 500, "Internal server error");

	// Verify none were called
	assert_server_fn_not_called("/api/server_fn/create_tweet");
	assert_server_fn_not_called("/api/server_fn/delete_tweet");
	assert_server_fn_not_called("/api/server_fn/list_tweets");

	clear_mocks();
}

// ============================================================================
// Shared Types Serialization Tests
// ============================================================================

/// Test TweetInfo serialization
#[wasm_bindgen_test]
fn test_tweet_info_serialization() {
	let tweet = mock_tweet_info();

	let json = serde_json::to_string(&tweet).expect("Should serialize TweetInfo");
	assert!(json.contains("testuser"));
	assert!(json.contains("Hello, this is a test tweet!"));
	assert!(json.contains("like_count"));
	assert!(json.contains("retweet_count"));
}

/// Test TweetInfo deserialization
#[wasm_bindgen_test]
fn test_tweet_info_deserialization() {
	let json = r#"{
        "id": "11111111-1111-1111-1111-111111111111",
        "user_id": "22222222-2222-2222-2222-222222222222",
        "username": "testuser",
        "content": "Test content",
        "like_count": 10,
        "retweet_count": 5,
        "created_at": "2025-01-01T12:00:00Z"
    }"#;

	let tweet: TweetInfo = serde_json::from_str(json).expect("Should deserialize TweetInfo");
	assert_eq!(tweet.username, "testuser");
	assert_eq!(tweet.content, "Test content");
	assert_eq!(tweet.like_count, 10);
	assert_eq!(tweet.retweet_count, 5);
}

/// Test CreateTweetRequest serialization
#[wasm_bindgen_test]
fn test_create_tweet_request_serialization() {
	let request = CreateTweetRequest {
		content: "This is my new tweet".to_string(),
	};

	let json = serde_json::to_string(&request).expect("Should serialize CreateTweetRequest");
	assert!(json.contains("This is my new tweet"));
	assert!(json.contains("content"));
}

/// Test TweetInfo roundtrip serialization
#[wasm_bindgen_test]
fn test_tweet_info_roundtrip() {
	let original = mock_tweet_info();
	let json = serde_json::to_string(&original).expect("Should serialize");
	let deserialized: TweetInfo = serde_json::from_str(&json).expect("Should deserialize");

	assert_eq!(original.id, deserialized.id);
	assert_eq!(original.user_id, deserialized.user_id);
	assert_eq!(original.username, deserialized.username);
	assert_eq!(original.content, deserialized.content);
	assert_eq!(original.like_count, deserialized.like_count);
	assert_eq!(original.retweet_count, deserialized.retweet_count);
}

/// Test tweet list serialization
#[wasm_bindgen_test]
fn test_tweet_list_serialization() {
	let tweets = mock_tweet_list();

	let json = serde_json::to_string(&tweets).expect("Should serialize tweet list");
	let deserialized: Vec<TweetInfo> =
		serde_json::from_str(&json).expect("Should deserialize tweet list");

	assert_eq!(tweets.len(), deserialized.len());
	assert_eq!(tweets[0].username, deserialized[0].username);
	assert_eq!(tweets[1].content, deserialized[1].content);
}

// ============================================================================
// TweetInfo Constructor Tests
// ============================================================================

/// Test TweetInfo::new constructor
#[wasm_bindgen_test]
fn test_tweet_info_new() {
	let id = Uuid::new_v4();
	let user_id = Uuid::new_v4();
	let username = "testuser".to_string();
	let content = "Test content".to_string();
	let like_count = 5;
	let retweet_count = 2;
	let created_at = "2025-01-01T12:00:00Z".to_string();

	let tweet = TweetInfo::new(
		id,
		user_id,
		username.clone(),
		content.clone(),
		like_count,
		retweet_count,
		created_at.clone(),
	);

	assert_eq!(tweet.id, id);
	assert_eq!(tweet.user_id, user_id);
	assert_eq!(tweet.username, username);
	assert_eq!(tweet.content, content);
	assert_eq!(tweet.like_count, like_count);
	assert_eq!(tweet.retweet_count, retweet_count);
	assert_eq!(tweet.created_at, created_at);
}

/// Test TweetInfo with zero counts
#[wasm_bindgen_test]
fn test_tweet_info_zero_counts() {
	let tweet = TweetInfo::new(
		Uuid::new_v4(),
		Uuid::new_v4(),
		"user".to_string(),
		"content".to_string(),
		0,
		0,
		"2025-01-01T12:00:00Z".to_string(),
	);

	assert_eq!(tweet.like_count, 0);
	assert_eq!(tweet.retweet_count, 0);
}
