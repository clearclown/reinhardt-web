//! issues application module
//!
//! A RESTful API application

use reinhardt::AppConfig;

pub mod admin;
pub mod models;
pub mod serializers;
pub mod urls;
pub mod views;

#[derive(AppConfig)]
#[app_config(name = "issues", label = "issues")]
pub struct IssuesConfig;

// Re-export as Issues for use in src/apps.rs
pub use IssuesConfig as Issues;
