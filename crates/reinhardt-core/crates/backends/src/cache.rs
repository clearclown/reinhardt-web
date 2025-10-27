//! Cache Backend Infrastructure
//!
//! This module provides specialized cache backends for high-performance caching.
//! Unlike the generic `Backend` trait, cache backends are optimized for
//! typical caching patterns including batch operations and binary data storage.
//!
//! # Available Backends
//!
//! - **Redis**: Distributed cache with connection pooling
//! - **Memcached**: High-performance memory cache with consistent hashing
//! - **DynamoDB**: Persistent cache with automatic TTL management
//!
//! # Examples
//!
//! ```
//! use reinhardt_backends::cache::{CacheBackend, redis::RedisCache};
//! use std::time::Duration;
//!
//! # #[cfg(feature = "redis-cache")]
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create Redis cache with connection pool
//! let cache = RedisCache::new("redis://localhost:6379").await?;
//!
//! // Store a value with TTL
//! cache.set("user:123", b"John Doe", Some(Duration::from_secs(3600))).await?;
//!
//! // Retrieve the value
//! let value = cache.get("user:123").await?;
//! assert_eq!(value, Some(b"John Doe".to_vec()));
//!
//! // Batch operations
//! let keys = vec!["key1".to_string(), "key2".to_string()];
//! let values = cache.get_many(&keys).await?;
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use std::time::Duration;
use thiserror::Error;

#[cfg(feature = "redis-cache")]
pub mod redis;

#[cfg(feature = "memcached-cache")]
pub mod memcached;

#[cfg(feature = "dynamodb-cache")]
pub mod dynamodb;

/// Cache-specific errors
#[derive(Debug, Error)]
pub enum CacheError {
    /// Key not found in cache
    #[error("Key not found: {0}")]
    NotFound(String),

    /// Connection error
    #[error("Connection error: {0}")]
    Connection(String),

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),

    /// Operation timeout
    #[error("Operation timeout: {0}")]
    Timeout(String),

    /// Internal cache error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Configuration error
    #[error("Configuration error: {0}")]
    Configuration(String),
}

/// Result type for cache operations
pub type CacheResult<T> = Result<T, CacheError>;

/// Cache backend trait for high-performance caching
///
/// This trait provides a specialized interface for cache backends,
/// optimized for common caching patterns including batch operations
/// and binary data storage.
///
/// # Design Principles
///
/// - All values are stored as binary data (`Vec<u8>`) for flexibility
/// - Batch operations are first-class citizens for performance
/// - TTL support is mandatory for cache expiration
/// - All operations are async for non-blocking I/O
///
/// # Examples
///
/// ```no_run
/// use reinhardt_backends::cache::CacheBackend;
/// use std::time::Duration;
///
/// async fn cache_example<C: CacheBackend>(cache: &C) {
///     // Basic operations
///     cache.set("key", b"value", Some(Duration::from_secs(60))).await.unwrap();
///     let value = cache.get("key").await.unwrap();
///     assert_eq!(value, Some(b"value".to_vec()));
///
///     // Batch operations
///     let items = vec![
///         ("key1".to_string(), b"value1".to_vec()),
///         ("key2".to_string(), b"value2".to_vec()),
///     ];
///     cache.set_many(&items, Some(Duration::from_secs(60))).await.unwrap();
/// }
/// ```
#[async_trait]
pub trait CacheBackend: Send + Sync {
    /// Retrieve a value from the cache
    ///
    /// Returns `None` if the key doesn't exist or has expired.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to retrieve
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use reinhardt_backends::cache::CacheBackend;
    /// # async fn example<C: CacheBackend>(cache: &C) {
    /// let value = cache.get("user:123").await.unwrap();
    /// if let Some(data) = value {
    ///     println!("Found: {:?}", data);
    /// }
    /// # }
    /// ```
    async fn get(&self, key: &str) -> CacheResult<Option<Vec<u8>>>;

    /// Store a value in the cache with optional TTL
    ///
    /// # Arguments
    ///
    /// * `key` - The key to store the value under
    /// * `value` - The binary data to store
    /// * `ttl` - Optional time-to-live duration
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use reinhardt_backends::cache::CacheBackend;
    /// # use std::time::Duration;
    /// # async fn example<C: CacheBackend>(cache: &C) {
    /// // Store with TTL
    /// cache.set("session:abc", b"user_data", Some(Duration::from_secs(3600))).await.unwrap();
    ///
    /// // Store without TTL (cache-dependent behavior)
    /// cache.set("permanent", b"data", None).await.unwrap();
    /// # }
    /// ```
    async fn set(&self, key: &str, value: &[u8], ttl: Option<Duration>) -> CacheResult<()>;

    /// Delete a key from the cache
    ///
    /// Returns `true` if the key existed, `false` otherwise.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use reinhardt_backends::cache::CacheBackend;
    /// # async fn example<C: CacheBackend>(cache: &C) {
    /// let deleted = cache.delete("old_key").await.unwrap();
    /// if deleted {
    ///     println!("Key was deleted");
    /// }
    /// # }
    /// ```
    async fn delete(&self, key: &str) -> CacheResult<bool>;

