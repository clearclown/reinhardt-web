//! Tests for AppConfig derive macro
//!
//! Tests that verify the correct generation of AppConfig factory methods.

use reinhardt_macros::AppConfig;

// Re-export reinhardt_apps module so that the macro can find it
#[allow(unused_imports)]
mod reinhardt_apps {
	pub use reinhardt_apps::*;
}

#[derive(AppConfig)]
#[app_config(name = "api", label = "api")]
pub struct ApiConfig;

#[derive(AppConfig)]
#[app_config(name = "todos", label = "todos", verbose_name = "TODO Application")]
pub struct TodosConfig;

#[derive(AppConfig)]
#[app_config(name = "users", label = "users")]
pub struct UsersConfig;

#[test]
fn test_basic_app_config() {
	let config = ApiConfig::config();
	assert_eq!(config.name, "api");
	assert_eq!(config.label, "api");
	assert_eq!(config.verbose_name, None);
}

#[test]
fn test_app_config_with_verbose_name() {
	let config = TodosConfig::config();
	assert_eq!(config.name, "todos");
	assert_eq!(config.label, "todos");
	assert_eq!(config.verbose_name, Some("TODO Application".to_string()));
}

#[test]
fn test_multiple_configs() {
	let api_config = ApiConfig::config();
	let todos_config = TodosConfig::config();
	let users_config = UsersConfig::config();

	assert_eq!(api_config.name, "api");
	assert_eq!(todos_config.name, "todos");
	assert_eq!(users_config.name, "users");
}

#[test]
fn test_config_builder_methods() {
	let config = ApiConfig::config()
		.with_path("/path/to/api")
		.with_default_auto_field("AutoField");

	assert_eq!(config.name, "api");
	assert_eq!(config.path, Some("/path/to/api".to_string()));
	assert_eq!(config.default_auto_field, Some("AutoField".to_string()));
}
