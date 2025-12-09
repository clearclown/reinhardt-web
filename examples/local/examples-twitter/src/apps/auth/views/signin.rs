//! Signin view handlers
//!
//! Handles user signin endpoints with JWT token generation.
//!
//! Uses reinhardt ORM (Manager/QuerySet) for database operations.

use crate::apps::auth::models::User;
use crate::apps::auth::serializers::{SigninRequest, SigninResponse, SigninUserInfo};
use crate::config::settings::get_settings;
use chrono::Utc;
use reinhardt::prelude::*;
use reinhardt::db::orm::{FilterOperator, FilterValue};
use reinhardt::{BaseUser, Error, JwtAuth, Json};
use validator::Validate;

/// Sign in a user and return JWT token
///
/// POST /accounts/auth/signin/
/// Request body:
/// ```json
/// {
///   "email": "user@example.com",
///   "password": "password123"
/// }
/// ```
/// Success response: 200 OK with JWT token and user info
/// Error responses:
/// - 400 Bad Request: Validation errors
/// - 401 Unauthorized: Invalid credentials or inactive user
#[post("/signin/", name = "signin", use_inject = true)]
pub async fn signin(
	Json(signin_req): Json<SigninRequest>,
	#[inject] db: DatabaseConnection,
) -> ViewResult<Response> {
	// Validate request
	signin_req
		.validate()
		.map_err(|e| Error::Validation(format!("Validation failed: {}", e)))?;

	// Find user by email using Manager/QuerySet API
	let manager = User::objects();
	let user_result = manager
		.filter(
			"email",
			FilterOperator::Eq,
			FilterValue::String(signin_req.email.trim().to_string()),
		)
		.first()
		.await;

	let user = match user_result {
		Ok(Some(u)) => u,
		Ok(None) => {
			return Err(Error::Authentication("Invalid credentials".into()));
		}
		Err(e) => {
			return Err(Error::Database(format!("Database error: {}", e)));
		}
	};

	// Check password using BaseUser trait
	let password_valid = user
		.check_password(&signin_req.password)
		.map_err(|e| Error::Authentication(format!("Password verification failed: {}", e)))?;

	if !password_valid {
		return Err(Error::Authentication("Invalid credentials".into()));
	}

	// Check if user is active
	if !user.is_active() {
		return Err(Error::Authentication("User account is inactive".into()));
	}

	// Get JWT secret from settings
	let settings = get_settings();
	let jwt_secret = settings.secret_key.as_bytes();

	// Generate JWT token
	let jwt_auth = JwtAuth::new(jwt_secret);
	let token = jwt_auth
		.generate_token(user.id.to_string(), user.username.clone())
		.map_err(|e| Error::Authentication(format!("Token generation failed: {}", e)))?;

	// Update last_login
	let mut updated_user = user.clone();
	updated_user.set_last_login(Utc::now());
	manager
		.update_with_conn(&db, &updated_user)
		.await
		.map_err(|e| Error::Database(format!("Failed to update last_login: {}", e)))?;

	// Build response
	let response = SigninResponse {
		token,
		user: SigninUserInfo {
			id: user.id.to_string(),
			username: user.username.clone(),
			email: user.email.clone(),
		},
	};

	Response::ok().with_json(&response).map_err(Into::into)
}
