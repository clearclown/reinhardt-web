//! Integration tests for Custom Views and Drag-and-Drop Reordering
//!
//! Tests custom view registration, rendering, and model reordering functionality

use reinhardt_admin::{
    CustomView, CustomViewRegistry, DragDropConfig, ReorderHandler, ReorderResult,
    ReorderableModel, ViewConfig,
};
use async_trait::async_trait;
use serde_json::json;
use std::collections::HashMap;

/// Simple test model for reordering
#[derive(Debug, Clone)]
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

    async fn get_id(&self) -> String {
        self.id.clone()
    }
}

/// Simple custom view for testing
struct DashboardView;

#[async_trait]
impl CustomView for DashboardView {
    fn config(&self) -> &ViewConfig {
        static CONFIG: once_cell::sync::Lazy<ViewConfig> =
            once_cell::sync::Lazy::new(|| {
                ViewConfig::builder()
                    .path("/dashboard")
                    .name("Dashboard")
                    .build()
            });
        &CONFIG
    }

    async fn render(&self, _context: &HashMap<String, String>) -> Result<String, String> {
        Ok("<h1>Custom Dashboard</h1>".to_string())
    }

    async fn has_permission(&self, _permissions: &[String]) -> bool {
        true // Allow all for testing
    }
}

/// Test: Register and find custom view
#[tokio::test]
async fn test_custom_view_registration_and_lookup() {
    let mut registry = CustomViewRegistry::new();

    // Register view
    let view = Box::new(DashboardView);
    registry.register(view).expect("Registration should succeed");

    // Find by path
    let found = registry.find_by_path("/dashboard");
    assert!(found.is_some());

    // Render the view
    let context = HashMap::new();
    let html = found
        .unwrap()
        .render(&context)
        .await
        .expect("Render should succeed");
    assert!(html.contains("Custom Dashboard"));
}

/// Test: Multiple custom views with different paths
#[tokio::test]
async fn test_multiple_custom_views() {
    struct ReportsView;

    #[async_trait]
    impl CustomView for ReportsView {
        fn config(&self) -> &ViewConfig {
            static CONFIG: once_cell::sync::Lazy<ViewConfig> =
                once_cell::sync::Lazy::new(|| {
                    ViewConfig::builder()
                        .path("/reports")
                        .name("Reports")
                        .template(Some("/templates/reports.html"))
                        .build()
                });
            &CONFIG
        }

        async fn render(&self, _context: &HashMap<String, String>) -> Result<String, String> {
            Ok("<h1>Reports</h1>".to_string())
        }

        async fn has_permission(&self, _permissions: &[String]) -> bool {
            true
        }
    }

    let mut registry = CustomViewRegistry::new();

    // Register multiple views
    registry
        .register(Box::new(DashboardView))
        .expect("Dashboard registration");
    registry
        .register(Box::new(ReportsView))
        .expect("Reports registration");

    // Verify both are registered
    assert_eq!(registry.views().len(), 2);

    // Find each view
    assert!(registry.find_by_path("/dashboard").is_some());
    assert!(registry.find_by_path("/reports").is_some());
    assert!(registry.find_by_path("/nonexistent").is_none());
}

/// Test: Basic item reordering
#[tokio::test]
async fn test_basic_reordering() {
    let config = DragDropConfig::new("order");
    let handler = ReorderHandler::new(config);

    // Create test items
    let mut items = vec![
        TestItem {
            id: "1".to_string(),
            name: "Item 1".to_string(),
            order: 0,
        },
        TestItem {
            id: "2".to_string(),
            name: "Item 2".to_string(),
            order: 1,
        },
        TestItem {
            id: "3".to_string(),
            name: "Item 3".to_string(),
            order: 2,
        },
    ];

    // Reorder: [0, 1, 2] → [2, 0, 1]
    let new_orders = vec![2, 0, 1];
    let result = handler.process_reorder(&mut items, new_orders).await;

    assert!(result.is_ok());
    let reorder_result = result.unwrap();
    assert!(reorder_result.success);
    assert_eq!(reorder_result.updated_count, 3);

    // Verify new order
    assert_eq!(items[0].order, 2);
    assert_eq!(items[1].order, 0);
    assert_eq!(items[2].order, 1);
}

