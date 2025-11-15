//! Integration tests for Custom Views and Drag-and-Drop Reordering
//!
//! Tests custom view registration, rendering, and model reordering functionality

use async_trait::async_trait;
use reinhardt_admin_panel::{
	CustomView, CustomViewRegistry, DragDropConfig, ReorderableModel, ViewConfig,
};
use std::collections::HashMap;

/// Simple test model for reordering
#[derive(Debug, Clone)]
#[allow(dead_code)]
#[allow(dead_code)]
struct TestItem {
	id: String,
	name: String,
	order: i32,
}

#[async_trait]
impl ReorderableModel for TestItem {
	async fn get_order(&self) -> i32 {
		self.order
	}

	async fn set_order(&mut self, new_order: i32) {
		self.order = new_order;
	}

	fn get_id(&self) -> String {
		self.id.clone()
	}
}

/// Simple custom view for testing
struct DashboardView;

#[async_trait]
impl CustomView for DashboardView {
	fn config(&self) -> ViewConfig {
		ViewConfig::builder()
			.path("/dashboard")
			.name("Dashboard")
			.build()
	}

	async fn render(&self, _context: HashMap<String, String>) -> String {
		"<h1>Custom Dashboard</h1>".to_string()
	}

	async fn has_permission(&self, _user: &(dyn std::any::Any + Send + Sync)) -> bool {
		true // Allow all for testing
	}
}

/// Test: Register and find custom view
#[tokio::test]
async fn test_custom_view_registration_and_lookup() {
	let mut registry = CustomViewRegistry::new();

	// Register view
	let view = Box::new(DashboardView);
	registry.register(view);

	// Find by path
	let found = registry.find_by_path("/dashboard");
	assert!(found.is_some());

	// Render the view
	let context = HashMap::new();
	let html = found.unwrap().render(context).await;
	assert!(html.contains("Custom Dashboard"));
}

/// Test: Multiple custom views with different paths
#[tokio::test]
async fn test_multiple_custom_views() {
	struct ReportsView;

	#[async_trait]
	impl CustomView for ReportsView {
		fn config(&self) -> ViewConfig {
			ViewConfig::builder()
				.path("/reports")
				.name("Reports")
				.template("/templates/reports.html")
				.build()
		}

		async fn render(&self, _context: HashMap<String, String>) -> String {
			"<h1>Reports</h1>".to_string()
		}

		async fn has_permission(&self, _user: &(dyn std::any::Any + Send + Sync)) -> bool {
			true
		}
	}

	let mut registry = CustomViewRegistry::new();

	// Register multiple views
	registry.register(Box::new(DashboardView));
	registry.register(Box::new(ReportsView));

	// Verify both are registered
	assert_eq!(registry.views().len(), 2);

	// Find each view
	assert!(registry.find_by_path("/dashboard").is_some());
	assert!(registry.find_by_path("/reports").is_some());
	assert!(registry.find_by_path("/nonexistent").is_none());
}

// TODO: ReorderHandler now requires database connection - needs refactoring
// ReorderHandler::new() signature changed to require:
// - config: DragDropConfig
// - connection: Arc<DatabaseConnection>
// - table_name: impl Into<String>
// - id_field: impl Into<String>
// process_reorder() now takes Vec<(String, i32)> and updates database directly
//
// /// Test: Basic item reordering
// #[tokio::test]
// async fn test_basic_reordering() {
// 	let config = DragDropConfig::new("order");
// 	let handler = ReorderHandler::new(config);
//
// 	// Create test items
// 	let mut items = vec![
// 		TestItem {
// 			id: "1".to_string(),
// 			name: "Item 1".to_string(),
// 			order: 0,
// 		},
// 		TestItem {
// 			id: "2".to_string(),
// 			name: "Item 2".to_string(),
// 			order: 1,
// 		},
// 		TestItem {
// 			id: "3".to_string(),
// 			name: "Item 3".to_string(),
// 			order: 2,
// 		},
// 	];
//
// 	// Reorder: [0, 1, 2] → [2, 0, 1]
// 	let new_orders = vec![2, 0, 1];
// 	let result = handler.process_reorder(&mut items, new_orders).await;
//
// 	assert!(result.is_ok());
// 	let reorder_result = result.unwrap();
// 	assert!(reorder_result.success);
// 	assert_eq!(reorder_result.updated_count, 3);
//
// 	// Verify new order
// 	assert_eq!(items[0].order, 2);
// 	assert_eq!(items[1].order, 0);
// 	assert_eq!(items[2].order, 1);
// }

