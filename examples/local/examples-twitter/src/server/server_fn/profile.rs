//! Profile server functions
//!
//! Server functions for user profile management.

use crate::apps::profile::models::Profile;
use crate::shared::types::{ProfileResponse, UpdateProfileRequest};
use reinhardt::DatabaseConnection;
use reinhardt::db::orm::{FilterOperator, FilterValue, Model};
use reinhardt::middleware::session::SessionData;
use reinhardt::pages::server_fn::{ServerFnError, server_fn};
use uuid::Uuid;
use validator::Validate;

/// Fetch user profile
#[server_fn(use_inject = true)]
pub async fn fetch_profile(
	user_id: Uuid,
	#[inject] _db: DatabaseConnection,
) -> std::result::Result<ProfileResponse, ServerFnError> {
	let profile = Profile::objects()
		.filter(
			Profile::field_user_id(),
			FilterOperator::Eq,
			FilterValue::String(user_id.to_string()),
		)
		.first()
		.await
		.map_err(|e| ServerFnError::server(500, format!("Database error: {}", e)))?
		.ok_or_else(|| ServerFnError::server(404, "Profile not found"))?;

	Ok(ProfileResponse::from(profile))
}

/// Update user profile
#[server_fn(use_inject = true)]
pub async fn update_profile(
	request: UpdateProfileRequest,
	#[inject] db: DatabaseConnection,
	#[inject] session: SessionData,
) -> std::result::Result<ProfileResponse, ServerFnError> {
	// Validate request
	request
		.validate()
		.map_err(|e| ServerFnError::server(400, format!("Validation failed: {}", e)))?;

	let user_id = session
		.get::<Uuid>("user_id")
		.ok_or_else(|| ServerFnError::server(401, "Not authenticated"))?;

	// Find existing profile
	let mut profile = Profile::objects()
		.filter(
			Profile::field_user_id(),
			FilterOperator::Eq,
			FilterValue::String(user_id.to_string()),
		)
		.first()
		.await
		.map_err(|e| ServerFnError::server(500, format!("Database error: {}", e)))?
		.ok_or_else(|| ServerFnError::server(404, "Profile not found"))?;

	// Update fields
	if let Some(bio) = request.bio {
		profile.set_bio(bio);
	}
	if let Some(avatar_url) = request.avatar_url {
		profile.set_avatar_url(avatar_url);
	}
	if let Some(location) = request.location {
		profile.set_location(Some(location));
	}
	if let Some(website) = request.website {
		profile.set_website(Some(website));
	}

	// Save to database
	Profile::objects()
		.update_with_conn(&db, &profile)
		.await
		.map_err(|e| ServerFnError::server(500, format!("Database error: {}", e)))?;

	Ok(ProfileResponse::from(profile))
}
