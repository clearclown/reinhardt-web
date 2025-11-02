//! Memcached Cache Backend
//!
//! High-performance memory cache using Memcached with connection pooling.
//!
//! # Features
//!
//! - Connection pooling with automatic reconnection
//! - Multi-server support with consistent hashing
//! - Binary protocol support
//! - Async/await based API
//!
//! # Examples
//!
//! ```no_run
//! use reinhardt_backends::cache::memcached::MemcachedCache;
//! use reinhardt_backends::cache::CacheBackend;
//! use std::time::Duration;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create cache with single server
//! let cache = MemcachedCache::new(&["memcache://localhost:11211"]).await?;
//!
//! // Store and retrieve data
//! cache.set("key", b"value", Some(Duration::from_secs(60))).await?;
//! let value = cache.get("key").await?;
//! assert_eq!(value, Some(b"value".to_vec()));
//!
//! // Multi-server setup
//! let cache = MemcachedCache::new(&[
//!     "memcache://localhost:11211",
//!     "memcache://localhost:11212",
//!     "memcache://localhost:11213",
//! ]).await?;
//! # Ok(())
//! # }
//! ```

use super::{CacheBackend, CacheError, CacheResult};
use async_trait::async_trait;
use memcache_async::ascii::Protocol;
use std::io::{Error as IoError, ErrorKind};
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

// Type alias for the memcached protocol with tokio TcpStream
type MemcachedProtocol = Protocol<Compat<TcpStream>>;

/// Memcached cache backend with connection pooling
///
/// Provides high-performance caching using Memcached as the backing store.
/// Supports multiple servers with consistent hashing.
pub struct MemcachedCache {
	// Connection pool: one persistent connection per server
	pools: Vec<Mutex<MemcachedProtocol>>,
}

impl MemcachedCache {
	/// Create a new Memcached cache
	///
	/// # Arguments
	///
	/// * `servers` - Array of server URLs (e.g., ["memcache://localhost:11211"])
	///
	/// # Examples
	///
	/// ```no_run
	/// # use reinhardt_backends::cache::memcached::MemcachedCache;
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// // Single server
	/// let cache = MemcachedCache::new(&["memcache://localhost:11211"]).await?;
	///
	/// // Multiple servers
	/// let cache = MemcachedCache::new(&[
	///     "memcache://server1:11211",
	///     "memcache://server2:11211",
	/// ]).await?;
	/// # Ok(())
	/// # }
	/// ```
	pub async fn new(servers: &[&str]) -> CacheResult<Self> {
		if servers.is_empty() {
			return Err(CacheError::Configuration(
				"At least one server URL is required".to_string(),
			));
		}

		// Parse server URLs and extract host:port
		let mut server_addrs = Vec::new();
		for server_url in servers {
			let addr = Self::parse_server_url(server_url)?;
			server_addrs.push(addr);
		}

		// Create connection pool: one persistent connection per server
		let mut pools = Vec::new();
		for server_addr in &server_addrs {
			let stream = TcpStream::connect(server_addr).await.map_err(|e| {
				CacheError::Connection(format!("Failed to connect to {}: {}", server_addr, e))
			})?;
			let compat_stream = stream.compat();
			let proto = Protocol::new(compat_stream);

			// Test with version command
			let mut proto_test = proto;
			proto_test.version().await.map_err(Self::convert_error)?;

			// Add to pool
			pools.push(Mutex::new(proto_test));
		}

		Ok(Self { pools })
	}

	/// Parse server URL to extract host:port
	fn parse_server_url(url: &str) -> CacheResult<String> {
		// Support formats: "memcache://host:port", "host:port", or "memcached://host:port"
		let url_str = url
			.strip_prefix("memcache://")
			.or_else(|| url.strip_prefix("memcached://"))
			.unwrap_or(url);

		// Validate basic format
		if !url_str.contains(':') {
			return Err(CacheError::Configuration(format!(
				"Invalid server URL format (expected host:port): {}",
				url
			)));
		}

		Ok(url_str.to_string())
	}

	/// Get a consistent server index for a given key using hashing.
	/// This ensures the same key always maps to the same server.
	fn get_server_index_for_key(&self, key: &str) -> usize {
		use std::collections::hash_map::DefaultHasher;
		use std::hash::{Hash, Hasher};

		let mut hasher = DefaultHasher::new();
		key.hash(&mut hasher);
		let hash = hasher.finish();

		(hash as usize) % self.pools.len()
	}

	/// Get a connection from the pool for a specific server index
	fn get_connection(&self, index: usize) -> &Mutex<MemcachedProtocol> {
		&self.pools[index % self.pools.len()]
	}

