//! Reset password view handlers
//!
//! Handles password reset token generation and confirmation endpoints.
//!
//! Endpoints:
//! - POST /reset-password/ - Generate reset token and store in DB
//! - POST /reset-password/confirm/ - Verify token and set new password

use crate::apps::auth::models::{PasswordResetToken, User};
use crate::apps::auth::serializers::{
	ResetPasswordConfirmRequest, ResetPasswordConfirmResponse, ResetPasswordRequest,
	ResetPasswordResponse,
};
use chrono::{Duration, Utc};
use reinhardt::db::DatabaseConnection;
use reinhardt::db::associations::ForeignKeyField;
use reinhardt::db::orm::{FilterOperator, FilterValue, Model};
use reinhardt::post;
use reinhardt::{BaseUser, Error, Json, Response, ViewResult};
use uuid::Uuid;
use validator::Validate;

/// Token validity duration in hours
const RESET_TOKEN_VALIDITY_HOURS: i64 = 24;

/// Request password reset token
///
/// POST /accounts/auth/reset-password/
/// Request body:
/// ```json
/// {
///   "email": "user@example.com"
/// }
/// ```
/// Success response: 200 OK with reset token (development only)
///
/// This endpoint:
/// 1. Validates the email format
/// 2. Looks up the user by email
/// 3. Generates a UUID v4 reset token
/// 4. Creates a PasswordResetToken record in the database
/// 5. Returns success (even if user doesn't exist, for security)
///
/// Note: For security reasons, this endpoint returns success even if the email
/// doesn't exist in the database. This prevents email enumeration attacks.
#[post("/reset-password/", name = "reset_password", use_inject = true)]
pub async fn reset_password(
	Json(reset_req): Json<ResetPasswordRequest>,
	#[inject] db: DatabaseConnection,
) -> ViewResult<Response> {
	// Validate request
	reset_req
		.validate()
		.map_err(|e| Error::Validation(format!("Validation failed: {}", e)))?;

	// Generate reset token (UUID v4)
	let reset_token = Uuid::new_v4().to_string();

	// Find user by email using Model::objects() API
	let user_result = User::objects()
		.filter(
			User::field_email(),
			FilterOperator::Eq,
			FilterValue::String(reset_req.email.trim().to_string()),
		)
		.first()
		.await;

	// Process user lookup result
	match user_result {
		Ok(Some(user)) => {
			// User exists - create PasswordResetToken
			let expires = Utc::now() + Duration::hours(RESET_TOKEN_VALIDITY_HOURS);

			let token_record = PasswordResetToken {
				id: Uuid::new_v4(),
				user: ForeignKeyField::new(),
				user_id: user.id,
				token: reset_token.clone(),
				expires_at: expires,
				created_at: Utc::now(),
				is_used: false,
			};

			// Save the token to database
			if let Err(_e) = PasswordResetToken::objects()
				.create_with_conn(&db, &token_record)
				.await
			{
				// Log error but still return success for security
				// In production, use proper logging
			}
		}
		Ok(None) => {
			// User doesn't exist - return success anyway to prevent email enumeration
		}
		Err(_e) => {
			// Database error - still return success for security
		}
	}

	let response = ResetPasswordResponse { reset_token };

	Response::ok().with_json(&response)
}

/// Confirm password reset with new password
///
/// POST /accounts/auth/reset-password/confirm/
/// Request body:
/// ```json
/// {
///   "token": "reset-token-uuid",
///   "new_password": "new-secure-password",
///   "new_password_confirmation": "new-secure-password"
/// }
/// ```
/// Success response: 200 OK with success message
/// Error responses:
/// - 400 Bad Request: Validation failed or passwords don't match
/// - 400 Bad Request: Invalid or expired token
///
/// This endpoint:
/// 1. Validates the request (token, password length, password match)
/// 2. Looks up the PasswordResetToken by token value
/// 3. Verifies the token hasn't expired and hasn't been used
/// 4. Sets the new password using BaseUser::set_password
/// 5. Marks the token as used in the database
#[post(
	"/reset-password/confirm/",
	name = "reset_password_confirm",
	use_inject = true
)]
pub async fn reset_password_confirm(
	Json(confirm_req): Json<ResetPasswordConfirmRequest>,
	#[inject] db: DatabaseConnection,
) -> ViewResult<Response> {
	// Validate request structure
	confirm_req
		.validate()
		.map_err(|e| Error::Validation(format!("Validation failed: {}", e)))?;

	// Check passwords match
	if !confirm_req.passwords_match() {
		return Err(Error::Validation("Passwords do not match".to_string()));
	}

	// Find PasswordResetToken by token value
	let token_result = PasswordResetToken::objects()
		.filter(
			PasswordResetToken::field_token(),
			FilterOperator::Eq,
			FilterValue::String(confirm_req.token.clone()),
		)
		.first()
		.await
		.map_err(|e| Error::Database(format!("Database error: {}", e)))?;

	let mut token_record = match token_result {
		Some(t) => t,
		None => {
			return Err(Error::Validation("Invalid reset token".to_string()));
		}
	};

	// Check if token is already used
	if token_record.is_used {
		return Err(Error::Validation(
			"Reset token has already been used".to_string(),
		));
	}

	// Check token expiration
	let now = Utc::now();
	if token_record.expires_at <= now {
		return Err(Error::Validation("Reset token has expired".to_string()));
	}

	// Find user by user_id from token record
	let user_result = User::objects()
		.filter(
			User::field_id(),
			FilterOperator::Eq,
			FilterValue::String(token_record.user_id.to_string()),
		)
		.first()
		.await
		.map_err(|e| Error::Database(format!("Database error: {}", e)))?;

	let mut user = match user_result {
		Some(u) => u,
		None => {
			return Err(Error::Database("User not found".to_string()));
		}
	};

	// Set new password using BaseUser trait
	user.set_password(&confirm_req.new_password)
		.map_err(|e| Error::Validation(format!("Failed to set password: {}", e)))?;

	// Save updated user to database
	User::objects()
		.update_with_conn(&db, &user)
		.await
		.map_err(|e| Error::Database(format!("Failed to update user: {}", e)))?;

	// Mark token as used
	token_record.is_used = true;
	PasswordResetToken::objects()
		.update_with_conn(&db, &token_record)
		.await
		.map_err(|e| Error::Database(format!("Failed to update token: {}", e)))?;

	let response = ResetPasswordConfirmResponse {
		message: "Password has been reset successfully".to_string(),
	};

	Response::ok().with_json(&response)
}
