//! Signout view handlers
//!
//! Handles user signout endpoints.
//!
//! This endpoint requires authentication via JWT token.

use crate::config::settings::get_settings;
use reinhardt::post;
use reinhardt::reinhardt_params::{Authorization, HeaderNamed};
use reinhardt::{Error, JwtAuth, Response, StatusCode, ViewResult};

/// Sign out a user
///
/// POST /accounts/auth/signout/
/// Headers:
///   Authorization: Bearer <token>
///
/// Success response: 204 No Content
/// Error responses:
/// - 401 Unauthorized: Missing or invalid token
#[post("/signout/", name = "signout", use_inject = true)]
pub async fn signout(auth_header: HeaderNamed<Authorization, String>) -> ViewResult<Response> {
	// Extract Bearer token from Authorization header
	let token = auth_header
		.strip_prefix("Bearer ")
		.ok_or_else(|| Error::Authentication("Invalid Authorization header format".into()))?;

	// Get JWT secret from settings
	let settings = get_settings();
	let jwt_secret = settings.secret_key.as_bytes();

	// Verify token
	let jwt_auth = JwtAuth::new(jwt_secret);
	jwt_auth
		.verify_token(token)
		.map_err(|e| Error::Authentication(format!("Invalid token: {}", e)))?;

	// Token is valid - in a real application, you would:
	// 1. Add the token to a blacklist (if using token blacklisting)
	// 2. Delete the session from session store (if using sessions)
	// For this example, we just return success as JWT tokens are stateless

	Ok(Response::new(StatusCode::NO_CONTENT))
}
