//! Shared type definitions for Reinhardt admin panel
//!
//! This crate provides common type definitions used by both the admin panel API
//! (backend) and UI (frontend) components.
//!
//! # Main modules
//!
//! - [`errors`]: Error types and result type alias
//! - [`models`]: Model information types
//! - [`requests`]: Request body types for API endpoints
//! - [`responses`]: Response types for API endpoints

pub mod errors;
pub mod models;
pub mod requests;
pub mod responses;

// Re-export all public types
pub use errors::*;
pub use models::*;
pub use requests::*;
pub use responses::*;
