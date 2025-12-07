//! DataLoader integration tests
//!
//! Tests DataLoader batch loading, caching, and N+1 query problem mitigation.

use async_trait::async_trait;
use reinhardt_graphql::context::{DataLoader, LoaderError};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

/// Test DataLoader that tracks load calls for N+1 verification
struct TestUserLoader {
	load_count: Arc<AtomicUsize>,
	load_many_count: Arc<AtomicUsize>,
}

impl TestUserLoader {
	fn new() -> Self {
		Self {
			load_count: Arc::new(AtomicUsize::new(0)),
			load_many_count: Arc::new(AtomicUsize::new(0)),
		}
	}

	fn load_count(&self) -> usize {
		self.load_count.load(Ordering::SeqCst)
	}

	fn load_many_count(&self) -> usize {
		self.load_many_count.load(Ordering::SeqCst)
	}
}

#[async_trait]
impl DataLoader for TestUserLoader {
	type Key = String;
	type Value = String;

	async fn load(&self, key: Self::Key) -> Result<Self::Value, LoaderError> {
		self.load_count.fetch_add(1, Ordering::SeqCst);
		Ok(format!("User: {}", key))
	}

	async fn load_many(&self, keys: Vec<Self::Key>) -> Result<Vec<Self::Value>, LoaderError> {
		self.load_many_count.fetch_add(1, Ordering::SeqCst);
		Ok(keys.into_iter().map(|k| format!("User: {}", k)).collect())
	}
}

/// Test: DataLoader batch loading (load_many)
#[tokio::test]
async fn test_dataloader_batch_loading() {
	let loader = TestUserLoader::new();

	// Load multiple keys in one batch
	let keys = vec!["1".to_string(), "2".to_string(), "3".to_string()];
	let results = loader.load_many(keys).await.unwrap();

	assert_eq!(results.len(), 3);
	assert_eq!(results[0], "User: 1");
	assert_eq!(results[1], "User: 2");
	assert_eq!(results[2], "User: 3");

	// Should have called load_many once (not load 3 times)
	assert_eq!(loader.load_many_count(), 1);
	assert_eq!(loader.load_count(), 0);
}

/// Test: DataLoader N+1 problem mitigation
#[tokio::test]
async fn test_dataloader_n_plus_1_mitigation() {
	let loader = TestUserLoader::new();

	// Simulate N+1 query pattern without batching (bad practice)
	for i in 1..=10 {
		let _result = loader.load(i.to_string()).await.unwrap();
	}

	// With naive approach: 10 individual loads
	assert_eq!(loader.load_count(), 10);

	// Now use load_many for the same 10 keys (good practice)
	let loader2 = TestUserLoader::new();
	let keys: Vec<String> = (1..=10).map(|i| i.to_string()).collect();
	let _results = loader2.load_many(keys).await.unwrap();

	// With batching: only 1 batch load
	assert_eq!(loader2.load_many_count(), 1);
	assert_eq!(loader2.load_count(), 0);
}

/// Test: DataLoader instance independence
#[tokio::test]
async fn test_dataloader_instance_independence() {
	let loader1 = TestUserLoader::new();
	let loader2 = TestUserLoader::new();

	// Verify different loaders have independent counters
	let _result1 = loader1.load("test".to_string()).await.unwrap();
	assert_eq!(loader1.load_count(), 1);
	assert_eq!(loader2.load_count(), 0);

	let _result2 = loader2.load("test".to_string()).await.unwrap();
	assert_eq!(loader1.load_count(), 1);
	assert_eq!(loader2.load_count(), 1);
}

