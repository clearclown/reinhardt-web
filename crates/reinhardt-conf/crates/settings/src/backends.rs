//! Backend implementations for dynamic settings

pub mod memory;

#[cfg(feature = "dynamic-redis")]
pub mod redis_backend;

#[cfg(feature = "dynamic-database")]
pub mod database;

pub use memory::MemoryBackend;

#[cfg(feature = "dynamic-redis")]
pub use redis_backend::RedisBackend;

#[cfg(feature = "dynamic-database")]
pub use database::DatabaseBackend;