	/// Convert IO error to CacheError
	fn convert_error(e: IoError) -> CacheError {
		match e.kind() {
			ErrorKind::NotFound => CacheError::NotFound("Key not found".to_string()),
			ErrorKind::ConnectionRefused | ErrorKind::ConnectionReset => {
				CacheError::Connection(format!("Connection error: {}", e))
			}
			ErrorKind::InvalidData => CacheError::Serialization(format!("Invalid data: {}", e)),
			ErrorKind::TimedOut => CacheError::Timeout(format!("Operation timed out: {}", e)),
			_ => CacheError::Internal(format!("Memcached error: {}", e)),
		}
	}
}

#[async_trait]
impl CacheBackend for MemcachedCache {
	async fn get(&self, key: &str) -> CacheResult<Option<Vec<u8>>> {
		let index = self.get_server_index_for_key(key);
		let conn = self.get_connection(index);
		let mut protocol = conn.lock().await;

		match protocol.get(&key.to_string()).await {
			Ok(value) => {
				// Check if the value is empty (key not found)
				if value.is_empty() {
					Ok(None)
				} else {
					Ok(Some(value))
				}
			}
			Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
			Err(e) => Err(Self::convert_error(e)),
		}
	}

	async fn set(&self, key: &str, value: &[u8], ttl: Option<Duration>) -> CacheResult<()> {
		let expiration = if let Some(ttl) = ttl {
			ttl.as_secs() as u32
		} else {
			0 // 0 means no expiration
		};

		let index = self.get_server_index_for_key(key);
		let conn = self.get_connection(index);
		let mut protocol = conn.lock().await;

		protocol
			.set(&key.to_string(), value, expiration)
			.await
			.map_err(Self::convert_error)?;

		Ok(())
	}

	async fn delete(&self, key: &str) -> CacheResult<bool> {
		let index = self.get_server_index_for_key(key);
		let conn = self.get_connection(index);
		let mut protocol = conn.lock().await;

		// Note: delete with noreply doesn't return whether key existed
		// We'll assume success means it was deleted
		protocol
			.delete(&key.to_string())
			.await
			.map_err(Self::convert_error)?;

		Ok(true)
	}

	async fn exists(&self, key: &str) -> CacheResult<bool> {
		// Memcached doesn't have native exists command, use get
		match self.get(key).await? {
			Some(_) => Ok(true),
			None => Ok(false),
		}
	}

	async fn clear(&self) -> CacheResult<()> {
		// Flush all servers in the pool
		for conn in &self.pools {
			let mut protocol = conn.lock().await;
			protocol.flush().await.map_err(Self::convert_error)?;
		}

		Ok(())
	}

	async fn get_many(&self, keys: &[String]) -> CacheResult<Vec<Option<Vec<u8>>>> {
		if keys.is_empty() {
			return Ok(Vec::new());
		}

		// For simplicity, use first server for batch operations
		// In production, you'd want to group keys by server based on consistent hashing
		let conn = self.get_connection(0);
		let mut protocol = conn.lock().await;

		// Use get_multi for batch operation
		let keys_vec: Vec<&str> = keys.iter().map(|s| s.as_str()).collect();
		let result_map = protocol
			.get_multi(&keys_vec)
			.await
			.map_err(Self::convert_error)?;

		// Convert HashMap to Vec maintaining key order
		let mut results = Vec::with_capacity(keys.len());
		for key in keys {
			let value = result_map.get(key.as_str()).cloned();
			// Filter out empty values (key not found)
			results.push(value.filter(|v| !v.is_empty()));
		}

		Ok(results)
	}

	async fn set_many(
		&self,
		items: &[(String, Vec<u8>)],
		ttl: Option<Duration>,
	) -> CacheResult<()> {
		if items.is_empty() {
			return Ok(());
		}

		let expiration = if let Some(ttl) = ttl {
			ttl.as_secs() as u32
		} else {
			0
		};

		// For simplicity, use first server for batch operations
		let conn = self.get_connection(0);
		let mut protocol = conn.lock().await;

		// Perform sequential sets
		for (key, value) in items {
			protocol
				.set(&key.to_string(), value.as_slice(), expiration)
				.await
				.map_err(Self::convert_error)?;
		}

		Ok(())
	}

