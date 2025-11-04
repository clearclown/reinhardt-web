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
use testcontainers::{ContainerAsync, GenericImage, ImageExt, runners::AsyncRunner};
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
/// Returns a tuple of (containers, connection_urls) for the 3 cluster nodes.
/// The containers are automatically cleaned up when the test ends.
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
///     #[future] redis_cluster_fixture: (Vec<testcontainers::ContainerAsync<testcontainers::GenericImage>>, Vec<String>)
/// ) {
///     let (_containers, urls) = redis_cluster_fixture.await;
///     // Use Redis Cluster at `urls`
/// }
/// ```
#[cfg(feature = "testcontainers")]
#[fixture]
pub async fn redis_cluster_fixture() -> (
	Vec<testcontainers::ContainerAsync<testcontainers::GenericImage>>,
	Vec<String>,
) {
	use testcontainers::GenericImage;
	use testcontainers::core::{ExecCommand, WaitFor};

	// Start 3 Redis nodes with cluster mode enabled
	let node1 = GenericImage::new("redis", "7-alpine")
		.with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
		.with_cmd(vec![
			"redis-server",
			"--cluster-enabled",
			"yes",
			"--cluster-config-file",
			"nodes.conf",
			"--cluster-node-timeout",
			"5000",
			"--appendonly",
			"no",
		])
		.start()
		.await
		.expect("Failed to start Redis node 1");

	let node2 = GenericImage::new("redis", "7-alpine")
		.with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
		.with_cmd(vec![
			"redis-server",
			"--cluster-enabled",
			"yes",
			"--cluster-config-file",
			"nodes.conf",
			"--cluster-node-timeout",
			"5000",
			"--appendonly",
			"no",
		])
		.start()
		.await
		.expect("Failed to start Redis node 2");

	let node3 = GenericImage::new("redis", "7-alpine")
		.with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
		.with_cmd(vec![
			"redis-server",
			"--cluster-enabled",
			"yes",
			"--cluster-config-file",
			"nodes.conf",
			"--cluster-node-timeout",
			"5000",
			"--appendonly",
			"no",
		])
		.start()
		.await
		.expect("Failed to start Redis node 3");

	let port1 = node1
		.get_host_port_ipv4(6379)
		.await
		.expect("Failed to get Redis node 1 port");
	let port2 = node2
		.get_host_port_ipv4(6379)
		.await
		.expect("Failed to get Redis node 2 port");
	let port3 = node3
		.get_host_port_ipv4(6379)
		.await
		.expect("Failed to get Redis node 3 port");

	let url1 = format!("redis://localhost:{}", port1);
	let url2 = format!("redis://localhost:{}", port2);
	let url3 = format!("redis://localhost:{}", port3);

	// Initialize cluster by creating cluster meet connections
	let meet_cmd2 = format!("redis-cli -p 6379 CLUSTER MEET 127.0.0.1 {}", port2);
	let meet_cmd3 = format!("redis-cli -p 6379 CLUSTER MEET 127.0.0.1 {}", port3);

	node1
		.exec(ExecCommand::new(vec!["sh", "-c", &meet_cmd2]))
		.await
		.ok();
	tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

	node1
		.exec(ExecCommand::new(vec!["sh", "-c", &meet_cmd3]))
		.await
		.ok();
	tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

	// Assign slots to each node
	// Node 1: slots 0-5460
	let slots1_cmd = "redis-cli -p 6379 CLUSTER ADDSLOTS $(seq 0 5460)";
	node1
		.exec(ExecCommand::new(vec!["sh", "-c", slots1_cmd]))
		.await
		.ok();

	// Node 2: slots 5461-10922
	let slots2_cmd = "redis-cli -p 6379 CLUSTER ADDSLOTS $(seq 5461 10922)";
	node2
		.exec(ExecCommand::new(vec!["sh", "-c", slots2_cmd]))
		.await
		.ok();

	// Node 3: slots 10923-16383
	let slots3_cmd = "redis-cli -p 6379 CLUSTER ADDSLOTS $(seq 10923 16383)";
	node3
		.exec(ExecCommand::new(vec!["sh", "-c", slots3_cmd]))
		.await
		.ok();

	// Wait for cluster to be ready
	tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

	// Return containers and URLs
	// Containers will be automatically cleaned up when they go out of scope
	(vec![node1, node2, node3], vec![url1, url2, url3])
}

/// Fixture providing a Memcached TestContainer
///
/// Returns a tuple of (container, connection_url).
/// The container is automatically cleaned up when the test ends.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::memcached_container;
/// use rstest::*;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_with_memcached(#[future] memcached_container: (ContainerAsync<GenericImage>, String)) {
///     let (_container, url) = memcached_container.await;
///     // Use Memcached at `url`
/// }
/// ```
#[cfg(feature = "testcontainers")]
#[fixture]
pub async fn memcached_container() -> (ContainerAsync<GenericImage>, String) {
	let container = GenericImage::new("memcached", "1.6-alpine")
		.with_exposed_port(11211.into())
		.start()
		.await
		.expect("Failed to start Memcached container");

	let port = container
		.get_host_port_ipv4(11211)
		.await
		.expect("Failed to get Memcached port");

	let url = format!("localhost:{}", port);

	(container, url)
}

