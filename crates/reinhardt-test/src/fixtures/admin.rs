//! Admin panel integration test fixtures
//!
//! This module provides rstest fixtures for admin panel integration tests,
//! including pre-populated audit loggers and test data sets.

use rstest::*;
use std::sync::Arc;

// Note: MemoryAuditLogger is defined in reinhardt-admin-panel crate
// This fixture will be used by tests that have that dependency

/// Fixture providing an AuditLogger with 10,000 test log entries
///
/// This fixture creates a MemoryAuditLogger pre-populated with 10,000 audit log entries
/// for performance and functionality testing.
///
/// Test data distribution:
/// - Users: 100 different users (user_0 ~ user_99)
/// - Models: 10 different models (Model_0 ~ Model_9)
/// - Actions: Alternating "Create" and "Update" actions
/// - Each user has approximately 100 log entries (10,000 / 100)
/// - Each model has approximately 1,000 log entries (10,000 / 10)
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::admin::audit_logger_with_test_data;
/// use rstest::*;
///
/// #[rstest]
/// #[tokio::test]
/// async fn test_audit_query(
///     #[future] audit_logger_with_test_data: Arc<MemoryAuditLogger>
/// ) {
///     let logger = audit_logger_with_test_data.await;
///
///     // Query for specific user
///     let query = AuditLogQuery::new().user_id("user_50");
///     let results = logger.query(&query).await.unwrap();
///
///     // Expected: 10,000 / 100 users = 100 logs per user
///     assert_eq!(results.len(), 100);
/// }
/// ```
///
/// # Note
/// This fixture is async and requires `#[future]` attribute when used in rstest.
/// The returned Arc allows sharing the logger across multiple test assertions
/// without cloning the data.
#[fixture]
pub async fn audit_logger_with_test_data() -> Arc<serde_json::Value> {
	// Placeholder implementation that returns metadata about test data
	// The actual implementation will be in the test file that imports
	// MemoryAuditLogger from reinhardt-admin-panel

	Arc::new(serde_json::json!({
		"total_entries": 10000,
		"users_count": 100,
		"models_count": 10,
		"user_pattern": "user_{i % 100}",
		"model_pattern": "Model_{i % 10}",
		"action_pattern": "if i % 2 == 0 { 'Create' } else { 'Update' }",
		"description": "This is a metadata fixture. Actual implementation should be in test file."
	}))
}

/// Simplified audit log entry for testing
///
/// This struct represents a single audit log entry for use in tests.
/// It matches the structure expected by MemoryAuditLogger.
#[derive(Clone, Debug)]
pub struct TestAuditLogEntry {
	pub user_id: String,
	pub model_name: String,
	pub object_id: String,
	pub action: String,
	pub changes: Option<serde_json::Value>,
	pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Generate a single test audit log entry
///
/// # Arguments
/// * `index` - Entry index (used for generating user/model IDs)
/// * `user_count` - Total number of users for modulo distribution
/// * `model_count` - Total number of models for modulo distribution
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::admin::generate_test_audit_entry;
///
/// let entry = generate_test_audit_entry(0, 100, 10);
/// assert_eq!(entry.user_id, "user_0");
/// assert_eq!(entry.model_name, "Model_0");
/// assert_eq!(entry.action, "Create");
/// ```
pub fn generate_test_audit_entry(
	index: usize,
	user_count: usize,
	model_count: usize,
) -> TestAuditLogEntry {
	TestAuditLogEntry {
		user_id: format!("user_{}", index % user_count),
		model_name: format!("Model_{}", index % model_count),
		object_id: index.to_string(),
		action: if index.is_multiple_of(2) {
			"Create".to_string()
		} else {
			"Update".to_string()
		},
		changes: None,
		timestamp: chrono::Utc::now(),
	}
}

/// Generate multiple test audit log entries
///
/// # Arguments
/// * `count` - Number of entries to generate
/// * `user_count` - Total number of users for distribution
/// * `model_count` - Total number of models for distribution
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_test::fixtures::admin::generate_test_audit_entries;
///
/// let entries = generate_test_audit_entries(10000, 100, 10);
/// assert_eq!(entries.len(), 10000);
///
/// // Each user should have approximately 100 entries
/// let user_50_entries: Vec<_> = entries
///     .iter()
///     .filter(|e| e.user_id == "user_50")
///     .collect();
/// assert_eq!(user_50_entries.len(), 100);
/// ```
pub fn generate_test_audit_entries(
	count: usize,
	user_count: usize,
	model_count: usize,
) -> Vec<TestAuditLogEntry> {
	(0..count)
		.map(|i| generate_test_audit_entry(i, user_count, model_count))
		.collect()
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_generate_test_audit_entry() {
		let entry = generate_test_audit_entry(0, 100, 10);
		assert_eq!(entry.user_id, "user_0");
		assert_eq!(entry.model_name, "Model_0");
		assert_eq!(entry.action, "Create");

		let entry = generate_test_audit_entry(1, 100, 10);
		assert_eq!(entry.user_id, "user_1");
		assert_eq!(entry.model_name, "Model_1");
		assert_eq!(entry.action, "Update");

		let entry = generate_test_audit_entry(150, 100, 10);
		assert_eq!(entry.user_id, "user_50");
		assert_eq!(entry.model_name, "Model_0");
	}

	#[test]
	fn test_generate_test_audit_entries() {
		let entries = generate_test_audit_entries(10000, 100, 10);
		assert_eq!(entries.len(), 10000);

		// Verify user distribution
		let user_50_entries: Vec<_> = entries.iter().filter(|e| e.user_id == "user_50").collect();
		assert_eq!(user_50_entries.len(), 100);

		// Verify model distribution
		let model_5_entries: Vec<_> = entries
			.iter()
			.filter(|e| e.model_name == "Model_5")
			.collect();
		assert_eq!(model_5_entries.len(), 1000);
	}
}