// TODO: ReorderHandler now requires database connection - needs refactoring
// /// Test: Reorder validation catches invalid orders
// #[tokio::test]
// async fn test_reorder_validation_errors() {
// 	let config = DragDropConfig::new("order");
// 	let handler = ReorderHandler::new(config);
//
// 	let mut items = vec![
// 		TestItem {
// 			id: "1".to_string(),
// 			name: "Item 1".to_string(),
// 			order: 0,
// 		},
// 		TestItem {
// 			id: "2".to_string(),
// 			name: "Item 2".to_string(),
// 			order: 1,
// 		},
// 	];
//
// 	// Test 1: Negative order
// 	let invalid_orders = vec![-1, 1];
// 	let result = handler.process_reorder(&mut items, invalid_orders).await;
// 	assert!(result.is_err());
// 	assert!(
// 		result
// 			.unwrap_err()
// 			.contains("Order values must be non-negative")
// 	);
//
// 	// Test 2: Duplicate orders
// 	let duplicate_orders = vec![0, 0];
// 	let result = handler.process_reorder(&mut items, duplicate_orders).await;
// 	assert!(result.is_err());
// 	assert!(result.unwrap_err().contains("Duplicate order values found"));
//
// 	// Test 3: Gap in sequence
// 	let gap_orders = vec![0, 2]; // Missing 1
// 	let result = handler.process_reorder(&mut items, gap_orders).await;
// 	assert!(result.is_err());
// 	assert!(
// 		result
// 			.unwrap_err()
// 			.contains("Order values must be continuous")
// 	);
// }

// TODO: ReorderHandler now requires database connection - needs refactoring
// /// Test: Adjacent item reordering
// #[tokio::test]
// async fn test_adjacent_reordering() {
// 	let config = DragDropConfig::new("order");
// 	let handler = ReorderHandler::new(config);
//
// 	let mut items = vec![
// 		TestItem {
// 			id: "1".to_string(),
// 			name: "First".to_string(),
// 			order: 0,
// 		},
// 		TestItem {
// 			id: "2".to_string(),
// 			name: "Second".to_string(),
// 			order: 1,
// 		},
// 		TestItem {
// 			id: "3".to_string(),
// 			name: "Third".to_string(),
// 			order: 2,
// 		},
// 	];
//
// 	// Swap first and second items
// 	let result = handler.reorder_adjacent(&mut items, 0, 1).await;
// 	assert!(result.is_ok());
//
// 	let reorder_result = result.unwrap();
// 	assert!(reorder_result.success);
// 	assert_eq!(reorder_result.updated_count, 2);
//
// 	// Verify swap
// 	assert_eq!(items[0].name, "First");
// 	assert_eq!(items[0].order, 1); // Swapped
// 	assert_eq!(items[1].name, "Second");
// 	assert_eq!(items[1].order, 0); // Swapped
// 	assert_eq!(items[2].order, 2); // Unchanged
// }