/// Fixture providing an SQLite in-memory database pool
///
/// Returns a connection pool that can be used for testing.
/// The database is automatically cleaned up when the pool is dropped.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::sqlite_pool;
/// use rstest::*;
/// use sqlx::SqlitePool;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_with_sqlite(#[future] sqlite_pool: SqlitePool) {
///     let pool = sqlite_pool.await;
///     // Use SQLite pool
/// }
/// ```
#[cfg(feature = "testcontainers")]
#[fixture]
pub async fn sqlite_pool() -> sqlx::SqlitePool {
	use sqlx::sqlite::SqlitePoolOptions;

	SqlitePoolOptions::new()
		.max_connections(5)
		.connect(":memory:")
		.await
		.expect("Failed to create SQLite pool")
}

/// Fixture providing a PostgreSQL connection pool (with TestContainer)
///
/// This combines the postgres_container fixture with a connection pool.
/// Both the pool and the container are automatically cleaned up.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::postgres_pool;
/// use rstest::*;
/// use sqlx::PgPool;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_with_postgres_pool(
///     #[future] postgres_pool: (ContainerAsync<testcontainers_modules::postgres::Postgres>, PgPool)
/// ) {
///     let (_container, pool) = postgres_pool.await;
///     // Use PostgreSQL pool
/// }
/// ```
#[cfg(feature = "testcontainers")]
#[fixture]
pub async fn postgres_pool() -> (ContainerAsync<Postgres>, sqlx::PgPool) {
	use sqlx::postgres::PgPoolOptions;

	let (container, url) = postgres_container().await;

	let pool = PgPoolOptions::new()
		.max_connections(5)
		.connect(&url)
		.await
		.expect("Failed to connect to PostgreSQL");

	(container, pool)
}

// ============================================================================
// WebSocket fixtures
// ============================================================================

/// Fixture providing a WebSocket RoomManager for testing
///
/// Returns an Arc-wrapped RoomManager that can be used across test functions.
/// The manager is automatically cleaned up when dropped.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::websocket_manager;
/// use rstest::*;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_with_websocket(websocket_manager: std::sync::Arc<reinhardt_websockets::RoomManager>) {
///     // Create rooms as needed
///     websocket_manager.create_room("test_room".to_string()).await;
///     // Use RoomManager for testing
/// }
/// ```
#[cfg(feature = "websockets")]
#[fixture]
pub fn websocket_manager() -> std::sync::Arc<reinhardt_websockets::RoomManager> {
	std::sync::Arc::new(reinhardt_websockets::RoomManager::new())
}

// ============================================================================
// テスト用モデル構造体
// ============================================================================
//
// これらは`reinhardt-associations`のテストで使用される汎用モデルです。
// テストユーティリティとして意図的に定義されているため、
// 一部フィールドが未使用でも許容されます。

/// 注文モデル（collection.rsテスト用）
#[allow(dead_code)]
#[derive(Clone)]
pub struct Order {
	pub id: u32,
	pub product_name: String,
}

/// ユーザーモデル（複数テスト用）
#[allow(dead_code)]
#[derive(Clone)]
pub struct User {
	pub id: i64,
	pub name: String,
}

/// ユーザーモデル（collectionテスト用、Orderリレーション付き）
#[allow(dead_code)]
#[derive(Clone)]
pub struct UserWithOrders {
	pub id: u32,
	pub orders: Vec<Order>,
}

/// 投稿モデル（foreign_key/loadingテスト用）
#[allow(dead_code)]
#[derive(Clone)]
pub struct Post {
	pub id: i64,
	pub title: String,
	pub author_id: i64,
}

/// 学生モデル（many_to_manyテスト用）
#[allow(dead_code)]
#[derive(Clone)]
pub struct Student {
	pub id: i64,
	pub name: String,
}

/// コースモデル（many_to_manyテスト用）
#[allow(dead_code)]
#[derive(Clone)]
pub struct Course {
	pub id: i64,
	pub name: String,
}

/// ユーザープロフィールモデル（one_to_oneテスト用）
#[allow(dead_code)]
#[derive(Clone)]
pub struct UserProfile {
	pub id: i64,
	pub user_id: i64,
	pub bio: String,
}

/// 住所モデル（proxyテスト用）
#[allow(dead_code)]
#[derive(Clone)]
pub struct Address {
	pub city: String,
	pub country: String,
}

/// ユーザーモデル（proxyテスト用、Address付き）
#[allow(dead_code)]
#[derive(Clone)]
pub struct UserWithAddress {
	pub id: u32,
	pub address: Address,
}
