//! Signout serializers
//!
//! Serializers for user signout endpoints

use serde::{Deserialize, Serialize};

/// Response data for successful signout
#[derive(Debug, Serialize, Deserialize)]
pub struct SignoutResponse {
	/// Success message
	pub message: String,
}