// TODO: ReorderHandler now requires database connection - needs refactoring
// /// Test: Reordering with custom view integration
// #[tokio::test]
// async fn test_reordering_with_custom_view() {
// 	// Custom view that manages reorderable items
// 	struct ItemManagerView {
// 		items: tokio::sync::Mutex<Vec<TestItem>>,
// 	}
//
// 	impl ItemManagerView {
// 		fn new() -> Self {
// 			let items = vec![
// 				TestItem {
// 					id: "a".to_string(),
// 					name: "Alpha".to_string(),
// 					order: 0,
// 				},
// 				TestItem {
// 					id: "b".to_string(),
// 					name: "Beta".to_string(),
// 					order: 1,
// 				},
// 				TestItem {
// 					id: "c".to_string(),
// 					name: "Gamma".to_string(),
// 					order: 2,
// 				},
// 			];
// 			Self {
// 				items: tokio::sync::Mutex::new(items),
// 			}
// 		}
//
// 		async fn reorder_items(&self, new_orders: Vec<i32>) -> ReorderResult {
// 			let config = DragDropConfig::new("order");
// 			let handler = ReorderHandler::new(config);
// 			let mut items = self.items.lock().await;
// 			handler
// 				.process_reorder(&mut *items, new_orders)
// 				.await
// 				.unwrap_or_else(|e| ReorderResult {
// 					updated_count: 0,
// 					success: false,
// 					error: Some(e),
// 				})
// 		}
// 	}
//
// 	#[async_trait]
// 	impl CustomView for ItemManagerView {
// 		fn config(&self) -> ViewConfig {
// 			ViewConfig::builder()
// 				.path("/items/manage")
// 				.name("Item Manager")
// 				.build()
// 		}
//
// 		async fn render(&self, _context: HashMap<String, String>) -> String {
// 			let items = self.items.lock().await;
// 			let mut html = String::from("<ul>");
// 			for item in items.iter() {
// 				html.push_str(&format!(
// 					"<li data-order='{}'>{}</li>",
// 					item.order, item.name
// 				));
// 			}
// 			html.push_str("</ul>");
// 			html
// 		}
//
// 		async fn has_permission(&self, _user: &(dyn std::any::Any + Send + Sync)) -> bool {
// 			true
// 		}
// 	}
//
// 	// Test the integrated view
// 	let view = ItemManagerView::new();
//
// 	// Initial render
// 	let context = HashMap::new();
// 	let initial_html = view.render(context.clone()).await;
// 	assert!(initial_html.contains("Alpha"));
// 	assert!(initial_html.contains("data-order='0'"));
//
// 	// Reorder: [0, 1, 2] → [2, 0, 1]
// 	let result = view.reorder_items(vec![2, 0, 1]).await;
// 	assert!(result.success);
// 	assert_eq!(result.updated_count, 3);
//
// 	// Render after reorder
// 	let after_html = view.render(context).await;
// 	assert!(after_html.contains("data-order='2'"));
// 	assert!(after_html.contains("data-order='0'"));
// 	assert!(after_html.contains("data-order='1'"));
// }

// TODO: ReorderHandler now requires database connection - needs refactoring
// /// Test: Disabled drag-drop configuration
// #[tokio::test]
// async fn test_drag_drop_disabled() {
// 	let config = DragDropConfig::builder()
// 		.order_field("order")
// 		.enabled(false)
// 		.build();
//
// 	let handler = ReorderHandler::new(config);
//
// 	let mut items = vec![TestItem {
// 		id: "1".to_string(),
// 		name: "Item".to_string(),
// 		order: 0,
// 	}];
//
// 	let result = handler.process_reorder(&mut items, vec![0]).await;
// 	assert!(result.is_err());
// 	assert!(result.unwrap_err().contains("Reordering is disabled"));
// }

/// Test: Custom JavaScript integration
#[tokio::test]
async fn test_custom_js_configuration() {
	let custom_js = r#"
        document.addEventListener('dragend', function(e) {
            updateOrder(e.target.dataset.id);
        });
    "#;

	let config = DragDropConfig::builder()
		.order_field("position")
		.custom_js(custom_js.to_string())
		.build();

	assert_eq!(config.order_field, "position");
	assert!(config.custom_js.is_some());
	assert!(config.custom_js.as_ref().unwrap().contains("dragend"));
}

