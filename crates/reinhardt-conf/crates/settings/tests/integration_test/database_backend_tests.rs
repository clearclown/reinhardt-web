//! Database backend integration tests

use crate::common::{database_helpers::*, init_test_logging, random_test_key, test_config_value};
use reinhardt_settings::dynamic::DynamicBackend;
use testcontainers::clients;

#[tokio::test]
async fn test_sqlite_backend() {
    init_test_logging();

    let backend = create_sqlite_backend().await;

    let key = random_test_key();
    let value = test_config_value("sqlite_test");

    // Set and get value
    backend
        .set(&key, value.clone())
        .await
        .expect("Failed to set value");
    let retrieved = backend.get(&key).await.expect("Failed to get value");

    assert_eq!(retrieved, Some(value));
}

#[tokio::test]
async fn test_postgres_backend() {
    init_test_logging();

    let docker = clients::Cli::default();
    let postgres_container = start_postgres(&docker);
    let backend = create_database_backend(&postgres_container.url).await;

    let key = random_test_key();
    let value = test_config_value("postgres_test");

    // Set and get value
    backend
        .set(&key, value.clone())
        .await
        .expect("Failed to set value");
    let retrieved = backend.get(&key).await.expect("Failed to get value");

    assert_eq!(retrieved, Some(value));
}

#[tokio::test]
async fn test_database_set_get() {
    init_test_logging();

    let backend = create_sqlite_backend().await;

    let key = random_test_key();
    let value = test_config_value("hello_database");

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
async fn test_database_update() {
    init_test_logging();

    let backend = create_sqlite_backend().await;

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
async fn test_database_delete() {
    init_test_logging();

    let backend = create_sqlite_backend().await;

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
async fn test_database_list() {
    init_test_logging();

    let backend = create_sqlite_backend().await;

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
async fn test_database_transactions() {
    init_test_logging();

    let backend = create_sqlite_backend().await;

    let keys = vec![random_test_key(), random_test_key(), random_test_key()];
    let value = test_config_value("transaction_test");

    // Perform multiple operations
    for key in &keys {
        backend
            .set(key, value.clone())
            .await
            .expect("Failed to set value");
    }

    // Verify all values were set
    for key in &keys {
        let retrieved = backend.get(key).await.expect("Failed to get value");
        assert_eq!(retrieved, Some(value.clone()));
    }
}

#[tokio::test]
async fn test_database_concurrent_writes() {
    init_test_logging();

    let backend = create_sqlite_backend().await;

    let key = random_test_key();
    let mut handles = vec![];

    // Spawn multiple tasks writing to different keys
    for i in 0..10 {
        let backend_clone = create_sqlite_backend().await;
        let test_key = format!("{}_{}", key, i);

        let handle = tokio::spawn(async move {
            let value = test_config_value(&format!("concurrent_{}", i));
            backend_clone
                .set(&test_key, value)
                .await
                .expect("Failed to set value");
        });

        handles.push(handle);
    }

    // Wait for all tasks to complete
    for handle in handles {
        handle.await.expect("Task panicked");
    }

    // Verify all values exist
    for i in 0..10 {
        let test_key = format!("{}_{}", key, i);
        let retrieved = backend.get(&test_key).await.expect("Failed to get value");
        assert!(retrieved.is_some());
    }
}

#[tokio::test]
async fn test_database_large_dataset() {
    init_test_logging();

    let backend = create_sqlite_backend().await;

    // Insert 100 key-value pairs
    for i in 0..100 {
        let key = format!("large_dataset_key_{}", i);
        let value = test_config_value(&format!("value_{}", i));
        backend.set(&key, value).await.expect("Failed to set value");
    }

    // Verify count
    let all_keys = backend.list().await.expect("Failed to list keys");
    assert!(
        all_keys.len() >= 100,
        "Expected at least 100 keys, found {}",
        all_keys.len()
    );

    // Verify random access
    let key = "large_dataset_key_42";
    let retrieved = backend.get(key).await.expect("Failed to get value");
    assert!(retrieved.is_some());
}

#[tokio::test]
async fn test_database_json_values() {
    init_test_logging();

    let backend = create_sqlite_backend().await;

    let key = random_test_key();
    let complex_value = serde_json::json!({
        "nested": {
            "array": [1, 2, 3],
            "object": {
                "key": "value"
            }
        },
        "number": 42,
        "boolean": true,
        "null": null
    });

    // Set complex JSON value
    backend
        .set(&key, complex_value.clone())
        .await
        .expect("Failed to set complex value");

    // Get and verify
    let retrieved = backend.get(&key).await.expect("Failed to get value");
    assert_eq!(retrieved, Some(complex_value));
}
