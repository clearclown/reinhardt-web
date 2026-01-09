//! Admin panel test fixtures for reinhardt-admin workspace
//!
//! This module provides rstest fixtures for testing reinhardt-admin components,
//! including AdminSite, AdminDatabase, ModelAdminConfig, and server functions.
//!
//! ## Features
//!
//! These fixtures require the `admin` feature to be enabled in reinhardt-test.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use reinhardt_test::fixtures::admin_panel::admin_site;
//! use rstest::*;
//!
//! #[rstest]
//! #[tokio::test]
//! async fn test_admin_site(#[future] admin_site: Arc<AdminSite>) {
//!     let site = admin_site.await;
//!     assert!(site.registered_models().is_empty());
//! }
//! ```

// Only compile when admin feature is enabled
#[cfg(feature = "admin")]
use {
	reinhardt_admin_core::{
		AdminDatabase, AdminSite, ModelAdmin, ModelAdminConfig, ModelAdminConfigBuilder,
	},
	reinhardt_admin_types::errors::AdminError,
	reinhardt_db::{DatabaseConnection, Model},
	reinhardt_di::SingletonScope,
	rstest::*,
	std::sync::Arc,
	tokio::sync::OnceCell,
};

/// Fixture providing a basic AdminSite instance
///
/// This fixture creates a new AdminSite with default configuration.
/// The site starts with no registered models.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::admin_panel::admin_site;
/// use rstest::*;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_admin_site_registration(
///     #[future] admin_site: Arc<AdminSite>,
///     #[future] model_admin_config: ModelAdminConfig,
/// ) {
///     let site = admin_site.await;
///     let config = model_admin_config.await;
///
///     site.register("TestModel", config).unwrap();
///     assert_eq!(site.registered_models(), vec!["TestModel".to_string()]);
/// }
/// ```
#[cfg(feature = "admin")]
#[fixture]
pub async fn admin_site() -> Arc<AdminSite> {
	Arc::new(AdminSite::new("Test Admin Site"))
}

/// Fixture providing a ModelAdminConfig for testing
///
/// This fixture creates a ModelAdminConfig with typical test configuration:
/// - Model name: "TestModel"
/// - Table name: "test_models"
/// - Primary key field: "id"
/// - List display: ["id", "name", "created_at"]
/// - List filter: ["status"]
/// - Search fields: ["name", "description"]
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::admin_panel::model_admin_config;
/// use rstest::*;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_model_admin_config(#[future] model_admin_config: ModelAdminConfig) {
///     let config = model_admin_config.await;
///     assert_eq!(config.model_name(), "TestModel");
///     assert_eq!(config.table_name(), "test_models");
/// }
/// ```
#[cfg(feature = "admin")]
#[fixture]
pub async fn model_admin_config() -> ModelAdminConfig {
	ModelAdminConfig::builder()
		.model_name("TestModel")
		.table_name("test_models")
		.list_display(vec!["id", "name", "created_at"])
		.list_filter(vec!["status"])
		.search_fields(vec!["name", "description"])
		.build()
		.expect("Failed to build ModelAdminConfig")
}

/// Fixture providing an AdminDatabase connected to a test PostgreSQL instance
///
/// This fixture uses the shared PostgreSQL container pattern from reinhardt-test
/// to provide an isolated database for each test. The database connection is
/// obtained from the `shared_postgres` fixture.
///
/// # Dependencies
/// - Requires `testcontainers` feature for PostgreSQL container
/// - Requires `admin` feature for AdminDatabase
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::admin_panel::admin_database;
/// use rstest::*;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_admin_database_operations(
///     #[future] admin_database: Arc<AdminDatabase>,
/// ) {
///     let db = admin_database.await;
///     // Test database operations
/// }
/// ```
#[cfg(all(feature = "admin", feature = "testcontainers"))]
#[fixture]
pub async fn admin_database(
	#[future] shared_db_pool: (sqlx::PgPool, String),
) -> Arc<AdminDatabase> {
	use reinhardt_db::postgres::PgConnection;

	let (pool, _database_name) = shared_db_pool.await;
	let connection = DatabaseConnection::Postgres(PgConnection::new(pool));
	Arc::new(AdminDatabase::new(connection))
}

