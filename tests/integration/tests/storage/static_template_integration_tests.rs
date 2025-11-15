//! Integration tests for static files and template system
//!
//! Tests the integration between reinhardt-static and reinhardt-templates,
//! verifying that templates can correctly generate URLs to static files
//! with and without manifest-based hashing.

use reinhardt_static::{
	init_template_static_config, init_template_static_config_with_manifest,
	ManifestStaticFilesStorage, StaticFilesConfig, TemplateStaticConfig,
};
use reinhardt_templates::{static_filter, StaticConfig};
use serial_test::serial;
use std::collections::HashMap;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
#[serial(static_config)]
fn test_static_filter_basic_integration() {
	// Initialize static configuration
	let config = StaticConfig {
		static_url: "/static/".to_string(),
		use_manifest: false,
		manifest: HashMap::new(),
	};

	reinhardt_templates::init_static_config(config);

	// Test static filter
	assert_eq!(
		static_filter("css/style.css").unwrap(),
		"/static/css/style.css"
	);
	assert_eq!(static_filter("js/app.js").unwrap(), "/static/js/app.js");
	assert_eq!(
		static_filter("images/logo.png").unwrap(),
		"/static/images/logo.png"
	);
}

#[test]
#[serial(static_config)]
fn test_static_filter_with_manifest() {
	// Create manifest with hashed filenames
	let mut manifest = HashMap::new();
	manifest.insert(
		"css/style.css".to_string(),
		"css/style.abc123.css".to_string(),
	);
	manifest.insert("js/app.js".to_string(), "js/app.def456.js".to_string());

	let config = StaticConfig {
		static_url: "/static/".to_string(),
		use_manifest: true,
		manifest,
	};

	reinhardt_templates::init_static_config(config);

	// Test static filter with hashed filenames
	assert_eq!(
		static_filter("css/style.css").unwrap(),
		"/static/css/style.abc123.css"
	);
	assert_eq!(
		static_filter("js/app.js").unwrap(),
		"/static/js/app.def456.js"
	);

	// Non-hashed file should use original path
	assert_eq!(
		static_filter("images/logo.png").unwrap(),
		"/static/images/logo.png"
	);
}

#[test]
#[serial(static_config)]
fn test_init_template_static_config_from_static_files_config() {
	let static_config = StaticFilesConfig {
		static_root: PathBuf::from("/var/www/static"),
		static_url: "/assets/".to_string(),
		staticfiles_dirs: vec![],
		media_url: None,
	};

	init_template_static_config(&static_config);

	// Verify the configuration was applied
	assert_eq!(
		static_filter("css/style.css").unwrap(),
		"/assets/css/style.css"
	);
}

#[tokio::test]
#[serial(static_config)]
async fn test_init_template_static_config_with_manifest() {
	let temp_dir = tempdir().unwrap();
	let static_root = temp_dir.path().to_path_buf();

	// Create a simple manifest file
	let manifest_content = r#"{
  "css/style.css": "css/style.abc123def.css",
  "js/app.js": "js/app.456789abc.js"
}"#;

	std::fs::write(static_root.join("staticfiles.json"), manifest_content).unwrap();

	// Create storage
	let storage = ManifestStaticFilesStorage::new(static_root, "/static/");

	// Initialize template config from storage
	init_template_static_config_with_manifest(&storage)
		.await
		.unwrap();

	// Verify manifest is loaded and applied
	assert_eq!(
		static_filter("css/style.css").unwrap(),
		"/static/css/style.abc123def.css"
	);
	assert_eq!(
		static_filter("js/app.js").unwrap(),
		"/static/js/app.456789abc.js"
	);
}

#[tokio::test]
#[serial(static_config)]
async fn test_template_static_config_conversion() {
	let temp_dir = tempdir().unwrap();
	let static_root = temp_dir.path().to_path_buf();

	// Create manifest
	let manifest_content = r#"{
  "main.js": "main.hash123.js",
  "app.css": "app.hash456.css"
}"#;

	std::fs::write(static_root.join("staticfiles.json"), manifest_content).unwrap();

	// Create storage and load config
	let storage = ManifestStaticFilesStorage::new(static_root, "/cdn/");
	let template_config = TemplateStaticConfig::from_storage(&storage).await.unwrap();

	assert_eq!(template_config.static_url, "/cdn/");
	assert!(template_config.use_manifest);
	assert_eq!(template_config.manifest.len(), 2);

	// Convert to reinhardt_templates::StaticConfig
	let static_config = StaticConfig::from(template_config);
	reinhardt_templates::init_static_config(static_config);

	// Verify it works
	assert_eq!(static_filter("main.js").unwrap(), "/cdn/main.hash123.js");
	assert_eq!(static_filter("app.css").unwrap(), "/cdn/app.hash456.css");
}

#[tokio::test]
#[serial(static_config)]
async fn test_template_static_config_missing_manifest() {
	let temp_dir = tempdir().unwrap();
	let static_root = temp_dir.path().to_path_buf();

	// No manifest file created
	let storage = ManifestStaticFilesStorage::new(static_root, "/static/");
	let template_config = TemplateStaticConfig::from_storage(&storage).await.unwrap();

	// Should not use manifest if file doesn't exist
	assert_eq!(template_config.static_url, "/static/");
	assert!(!template_config.use_manifest);
	assert!(template_config.manifest.is_empty());
}

#[test]
#[serial(static_config)]
fn test_custom_static_url() {
	let config = StaticConfig {
		static_url: "https://cdn.example.com/assets/".to_string(),
		use_manifest: false,
		manifest: HashMap::new(),
	};

	reinhardt_templates::init_static_config(config);

	assert_eq!(
		static_filter("css/style.css").unwrap(),
		"https://cdn.example.com/assets/css/style.css"
	);
}

#[test]
#[serial(static_config)]
fn test_leading_slash_normalization() {
	let config = StaticConfig {
		static_url: "/static/".to_string(),
		use_manifest: false,
		manifest: HashMap::new(),
	};

	reinhardt_templates::init_static_config(config);

	// Leading slash should be removed
	assert_eq!(
		static_filter("/css/style.css").unwrap(),
		"/static/css/style.css"
	);
	assert_eq!(
		static_filter("css/style.css").unwrap(),
		"/static/css/style.css"
	);
}

#[test]
#[serial(static_config)]
fn test_trailing_slash_in_static_url() {
	let config = StaticConfig {
		static_url: "/static".to_string(), // No trailing slash
		use_manifest: false,
		manifest: HashMap::new(),
	};

	reinhardt_templates::init_static_config(config);

	assert_eq!(
		static_filter("css/style.css").unwrap(),
		"/static/css/style.css"
	);
}