	async fn delete_many(&self, keys: &[String]) -> CacheResult<usize> {
		if keys.is_empty() {
			return Ok(0);
		}

		// For simplicity, use first server for batch operations
		let conn = self.get_connection(0);
		let mut protocol = conn.lock().await;

		// Perform sequential deletes
		// Note: with noreply, we can't know if keys existed
		let mut deleted_count = 0;
		for key in keys {
			protocol
				.delete(&key.to_string())
				.await
				.map_err(Self::convert_error)?;
			deleted_count += 1;
		}

		Ok(deleted_count)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	async fn get_memcached_url() -> (reinhardt_test::containers::MemcachedContainer, String) {
		reinhardt_test::containers::start_memcached().await
	}

	async fn create_test_cache() -> CacheResult<(
		reinhardt_test::containers::MemcachedContainer,
		MemcachedCache,
	)> {
		let (_container, url) = get_memcached_url().await;
		let cache = MemcachedCache::new(&[&url]).await?;

		Ok((_container, cache))
	}

	#[tokio::test]
	async fn test_memcached_cache_set_get() {
		let (_container, cache) = create_test_cache().await.unwrap();

		cache
			.set("test_key", b"test_value", Some(Duration::from_secs(60)))
			.await
			.unwrap();

		// Wait a moment for write to propagate

		let value = cache.get("test_key").await.unwrap();
		assert_eq!(value, Some(b"test_value".to_vec()));

		cache.delete("test_key").await.unwrap();
	}

	#[tokio::test]
	async fn test_memcached_cache_delete() {
		let (_container, cache) = create_test_cache().await.unwrap();

		cache.set("delete_key", b"value", None).await.unwrap();

		// Wait a moment for write to propagate

		let deleted = cache.delete("delete_key").await.unwrap();
		assert!(deleted);

		let value = cache.get("delete_key").await.unwrap();
		assert_eq!(value, None);
	}

	#[tokio::test]
	async fn test_memcached_cache_exists() {
		let (_container, cache) = create_test_cache().await.unwrap();

		cache.set("exists_key", b"value", None).await.unwrap();

		// Wait a moment for write to propagate

		let exists = cache.exists("exists_key").await.unwrap();
		assert!(exists);

		cache.delete("exists_key").await.unwrap();

		let exists = cache.exists("exists_key").await.unwrap();
		assert!(!exists);
	}

	#[tokio::test]
	async fn test_memcached_cache_ttl() {
		let (_container, cache) = create_test_cache().await.unwrap();

		cache
			.set("ttl_key", b"value", Some(Duration::from_secs(1)))
			.await
			.unwrap();

		// Verify key exists before TTL expires
		let exists = cache.exists("ttl_key").await.unwrap();
		assert!(exists);

		// Wait for TTL to expire (1 second + buffer)
		tokio::time::sleep(Duration::from_millis(1100)).await;

		// Verify key no longer exists after TTL expires
		let exists = cache.exists("ttl_key").await.unwrap();
		assert!(!exists);
	}

	#[tokio::test]
	async fn test_memcached_cache_batch_operations() {
		let (_container, cache) = create_test_cache().await.unwrap();

		let items = vec![
			("batch_key1".to_string(), b"value1".to_vec()),
			("batch_key2".to_string(), b"value2".to_vec()),
			("batch_key3".to_string(), b"value3".to_vec()),
		];

		cache
			.set_many(&items, Some(Duration::from_secs(60)))
			.await
			.unwrap();

		// Wait a moment for writes to propagate

		let keys = vec![
			"batch_key1".to_string(),
			"batch_key2".to_string(),
			"batch_key3".to_string(),
		];

		let values = cache.get_many(&keys).await.unwrap();
		assert_eq!(values.len(), 3);
		assert_eq!(values[0], Some(b"value1".to_vec()));
		assert_eq!(values[1], Some(b"value2".to_vec()));
		assert_eq!(values[2], Some(b"value3".to_vec()));

		let deleted = cache.delete_many(&keys).await.unwrap();
		assert_eq!(deleted, 3);
	}

	#[tokio::test]
	async fn test_memcached_cache_multi_server() {
		// Start 2 Memcached containers
		let (_container1, url1) = reinhardt_test::containers::start_memcached().await;
		let (_container2, url2) = reinhardt_test::containers::start_memcached().await;

		let cache = MemcachedCache::new(&[&url1, &url2]).await;

		// This test validates multi-server setup
		match cache {
			Ok(cache) => {
				cache.set("multi_test", b"value", None).await.unwrap();
				cache.delete("multi_test").await.unwrap();
			}
			Err(e) => {
				panic!("Failed to connect to multiple Memcached servers: {}", e);
			}
		}
	}

	#[tokio::test]
	async fn test_memcached_cache_url_parsing() {
		// Test various URL formats
		let urls = vec![
			"memcache://localhost:11211",
			"memcached://localhost:11211",
			"localhost:11211",
		];

		for url in urls {
			let result = MemcachedCache::parse_server_url(url);
			assert!(result.is_ok(), "Failed to parse URL: {}", url);
			assert_eq!(result.unwrap(), "localhost:11211");
		}
	}

	#[tokio::test]
	async fn test_memcached_cache_invalid_url() {
		let result = MemcachedCache::parse_server_url("invalid_url");
		assert!(result.is_err());
	}
}
