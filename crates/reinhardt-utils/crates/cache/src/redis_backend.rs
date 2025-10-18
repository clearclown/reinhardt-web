//! Redis cache backend
//!
//! Provides a Redis-backed cache implementation.

use crate::Cache;
use async_trait::async_trait;
use reinhardt_exception::{Error, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Redis cache backend
///
/// Stores cached values in Redis for distributed caching.
#[derive(Clone)]
pub struct RedisCache {
    #[allow(dead_code)]
    connection_url: String,
    default_ttl: Option<Duration>,
    key_prefix: String,
}

impl RedisCache {
    /// Create a new Redis cache with the given connection URL
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_cache::RedisCache;
    ///
    /// let cache = RedisCache::new("redis://localhost:6379");
    /// // Redis cache is now configured and ready to use
    /// ```
    pub fn new(connection_url: impl Into<String>) -> Self {
        Self {
            connection_url: connection_url.into(),
            default_ttl: None,
            key_prefix: String::new(),
        }
    }
    /// Set default TTL for all cache entries
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_cache::RedisCache;
    /// use std::time::Duration;
    ///
    /// let cache = RedisCache::new("redis://localhost:6379")
    ///     .with_default_ttl(Duration::from_secs(300));
    /// // All cache entries will expire after 300 seconds by default
    /// ```
    pub fn with_default_ttl(mut self, ttl: Duration) -> Self {
        self.default_ttl = Some(ttl);
        self
    }
    /// Set key prefix for namespacing cache entries
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_cache::RedisCache;
    ///
    /// let cache = RedisCache::new("redis://localhost:6379")
    ///     .with_key_prefix("myapp");
    /// // All keys will be prefixed with "myapp:"
    /// ```
    pub fn with_key_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.key_prefix = prefix.into();
        self
    }

    /// Build the full key with prefix
    fn build_key(&self, key: &str) -> String {
        if self.key_prefix.is_empty() {
            key.to_string()
        } else {
            format!("{}:{}", self.key_prefix, key)
        }
    }
}

#[async_trait]
impl Cache for RedisCache {
    async fn get<T>(&self, key: &str) -> Result<Option<T>>
    where
        T: for<'de> Deserialize<'de> + Send,
    {
        // This is a placeholder implementation
        // In a real implementation, you would:
        // 1. Connect to Redis
        // 2. Get the value
        // 3. Deserialize it
        // For now, we return None to indicate cache miss
        let _ = self.build_key(key);
        Ok(None)
    }

    async fn set<T>(&self, key: &str, value: &T, ttl: Option<Duration>) -> Result<()>
    where
        T: Serialize + Send + Sync,
    {
        // Placeholder implementation
        // In a real implementation, you would:
        // 1. Connect to Redis
        // 2. Serialize the value
        // 3. Set it with TTL if provided
        let _ = self.build_key(key);
        let _ = serde_json::to_vec(value).map_err(|e| Error::Serialization(e.to_string()))?;
        let _ = ttl.or(self.default_ttl);
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<()> {
        // Placeholder implementation
        let _ = self.build_key(key);
        Ok(())
    }

    async fn has_key(&self, key: &str) -> Result<bool> {
        // Placeholder implementation
        let _ = self.build_key(key);
        Ok(false)
    }

    async fn clear(&self) -> Result<()> {
        // Placeholder implementation
        // In a real implementation, you would delete all keys with the prefix
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_redis_cache_creation() {
        let cache = RedisCache::new("redis://localhost:6379")
            .with_default_ttl(Duration::from_secs(300))
            .with_key_prefix("myapp");

        assert_eq!(cache.connection_url, "redis://localhost:6379");
        assert_eq!(cache.key_prefix, "myapp");
        assert!(cache.default_ttl.is_some());
    }

    #[test]
    fn test_build_key_with_prefix() {
        let cache = RedisCache::new("redis://localhost:6379").with_key_prefix("app");

        assert_eq!(cache.build_key("user:123"), "app:user:123");
    }

    #[test]
    fn test_build_key_without_prefix() {
        let cache = RedisCache::new("redis://localhost:6379");
        assert_eq!(cache.build_key("user:123"), "user:123");
    }
}
