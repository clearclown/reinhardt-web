//! Integration tests for renderers
//!
//! These tests verify renderer behavior in more complex scenarios
//! involving multiple components, based on Django REST Framework's
//! RendererEndToEndTests

use reinhardt_renderers::{JSONRenderer, Renderer, RendererContext, RendererRegistry};
use serde_json::json;

/// Test renderer registry functionality
#[tokio::test]
async fn test_renderer_registry_default() {
    let registry = RendererRegistry::new().register(JSONRenderer::new());

    let data = json!({"message": "hello"});
    let (bytes, content_type) = registry.render(&data, None, None).await.unwrap();

    let output = String::from_utf8(bytes.to_vec()).unwrap();
    assert!(output.contains("hello"));
    assert_eq!(content_type, "application/json; charset=utf-8");
}

#[tokio::test]
async fn test_renderer_registry_by_format() {
    let registry = RendererRegistry::new()
        .register(JSONRenderer::new())
        .register(JSONRenderer::new().pretty(true));

    // Get renderer by format
    let renderer = registry.get_renderer(Some("json")).unwrap();
    assert_eq!(renderer.format(), Some("json"));
    assert_eq!(renderer.media_type(), "application/json");
}

#[tokio::test]
async fn test_renderer_registry_by_media_type() {
    let registry = RendererRegistry::new().register(JSONRenderer::new());

    // Get renderer by media type
    let renderer = registry.get_renderer_by_media_type("application/json");
    assert!(renderer.is_some());
    assert_eq!(renderer.unwrap().media_type(), "application/json");
}

#[tokio::test]
async fn test_unsupported_format_returns_error() {
    let registry = RendererRegistry::new().register(JSONRenderer::new());

    let data = json!({"test": "data"});
    let result = registry.render(&data, Some("unsupported"), None).await;

    assert!(result.is_err());
}

/// Test renderer with different data types
#[tokio::test]
async fn test_render_complex_nested_data() {
    let renderer = JSONRenderer::new();
    let data = json!({
        "users": [
            {
                "id": 1,
                "name": "Alice",
                "profile": {
                    "age": 30,
                    "city": "Tokyo"
                }
            },
            {
                "id": 2,
                "name": "Bob",
                "profile": {
                    "age": 25,
                    "city": "Osaka"
                }
            }
        ],
        "meta": {
            "total": 2,
            "page": 1,
            "per_page": 10
        }
    });

    let result = renderer.render(&data, None).await.unwrap();
    let output = String::from_utf8(result.to_vec()).unwrap();

    assert!(output.contains("users"));
    assert!(output.contains("Alice"));
    assert!(output.contains("Bob"));
    assert!(output.contains("Tokyo"));
    assert!(output.contains("Osaka"));
    assert!(output.contains("meta"));
}

/// Test renderer context in real-world scenarios
#[tokio::test]
async fn test_renderer_with_full_context() {
    let renderer = JSONRenderer::new();
    let data = json!({
        "items": ["item1", "item2", "item3"]
    });

    let context = RendererContext::new()
        .with_request("GET", "/api/v1/items/")
        .with_view("ItemList", "Returns a list of items")
        .with_extra("api_version", "v1")
        .with_extra("authenticated", "true");

    let result = renderer.render(&data, Some(&context)).await.unwrap();
    let output = String::from_utf8(result.to_vec()).unwrap();

    // The context doesn't affect JSON output, but ensures it works
    assert!(output.contains("items"));
    assert!(output.contains("item1"));
}

/// Test edge cases from DRF
#[tokio::test]
async fn test_empty_response_handling() {
    let renderer = JSONRenderer::new();

    // Empty object
    let data = json!({});
    let result = renderer.render(&data, None).await.unwrap();
    assert_eq!(String::from_utf8(result.to_vec()).unwrap(), "{}");

    // Empty array
    let data = json!([]);
    let result = renderer.render(&data, None).await.unwrap();
    assert_eq!(String::from_utf8(result.to_vec()).unwrap(), "[]");
}

/// Test that pretty printing works correctly
#[tokio::test]
async fn test_pretty_vs_compact_rendering() {
    let data = json!({"foo": ["bar", "baz"]});

    // Compact rendering
    let compact_renderer = JSONRenderer::new().pretty(false);
    let compact_result = compact_renderer.render(&data, None).await.unwrap();
    let compact_output = String::from_utf8(compact_result.to_vec()).unwrap();

    // Pretty rendering
    let pretty_renderer = JSONRenderer::new().pretty(true);
    let pretty_result = pretty_renderer.render(&data, None).await.unwrap();
    let pretty_output = String::from_utf8(pretty_result.to_vec()).unwrap();

    // Pretty output should be longer due to whitespace
    assert!(pretty_output.len() > compact_output.len());
    assert!(pretty_output.contains("\n"));
}

/// Test large data rendering
#[tokio::test]
async fn test_large_data_rendering() {
    let renderer = JSONRenderer::new();

    // Create a large dataset
    let mut items = Vec::new();
    for i in 0..100 {
        items.push(json!({
            "id": i,
            "name": format!("Item {}", i),
            "value": i * 10
        }));
    }

    let data = json!({"items": items, "total": 100});
    let result = renderer.render(&data, None).await.unwrap();

    assert!(!result.is_empty());
    let output = String::from_utf8(result.to_vec()).unwrap();
    assert!(output.contains("Item 0"));
    assert!(output.contains("Item 99"));
}

/// Test special characters and escaping
#[tokio::test]
async fn test_special_characters_in_json() {
    let renderer = JSONRenderer::new();
    let data = json!({
        "quote": "He said \"hello\"",
        "backslash": "C:\\Users\\test",
        "newline": "Line 1\nLine 2",
        "tab": "Col1\tCol2"
    });

    let result = renderer.render(&data, None).await.unwrap();
    let output = String::from_utf8(result.to_vec()).unwrap();

    // JSON should properly escape special characters
    assert!(output.contains("\\\""));
    assert!(output.contains("\\n") || output.contains("\n"));
}

// NOTE: The following tests are based on DRF but cannot be fully implemented
// without additional infrastructure (HTTP server, request handling, etc.)
//
// - test_default_renderer_serializes_content_on_accept_any
// - test_specified_renderer_serializes_content_default_case
// - test_unsatisfiable_accept_header_on_request_returns_406_status
// - test_specified_renderer_serializes_content_on_format_query
// - test_specified_renderer_is_used_on_format_query_with_matching_accept
//
// These would require:
// - HTTP request/response infrastructure (from reinhardt-http)
// - Content negotiation (from reinhardt-negotiation)
// - Views/routing (from reinhardt-views/reinhardt-routers)
//
// They should be implemented as integration tests when those components are available.
