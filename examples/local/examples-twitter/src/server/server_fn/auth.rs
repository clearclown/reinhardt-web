//! Authentication server functions
//!
//! Server functions for user authentication and session management.

use crate::apps::auth::models::User;
use crate::shared::types::{LoginRequest, RegisterRequest, UserInfo};
use reinhardt::db::orm::{FilterOperator, FilterValue, Model};
use reinhardt::middleware::session::{SessionData, SessionStoreRef};
use reinhardt::pages::server_fn::{ServerFnError, server_fn};
use reinhardt::{BaseUser, DatabaseConnection};
use uuid::Uuid;
use validator::Validate;

/// Login user and create session
#[server_fn(use_inject = true)]
pub async fn login(
	request: LoginRequest,
	#[inject] _db: DatabaseConnection,
	#[inject] session: SessionData,
	#[inject] store: SessionStoreRef,
) -> std::result::Result<UserInfo, ServerFnError> {
	// Validate request
	request
		.validate()
		.map_err(|e| ServerFnError::application(format!("Validation failed: {}", e)))?;

	// Find user by email
	let manager = User::objects();
	let user = manager
		.filter(
			User::field_email(),
			FilterOperator::Eq,
			FilterValue::String(request.email.trim().to_string()),
		)
		.first()
		.await
		.map_err(|e| ServerFnError::application(format!("Database error: {}", e)))?
		.ok_or_else(|| ServerFnError::server(401, "Invalid credentials"))?;

	// Check password
	let password_valid = user
		.check_password(&request.password)
		.map_err(|e| ServerFnError::application(format!("Password verification failed: {}", e)))?;

	if !password_valid {
		return Err(ServerFnError::server(401, "Invalid credentials"));
	}

	// Check if user is active
	if !user.is_active() {
		return Err(ServerFnError::server(403, "User account is inactive"));
	}

	// Set user ID in session
	let mut updated_session = session;
	updated_session
		.set("user_id".to_string(), user.id())
		.map_err(|e| ServerFnError::application(format!("Session error: {}", e)))?;

	// Save updated session to store
	store.inner().save(updated_session);

	Ok(UserInfo::from(user))
}

/// Register new user
#[server_fn(use_inject = true)]
pub async fn register(
	request: RegisterRequest,
	#[inject] db: DatabaseConnection,
) -> std::result::Result<(), ServerFnError> {
	// Validate request
	request
		.validate()
		.map_err(|e| ServerFnError::application(format!("Validation failed: {}", e)))?;

	// Validate password match
	request
		.validate_passwords_match()
		.map_err(ServerFnError::application)?;

	// Check if user already exists
	let existing = User::objects()
		.filter(
			User::field_email(),
			FilterOperator::Eq,
			FilterValue::String(request.email.trim().to_string()),
		)
		.first()
		.await
		.map_err(|e| ServerFnError::application(format!("Database error: {}", e)))?;

	if existing.is_some() {
		return Err(ServerFnError::application(
			"Email already exists".to_string(),
		));
	}

	// Create new user
	let mut new_user = User::new(
		request.username.trim().to_string(),
		request.email.trim().to_string(),
		None,
		true,
		None,
	);

	// Set password
	new_user
		.set_password(&request.password)
		.map_err(|e| ServerFnError::application(format!("Password hashing failed: {}", e)))?;

	// Save to database
	User::objects()
		.create_with_conn(&db, &new_user)
		.await
		.map_err(|e| ServerFnError::application(format!("Database error: {}", e)))?;

	Ok(())
}

/// Logout user
#[server_fn(use_inject = true)]
pub async fn logout(
	#[inject] session: SessionData,
	#[inject] store: SessionStoreRef,
) -> std::result::Result<(), ServerFnError> {
	// Delete session from store
	store.inner().delete(&session.id);
	Ok(())
}

/// Get current logged-in user
#[server_fn(use_inject = true)]
pub async fn current_user(
	#[inject] _db: DatabaseConnection,
	#[inject] session: SessionData,
) -> std::result::Result<Option<UserInfo>, ServerFnError> {
	// Get user ID from session
	let user_id = match session.get::<Uuid>("user_id") {
		Some(id) => id,
		None => return Ok(None),
	};

	// Find user by ID
	let user = User::objects()
		.filter(
			User::field_id(),
			FilterOperator::Eq,
			FilterValue::String(user_id.to_string()),
		)
		.first()
		.await
		.map_err(|e| ServerFnError::application(format!("Database error: {}", e)))?;

	Ok(user.map(UserInfo::from))
}
