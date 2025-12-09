//! Verify password view handlers
//!
//! Handles password verification endpoints.
//!
//! This endpoint requires authentication via JWT token.

use crate::apps::auth::models::User;
use crate::apps::auth::serializers::{VerifyPasswordRequest, VerifyPasswordResponse};
use crate::config::settings::get_settings;
use reinhardt::db::orm::Model;
use reinhardt::db::DatabaseConnection;
use reinhardt::post;
use reinhardt::reinhardt_params::{Authorization, HeaderNamed};
use reinhardt::{BaseUser, Error, Json, JwtAuth, Response, ViewResult};
use uuid::Uuid;
use validator::Validate;

/// Verify current user's password
///
/// POST /accounts/auth/verify-password/
/// Headers:
///   Authorization: Bearer <token>
/// Request body:
/// ```json
/// {
///   "password": "current_password"
/// }
/// ```
/// Success response: 200 OK with { "valid": true/false }
/// Error responses:
/// - 400 Bad Request: Validation errors
/// - 401 Unauthorized: Missing or invalid token
#[post("/verify-password/", name = "verify_password", use_inject = true)]
pub async fn verify_password(
	auth_header: HeaderNamed<Authorization, String>,
	Json(verify_req): Json<VerifyPasswordRequest>,
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
	verify_req
		.validate()
		.map_err(|e| Error::Validation(format!("Validation failed: {}", e)))?;

	// Get user ID from claims
	let user_id = Uuid::parse_str(&claims.sub)
		.map_err(|_| Error::Authentication("Invalid user ID in token".into()))?;

	// Find user by ID using Model::objects() API
	let user = User::objects()
		.filter_by(User::field_id().eq(user_id))
		.first_with_db(&db)
		.await
		.map_err(|e| Error::Database(format!("Database error: {}", e)))?
		.ok_or_else(|| Error::Authentication("User not found".into()))?;

	// Check password using BaseUser trait
	let valid = user.check_password(&verify_req.password).unwrap_or(false);

	let response = VerifyPasswordResponse { valid };

	Response::ok().with_json(&response).map_err(Into::into)
}
