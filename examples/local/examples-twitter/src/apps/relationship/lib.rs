//! relationship application module
//!
//! User relationship models for examples-twitter

use reinhardt::app_config;

pub mod shared;
pub mod urls;

#[cfg(target_arch = "wasm32")]
pub mod client;

#[cfg(not(target_arch = "wasm32"))]
pub mod server;

#[cfg(test)]
pub mod tests;

#[app_config(
	name = "relationship",
	label = "relationship",
	verbose_name = "User Relationships"
)]
pub struct RelationshipConfig;