/// Test: DataLoader error handling in batch load
#[tokio::test]
async fn test_dataloader_batch_error_handling() {
	struct ErrorLoader;

	#[async_trait]
	impl DataLoader for ErrorLoader {
		type Key = String;
		type Value = String;

		async fn load(&self, key: Self::Key) -> Result<Self::Value, LoaderError> {
			if key == "error" {
				Err(LoaderError::NotFound(key))
			} else {
				Ok(format!("Value: {}", key))
			}
		}

		async fn load_many(&self, keys: Vec<Self::Key>) -> Result<Vec<Self::Value>, LoaderError> {
			// Fail if any key is "error"
			if keys.iter().any(|k| k == "error") {
				Err(LoaderError::NotFound("error key in batch".to_string()))
			} else {
				Ok(keys.into_iter().map(|k| format!("Value: {}", k)).collect())
			}
		}
	}

	let loader = ErrorLoader;

	// Individual error
	let result = loader.load("error".to_string()).await;
	assert!(result.is_err());
	match result {
		Err(LoaderError::NotFound(msg)) => assert_eq!(msg, "error"),
		_ => panic!("Expected NotFound error"),
	}

	// Batch error
	let keys = vec!["ok".to_string(), "error".to_string()];
	let result = loader.load_many(keys).await;
	assert!(result.is_err());
}

/// Test: Multiple different DataLoaders
#[tokio::test]
async fn test_multiple_dataloader_types() {
	struct UserLoader;
	struct PostLoader;

	#[async_trait]
	impl DataLoader for UserLoader {
		type Key = i32;
		type Value = String;

		async fn load(&self, key: Self::Key) -> Result<Self::Value, LoaderError> {
			Ok(format!("User {}", key))
		}

		async fn load_many(&self, keys: Vec<Self::Key>) -> Result<Vec<Self::Value>, LoaderError> {
			Ok(keys.iter().map(|k| format!("User {}", k)).collect())
		}
	}

	#[async_trait]
	impl DataLoader for PostLoader {
		type Key = i32;
		type Value = String;

		async fn load(&self, key: Self::Key) -> Result<Self::Value, LoaderError> {
			Ok(format!("Post {}", key))
		}

		async fn load_many(&self, keys: Vec<Self::Key>) -> Result<Vec<Self::Value>, LoaderError> {
			Ok(keys.iter().map(|k| format!("Post {}", k)).collect())
		}
	}

	// Use both loaders independently
	let user_loader = UserLoader;
	let post_loader = PostLoader;

	let user_result = user_loader.load(1).await.unwrap();
	let post_result = post_loader.load(2).await.unwrap();

	assert_eq!(user_result, "User 1");
	assert_eq!(post_result, "Post 2");
}

/// Test: DataLoader performance comparison (single vs batch)
#[tokio::test]
async fn test_dataloader_performance_comparison() {
	use std::time::Instant;

	struct SlowLoader {
		batch_count: Arc<AtomicUsize>,
	}

	#[async_trait]
	impl DataLoader for SlowLoader {
		type Key = i32;
		type Value = String;

		async fn load(&self, key: Self::Key) -> Result<Self::Value, LoaderError> {
			// Simulate slow DB query
			tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
			Ok(format!("Data {}", key))
		}

		async fn load_many(&self, keys: Vec<Self::Key>) -> Result<Vec<Self::Value>, LoaderError> {
			self.batch_count.fetch_add(1, Ordering::SeqCst);
			// Simulate one batch DB query (much faster than N individual queries)
			tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
			Ok(keys.iter().map(|k| format!("Data {}", k)).collect())
		}
	}

	let loader = SlowLoader {
		batch_count: Arc::new(AtomicUsize::new(0)),
	};

	// Batch load 10 items
	let start = Instant::now();
	let keys: Vec<i32> = (1..=10).collect();
	let _results = loader.load_many(keys).await.unwrap();
	let batch_duration = start.elapsed();

	// Batch should be called only once
	assert_eq!(loader.batch_count.load(Ordering::SeqCst), 1);

	// Batch should be faster than theoretical 10 individual loads (100ms)
	assert!(
		batch_duration.as_millis() < 100,
		"Batch load should be faster than individual loads"
	);
}
