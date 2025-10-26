//! # Reinhardt Cache
//!
//! Caching framework for Reinhardt.
//!
//! ## Features
//!
//! - **InMemoryCache**: Simple in-memory cache backend
//! - **FileCache**: File-based persistent cache backend
//! - **RedisCache**: Redis-backed cache (requires redis-backend feature)
//! - **MemcachedCache**: Memcached-backed cache (requires memcached-backend feature)
//! - **HybridCache**: Multi-tier caching (memory + distributed)
//! - **RedisClusterCache**: Redis Cluster support (requires redis-cluster feature)
//! - **RedisSentinelCache**: Redis Sentinel support (requires redis-sentinel feature)
//! - **Pub/Sub**: Cache invalidation via Redis channels (requires redis-backend feature)
//! - **Cache Warming**: Pre-populate cache on startup
//! - **Cache Tags**: Tag-based invalidation for related entries
//! - TTL support for automatic expiration
//! - Async-first API
//!
//! ## Example
//!
//! ```rust
//! use reinhardt_cache::{Cache, InMemoryCache};
//!
//! # async fn example() {
//! let cache = InMemoryCache::new();
//!
//! // Set a value
//! cache.set("key", &"value".to_string(), None).await.unwrap();
//!
//! // Get a value
//! let value: Option<String> = cache.get("key").await.unwrap();
//! assert_eq!(value, Some("value".to_string()));
//!
//! // Delete a value
//! cache.delete("key").await.unwrap();
//! # }
//! ```
//!

mod cache_trait;
mod entry;
mod in_memory;
mod key_builder;
mod statistics;

pub mod di_support;
pub mod file_backend;
pub mod middleware;
pub mod tags;
pub mod warming;

#[cfg(feature = "redis-backend")]
pub mod redis_backend;

#[cfg(feature = "memcached-backend")]
pub mod memcached;

pub mod hybrid;

#[cfg(feature = "redis-cluster")]
pub mod redis_cluster;

#[cfg(feature = "redis-sentinel")]
pub mod redis_sentinel;

#[cfg(feature = "redis-backend")]
pub mod pubsub;

// Re-export exception types
pub use reinhardt_exception::Result;

// Re-export core items
pub use cache_trait::Cache;
pub use in_memory::InMemoryCache;
pub use key_builder::CacheKeyBuilder;
pub use statistics::{CacheEntryInfo, CacheStatistics};

// Re-export middleware and Redis backend
pub use middleware::{CacheMiddleware, CacheMiddlewareConfig};

#[cfg(feature = "redis-backend")]
pub use redis_backend::RedisCache;

#[cfg(feature = "memcached-backend")]
pub use memcached::MemcachedCache;

pub use hybrid::HybridCache;

#[cfg(feature = "redis-cluster")]
pub use redis_cluster::RedisClusterCache;

#[cfg(feature = "redis-sentinel")]
pub use redis_sentinel::RedisSentinelCache;

#[cfg(feature = "redis-backend")]
pub use pubsub::{CacheInvalidation, CacheInvalidationChannel};

// Re-export DI support
pub use di_support::CacheService;
#[cfg(feature = "redis-backend")]
pub use di_support::RedisConfig;

// Re-export file backend
pub use file_backend::FileCache;

// Re-export cache warming
pub use warming::{BatchWarmer, CacheWarmer, FunctionWarmer, ParallelWarmer};

// Re-export cache tags
pub use tags::{TaggedCache, TaggedCacheWrapper};
