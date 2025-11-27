//! api application module
//!
//! A RESTful API application demonstrating REST features

use reinhardt::AppConfig;

pub mod admin;
pub mod migrations;
pub mod models;
pub mod serializers;
pub mod storage;
pub mod urls;
pub mod views;

#[derive(AppConfig)]
#[app_config(name = "api", label = "api")]
pub struct ApiConfig;
