//! CockroachDB integration tests using TestContainers
//!
//! These tests require Docker to be running.
//!
//! Run these tests with:
//! ```bash
//! cargo test --test cockroachdb_integration_tests --features cockroachdb-backend -- --test-threads=1
//! ```

#[cfg(feature = "cockroachdb-backend")]
mod cockroachdb_tests {
	use reinhardt_db::backends::cockroachdb::{
		CockroachDBBackend, CockroachDBSchemaEditor, CockroachDBTransactionManager,
	};
	use reinhardt_db::backends::postgresql::schema::PostgreSQLSchemaEditor;
	use reinhardt_db::schema::BaseDatabaseSchemaEditor;
	use serial_test::serial;
	use sqlx::{PgPool, Row};
	use std::sync::Arc;
	use testcontainers::{
		ContainerAsync, core::WaitFor, images::generic::GenericImage, runners::AsyncRunner,
	};
	use tokio::sync::OnceCell;

	// Global container that will be reused across tests
	static CONTAINER: OnceCell<Arc<CockroachDBContainer>> = OnceCell::const_new();

	struct CockroachDBContainer {
		_container: ContainerAsync<GenericImage>,
		connection_string: String,
	}

	async fn get_cockroachdb_container() -> Arc<CockroachDBContainer> {
		CONTAINER
			.get_or_init(|| async {
				// CockroachDB single-node mode for testing
				let image = GenericImage::new("cockroachdb/cockroach", "v23.1.0")
					.with_wait_for(WaitFor::message_on_stdout("node starting"))
					.with_cmd(vec![
						"start-single-node".to_string(),
						"--insecure".to_string(),
						"--store=type=mem,size=1GiB".to_string(),
					])
					.with_exposed_port(26257.into());

				let container = image.start().await.expect("Failed to start CockroachDB");
				let port = container
					.get_host_port_ipv4(26257)
					.await
					.expect("Failed to get port");

				let connection_string = format!("postgresql://root@127.0.0.1:{}/defaultdb", port);

				// Wait for CockroachDB to be ready
				tokio::time::sleep(std::time::Duration::from_secs(5)).await;

				Arc::new(CockroachDBContainer {
					_container: container,
					connection_string,
				})
			})
			.await
			.clone()
	}

	async fn create_test_pool() -> PgPool {
		let container = get_cockroachdb_container().await;
		PgPool::connect(&container.connection_string)
			.await
			.expect("Failed to connect to CockroachDB")
	}

	async fn cleanup_test_tables(pool: &PgPool) {
		let _ = sqlx::query("DROP TABLE IF EXISTS test_users CASCADE")
			.execute(pool)
			.await;
		let _ = sqlx::query("DROP TABLE IF EXISTS test_events CASCADE")
			.execute(pool)
			.await;
	}

