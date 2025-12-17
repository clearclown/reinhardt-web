//! projects application module
//!
//! A RESTful API application

use reinhardt::AppConfig;

pub mod admin;
pub mod models;
pub mod serializers;
pub mod urls;
pub mod views;

#[derive(AppConfig)]
#[app_config(name = "projects", label = "projects")]
pub struct ProjectsConfig;

// Re-export as Projects for use in src/apps.rs
pub use ProjectsConfig as Projects;