/// Test: Reorder validation catches invalid orders
#[tokio::test]
async fn test_reorder_validation_errors() {
    let config = DragDropConfig::new("order");
    let handler = ReorderHandler::new(config);

    let mut items = vec![
        TestItem {
            id: "1".to_string(),
            name: "Item 1".to_string(),
            order: 0,
        },
        TestItem {
            id: "2".to_string(),
            name: "Item 2".to_string(),
            order: 1,
        },
    ];

    // Test 1: Negative order
    let invalid_orders = vec![-1, 1];
    let result = handler.process_reorder(&mut items, invalid_orders).await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Order values must be non-negative"));

    // Test 2: Duplicate orders
    let duplicate_orders = vec![0, 0];
    let result = handler.process_reorder(&mut items, duplicate_orders).await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Duplicate order values found"));

    // Test 3: Gap in sequence
    let gap_orders = vec![0, 2]; // Missing 1
    let result = handler.process_reorder(&mut items, gap_orders).await;
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .contains("Order values must be continuous"));
}

/// Test: Adjacent item reordering
#[tokio::test]
async fn test_adjacent_reordering() {
    let config = DragDropConfig::new("order");
    let handler = ReorderHandler::new(config);

    let mut items = vec![
        TestItem {
            id: "1".to_string(),
            name: "First".to_string(),
            order: 0,
        },
        TestItem {
            id: "2".to_string(),
            name: "Second".to_string(),
            order: 1,
        },
        TestItem {
            id: "3".to_string(),
            name: "Third".to_string(),
            order: 2,
        },
    ];

    // Swap first and second items
    let result = handler.reorder_adjacent(&mut items, 0, 1).await;
    assert!(result.is_ok());

    let reorder_result = result.unwrap();
    assert!(reorder_result.success);
    assert_eq!(reorder_result.updated_count, 2);

    // Verify swap
    assert_eq!(items[0].name, "First");
    assert_eq!(items[0].order, 1); // Swapped
    assert_eq!(items[1].name, "Second");
    assert_eq!(items[1].order, 0); // Swapped
    assert_eq!(items[2].order, 2); // Unchanged
}

/// Test: Reordering with custom view integration
#[tokio::test]
async fn test_reordering_with_custom_view() {
    // Custom view that manages reorderable items
    struct ItemManagerView {
        items: tokio::sync::Mutex<Vec<TestItem>>,
    }

    impl ItemManagerView {
        fn new() -> Self {
            let items = vec![
                TestItem {
                    id: "a".to_string(),
                    name: "Alpha".to_string(),
                    order: 0,
                },
                TestItem {
                    id: "b".to_string(),
                    name: "Beta".to_string(),
                    order: 1,
                },
                TestItem {
                    id: "c".to_string(),
                    name: "Gamma".to_string(),
                    order: 2,
                },
            ];
            Self {
                items: tokio::sync::Mutex::new(items),
            }
        }

        async fn reorder_items(&self, new_orders: Vec<i32>) -> ReorderResult {
            let config = DragDropConfig::new("order");
            let handler = ReorderHandler::new(config);
            let mut items = self.items.lock().await;
            handler
                .process_reorder(&mut *items, new_orders)
                .await
                .unwrap_or_else(|e| ReorderResult {
                    updated_count: 0,
                    success: false,
                    error: Some(e),
                })
        }
    }

    #[async_trait]
    impl CustomView for ItemManagerView {
        fn config(&self) -> &ViewConfig {
            static CONFIG: once_cell::sync::Lazy<ViewConfig> =
                once_cell::sync::Lazy::new(|| {
                    ViewConfig::builder()
                        .path("/items/manage")
                        .name("Item Manager")
                        .build()
                });
            &CONFIG
        }

        async fn render(&self, _context: &HashMap<String, String>) -> Result<String, String> {
            let items = self.items.lock().await;
            let mut html = String::from("<ul>");
            for item in items.iter() {
                html.push_str(&format!(
                    "<li data-order='{}'>{}</li>",
                    item.order, item.name
                ));
            }
            html.push_str("</ul>");
            Ok(html)
        }

        async fn has_permission(&self, _permissions: &[String]) -> bool {
            true
        }
    }

    // Test the integrated view
    let view = ItemManagerView::new();

    // Initial render
    let context = HashMap::new();
    let initial_html = view.render(&context).await.expect("Initial render");
    assert!(initial_html.contains("Alpha"));
    assert!(initial_html.contains("data-order='0'"));

    // Reorder: [0, 1, 2] → [2, 0, 1]
    let result = view.reorder_items(vec![2, 0, 1]).await;
    assert!(result.success);
    assert_eq!(result.updated_count, 3);

    // Render after reorder
    let after_html = view.render(&context).await.expect("After render");
    assert!(after_html.contains("data-order='2'"));
    assert!(after_html.contains("data-order='0'"));
    assert!(after_html.contains("data-order='1'"));
}