    /// Check if a key exists in the cache
    ///
    /// Returns `true` if the key exists and hasn't expired.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use reinhardt_backends::cache::CacheBackend;
    /// # async fn example<C: CacheBackend>(cache: &C) {
    /// if cache.exists("user:123").await.unwrap() {
    ///     println!("User is in cache");
    /// }
    /// # }
    /// ```
    async fn exists(&self, key: &str) -> CacheResult<bool>;

    /// Clear all keys from the cache
    ///
    /// **Warning**: This operation removes all data from the cache.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use reinhardt_backends::cache::CacheBackend;
    /// # async fn example<C: CacheBackend>(cache: &C) {
    /// cache.clear().await.unwrap();
    /// println!("Cache cleared");
    /// # }
    /// ```
    async fn clear(&self) -> CacheResult<()>;

    /// Retrieve multiple values in a single operation
    ///
    /// Returns a vector of `Option<Vec<u8>>` corresponding to each key.
    /// Missing or expired keys will be `None`.
    ///
    /// # Arguments
    ///
    /// * `keys` - Slice of keys to retrieve
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use reinhardt_backends::cache::CacheBackend;
    /// # async fn example<C: CacheBackend>(cache: &C) {
    /// let keys = vec!["key1".to_string(), "key2".to_string(), "key3".to_string()];
    /// let values = cache.get_many(&keys).await.unwrap();
    ///
    /// for (key, value) in keys.iter().zip(values.iter()) {
    ///     match value {
    ///         Some(data) => println!("{}: found", key),
    ///         None => println!("{}: not found", key),
    ///     }
    /// }
    /// # }
    /// ```
    async fn get_many(&self, keys: &[String]) -> CacheResult<Vec<Option<Vec<u8>>>>;

    /// Store multiple key-value pairs in a single operation
    ///
    /// All items will have the same TTL applied.
    ///
    /// # Arguments
    ///
    /// * `items` - Slice of (key, value) tuples
    /// * `ttl` - Optional time-to-live duration for all items
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use reinhardt_backends::cache::CacheBackend;
    /// # use std::time::Duration;
    /// # async fn example<C: CacheBackend>(cache: &C) {
    /// let items = vec![
    ///     ("user:1".to_string(), b"Alice".to_vec()),
    ///     ("user:2".to_string(), b"Bob".to_vec()),
    ///     ("user:3".to_string(), b"Charlie".to_vec()),
    /// ];
    ///
    /// cache.set_many(&items, Some(Duration::from_secs(3600))).await.unwrap();
    /// # }
    /// ```
    async fn set_many(&self, items: &[(String, Vec<u8>)], ttl: Option<Duration>)
        -> CacheResult<()>;

    /// Delete multiple keys in a single operation
    ///
    /// Returns the number of keys that were actually deleted.
    ///
    /// # Arguments
    ///
    /// * `keys` - Slice of keys to delete
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use reinhardt_backends::cache::CacheBackend;
    /// # async fn example<C: CacheBackend>(cache: &C) {
    /// let keys = vec!["old:1".to_string(), "old:2".to_string()];
    /// let deleted = cache.delete_many(&keys).await.unwrap();
    /// println!("Deleted {} keys", deleted);
    /// # }
    /// ```
    async fn delete_many(&self, keys: &[String]) -> CacheResult<usize>;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that CacheBackend trait can be used with generic types
    async fn test_cache_generic<C: CacheBackend>(cache: &C) -> CacheResult<()> {
        cache
            .set("test", b"value", Some(Duration::from_secs(60)))
            .await?;
        let value = cache.get("test").await?;
        assert_eq!(value, Some(b"value".to_vec()));
        Ok(())
    }

    // Dummy implementation for trait validation
    struct DummyCache;

    #[async_trait]
    impl CacheBackend for DummyCache {
        async fn get(&self, _key: &str) -> CacheResult<Option<Vec<u8>>> {
            Ok(None)
        }

        async fn set(&self, _key: &str, _value: &[u8], _ttl: Option<Duration>) -> CacheResult<()> {
            Ok(())
        }

        async fn delete(&self, _key: &str) -> CacheResult<bool> {
            Ok(false)
        }

        async fn exists(&self, _key: &str) -> CacheResult<bool> {
            Ok(false)
        }

        async fn clear(&self) -> CacheResult<()> {
            Ok(())
        }

        async fn get_many(&self, keys: &[String]) -> CacheResult<Vec<Option<Vec<u8>>>> {
            Ok(vec![None; keys.len()])
        }

        async fn set_many(
            &self,
            _items: &[(String, Vec<u8>)],
            _ttl: Option<Duration>,
        ) -> CacheResult<()> {
            Ok(())
        }

        async fn delete_many(&self, _keys: &[String]) -> CacheResult<usize> {
            Ok(0)
        }
    }

    #[tokio::test]
    async fn test_cache_trait_usage() {
        let cache = DummyCache;
        test_cache_generic(&cache).await.unwrap();
    }
}
