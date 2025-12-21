//! Server function tests
//!
//! Tests for server functions including authentication, profile, tweets, and relationships.

use examples_twitter::shared::types::{LoginRequest, RegisterRequest};
use validator::Validate;

#[cfg(test)]
mod auth_tests {
	use super::*;

	#[test]
	fn test_register_request_validation_valid() {
		let request = RegisterRequest {
			username: "testuser".to_string(),
			email: "test@example.com".to_string(),
			password: "password123".to_string(),
			password_confirmation: "password123".to_string(),
		};

		assert!(request.validate().is_ok());
		assert!(request.validate_passwords_match().is_ok());
	}

	#[test]
	fn test_register_request_validation_short_username() {
		let request = RegisterRequest {
			username: "ab".to_string(), // Too short (< 3 chars)
			email: "test@example.com".to_string(),
			password: "password123".to_string(),
			password_confirmation: "password123".to_string(),
		};

		assert!(request.validate().is_err());
	}

	#[test]
	fn test_register_request_validation_invalid_email() {
		let request = RegisterRequest {
			username: "testuser".to_string(),
			email: "invalid-email".to_string(), // Invalid email format
			password: "password123".to_string(),
			password_confirmation: "password123".to_string(),
		};

		assert!(request.validate().is_err());
	}

	#[test]
	fn test_register_request_validation_short_password() {
		let request = RegisterRequest {
			username: "testuser".to_string(),
			email: "test@example.com".to_string(),
			password: "short".to_string(), // Too short (< 8 chars)
			password_confirmation: "short".to_string(),
		};

		assert!(request.validate().is_err());
	}

	#[test]
	fn test_register_request_passwords_mismatch() {
		let request = RegisterRequest {
			username: "testuser".to_string(),
			email: "test@example.com".to_string(),
			password: "password123".to_string(),
			password_confirmation: "different123".to_string(), // Mismatch
		};

		assert!(request.validate_passwords_match().is_err());
	}

	#[test]
	fn test_login_request_validation_valid() {
		let request = LoginRequest {
			email: "test@example.com".to_string(),
			password: "password123".to_string(),
		};

		assert!(request.validate().is_ok());
	}

	#[test]
	fn test_login_request_validation_invalid_email() {
		let request = LoginRequest {
			email: "invalid-email".to_string(), // Invalid email format
			password: "password123".to_string(),
		};

		assert!(request.validate().is_err());
	}

	#[test]
	fn test_login_request_validation_empty_password() {
		let request = LoginRequest {
			email: "test@example.com".to_string(),
			password: "".to_string(), // Empty password
		};

		assert!(request.validate().is_err());
	}
}

#[cfg(test)]
mod profile_tests {
	use examples_twitter::shared::types::UpdateProfileRequest;
	use validator::Validate;

	#[test]
	fn test_update_profile_request_validation_valid() {
		let request = UpdateProfileRequest {
			bio: Some("This is my bio".to_string()),
			avatar_url: Some("https://example.com/avatar.jpg".to_string()),
			location: Some("New York".to_string()),
			website: Some("https://example.com".to_string()),
		};

		assert!(request.validate().is_ok());
	}

	#[test]
	fn test_update_profile_request_validation_long_bio() {
		let request = UpdateProfileRequest {
			bio: Some("a".repeat(501)), // Too long (> 500 chars)
			avatar_url: None,
			location: None,
			website: None,
		};

		assert!(request.validate().is_err());
	}

	#[test]
	fn test_update_profile_request_validation_invalid_avatar_url() {
		let request = UpdateProfileRequest {
			bio: None,
			avatar_url: Some("not-a-url".to_string()), // Invalid URL
			location: None,
			website: None,
		};

		assert!(request.validate().is_err());
	}

	#[test]
	fn test_update_profile_request_validation_long_location() {
		let request = UpdateProfileRequest {
			bio: None,
			avatar_url: None,
			location: Some("a".repeat(101)), // Too long (> 100 chars)
			website: None,
		};

		assert!(request.validate().is_err());
	}

	#[test]
	fn test_update_profile_request_validation_invalid_website() {
		let request = UpdateProfileRequest {
			bio: None,
			avatar_url: None,
			location: None,
			website: Some("not-a-url".to_string()), // Invalid URL
		};

		assert!(request.validate().is_err());
	}
}

#[cfg(test)]
mod tweet_tests {
	use examples_twitter::shared::types::CreateTweetRequest;
	use validator::Validate;

	#[test]
	fn test_create_tweet_request_validation_valid() {
		let request = CreateTweetRequest {
			content: "This is a valid tweet!".to_string(),
		};

		assert!(request.validate().is_ok());
	}

	#[test]
	fn test_create_tweet_request_validation_empty() {
		let request = CreateTweetRequest {
			content: "".to_string(), // Empty content
		};

		assert!(request.validate().is_err());
	}

	#[test]
	fn test_create_tweet_request_validation_too_long() {
		let request = CreateTweetRequest {
			content: "a".repeat(281), // Too long (> 280 chars)
		};

		assert!(request.validate().is_err());
	}

	#[test]
	fn test_create_tweet_request_validation_at_limit() {
		let request = CreateTweetRequest {
			content: "a".repeat(280), // Exactly 280 chars
		};

		assert!(request.validate().is_ok());
	}
}
