//! Change password view handlers
//!
//! Handles password change endpoints.
//!
//! This endpoint requires authentication via JWT token.

use crate::apps::auth::models::User;
use crate::apps::auth::serializers::ChangePasswordRequest;
use crate::config::settings::get_settings;
use reinhardt::prelude::*;
use reinhardt::db::orm::{FilterOperator, FilterValue};
use reinhardt::reinhardt_params::{Authorization, HeaderNamed};
use reinhardt::{BaseUser, Error, Json, JwtAuth};
use uuid::Uuid;
use validator::Validate;

/// Change current user's password
///
/// POST /accounts/auth/change-password/
/// Headers:
///   Authorization: Bearer <token>
/// Request body:
/// ```json
/// {
///   "current_password": "old_password",
///   "new_password": "new_password123",
///   "new_password_confirmation": "new_password123"
/// }
/// ```
/// Success response: 204 No Content
/// Error responses:
/// - 400 Bad Request: Validation errors or passwords don't match
/// - 401 Unauthorized: Missing/invalid token or wrong current password
#[post("/change-password/", name = "change_password", use_inject = true)]
pub async fn change_password(
	auth_header: HeaderNamed<Authorization, String>,
	Json(change_req): Json<ChangePasswordRequest>,
	#[inject] db: DatabaseConnection,
) -> ViewResult<Response> {
	// Extract Bearer token from Authorization header
	let token = auth_header
		.strip_prefix("Bearer ")
		.ok_or_else(|| Error::Authentication("Invalid Authorization header format".into()))?;

	// Get JWT secret from settings
	let settings = get_settings();
	let jwt_secret = settings.secret_key.as_bytes();

	let jwt_auth = JwtAuth::new(jwt_secret);
	let claims = jwt_auth
		.verify_token(token)
		.map_err(|e| Error::Authentication(format!("Invalid token: {}", e)))?;

	// Validate request
	change_req
		.validate()
		.map_err(|e| Error::Validation(format!("Validation failed: {}", e)))?;

	// Validate passwords match
	change_req
		.validate_passwords_match()
		.map_err(|e| Error::Validation(format!("Password validation failed: {}", e)))?;

	// Get user ID from claims
	let user_id = Uuid::parse_str(&claims.sub)
		.map_err(|_| Error::Authentication("Invalid user ID in token".into()))?;

	// Find user by ID using Manager API
	let manager = User::objects();
	let user = manager
		.filter(
			"id",
			FilterOperator::Eq,
			FilterValue::String(user_id.to_string()),
		)
		.first()
		.await
		.map_err(|e| Error::Database(format!("Database error: {}", e)))?
		.ok_or_else(|| Error::Authentication("User not found".into()))?;

	// Verify current password using BaseUser trait
	let current_password_valid = user
		.check_password(&change_req.current_password)
		.map_err(|e| Error::Authentication(format!("Password verification failed: {}", e)))?;

	if !current_password_valid {
		return Err(Error::Authentication("Current password is incorrect".into()));
	}

	// Set new password
	let mut updated_user = user.clone();
	updated_user
		.set_password(&change_req.new_password)
		.map_err(|e| Error::Database(format!("Password hashing failed: {}", e)))?;

	// Update user in database using Manager
	manager
		.update_with_conn(&db, &updated_user)
		.await
		.map_err(|e| Error::Database(format!("Failed to update password: {}", e)))?;

	Ok(Response::new(StatusCode::NO_CONTENT))
}
