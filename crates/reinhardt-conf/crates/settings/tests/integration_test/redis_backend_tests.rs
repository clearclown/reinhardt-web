//! Redis backend integration tests

use crate::common::{init_test_logging, random_test_key, redis_helpers::*, test_config_value};
use reinhardt_settings::dynamic::DynamicBackend;
use testcontainers::clients;

#[tokio::test]
async fn test_redis_connection() {
    init_test_logging();

    let docker = clients::Cli::default();
    let redis_container = start_redis(&docker);
    let backend = create_redis_backend(&redis_container.url);

    // Test basic connectivity by setting and getting a value
    let key = random_test_key();
    let value = test_config_value("test_connection");

    backend
        .set(&key, value.clone())
        .await
        .expect("Failed to set value");
    let retrieved = backend.get(&key).await.expect("Failed to get value");

    assert_eq!(retrieved, Some(value));
}

#[tokio::test]
async fn test_redis_set_get() {
    init_test_logging();

    let docker = clients::Cli::default();
    let redis_container = start_redis(&docker);
    let backend = create_redis_backend(&redis_container.url);

    let key = random_test_key();
    let value = test_config_value("hello_redis");

    // Set value
    backend
        .set(&key, value.clone())
        .await
        .expect("Failed to set value");

    // Get value
    let retrieved = backend.get(&key).await.expect("Failed to get value");
    assert_eq!(retrieved, Some(value));

    // Get non-existent key
    let non_existent = backend
        .get("non_existent_key")
        .await
        .expect("Failed to get non-existent key");
    assert_eq!(non_existent, None);
}

#[tokio::test]
async fn test_redis_update() {
    init_test_logging();

    let docker = clients::Cli::default();
    let redis_container = start_redis(&docker);
    let backend = create_redis_backend(&redis_container.url);

    let key = random_test_key();
    let value1 = test_config_value("initial_value");
    let value2 = test_config_value("updated_value");

    // Set initial value
    backend
        .set(&key, value1)
        .await
        .expect("Failed to set initial value");

    // Update value
    backend
        .set(&key, value2.clone())
        .await
        .expect("Failed to update value");

    // Verify updated value
    let retrieved = backend.get(&key).await.expect("Failed to get value");
    assert_eq!(retrieved, Some(value2));
}

#[tokio::test]
async fn test_redis_delete() {
    init_test_logging();

    let docker = clients::Cli::default();
    let redis_container = start_redis(&docker);
    let backend = create_redis_backend(&redis_container.url);

    let key = random_test_key();
    let value = test_config_value("to_be_deleted");

    // Set value
    backend.set(&key, value).await.expect("Failed to set value");

    // Delete value
    backend.delete(&key).await.expect("Failed to delete value");

    // Verify deletion
    let retrieved = backend.get(&key).await.expect("Failed to get deleted key");
    assert_eq!(retrieved, None);
}

#[tokio::test]
async fn test_redis_list() {
    init_test_logging();

    let docker = clients::Cli::default();
    let redis_container = start_redis(&docker);
    let backend = create_redis_backend(&redis_container.url);

    // Set multiple values
    let keys = vec![random_test_key(), random_test_key(), random_test_key()];

    for key in &keys {
        backend
            .set(key, test_config_value("list_test"))
            .await
            .expect("Failed to set value");
    }

    // List all keys
    let all_keys = backend.list().await.expect("Failed to list keys");

    // Verify all our keys are present
    for key in &keys {
        assert!(all_keys.contains(key), "Key {} not found in list", key);
    }
}

#[tokio::test]
async fn test_redis_persistence() {
    init_test_logging();

    let docker = clients::Cli::default();
    let redis_container = start_redis(&docker);
    let url = redis_container.url.clone();

    let key = random_test_key();
    let value = test_config_value("persistent_value");

    // Create first backend and set value
    {
        let backend1 = create_redis_backend(&url);
        backend1
            .set(&key, value.clone())
            .await
            .expect("Failed to set value");
    }

    // Create second backend and verify value persists
    {
        let backend2 = create_redis_backend(&url);
        let retrieved = backend2.get(&key).await.expect("Failed to get value");
        assert_eq!(retrieved, Some(value));
    }
}

#[tokio::test]
async fn test_redis_concurrent_access() {
    init_test_logging();

    let docker = clients::Cli::default();
    let redis_container = start_redis(&docker);
    let backend = create_redis_backend(&redis_container.url);

    let key = random_test_key();
    let mut handles = vec![];

    // Spawn multiple tasks writing to the same key
    for i in 0..10 {
        let backend_clone = create_redis_backend(&redis_container.url);
        let key_clone = key.clone();

        let handle = tokio::spawn(async move {
            let value = test_config_value(&format!("concurrent_{}", i));
            backend_clone
                .set(&key_clone, value)
                .await
                .expect("Failed to set value");
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.expect("Task panicked");
    }

    // Verify a value exists (last write wins)
    let retrieved = backend.get(&key).await.expect("Failed to get value");
    assert!(retrieved.is_some());
}

#[tokio::test]
async fn test_redis_prefix_isolation() {
    init_test_logging();

    let docker = clients::Cli::default();
    let redis_container = start_redis(&docker);

    // Create two backends with different prefixes
    let backend1 =
        reinhardt_settings::backends::RedisBackend::with_prefix(&redis_container.url, "app1:")
            .expect("Failed to create backend1");
    let backend2 =
        reinhardt_settings::backends::RedisBackend::with_prefix(&redis_container.url, "app2:")
            .expect("Failed to create backend2");

    let key = "shared_key";
    let value1 = test_config_value("app1_value");
    let value2 = test_config_value("app2_value");

    // Set values in both backends
    backend1
        .set(key, value1.clone())
        .await
        .expect("Failed to set in backend1");
    backend2
        .set(key, value2.clone())
        .await
        .expect("Failed to set in backend2");

    // Verify isolation
    let retrieved1 = backend1
        .get(key)
        .await
        .expect("Failed to get from backend1");
    let retrieved2 = backend2
        .get(key)
        .await
        .expect("Failed to get from backend2");

    assert_eq!(retrieved1, Some(value1));
    assert_eq!(retrieved2, Some(value2));
}