/// Test: Permission-based custom view access
#[tokio::test]
async fn test_permission_based_view_access() {
	struct AdminOnlyView;

	#[async_trait]
	impl CustomView for AdminOnlyView {
		fn config(&self) -> ViewConfig {
			ViewConfig::builder()
				.path("/admin/settings")
				.name("Admin Settings")
				.permission("admin.settings.view")
				.build()
		}

		async fn render(&self, _context: HashMap<String, String>) -> String {
			"<h1>Admin Settings</h1>".to_string()
		}

		async fn has_permission(&self, user: &(dyn std::any::Any + Send + Sync)) -> bool {
			use reinhardt_auth::{SimpleUser, User};

			// For testing: check if user has admin.settings.view permission
			if let Some(simple_user) = user.downcast_ref::<SimpleUser>() {
				// In a real implementation, check user permissions
				// For testing, just check if user is staff
				simple_user.is_staff()
			} else {
				false
			}
		}
	}

	let view = AdminOnlyView;

	// Test with admin user (staff)
	let admin_user = reinhardt_auth::SimpleUser {
		id: uuid::Uuid::new_v4(),
		username: "admin".to_string(),
		email: "admin@example.com".to_string(),
		is_active: true,
		is_admin: true,
		is_staff: true, // Required for has_permission to return true
		is_superuser: true,
	};
	assert!(
		view.has_permission(&admin_user as &(dyn std::any::Any + Send + Sync))
			.await
	);

	// Test with regular user (not staff)
	let regular_user = reinhardt_auth::SimpleUser {
		id: uuid::Uuid::new_v4(),
		username: "user".to_string(),
		email: "user@example.com".to_string(),
		is_active: true,
		is_admin: false,
		is_staff: false, // Not staff, should fail permission check
		is_superuser: false,
	};
	assert!(
		!view
			.has_permission(&regular_user as &(dyn std::any::Any + Send + Sync))
			.await
	);
}

// TODO: ReorderHandler now requires database connection - needs refactoring
// /// Test: Large-scale reordering performance
// #[tokio::test]
// async fn test_large_scale_reordering() {
// 	let config = DragDropConfig::new("order");
// 	let handler = ReorderHandler::new(config);
//
// 	// Create 100 items
// 	let mut items: Vec<TestItem> = (0..100)
// 		.map(|i| TestItem {
// 			id: i.to_string(),
// 			name: format!("Item {}", i),
// 			order: i,
// 		})
// 		.collect();
//
// 	// Reverse order: [0..99] → [99..0]
// 	let new_orders: Vec<i32> = (0..100).rev().collect();
//
// 	let start = std::time::Instant::now();
// 	let result = handler.process_reorder(&mut items, new_orders).await;
// 	let duration = start.elapsed();
//
// 	assert!(result.is_ok());
// 	let reorder_result = result.unwrap();
// 	assert!(reorder_result.success);
// 	assert_eq!(reorder_result.updated_count, 100);
//
// 	// Performance check: should complete in under 100ms
// 	assert!(
// 		duration.as_millis() < 100,
// 		"Reordering 100 items took too long: {:?}",
// 		duration
// 	);
//
// 	// Verify order is reversed
// 	assert_eq!(items[0].order, 99);
// 	assert_eq!(items[99].order, 0);
// }

/// Test: Reordering with context data in custom view
#[tokio::test]
async fn test_custom_view_with_reorder_context() {
	struct CategoryView;

	#[async_trait]
	impl CustomView for CategoryView {
		fn config(&self) -> ViewConfig {
			ViewConfig::builder()
				.path("/categories")
				.name("Categories")
				.build()
		}

		async fn render(&self, context: HashMap<String, String>) -> String {
			let reorder_enabled = context
				.get("reorder_enabled")
				.map(|v| v == "true")
				.unwrap_or(false);

			if reorder_enabled {
				"<div class='reorderable'>Categories with drag-drop</div>".to_string()
			} else {
				"<div>Categories (read-only)</div>".to_string()
			}
		}

		async fn has_permission(&self, _user: &(dyn std::any::Any + Send + Sync)) -> bool {
			true
		}
	}

	let view = CategoryView;

	// Context with reordering enabled
	let mut context_enabled = HashMap::new();
	context_enabled.insert("reorder_enabled".to_string(), "true".to_string());

	let html_enabled = view.render(context_enabled).await;
	assert!(html_enabled.contains("reorderable"));
	assert!(html_enabled.contains("drag-drop"));

	// Context with reordering disabled
	let mut context_disabled = HashMap::new();
	context_disabled.insert("reorder_enabled".to_string(), "false".to_string());

	let html_disabled = view.render(context_disabled).await;
	assert!(html_disabled.contains("read-only"));
	assert!(!html_disabled.contains("drag-drop"));
}