/// Test: Disabled drag-drop configuration
#[tokio::test]
async fn test_drag_drop_disabled() {
    let config = DragDropConfig::builder()
        .order_field("order")
        .enabled(false)
        .build();

    let handler = ReorderHandler::new(config);

    let mut items = vec![TestItem {
        id: "1".to_string(),
        name: "Item".to_string(),
        order: 0,
    }];

    let result = handler.process_reorder(&mut items, vec![0]).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("Reordering is disabled"));
}

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
        .custom_js(Some(custom_js.to_string()))
        .build();

    assert_eq!(config.order_field(), "position");
    assert!(config.custom_js().is_some());
    assert!(config.custom_js().unwrap().contains("dragend"));
}

/// Test: Permission-based custom view access
#[tokio::test]
async fn test_permission_based_view_access() {
    struct AdminOnlyView;

    #[async_trait]
    impl CustomView for AdminOnlyView {
        fn config(&self) -> &ViewConfig {
            static CONFIG: once_cell::sync::Lazy<ViewConfig> =
                once_cell::sync::Lazy::new(|| {
                    ViewConfig::builder()
                        .path("/admin/settings")
                        .name("Admin Settings")
                        .required_permission(Some("admin.settings.view"))
                        .build()
                });
            &CONFIG
        }

        async fn render(&self, _context: &HashMap<String, String>) -> Result<String, String> {
            Ok("<h1>Admin Settings</h1>".to_string())
        }

        async fn has_permission(&self, permissions: &[String]) -> bool {
            permissions.contains(&"admin.settings.view".to_string())
        }
    }

    let view = AdminOnlyView;

    // Test with admin permissions
    let admin_perms = vec!["admin.settings.view".to_string()];
    assert!(view.has_permission(&admin_perms).await);

    // Test without admin permissions
    let user_perms = vec!["user.profile.view".to_string()];
    assert!(!view.has_permission(&user_perms).await);
}

/// Test: Large-scale reordering performance
#[tokio::test]
async fn test_large_scale_reordering() {
    let config = DragDropConfig::new("order");
    let handler = ReorderHandler::new(config);

    // Create 100 items
    let mut items: Vec<TestItem> = (0..100)
        .map(|i| TestItem {
            id: i.to_string(),
            name: format!("Item {}", i),
            order: i,
        })
        .collect();

    // Reverse order: [0..99] → [99..0]
    let new_orders: Vec<i32> = (0..100).rev().collect();

    let start = std::time::Instant::now();
    let result = handler.process_reorder(&mut items, new_orders).await;
    let duration = start.elapsed();

    assert!(result.is_ok());
    let reorder_result = result.unwrap();
    assert!(reorder_result.success);
    assert_eq!(reorder_result.updated_count, 100);

    // Performance check: should complete in under 100ms
    assert!(
        duration.as_millis() < 100,
        "Reordering 100 items took too long: {:?}",
        duration
    );

    // Verify order is reversed
    assert_eq!(items[0].order, 99);
    assert_eq!(items[99].order, 0);
}

/// Test: Reordering with context data in custom view
#[tokio::test]
async fn test_custom_view_with_reorder_context() {
    struct CategoryView;

    #[async_trait]
    impl CustomView for CategoryView {
        fn config(&self) -> &ViewConfig {
            static CONFIG: once_cell::sync::Lazy<ViewConfig> =
                once_cell::sync::Lazy::new(|| {
                    ViewConfig::builder()
                        .path("/categories")
                        .name("Categories")
                        .build()
                });
            &CONFIG
        }

        async fn render(&self, context: &HashMap<String, String>) -> Result<String, String> {
            let reorder_enabled = context
                .get("reorder_enabled")
                .map(|v| v == "true")
                .unwrap_or(false);

            if reorder_enabled {
                Ok("<div class='reorderable'>Categories with drag-drop</div>".to_string())
            } else {
                Ok("<div>Categories (read-only)</div>".to_string())
            }
        }

        async fn has_permission(&self, _permissions: &[String]) -> bool {
            true
        }
    }

    let view = CategoryView;

    // Context with reordering enabled
    let mut context_enabled = HashMap::new();
    context_enabled.insert("reorder_enabled".to_string(), "true".to_string());

    let html_enabled = view
        .render(&context_enabled)
        .await
        .expect("Render with reorder");
    assert!(html_enabled.contains("reorderable"));
    assert!(html_enabled.contains("drag-drop"));

    // Context with reordering disabled
    let mut context_disabled = HashMap::new();
    context_disabled.insert("reorder_enabled".to_string(), "false".to_string());

    let html_disabled = view
        .render(&context_disabled)
        .await
        .expect("Render without reorder");
    assert!(html_disabled.contains("read-only"));
    assert!(!html_disabled.contains("drag-drop"));
}
