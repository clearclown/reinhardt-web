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

    // 厳密なJSON構造検証
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .expect("出力が有効なJSONである必要があります");

    assert_eq!(
        parsed.get("message"),
        Some(&json!("hello")),
        "message キーの値が 'hello' である必要があります"
    );
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
    assert_eq!(renderer.media_type(), "application/json; charset=utf-8");
}

#[tokio::test]
async fn test_renderer_registry_by_media_type() {
    let registry = RendererRegistry::new().register(JSONRenderer::new());

    // Get renderer by media type
    let renderer = registry.get_renderer_by_media_type("application/json");
    assert!(renderer.is_some());
    assert_eq!(
        renderer.unwrap().media_type(),
        "application/json; charset=utf-8"
    );
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

    // 厳密なJSON構造検証
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .expect("出力が有効なJSONである必要があります");

    // users配列の検証
    let users = parsed.get("users")
        .and_then(|v| v.as_array())
        .expect("users キーが配列である必要があります");

    assert_eq!(users.len(), 2, "users配列の長さが2である必要があります");

    // Alice の検証
    let alice = &users[0];
    assert_eq!(alice.get("id"), Some(&json!(1)), "Alice の id が 1 である必要があります");
    assert_eq!(alice.get("name"), Some(&json!("Alice")), "名前が Alice である必要があります");
    let alice_profile = alice.get("profile").expect("Alice のプロフィールが存在する必要があります");
    assert_eq!(alice_profile.get("age"), Some(&json!(30)), "Alice の年齢が 30 である必要があります");
    assert_eq!(alice_profile.get("city"), Some(&json!("Tokyo")), "Alice の都市が Tokyo である必要があります");

    // Bob の検証
    let bob = &users[1];
    assert_eq!(bob.get("id"), Some(&json!(2)), "Bob の id が 2 である必要があります");
    assert_eq!(bob.get("name"), Some(&json!("Bob")), "名前が Bob である必要があります");
    let bob_profile = bob.get("profile").expect("Bob のプロフィールが存在する必要があります");
    assert_eq!(bob_profile.get("age"), Some(&json!(25)), "Bob の年齢が 25 である必要があります");
    assert_eq!(bob_profile.get("city"), Some(&json!("Osaka")), "Bob の都市が Osaka である必要があります");

    // meta の検証
    let meta = parsed.get("meta").expect("meta が存在する必要があります");
    assert_eq!(meta.get("total"), Some(&json!(2)), "total が 2 である必要があります");
    assert_eq!(meta.get("page"), Some(&json!(1)), "page が 1 である必要があります");
    assert_eq!(meta.get("per_page"), Some(&json!(10)), "per_page が 10 である必要があります");
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

    // 厳密なJSON構造検証
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .expect("出力が有効なJSONである必要があります");

    let items = parsed.get("items")
        .and_then(|v| v.as_array())
        .expect("items キーが配列である必要があります");

    assert_eq!(items.len(), 3, "items配列の長さが3である必要があります");
    assert_eq!(items[0], json!("item1"), "最初の要素が 'item1' である必要があります");
    assert_eq!(items[1], json!("item2"), "2番目の要素が 'item2' である必要があります");
    assert_eq!(items[2], json!("item3"), "3番目の要素が 'item3' である必要があります");
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

    // 厳密なJSON構造検証
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .expect("出力が有効なJSONである必要があります");

    let items = parsed.get("items")
        .and_then(|v| v.as_array())
        .expect("items キーが配列である必要があります");

    assert_eq!(items.len(), 100, "items配列の長さが100である必要があります");

    // 最初と最後の要素を検証
    let first_item = &items[0];
    assert_eq!(first_item.get("id"), Some(&json!(0)), "最初の要素の id が 0 である必要があります");
    assert_eq!(first_item.get("name"), Some(&json!("Item 0")), "最初の要素の name が 'Item 0' である必要があります");
    assert_eq!(first_item.get("value"), Some(&json!(0)), "最初の要素の value が 0 である必要があります");

    let last_item = &items[99];
    assert_eq!(last_item.get("id"), Some(&json!(99)), "最後の要素の id が 99 である必要があります");
    assert_eq!(last_item.get("name"), Some(&json!("Item 99")), "最後の要素の name が 'Item 99' である必要があります");
    assert_eq!(last_item.get("value"), Some(&json!(990)), "最後の要素の value が 990 である必要があります");

    assert_eq!(parsed.get("total"), Some(&json!(100)), "total が 100 である必要があります");
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

/// Test default renderer serializes content on Accept: */*
#[tokio::test]
async fn test_default_renderer_serializes_content_on_accept_any() {
    let registry = RendererRegistry::new().register(JSONRenderer::new());

    let data = json!({"message": "hello world"});
    let context = RendererContext::new().with_accept_header("*/*");

    let result = registry.render(&data, None, Some(&context)).await;
    assert!(result.is_ok());

    let (bytes, content_type) = result.unwrap();
    assert_eq!(content_type, "application/json; charset=utf-8");

    let output = String::from_utf8(bytes.to_vec()).unwrap();

    // 厳密なJSON構造検証
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .expect("出力が有効なJSONである必要があります");

    assert_eq!(
        parsed.get("message"),
        Some(&json!("hello world")),
        "message キーの値が 'hello world' である必要があります"
    );
}

/// Test specified renderer serializes content (default case - no Accept header)
#[tokio::test]
async fn test_specified_renderer_serializes_content_default_case() {
    let registry = RendererRegistry::new().register(JSONRenderer::new());

    let data = json!({"status": "success", "code": 200});
    let context = RendererContext::new();

    let result = registry.render(&data, None, Some(&context)).await;
    assert!(result.is_ok());

    let (bytes, content_type) = result.unwrap();
    assert_eq!(content_type, "application/json; charset=utf-8");

    let output = String::from_utf8(bytes.to_vec()).unwrap();

    // 厳密なJSON構造検証
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .expect("出力が有効なJSONである必要があります");

    assert_eq!(
        parsed.get("status"),
        Some(&json!("success")),
        "status キーの値が 'success' である必要があります"
    );
    assert_eq!(
        parsed.get("code"),
        Some(&json!(200)),
        "code キーの値が 200 である必要があります"
    );
}

/// Test unsatisfiable Accept header returns 406 Not Acceptable
#[tokio::test]
async fn test_unsatisfiable_accept_header_on_request_returns_406_status() {
    let registry = RendererRegistry::new().register(JSONRenderer::new());

    let data = json!({"test": "data"});
    let context = RendererContext::new().with_accept_header("application/xml");

    let result = registry.render(&data, None, Some(&context)).await;
    assert!(result.is_err());

    let err = result.unwrap_err();
    let err_msg = err.to_string();
    assert!(
        err_msg.contains("406") || err_msg.contains("Not Acceptable"),
        "Expected 406 error, got: {}",
        err_msg
    );
}

/// Test specified renderer is used on format query parameter
#[tokio::test]
async fn test_specified_renderer_serializes_content_on_format_query() {
    let registry = RendererRegistry::new().register(JSONRenderer::new());

    let data = json!({"format": "test"});
    let context = RendererContext::new().with_format_param("json");

    let result = registry.render(&data, None, Some(&context)).await;
    assert!(result.is_ok());

    let (bytes, content_type) = result.unwrap();
    assert_eq!(content_type, "application/json; charset=utf-8");

    let output = String::from_utf8(bytes.to_vec()).unwrap();

    // 厳密なJSON構造検証
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .expect("出力が有効なJSONである必要があります");

    assert_eq!(
        parsed.get("format"),
        Some(&json!("test")),
        "format キーの値が 'test' である必要があります"
    );
}

/// Test specified renderer is used on format query with matching Accept header
#[tokio::test]
async fn test_specified_renderer_is_used_on_format_query_with_matching_accept() {
    let registry = RendererRegistry::new().register(JSONRenderer::new());

    let data = json!({"combined": "test"});
    let context = RendererContext::new()
        .with_format_param("json")
        .with_accept_header("application/json");

    let result = registry.render(&data, None, Some(&context)).await;
    assert!(result.is_ok());

    let (bytes, content_type) = result.unwrap();
    assert_eq!(content_type, "application/json; charset=utf-8");

    let output = String::from_utf8(bytes.to_vec()).unwrap();

    // 厳密なJSON構造検証
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .expect("出力が有効なJSONである必要があります");

    assert_eq!(
        parsed.get("combined"),
        Some(&json!("test")),
        "combined キーの値が 'test' である必要があります"
    );
}

/// Test Accept header with quality values
#[tokio::test]
async fn test_accept_header_with_quality_values() {
    let registry = RendererRegistry::new().register(JSONRenderer::new());

    let data = json!({"quality": "test"});
    // Prefer application/json over other types
    let context =
        RendererContext::new().with_accept_header("text/html; q=0.8, application/json; q=0.9");

    let result = registry.render(&data, None, Some(&context)).await;
    assert!(result.is_ok());

    let (bytes, content_type) = result.unwrap();
    assert_eq!(content_type, "application/json; charset=utf-8");

    let output = String::from_utf8(bytes.to_vec()).unwrap();

    // 厳密なJSON構造検証
    let parsed: serde_json::Value = serde_json::from_str(&output)
        .expect("出力が有効なJSONである必要があります");

    assert_eq!(
        parsed.get("quality"),
        Some(&json!("test")),
        "quality キーの値が 'test' である必要があります"
    );
}

/// Test format parameter takes precedence over Accept header
#[tokio::test]
async fn test_format_parameter_takes_precedence_over_accept_header() {
    let registry = RendererRegistry::new().register(JSONRenderer::new());

    let data = json!({"precedence": "format"});
    let context = RendererContext::new()
        .with_format_param("json")
        .with_accept_header("text/html"); // This should be ignored

    let result = registry.render(&data, None, Some(&context)).await;
    assert!(result.is_ok());

    let (bytes, content_type) = result.unwrap();
    assert_eq!(content_type, "application/json; charset=utf-8");
}
