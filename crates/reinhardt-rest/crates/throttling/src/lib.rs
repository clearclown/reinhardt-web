//! Rate limiting for Reinhardt framework
//!
//! ## Planned Features
//! TODO: Distributed consensus for rate limit synchronization
//! TODO: Graceful degradation under backend failure
//! TODO: Rate limit warmup and cooldown strategies
//! TODO: Adaptive rate limiting based on system load
//! TODO: Rate limit analytics and reporting
//! TODO: Leaky bucket algorithm implementation
//! TODO: Memcached backend support
//! TODO: Database-backed rate limiting
//! TODO: Multi-tier caching with fallback
//! TODO: Custom backend plugin system
//! TODO: Concurrent request throttling
//! TODO: Bandwidth throttling
//! TODO: Geo-based rate limiting
//! TODO: Time-of-day based rate limiting
//! TODO: Dynamic rate adjustment

pub mod anon;
pub mod backend;
pub mod burst;
pub mod scoped;
pub mod throttle;
pub mod tiered;
pub mod time_provider;
pub mod token_bucket;
pub mod user;

pub use anon::AnonRateThrottle;
pub use backend::{MemoryBackend, ThrottleBackend};
pub use burst::BurstRateThrottle;
pub use scoped::ScopedRateThrottle;
pub use throttle::{Throttle, ThrottleError, ThrottleResult};
pub use tiered::{Tier, TieredRateThrottle};
pub use time_provider::{MockTimeProvider, SystemTimeProvider, TimeProvider};
pub use token_bucket::{TokenBucket, TokenBucketConfig, TokenBucketConfigBuilder};
pub use user::UserRateThrottle;

#[cfg(feature = "redis-backend")]
pub use backend::RedisBackend;