/// Fixture providing a test database with a pre-created table
///
/// This fixture creates a test table with a simple schema for testing
/// admin operations. The table has columns: id, name, status, created_at.
///
/// Returns a tuple of (PgPool, table_name) where table_name is the
/// created table's name.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::admin_panel::test_model_with_table;
/// use rstest::*;
/// use sqlx::PgPool;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_with_table(
///     #[future] test_model_with_table: (PgPool, String),
/// ) {
///     let (pool, table_name) = test_model_with_table.await;
///     // Use the pre-created table
/// }
/// ```
#[cfg(all(feature = "admin", feature = "testcontainers"))]
#[fixture]
pub async fn test_model_with_table(
	#[future] shared_db_pool: (sqlx::PgPool, String),
) -> (sqlx::PgPool, String) {
	use sqlx::{Executor, PgPool};

	let (pool, database_name) = shared_db_pool.await;
	let table_name = format!("test_models_{}", uuid::Uuid::new_v4().simple());

	// Create test table
	let create_table_sql = format!(
		"CREATE TABLE {} (
            id SERIAL PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            status VARCHAR(50) DEFAULT 'active',
            created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )",
		table_name
	);

	pool.execute(create_table_sql.as_str())
		.await
		.expect("Failed to create test table");

	// Insert some test data
	let insert_sql = format!(
		"INSERT INTO {} (name, status) VALUES
        ('Test Item 1', 'active'),
        ('Test Item 2', 'inactive'),
        ('Test Item 3', 'active')",
		table_name
	);

	pool.execute(insert_sql.as_str())
		.await
		.expect("Failed to insert test data");

	(pool, table_name)
}

/// Fixture providing a complete server function test context
///
/// This fixture provides both AdminSite and AdminDatabase configured
/// for testing server functions. The AdminSite has a registered model,
/// and the AdminDatabase is connected to a test database with data.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::admin_panel::server_fn_test_context;
/// use rstest::*;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_server_function_context(
///     #[future] server_fn_test_context: (Arc<AdminSite>, Arc<AdminDatabase>),
/// ) {
///     let (site, db) = server_fn_test_context.await;
///     // Test server functions with proper DI context
/// }
/// ```
#[cfg(all(feature = "admin", feature = "testcontainers"))]
#[fixture]
pub async fn server_fn_test_context(
	#[future] admin_site: Arc<AdminSite>,
	#[future] model_admin_config: ModelAdminConfig,
	#[future] admin_database: Arc<AdminDatabase>,
) -> (Arc<AdminSite>, Arc<AdminDatabase>) {
	let site = admin_site.await;
	let config = model_admin_config.await;
	let db = admin_database.await;

	// Register the model in the site
	site.register(config.model_name(), config)
		.expect("Failed to register model");

	(site, db)
}

/// Helper function to create DI container for admin tests
///
/// This function configures a SingletonScope with AdminSite and
/// AdminDatabase for dependency injection testing.
#[cfg(feature = "admin")]
pub fn configure_admin_di(
	singleton: &SingletonScope,
	site: Arc<AdminSite>,
	db: Arc<AdminDatabase>,
) {
	use reinhardt_di::{Injectable, Scope};

	singleton.register_singleton(site.clone() as Arc<dyn Injectable>);
	singleton.register_singleton(db.clone() as Arc<dyn Injectable>);
}

#[cfg(all(feature = "admin", test))]
mod tests {
	use super::*;
	use rstest::*;

	#[rstest]
	#[tokio::test]
	async fn test_admin_site_fixture(#[future] admin_site: Arc<AdminSite>) {
		let site = admin_site.await;
		assert_eq!(site.name(), "Test Admin Site");
		assert!(site.registered_models().is_empty());
	}

	#[rstest]
	#[tokio::test]
	async fn test_model_admin_config_fixture(#[future] model_admin_config: ModelAdminConfig) {
		let config = model_admin_config.await;
		assert_eq!(config.model_name(), "TestModel");
		assert_eq!(config.table_name(), "test_models");
		assert_eq!(config.list_display(), vec!["id", "name", "created_at"]);
	}

	// Note: Tests for database fixtures require testcontainers feature
	// and are typically run in integration tests rather than unit tests
}
