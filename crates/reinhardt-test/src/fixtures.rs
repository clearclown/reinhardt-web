use rstest::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, thiserror::Error)]
pub enum FixtureError {
	#[error("Fixture not found: {0}")]
	NotFound(String),
	#[error("Load error: {0}")]
	Load(String),
	#[error("Parse error: {0}")]
	Parse(String),
}

pub type FixtureResult<T> = Result<T, FixtureError>;

/// Fixture data loader
pub struct FixtureLoader {
	fixtures: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

impl FixtureLoader {
	/// Create a new fixture loader
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_test::fixtures::FixtureLoader;
	///
	/// let loader = FixtureLoader::new();
	// Loader is ready to load fixtures
	/// ```
	pub fn new() -> Self {
		Self {
			fixtures: Arc::new(RwLock::new(HashMap::new())),
		}
	}
	/// Load fixture from JSON string
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_test::fixtures::FixtureLoader;
	///
	/// # tokio_test::block_on(async {
	/// let loader = FixtureLoader::new();
	/// let json = r#"{"id": 1, "name": "Test"}"#;
	/// loader.load_from_json("test".to_string(), json).await.unwrap();
	/// assert!(loader.exists("test").await);
	/// # });
	/// ```
	pub async fn load_from_json(&self, name: String, json: &str) -> FixtureResult<()> {
		let value: serde_json::Value =
			serde_json::from_str(json).map_err(|e| FixtureError::Parse(e.to_string()))?;

		self.fixtures.write().await.insert(name, value);
		Ok(())
	}
	/// Load fixture data
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_test::fixtures::FixtureLoader;
	/// use serde::Deserialize;
	///
	/// #[derive(Deserialize)]
	/// struct User {
	///     id: i32,
	///     name: String,
	/// }
	///
	/// # tokio_test::block_on(async {
	/// let loader = FixtureLoader::new();
	/// let json = r#"{"id": 1, "name": "Alice"}"#;
	/// loader.load_from_json("user".to_string(), json).await.unwrap();
	/// let user: User = loader.load("user").await.unwrap();
	/// assert_eq!(user.id, 1);
	/// assert_eq!(user.name, "Alice");
	/// # });
	/// ```
	pub async fn load<T: for<'de> Deserialize<'de>>(&self, name: &str) -> FixtureResult<T> {
		let fixtures = self.fixtures.read().await;
		let value = fixtures
			.get(name)
			.ok_or_else(|| FixtureError::NotFound(name.to_string()))?;

		serde_json::from_value(value.clone()).map_err(|e| FixtureError::Parse(e.to_string()))
	}
	/// Get raw fixture value
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_test::fixtures::FixtureLoader;
	///
	/// # tokio_test::block_on(async {
	/// let loader = FixtureLoader::new();
	/// let json = r#"{"status": "active"}"#;
	/// loader.load_from_json("config".to_string(), json).await.unwrap();
	/// let value = loader.get("config").await.unwrap();
	/// assert!(value.is_object());
	/// # });
	/// ```
	pub async fn get(&self, name: &str) -> FixtureResult<serde_json::Value> {
		let fixtures = self.fixtures.read().await;
		fixtures
			.get(name)
			.cloned()
			.ok_or_else(|| FixtureError::NotFound(name.to_string()))
	}
	/// Check if fixture exists
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_test::fixtures::FixtureLoader;
	///
	/// # tokio_test::block_on(async {
	/// let loader = FixtureLoader::new();
	/// assert!(!loader.exists("missing").await);
	/// loader.load_from_json("test".to_string(), "{}").await.unwrap();
	/// assert!(loader.exists("test").await);
	/// # });
	/// ```
	pub async fn exists(&self, name: &str) -> bool {
		self.fixtures.read().await.contains_key(name)
	}
	/// Clear all fixtures
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_test::fixtures::FixtureLoader;
	///
	/// # tokio_test::block_on(async {
	/// let loader = FixtureLoader::new();
	/// loader.load_from_json("test".to_string(), "{}").await.unwrap();
	/// assert_eq!(loader.list().await.len(), 1);
	/// loader.clear().await;
	/// assert_eq!(loader.list().await.len(), 0);
	/// # });
	/// ```
	pub async fn clear(&self) {
		self.fixtures.write().await.clear();
	}
	/// List all fixture names
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_test::fixtures::FixtureLoader;
	///
	/// # tokio_test::block_on(async {
	/// let loader = FixtureLoader::new();
	/// loader.load_from_json("test1".to_string(), "{}").await.unwrap();
	/// loader.load_from_json("test2".to_string(), "{}").await.unwrap();
	/// let names = loader.list().await;
	/// assert_eq!(names.len(), 2);
	/// assert!(names.contains(&"test1".to_string()));
	/// # });
	/// ```
	pub async fn list(&self) -> Vec<String> {
		self.fixtures.read().await.keys().cloned().collect()
	}
}

impl Default for FixtureLoader {
	fn default() -> Self {
		Self::new()
	}
}

/// Factory trait for creating test data
pub trait Factory<T>: Send + Sync {
	fn build(&self) -> T;
	fn build_batch(&self, count: usize) -> Vec<T> {
		(0..count).map(|_| self.build()).collect()
	}
}

/// Simple factory builder
pub struct FactoryBuilder<T, F>
where
	F: Fn() -> T + Send + Sync,
{
	builder: F,
	_phantom: std::marker::PhantomData<T>,
}

/// Generate a random test key using UUID
///
/// # Examples
///
/// ```
/// use reinhardt_test::fixtures::random_test_key;
///
/// let key = random_test_key();
/// assert!(key.starts_with("test_key_"));
/// ```
pub fn random_test_key() -> String {
	use uuid::Uuid;
	format!("test_key_{}", Uuid::new_v4().simple())
}

/// Generate test configuration data with timestamp
///
/// # Examples
///
/// ```
/// use reinhardt_test::fixtures::test_config_value;
///
/// let value = test_config_value("my_value");
/// assert_eq!(value["value"], "my_value");
/// ```
pub fn test_config_value(value: &str) -> serde_json::Value {
	serde_json::json!({
		"value": value,
		"timestamp": chrono::Utc::now().to_rfc3339(),
	})
}

impl<T, F> FactoryBuilder<T, F>
where
	F: Fn() -> T + Send + Sync,
{
	/// Create a new factory builder
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_test::fixtures::{FactoryBuilder, Factory};
	///
	/// #[derive(Debug, PartialEq)]
	/// struct TestData { id: i32 }
	///
	/// let factory = FactoryBuilder::new(|| TestData { id: 42 });
	/// let item = factory.build();
	/// assert_eq!(item.id, 42);
	/// ```
	pub fn new(builder: F) -> Self {
		Self {
			builder,
			_phantom: std::marker::PhantomData,
		}
	}
}

impl<T, F> Factory<T> for FactoryBuilder<T, F>
where
	F: Fn() -> T + Send + Sync,
	T: Send + Sync,
{
	fn build(&self) -> T {
		(self.builder)()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use serde::Serialize;

	#[derive(Debug, Serialize, Deserialize, PartialEq)]
	struct TestData {
		id: i32,
		name: String,
	}

	#[tokio::test]
	async fn test_fixture_loader() {
		let loader = FixtureLoader::new();
		let json = r#"{"id": 1, "name": "Test"}"#;

		loader
			.load_from_json("test".to_string(), json)
			.await
			.unwrap();

		let data: TestData = loader.load("test").await.unwrap();
		assert_eq!(data.id, 1);
		assert_eq!(data.name, "Test");
	}

	#[tokio::test]
	async fn test_fixture_not_found() {
		let loader = FixtureLoader::new();
		let result: FixtureResult<TestData> = loader.load("missing").await;
		assert!(result.is_err());
	}

	#[test]
	fn test_factory_builder() {
		let factory = FactoryBuilder::new(|| TestData {
			id: 1,
			name: "Test".to_string(),
		});

		let data = factory.build();
		assert_eq!(data.id, 1);

		let batch = factory.build_batch(3);
		assert_eq!(batch.len(), 3);
	}
}

// ============================================================================
// rstest integration: Fixtures for common test resources
// ============================================================================

/// Fixture providing a FixtureLoader instance
///
/// Use this fixture in tests that need to load JSON fixture data.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::fixture_loader;
/// use rstest::*;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_with_fixtures(fixture_loader: reinhardt_test::fixtures::FixtureLoader) {
///     fixture_loader.load_from_json("test".to_string(), r#"{"id": 1}"#).await.unwrap();
///     // ...
/// }
/// ```
#[fixture]
pub fn fixture_loader() -> FixtureLoader {
	FixtureLoader::new()
}

/// Fixture providing an APIClient instance
///
/// Use this fixture in tests that need to make test HTTP requests.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::api_client;
/// use rstest::*;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_api_request(api_client: reinhardt_test::client::APIClient) {
///     // Make requests with client
/// }
/// ```
#[fixture]
pub fn api_client() -> crate::client::APIClient {
	crate::client::APIClient::new()
}

/// Fixture providing a temporary directory that is automatically cleaned up
///
/// # Examples
///
/// ```rust
/// use reinhardt_test::fixtures::temp_dir;
/// use rstest::*;
///
/// #[rstest]
/// fn test_with_temp_dir(temp_dir: tempfile::TempDir) {
///     let path = temp_dir.path();
///     std::fs::write(path.join("test.txt"), "data").unwrap();
///     // temp_dir is automatically cleaned up when test ends
/// }
/// ```
#[fixture]
pub fn temp_dir() -> tempfile::TempDir {
	tempfile::tempdir().expect("Failed to create temporary directory")
}

// ============================================================================
// TestContainers fixtures (optional, requires "testcontainers" feature)
// ============================================================================

#[cfg(feature = "testcontainers")]
use testcontainers::{ContainerAsync, runners::AsyncRunner};
#[cfg(feature = "testcontainers")]
use testcontainers_modules::{postgres::Postgres, redis::Redis};

/// Fixture providing a PostgreSQL TestContainer
///
/// Returns a tuple of (container, connection_url).
/// The container is automatically cleaned up when the test ends.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::postgres_container;
/// use rstest::*;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_with_postgres(#[future] postgres_container: (ContainerAsync<Postgres>, String)) {
///     let (_container, url) = postgres_container.await;
///     // Use PostgreSQL database at `url`
/// }
/// ```
#[cfg(feature = "testcontainers")]
#[fixture]
pub async fn postgres_container() -> (ContainerAsync<Postgres>, String) {
	let container = Postgres::default()
		.start()
		.await
		.expect("Failed to start PostgreSQL container");

	let port = container
		.get_host_port_ipv4(5432)
		.await
		.expect("Failed to get PostgreSQL port");

	let url = format!("postgres://postgres:postgres@localhost:{}/postgres", port);

	(container, url)
}

/// Fixture providing a Redis TestContainer
///
/// Returns a tuple of (container, connection_url).
/// The container is automatically cleaned up when the test ends.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::redis_container;
/// use rstest::*;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_with_redis(#[future] redis_container: (ContainerAsync<Redis>, String)) {
///     let (_container, url) = redis_container.await;
///     // Use Redis at `url`
/// }
/// ```
#[cfg(feature = "testcontainers")]
#[fixture]
pub async fn redis_container() -> (ContainerAsync<Redis>, String) {
	let container = Redis::default()
		.start()
		.await
		.expect("Failed to start Redis container");

	let port = container
		.get_host_port_ipv4(6379)
		.await
		.expect("Failed to get Redis port");

	let url = format!("redis://localhost:{}", port);

	(container, url)
}

/// Fixture providing a Redis Cluster TestContainer setup
///
/// Returns a tuple of (container, connection_urls) for the cluster nodes.
/// The container is automatically cleaned up when the test ends.
///
/// # Notes
///
/// This fixture uses the `grokzen/redis-cluster` Docker image which provides
/// a pre-configured Redis Cluster with 6 nodes (3 masters + 3 replicas).
/// Ports 7000-7005 are exposed for the 6 nodes.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::redis_cluster_fixture;
/// use rstest::*;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_with_redis_cluster(
///     #[future] redis_cluster_fixture: (testcontainers::ContainerAsync<testcontainers::GenericImage>, Vec<String>)
/// ) {
///     let (_container, urls) = redis_cluster_fixture.await;
///     // Use Redis Cluster at `urls`
/// }
/// ```
#[cfg(feature = "testcontainers")]
#[fixture]
pub async fn redis_cluster_fixture() -> (
	testcontainers::ContainerAsync<testcontainers::GenericImage>,
	Vec<String>,
) {
	use testcontainers::GenericImage;
	use testcontainers::core::WaitFor;

	// Start Redis Cluster using grokzen/redis-cluster image
	// This image provides a pre-configured cluster with 6 nodes (3 masters + 3 replicas)
	let cluster = GenericImage::new("grokzen/redis-cluster", "latest")
		.with_wait_for(WaitFor::message_on_stdout("Cluster state changed: ok"))
		.start()
		.await
		.expect("Failed to start Redis Cluster container");

	// Get host ports for the 6 Redis nodes (internal ports 7000-7005)
	let mut urls = Vec::new();
	for internal_port in 7000..=7005 {
		let host_port = cluster
			.get_host_port_ipv4(internal_port)
			.await
			.unwrap_or_else(|_| panic!("Failed to get host port for Redis node {}", internal_port));
		urls.push(format!("redis://localhost:{}", host_port));
	}

	eprintln!("Redis Cluster nodes ready:");
	for (i, url) in urls.iter().enumerate() {
		eprintln!("  Node {}: {}", i, url);
	}

	// Return container and URLs
	// Container will be automatically cleaned up when it goes out of scope
	(cluster, urls)
}

/// LocalStack container fixture for AWS services testing
///
/// This fixture provides a LocalStack container that emulates AWS services locally.
/// Useful for testing AWS integrations without actual AWS credentials.
///
/// # Examples
///
/// ```no_run
/// use rstest::*;
/// use reinhardt_test::fixtures::localstack_fixture;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_with_localstack(
///     #[future] localstack_fixture: (ContainerAsync<GenericImage>, String)
/// ) {
///     let (_container, endpoint_url) = localstack_fixture.await;
///     // Use endpoint_url to configure AWS SDK
/// }
/// ```
#[cfg(feature = "testcontainers")]
#[fixture]
pub async fn localstack_fixture() -> (
	testcontainers::ContainerAsync<testcontainers::GenericImage>,
	String,
) {
	use std::time::Duration;
	use testcontainers::{GenericImage, ImageExt, runners::AsyncRunner};

	// LocalStack community image - minimal configuration for faster startup
	// No wait condition - rely on port mapping and sleep instead
	let localstack = GenericImage::new("localstack/localstack", "latest")
		.with_env_var("SERVICES", "secretsmanager") // Only enable Secrets Manager service
		.with_env_var("EDGE_PORT", "4566") // Default LocalStack edge port
		.start()
		.await
		.expect("Failed to start LocalStack container");

	// Get the mapped port for LocalStack edge port (4566)
	let port = localstack
		.get_host_port_ipv4(4566)
		.await
		.expect("Failed to get LocalStack port");

	// Construct endpoint URL
	let endpoint_url = format!("http://localhost:{}", port);

	eprintln!("LocalStack started at: {}", endpoint_url);

	// Wait for LocalStack to fully initialize (no log watching, just sleep)
	tokio::time::sleep(Duration::from_secs(5)).await;

	(localstack, endpoint_url)
}

// ============================================================================
// Advanced Setup/Teardown Fixtures using resource.rs
// ============================================================================

#[cfg(feature = "testcontainers")]
pub use suite_resources::*;

/// Suite-wide shared resources using `resource.rs` SuiteResource pattern
#[cfg(feature = "testcontainers")]
mod suite_resources {
	use super::*;
	use crate::resource::{SuiteGuard, SuiteResource, acquire_suite};
	use std::sync::{Mutex, OnceLock, Weak};

	#[cfg(feature = "testcontainers")]
	use testcontainers::core::{ContainerPort, WaitFor};

	/// Suite-wide PostgreSQL container resource
	///
	/// This resource is shared across all tests in the suite and automatically
	/// cleaned up when the last test completes. Uses `SuiteResource` pattern
	/// from `resource.rs` for safe lifecycle management.
	///
	/// ## Example
	///
	/// ```rust
	/// use reinhardt_test::fixtures::*;
	/// use rstest::*;
	///
	/// #[rstest]
	/// #[tokio::test]
	/// async fn test_database_query(postgres_suite: SuiteGuard<PostgresSuiteResource>) {
	///     let pool = &postgres_suite.pool;
	///     let result = sqlx::query("SELECT 1").fetch_one(pool).await;
	///     assert!(result.is_ok());
	/// }
	/// ```
	pub struct PostgresSuiteResource {
		#[allow(dead_code)]
		pub container: testcontainers::ContainerAsync<testcontainers::GenericImage>,
		pub pool: sqlx::postgres::PgPool,
		pub port: u16,
		pub database_url: String,
	}

	impl SuiteResource for PostgresSuiteResource {
		fn init() -> Self {
			// Block on async initialization (SuiteResource::init is sync)
			tokio::task::block_in_place(|| {
				tokio::runtime::Handle::current().block_on(async { Self::init_async().await })
			})
		}
	}

	impl PostgresSuiteResource {
		async fn init_async() -> Self {
			use testcontainers::{GenericImage, ImageExt, runners::AsyncRunner};

			let postgres = GenericImage::new("postgres", "17-alpine")
				.with_wait_for(WaitFor::message_on_stderr(
					"database system is ready to accept connections",
				))
				.with_exposed_port(ContainerPort::Tcp(5432))
				.with_env_var("POSTGRES_HOST_AUTH_METHOD", "trust")
				.start()
				.await
				.expect("Failed to start PostgreSQL container");

			let port = postgres
				.get_host_port_ipv4(ContainerPort::Tcp(5432))
				.await
				.expect("Failed to get PostgreSQL port");

			let database_url = format!("postgres://postgres@localhost:{}/postgres", port);

			// Retry connection with exponential backoff
			let pool = retry_connect_postgres(&database_url, 10).await;

			Self {
				container: postgres,
				pool,
				port,
				database_url,
			}
		}
	}

	async fn retry_connect_postgres(url: &str, max_retries: u32) -> sqlx::postgres::PgPool {
		use sqlx::postgres::PgPoolOptions;
		use std::time::Duration;

		for attempt in 0..max_retries {
			match PgPoolOptions::new()
				.max_connections(5)
				.acquire_timeout(Duration::from_secs(3))
				.connect(url)
				.await
			{
				Ok(pool) => return pool,
				Err(e) if attempt < max_retries - 1 => {
					eprintln!(
						"Connection attempt {} failed: {}. Retrying...",
						attempt + 1,
						e
					);
					tokio::time::sleep(Duration::from_millis(100 * (attempt as u64 + 1))).await;
				}
				Err(e) => panic!(
					"Failed to connect to PostgreSQL after {} retries: {}",
					max_retries, e
				),
			}
		}
		unreachable!()
	}

	static POSTGRES_SUITE: OnceLock<Mutex<Weak<PostgresSuiteResource>>> = OnceLock::new();

	/// Acquire shared PostgreSQL suite resource
	///
	/// This fixture provides a suite-wide PostgreSQL container that is shared
	/// across all tests and automatically cleaned up when the last test completes.
	///
	/// ## Example
	///
	/// ```rust
	/// use reinhardt_test::fixtures::*;
	/// use rstest::*;
	///
	/// #[rstest]
	/// #[tokio::test]
	/// async fn test_example(postgres_suite: SuiteGuard<PostgresSuiteResource>) {
	///     let pool = &postgres_suite.pool;
	///     // Use pool in test
	/// }
	/// ```
	#[fixture]
	pub fn postgres_suite() -> SuiteGuard<PostgresSuiteResource> {
		acquire_suite(&POSTGRES_SUITE)
	}

	/// Suite-wide MySQL container resource
	pub struct MySqlSuiteResource {
		#[allow(dead_code)]
		pub container: testcontainers::ContainerAsync<testcontainers::GenericImage>,
		pub pool: sqlx::mysql::MySqlPool,
		pub port: u16,
		pub database_url: String,
	}

	impl SuiteResource for MySqlSuiteResource {
		fn init() -> Self {
			tokio::task::block_in_place(|| {
				tokio::runtime::Handle::current().block_on(async { Self::init_async().await })
			})
		}
	}

	impl MySqlSuiteResource {
		async fn init_async() -> Self {
			use testcontainers::{GenericImage, ImageExt, runners::AsyncRunner};

			let mysql = GenericImage::new("mysql", "8.0")
				.with_wait_for(WaitFor::message_on_stderr("ready for connections"))
				.with_exposed_port(ContainerPort::Tcp(3306))
				.with_env_var("MYSQL_ROOT_PASSWORD", "test")
				.with_env_var("MYSQL_DATABASE", "test")
				.start()
				.await
				.expect("Failed to start MySQL container");

			let port = mysql
				.get_host_port_ipv4(ContainerPort::Tcp(3306))
				.await
				.expect("Failed to get MySQL port");

			let database_url = format!("mysql://root:test@localhost:{}/test", port);

			let pool = retry_connect_mysql(&database_url, 10).await;

			Self {
				container: mysql,
				pool,
				port,
				database_url,
			}
		}
	}

	async fn retry_connect_mysql(url: &str, max_retries: u32) -> sqlx::mysql::MySqlPool {
		use sqlx::mysql::MySqlPoolOptions;
		use std::time::Duration;

		for attempt in 0..max_retries {
			match MySqlPoolOptions::new()
				.max_connections(5)
				.acquire_timeout(Duration::from_secs(3))
				.connect(url)
				.await
			{
				Ok(pool) => return pool,
				Err(e) if attempt < max_retries - 1 => {
					eprintln!(
						"Connection attempt {} failed: {}. Retrying...",
						attempt + 1,
						e
					);
					tokio::time::sleep(Duration::from_millis(100 * (attempt as u64 + 1))).await;
				}
				Err(e) => panic!(
					"Failed to connect to MySQL after {} retries: {}",
					max_retries, e
				),
			}
		}
		unreachable!()
	}

	static MYSQL_SUITE: OnceLock<Mutex<Weak<MySqlSuiteResource>>> = OnceLock::new();

	/// Acquire shared MySQL suite resource
	#[fixture]
	pub fn mysql_suite() -> SuiteGuard<MySqlSuiteResource> {
		acquire_suite(&MYSQL_SUITE)
	}
}

// ============================================================================
// Per-test Resources using TestResource pattern
// ============================================================================

pub use test_resources::*;

/// Per-test resources using `resource.rs` TestResource pattern
mod test_resources {
	use super::*;
	use crate::resource::{TeardownGuard, TestResource};
	use std::path::PathBuf;

	/// Per-test template directory resource with automatic cleanup
	///
	/// Creates a temporary directory for template files and automatically
	/// removes it when the test completes.
	///
	/// ## Example
	///
	/// ```rust
	/// use reinhardt_test::fixtures::*;
	/// use rstest::*;
	///
	/// #[rstest]
	/// fn test_template_rendering(template_dir: TeardownGuard<TemplateDirResource>) {
	///     let dir = template_dir.path();
	///     // Write template files to dir
	///     // Directory is automatically cleaned up
	/// }
	/// ```
	pub struct TemplateDirResource {
		path: PathBuf,
	}

	impl TemplateDirResource {
		pub fn path(&self) -> &PathBuf {
			&self.path
		}
	}

	impl TestResource for TemplateDirResource {
		fn setup() -> Self {
			let test_id = uuid::Uuid::new_v4();
			let path = PathBuf::from(format!("/tmp/reinhardt_template_test_{}", test_id));
			std::fs::create_dir_all(&path).expect("Failed to create template test directory");
			Self { path }
		}

		fn teardown(&mut self) {
			if self.path.exists() {
				std::fs::remove_dir_all(&self.path)
					.unwrap_or_else(|e| eprintln!("Failed to cleanup template directory: {}", e));
			}
		}
	}

	/// Per-test template directory fixture
	#[fixture]
	pub fn template_dir() -> TeardownGuard<TemplateDirResource> {
		TeardownGuard::new()
	}
}

// ================================================================================
// Mock Database Connection Fixtures
// ================================================================================

#[cfg(feature = "testcontainers")]
pub use mock_database::*;

#[cfg(feature = "testcontainers")]
mod mock_database {
	use reinhardt_db::backends::Result;
	use reinhardt_db::backends::backend::DatabaseBackend as BackendTrait;
	use reinhardt_db::backends::connection::DatabaseConnection as BackendsConnection;
	use reinhardt_db::backends::types::{DatabaseType, QueryResult, QueryValue, Row};
	use reinhardt_orm::{DatabaseBackend, DatabaseConnection};
	use rstest::*;
	use std::sync::Arc;

	/// Mock backend implementation for database testing
	///
	/// This mock backend provides a no-op implementation of all database operations,
	/// useful for testing code that depends on DatabaseConnection without requiring
	/// an actual database.
	struct MockBackend;

	#[async_trait::async_trait]
	impl BackendTrait for MockBackend {
		fn database_type(&self) -> DatabaseType {
			DatabaseType::Postgres
		}

		fn placeholder(&self, index: usize) -> String {
			format!("${}", index)
		}

		fn supports_returning(&self) -> bool {
			true
		}

		fn supports_on_conflict(&self) -> bool {
			true
		}

		async fn execute(&self, _sql: &str, _params: Vec<QueryValue>) -> Result<QueryResult> {
			Ok(QueryResult { rows_affected: 0 })
		}

		async fn fetch_one(&self, _sql: &str, _params: Vec<QueryValue>) -> Result<Row> {
			Ok(Row::new())
		}

		async fn fetch_all(&self, _sql: &str, _params: Vec<QueryValue>) -> Result<Vec<Row>> {
			Ok(Vec::new())
		}

		async fn fetch_optional(
			&self,
			_sql: &str,
			_params: Vec<QueryValue>,
		) -> Result<Option<Row>> {
			Ok(None)
		}

		fn as_any(&self) -> &dyn std::any::Any {
			self
		}
	}

	/// Fixture for creating a mock database connection
	///
	/// Returns a DatabaseConnection with a mock backend that doesn't perform
	/// actual database operations. Useful for testing code that requires a
	/// connection but doesn't need real data.
	///
	/// # Example
	///
	/// ```rust
	/// use reinhardt_test::fixtures::mock_connection;
	/// use rstest::*;
	///
	/// #[rstest]
	/// fn test_with_mock_db(mock_connection: DatabaseConnection) {
	///     // Use mock_connection for testing
	/// }
	/// ```
	#[fixture]
	pub fn mock_connection() -> DatabaseConnection {
		let mock_backend = Arc::new(MockBackend);
		let backends_conn = BackendsConnection::new(mock_backend);
		DatabaseConnection::new(DatabaseBackend::Postgres, backends_conn)
	}
}
