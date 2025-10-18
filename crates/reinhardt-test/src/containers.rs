//! TestContainers integration for database testing
//!
//! Provides automatic Docker container management for testing with real databases.
//! Containers are automatically started and cleaned up during tests.

use std::sync::Arc;
use testcontainers::{Container, Image, RunnableImage};
use testcontainers_modules::mysql::Mysql;
use testcontainers_modules::postgres::Postgres;
use testcontainers_modules::redis::Redis as RedisImage;

/// Common interface for database test containers
#[async_trait::async_trait]
pub trait TestDatabase: Send + Sync {
    /// Get the database connection URL
    fn connection_url(&self) -> String;

    /// Get the database type (postgres, mysql, etc.)
    fn database_type(&self) -> &'static str;

    /// Wait for the database to be ready
    async fn wait_ready(&self) -> Result<(), Box<dyn std::error::Error>>;
}

/// PostgreSQL test container
pub struct PostgresContainer<'a> {
    container: Container<'a, Postgres>,
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
}

impl<'a> PostgresContainer<'a> {
    /// Create a new PostgreSQL container with default settings
    ///
    /// # Examples
    ///
    /// ```
    pub fn new(docker: &'a testcontainers::clients::Cli) -> Self {
        Self::with_credentials(docker, "postgres", "postgres", "test")
    }
    /// Create a PostgreSQL container with custom credentials
    ///
    /// # Examples
    ///
    /// ```
    pub fn with_credentials(
        docker: &'a testcontainers::clients::Cli,
        username: &str,
        password: &str,
        database: &str,
    ) -> Self {
        let image = Postgres::default()
            .with_env_var("POSTGRES_USER", username)
            .with_env_var("POSTGRES_PASSWORD", password)
            .with_env_var("POSTGRES_DB", database);

        let container = docker.run(image);
        let port = container.get_host_port_ipv4(5432);

        Self {
            container,
            host: "localhost".to_string(),
            port,
            database: database.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
    /// Get the container port
    ///
    /// # Examples
    ///
    /// ```
    pub fn port(&self) -> u16 {
        self.port
    }
}

#[async_trait::async_trait]
impl<'a> TestDatabase for PostgresContainer<'a> {
    fn connection_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    fn database_type(&self) -> &'static str {
        "postgres"
    }

    async fn wait_ready(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Try to connect to ensure database is ready
        let url = self.connection_url();
        let pool = sqlx::postgres::PgPool::connect(&url).await?;
        sqlx::query("SELECT 1").execute(&pool).await?;
        pool.close().await;
        Ok(())
    }
}

/// MySQL test container
pub struct MySqlContainer<'a> {
    container: Container<'a, Mysql>,
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
}

impl<'a> MySqlContainer<'a> {
    /// Create a new MySQL container with default settings
    ///
    /// # Examples
    ///
    /// ```
    pub fn new(docker: &'a testcontainers::clients::Cli) -> Self {
        Self::with_credentials(docker, "root", "test", "test")
    }
    /// Create a MySQL container with custom credentials
    ///
    /// # Examples
    ///
    /// ```
    pub fn with_credentials(
        docker: &'a testcontainers::clients::Cli,
        username: &str,
        password: &str,
        database: &str,
    ) -> Self {
        let image = Mysql::default()
            .with_env_var("MYSQL_ROOT_PASSWORD", password)
            .with_env_var("MYSQL_DATABASE", database);

        let container = docker.run(image);
        let port = container.get_host_port_ipv4(3306);

        Self {
            container,
            host: "localhost".to_string(),
            port,
            database: database.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
    /// Get the container port
    ///
    /// # Examples
    ///
    /// ```
    pub fn port(&self) -> u16 {
        self.port
    }
}

#[async_trait::async_trait]
impl<'a> TestDatabase for MySqlContainer<'a> {
    fn connection_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database
        )
    }

    fn database_type(&self) -> &'static str {
        "mysql"
    }

    async fn wait_ready(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Try to connect to ensure database is ready
        let url = self.connection_url();
        let pool = sqlx::mysql::MySqlPool::connect(&url).await?;
        sqlx::query("SELECT 1").execute(&pool).await?;
        pool.close().await;
        Ok(())
    }
}

/// Redis test container
pub struct RedisContainer<'a> {
    container: Container<'a, RedisImage>,
    host: String,
    port: u16,
}

impl<'a> RedisContainer<'a> {
    /// Create a new Redis container
    ///
    /// # Examples
    ///
    /// ```
    pub fn new(docker: &'a testcontainers::clients::Cli) -> Self {
        let image = RedisImage::default();
        let container = docker.run(image);
        let port = container.get_host_port_ipv4(6379);

        Self {
            container,
            host: "localhost".to_string(),
            port,
        }
    }
    /// Get the connection URL for Redis
    ///
    /// # Examples
    ///
    /// ```
    pub fn connection_url(&self) -> String {
        format!("redis://{}:{}", self.host, self.port)
    }
    /// Get the container port
    ///
    /// # Examples
    ///
    /// ```
    pub fn port(&self) -> u16 {
        self.port
    }
}

/// Helper function to run a test with a database container
///
/// # Example
/// ```ignore
/// use reinhardt_test::containers::{with_postgres, PostgresContainer};
///
/// #[tokio::test]
/// async fn test_with_database() {
///     with_postgres(|db| async move {
///         let url = db.connection_url();
///         // Use database...
///         Ok(())
///     }).await.unwrap();
/// }
/// ```
pub async fn with_postgres<F, Fut>(f: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(PostgresContainer) -> Fut,
    Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>,
{
    let docker = testcontainers::clients::Cli::default();
    let container = PostgresContainer::new(&docker);
    container.wait_ready().await?;
    f(container).await
}
/// Helper function to run a test with a MySQL container
///
/// # Examples
///
/// ```
/// ```
pub async fn with_mysql<F, Fut>(f: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(MySqlContainer) -> Fut,
    Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>,
{
    let docker = testcontainers::clients::Cli::default();
    let container = MySqlContainer::new(&docker);
    container.wait_ready().await?;
    f(container).await
}
/// Helper function to run a test with a Redis container
///
/// # Examples
///
/// ```
/// ```
pub async fn with_redis<F, Fut>(f: F) -> Result<(), Box<dyn std::error::Error>>
where
    F: FnOnce(RedisContainer) -> Fut,
    Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>>,
{
    let docker = testcontainers::clients::Cli::default();
    let container = RedisContainer::new(&docker);
    f(container).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore] // Requires Docker
    async fn test_postgres_container() {
        with_postgres(|db| async move {
            let url = db.connection_url();
            assert!(url.starts_with("postgres://"));
            assert_eq!(db.database_type(), "postgres");
            Ok(())
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    #[ignore] // Requires Docker
    async fn test_mysql_container() {
        with_mysql(|db| async move {
            let url = db.connection_url();
            assert!(url.starts_with("mysql://"));
            assert_eq!(db.database_type(), "mysql");
            Ok(())
        })
        .await
        .unwrap();
    }

    #[tokio::test]
    #[ignore] // Requires Docker
    async fn test_redis_container() {
        with_redis(|redis| async move {
            let url = redis.connection_url();
            assert!(url.starts_with("redis://"));
            Ok(())
        })
        .await
        .unwrap();
    }
}
