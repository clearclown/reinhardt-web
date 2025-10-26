//! # Reinhardt Cache
//!
//! Caching framework for Reinhardt.
//!
//! ## Features
//!
//! - **InMemoryCache**: Simple in-memory cache backend
//! - **FileCache**: File-based persistent cache backend
//! - **RedisCache**: Redis-backed cache (requires redis feature)
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
//! ## Planned Features
//! TODO: Memcached backend - Memcached integration (dependency declared but not implemented)
//! TODO: Hybrid cache - Multi-tier caching (memory + distributed)
//! TODO: Per-view caching - View-level cache decorators
//! TODO: Template fragment caching - Selective template output caching
//! TODO: QuerySet caching - Automatic ORM query result caching
//! TODO: Write-through - Synchronous cache updates
//! TODO: Write-behind - Asynchronous cache updates
//! TODO: Cache-aside - Application-managed caching
//! TODO: Read-through - Automatic cache population on miss
//! TODO: Event hooks - Pre/post cache operations callbacks
//! TODO: Redis Cluster support - Distributed Redis deployments
//! TODO: Redis Sentinel support - High availability configurations
//! TODO: Pub/Sub support - Cache invalidation via Redis channels

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
