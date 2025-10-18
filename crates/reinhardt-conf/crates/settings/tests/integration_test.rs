//! Integration tests for reinhardt-settings

#[path = "common/mod.rs"]
mod common;

#[cfg(feature = "dynamic-redis")]
#[path = "integration_test/redis_backend_tests.rs"]
mod redis_backend_tests;

#[cfg(feature = "dynamic-database")]
#[path = "integration_test/database_backend_tests.rs"]
mod database_backend_tests;

#[cfg(feature = "encryption")]
#[path = "integration_test/encryption_tests.rs"]
mod encryption_tests;
