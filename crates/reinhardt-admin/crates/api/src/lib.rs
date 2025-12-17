//! # Reinhardt Admin API
//!
//! Backend JSON API for Reinhardt admin panel.
//!
//! ## Overview
//!
//! This crate provides the backend JSON API implementation for the Reinhardt admin panel.
//! It handles all business logic, database operations, and model management.
//!
//! ## Features
//!
//! - ✅ RESTful JSON API endpoints
//! - ✅ CRUD operations (list, detail, create, update, delete)
//! - ✅ List views with filtering, searching, sorting, and pagination
//! - ✅ Import/Export (JSON, CSV, TSV)
//!
//! ## Architecture
//!
//! This crate is part of the 3-crate admin panel architecture:
//!
//! 1. **reinhardt-admin-types**: Shared type definitions (requests, responses, errors)
//! 2. **reinhardt-admin-api** (this crate): Backend JSON API implementation
//! 3. **reinhardt-admin-ui**: WASM CSR frontend (Dominator + futures-signals)
//!
//! ## Quick Start
//!
//! ### 1. Define Your Model
//!
//! ```rust,ignore
//! use reinhardt_orm::Model;
//! #[model(table_name = "users")]
//! pub struct User {
//!     #[field(primary_key)]
//!     pub id: i64,
//!     pub username: String,
//!     pub email: String,
//!     pub is_active: bool,
//! }
//! ```
//!
//! ### 2. Register with Admin Site
//!
//! ```rust,ignore
//! use reinhardt_admin_api::{AdminSite, ModelAdmin};
//!
//! // Define your admin implementation
//! struct UserAdmin;
//! impl ModelAdmin for UserAdmin {
//!     fn model_name(&self) -> &str { "User" }
//!     fn table_name(&self) -> &str { "users" }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let admin = AdminSite::new("My Admin");
//!     admin.register("User", UserAdmin).unwrap();
//!     // Use admin_routes() to generate routes
//! }
//! ```
//!
//! ### 3. Set Up Routes
//!
//! ```rust,ignore
//! use reinhardt_admin_api::admin_routes;
//!
//! let router = admin_routes();
//! // Add router to your Reinhardt application
//! ```
//!
//! ## Security
//!
//! Admin API should **only** be accessible to trusted internal staff. Always ensure:
//! - Proper authentication (session, JWT, etc.)
//! - HTTPS in production
//! - CORS configuration for frontend (same-origin recommended)

// Core modules
pub mod database;
pub mod export;
pub mod handlers;
pub mod import;
pub mod model_admin;
pub mod router;
pub mod site;

// Re-export types from types crate
pub use reinhardt_admin_types::*;

// Re-exports from this crate
pub use database::{AdminDatabase, AdminRecord};
pub use export::{
	CsvExporter, ExportBuilder, ExportConfig, ExportResult, JsonExporter, TsvExporter,
};
pub use handlers::ExportQueryParams;
pub use import::{
	CsvImporter, ImportBuilder, ImportConfig, ImportError, ImportFormat, ImportResult,
	JsonImporter, TsvImporter,
};
pub use model_admin::{ModelAdmin, ModelAdminConfig};
pub use router::{AdminRouter, admin_routes};
pub use site::AdminSite;