	// Basic Backend Tests

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_backend_creation() {
		let pg_editor = PostgreSQLSchemaEditor::new();
		let backend = CockroachDBBackend::new(pg_editor);

		assert_eq!(backend.database_name(), "cockroachdb");
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_supported_features() {
		let pg_editor = PostgreSQLSchemaEditor::new();
		let backend = CockroachDBBackend::new(pg_editor);

		assert!(backend.supports_feature("multi_region"));
		assert!(backend.supports_feature("distributed_transactions"));
		assert!(backend.supports_feature("as_of_system_time"));
		assert!(backend.supports_feature("regional_by_row"));
		assert!(backend.supports_feature("regional_by_table"));
		assert!(backend.supports_feature("global_tables"));
		assert!(!backend.supports_feature("unknown_feature"));

		let features = backend.supported_features();
		assert!(features.contains(&"multi_region"));
		assert!(features.contains(&"distributed_transactions"));
	}

	// Schema Editor Tests

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_schema_editor_locality_sql() {
		let pg_editor = PostgreSQLSchemaEditor::new();
		let backend = CockroachDBBackend::new(pg_editor);
		let editor = backend.schema_editor();

		let sql = editor.create_table_with_locality_sql(
			"users",
			&[("id", "UUID PRIMARY KEY"), ("name", "VARCHAR(100)")],
			"REGIONAL BY ROW",
		);

		assert!(sql.contains("CREATE TABLE"));
		assert!(sql.contains("\"users\""));
		assert!(sql.contains("LOCALITY REGIONAL BY ROW"));
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_schema_editor_alter_locality_sql() {
		let pg_editor = PostgreSQLSchemaEditor::new();
		let backend = CockroachDBBackend::new(pg_editor);
		let editor = backend.schema_editor();

		let sql = editor.alter_table_locality_sql("users", "REGIONAL BY TABLE");

		assert!(sql.contains("ALTER TABLE"));
		assert!(sql.contains("\"users\""));
		assert!(sql.contains("SET LOCALITY REGIONAL BY TABLE"));
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_schema_editor_partitioned_table_sql() {
		let pg_editor = PostgreSQLSchemaEditor::new();
		let backend = CockroachDBBackend::new(pg_editor);
		let editor = backend.schema_editor();

		let sql = editor.create_partitioned_table_sql(
			"events",
			&[
				("id", "UUID PRIMARY KEY"),
				("region", "VARCHAR(50)"),
				("data", "JSONB"),
			],
			"region",
			&[
				("us_east", "'us-east-1', 'us-east-2'"),
				("us_west", "'us-west-1', 'us-west-2'"),
			],
		);

		assert!(sql.contains("CREATE TABLE"));
		assert!(sql.contains("PARTITION BY LIST"));
		assert!(sql.contains("\"region\""));
		assert!(sql.contains("PARTITION us_east VALUES IN"));
		assert!(sql.contains("PARTITION us_west VALUES IN"));
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_schema_editor_index_with_storing_sql() {
		let pg_editor = PostgreSQLSchemaEditor::new();
		let backend = CockroachDBBackend::new(pg_editor);
		let editor = backend.schema_editor();

		let sql = editor.create_index_with_storing_sql(
			"idx_email",
			"users",
			&["email"],
			&["name", "created_at"],
			false,
			None,
		);

		assert!(sql.contains("CREATE INDEX"));
		assert!(sql.contains("\"idx_email\""));
		assert!(sql.contains("ON \"users\""));
		assert!(sql.contains("\"email\""));
		assert!(sql.contains("STORING"));
		assert!(sql.contains("\"name\""));
		assert!(sql.contains("\"created_at\""));
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_schema_editor_unique_index_with_condition() {
		let pg_editor = PostgreSQLSchemaEditor::new();
		let backend = CockroachDBBackend::new(pg_editor);
		let editor = backend.schema_editor();

		let sql = editor.create_index_with_storing_sql(
			"idx_active_users",
			"users",
			&["email"],
			&["name"],
			true,
			Some("active = true"),
		);

		assert!(sql.contains("CREATE UNIQUE INDEX"));
		assert!(sql.contains("WHERE active = true"));
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_schema_editor_as_of_system_time_sql() {
		let pg_editor = PostgreSQLSchemaEditor::new();
		let backend = CockroachDBBackend::new(pg_editor);
		let editor = backend.schema_editor();

		let sql = editor.as_of_system_time_sql("SELECT * FROM users WHERE id = $1", "-5s");

		assert!(sql.contains("SELECT * FROM users WHERE id = $1"));
		assert!(sql.contains("AS OF SYSTEM TIME -5s"));
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_schema_editor_show_regions_sql() {
		let pg_editor = PostgreSQLSchemaEditor::new();
		let backend = CockroachDBBackend::new(pg_editor);
		let editor = backend.schema_editor();

		let sql = editor.show_regions_sql();
		assert_eq!(sql, "SHOW REGIONS");
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_schema_editor_survival_goal_sql() {
		let pg_editor = PostgreSQLSchemaEditor::new();
		let backend = CockroachDBBackend::new(pg_editor);
		let editor = backend.schema_editor();

		let sql = editor.show_survival_goal_sql();
		assert_eq!(sql, "SHOW SURVIVAL GOAL");
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_schema_editor_set_primary_region_sql() {
		let pg_editor = PostgreSQLSchemaEditor::new();
		let backend = CockroachDBBackend::new(pg_editor);
		let editor = backend.schema_editor();

		let sql = editor.set_primary_region_sql("mydb", "us-east-1");

		assert!(sql.contains("ALTER DATABASE"));
		assert!(sql.contains("\"mydb\""));
		assert!(sql.contains("SET PRIMARY REGION"));
		assert!(sql.contains("\"us-east-1\""));
	}

	// Real Database Integration Tests

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_create_table_with_pool() {
		let pool = create_test_pool().await;
		cleanup_test_tables(&pool).await;

		// Create a simple table using PostgreSQL-compatible schema editor
		let pg_editor = PostgreSQLSchemaEditor::new();
		let mut editor = CockroachDBSchemaEditor::new(pg_editor);

		let create_sql = "CREATE TABLE test_users (id UUID PRIMARY KEY, name VARCHAR(100))";
		editor
			.execute(create_sql)
			.await
			.expect("Failed to create table");

		// Verify table exists by querying it
		let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM test_users")
			.fetch_one(&pool)
			.await
			.expect("Failed to query table");

		assert_eq!(count, 0);

		cleanup_test_tables(&pool).await;
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_insert_and_query() {
		let pool = create_test_pool().await;
		cleanup_test_tables(&pool).await;

		// Create table
		sqlx::query("CREATE TABLE test_users (id SERIAL PRIMARY KEY, name VARCHAR(100))")
			.execute(&pool)
			.await
			.expect("Failed to create table");

		// Insert data
		sqlx::query("INSERT INTO test_users (name) VALUES ($1)")
			.bind("Alice")
			.execute(&pool)
			.await
			.expect("Failed to insert");

		// Query data
		let name: String = sqlx::query_scalar("SELECT name FROM test_users WHERE id = 1")
			.fetch_one(&pool)
			.await
			.expect("Failed to query");

		assert_eq!(name, "Alice");

		cleanup_test_tables(&pool).await;
	}

	// Distributed Transaction Tests

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_transaction_manager_basic() {
		let pool = create_test_pool().await;
		cleanup_test_tables(&pool).await;

		sqlx::query("CREATE TABLE test_users (id SERIAL PRIMARY KEY, balance INT)")
			.execute(&pool)
			.await
			.expect("Failed to create table");

		let tx_manager = CockroachDBTransactionManager::new(pool.clone());

		// Execute transaction
		tx_manager
			.execute_with_retry(|tx| {
				Box::pin(async move {
					sqlx::query("INSERT INTO test_users (balance) VALUES ($1)")
						.bind(100)
						.execute(&mut **tx)
						.await?;
					Ok(())
				})
			})
			.await
			.expect("Transaction failed");

		// Verify data was committed
		let balance: i32 = sqlx::query_scalar("SELECT balance FROM test_users WHERE id = 1")
			.fetch_one(&pool)
			.await
			.expect("Failed to query");

		assert_eq!(balance, 100);

		cleanup_test_tables(&pool).await;
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_transaction_with_priority() {
		let pool = create_test_pool().await;
		cleanup_test_tables(&pool).await;

		sqlx::query("CREATE TABLE test_users (id SERIAL PRIMARY KEY, name VARCHAR(100))")
			.execute(&pool)
			.await
			.expect("Failed to create table");

		let tx_manager = CockroachDBTransactionManager::new(pool.clone());

		// Execute with HIGH priority
		tx_manager
			.execute_with_priority("HIGH", |tx| {
				Box::pin(async move {
					sqlx::query("INSERT INTO test_users (name) VALUES ($1)")
						.bind("HighPriority")
						.execute(&mut **tx)
						.await?;
					Ok(())
				})
			})
			.await
			.expect("Transaction failed");

		let name: String = sqlx::query_scalar("SELECT name FROM test_users WHERE id = 1")
			.fetch_one(&pool)
			.await
			.expect("Failed to query");

		assert_eq!(name, "HighPriority");

		cleanup_test_tables(&pool).await;
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_get_cluster_info() {
		let pool = create_test_pool().await;
		let tx_manager = CockroachDBTransactionManager::new(pool);

		let info = tx_manager
			.get_cluster_info()
			.await
			.expect("Failed to get cluster info");

		// Version should start with 'v' (e.g., "v23.1.0")
		assert!(info.version.starts_with('v'));
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_transaction_retry_configuration() {
		let pool = create_test_pool().await;

		let tx_manager = CockroachDBTransactionManager::new(pool.clone())
			.with_max_retries(10)
			.with_base_backoff(std::time::Duration::from_millis(200));

		// Just verify the manager was created with custom config
		// (actual retry logic is tested in unit tests)
		assert!(tx_manager.pool().is_closed() == false);
	}

	// AS OF SYSTEM TIME Tests

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_as_of_system_time_query() {
		let pool = create_test_pool().await;
		cleanup_test_tables(&pool).await;

		// Create table and insert data
		sqlx::query("CREATE TABLE test_users (id SERIAL PRIMARY KEY, name VARCHAR(100))")
			.execute(&pool)
			.await
			.expect("Failed to create table");

		sqlx::query("INSERT INTO test_users (name) VALUES ($1)")
			.bind("Alice")
			.execute(&pool)
			.await
			.expect("Failed to insert");

		// Wait a moment
		tokio::time::sleep(std::time::Duration::from_millis(100)).await;

		// Query historical data (5 seconds ago should be empty)
		let count: i64 =
			sqlx::query_scalar("SELECT COUNT(*) FROM test_users AS OF SYSTEM TIME '-5s'")
				.fetch_one(&pool)
				.await
				.expect("Failed to query");

		// Table didn't exist 5 seconds ago
		assert_eq!(count, 0);

		// Query current data
		let current_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM test_users")
			.fetch_one(&pool)
			.await
			.expect("Failed to query");

		assert_eq!(current_count, 1);

		cleanup_test_tables(&pool).await;
	}

	// PostgreSQL Compatibility Tests

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_postgresql_compatibility() {
		let pool = create_test_pool().await;
		cleanup_test_tables(&pool).await;

		// CockroachDB should support PostgreSQL wire protocol
		// Test JSONB support (PostgreSQL feature)
		sqlx::query("CREATE TABLE test_events (id SERIAL PRIMARY KEY, data JSONB)")
			.execute(&pool)
			.await
			.expect("Failed to create table");

		sqlx::query("INSERT INTO test_events (data) VALUES ($1)")
			.bind(serde_json::json!({"key": "value"}))
			.execute(&pool)
			.await
			.expect("Failed to insert JSONB");

		// Query JSONB data
		let data: serde_json::Value =
			sqlx::query_scalar("SELECT data FROM test_events WHERE id = 1")
				.fetch_one(&pool)
				.await
				.expect("Failed to query");

		assert_eq!(data["key"], "value");

		cleanup_test_tables(&pool).await;
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_concurrent_transactions() {
		let pool = create_test_pool().await;
		cleanup_test_tables(&pool).await;

		sqlx::query("CREATE TABLE test_users (id SERIAL PRIMARY KEY, name VARCHAR(100))")
			.execute(&pool)
			.await
			.expect("Failed to create table");

		let tx_manager1 = CockroachDBTransactionManager::new(pool.clone());
		let tx_manager2 = CockroachDBTransactionManager::new(pool.clone());

		// Run two transactions concurrently
		let handle1 = tokio::spawn(async move {
			tx_manager1
				.execute_with_retry(|tx| {
					Box::pin(async move {
						sqlx::query("INSERT INTO test_users (name) VALUES ($1)")
							.bind("Concurrent1")
							.execute(&mut **tx)
							.await?;
						Ok(())
					})
				})
				.await
				.expect("Transaction 1 failed");
		});

		let handle2 = tokio::spawn(async move {
			tx_manager2
				.execute_with_retry(|tx| {
					Box::pin(async move {
						sqlx::query("INSERT INTO test_users (name) VALUES ($1)")
							.bind("Concurrent2")
							.execute(&mut **tx)
							.await?;
						Ok(())
					})
				})
				.await
				.expect("Transaction 2 failed");
		});

		handle1.await.expect("Task 1 failed");
		handle2.await.expect("Task 2 failed");

		// Verify both transactions committed
		let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM test_users")
			.fetch_one(&pool)
			.await
			.expect("Failed to count");

		assert_eq!(count, 2);

		cleanup_test_tables(&pool).await;
	}

	#[tokio::test]
	#[serial(cockroachdb)]
	async fn test_uuid_primary_key() {
		let pool = create_test_pool().await;
		cleanup_test_tables(&pool).await;

		// CockroachDB recommends UUID for distributed primary keys
		sqlx::query(
			"CREATE TABLE test_users (id UUID PRIMARY KEY DEFAULT gen_random_uuid(), name VARCHAR(100))",
		)
		.execute(&pool)
		.await
		.expect("Failed to create table");

		sqlx::query("INSERT INTO test_users (name) VALUES ($1)")
			.bind("Alice")
			.execute(&pool)
			.await
			.expect("Failed to insert");

		// Verify UUID was generated
		let id: uuid::Uuid = sqlx::query_scalar("SELECT id FROM test_users WHERE name = 'Alice'")
			.fetch_one(&pool)
			.await
			.expect("Failed to query");

		assert_ne!(id, uuid::Uuid::nil());

		cleanup_test_tables(&pool).await;
	}
}
