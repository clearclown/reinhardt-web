//! Schema Serialization Tests
//!
//! Tests for JSON and YAML rendering of OpenAPI schemas.

use openapi::openapi::ResponseBuilder;
use openapi::{OpenApiSchema, OpenApiSchemaExt, Operation, OperationExt, PathItem};

#[test]
fn test_schema_rendering_to_json() {
    // Test JSON rendering
    let schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");
    let json = schema.to_json().expect("Failed to render JSON");

    assert!(json.contains(r#""openapi":"#) || json.contains(r#""openapi" :"#));
    assert!(json.contains(r#""title":"#) || json.contains(r#""title" :"#));
    assert!(json.contains("Test API"));
    assert!(json.contains("1.0.0"));
}

#[test]
fn test_schema_rendering_to_yaml() {
    // Test YAML rendering
    let schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");
    let yaml = schema.to_yaml().expect("Failed to render YAML");

    assert!(yaml.contains("openapi:"));
    assert!(yaml.contains("Test API"));
    assert!(yaml.contains("1.0.0"));
}

#[test]
fn test_openapi_yaml_rendering_without_aliases() {
    // Test YAML rendering doesn't use aliases
    let mut schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");

    // Add multiple paths with similar responses
    let mut path1 = PathItem::default();
    let mut op1 = <Operation as OperationExt>::new();
    let response1 = ResponseBuilder::new().description("Success").build();
    op1.add_response("200", response1);
    path1.get = Some(op1);

    let mut path2 = PathItem::default();
    let mut op2 = <Operation as OperationExt>::new();
    let response2 = ResponseBuilder::new().description("Success").build();
    op2.add_response("200", response2);
    path2.get = Some(op2);

    schema.add_path("/items/".to_string(), path1);
    schema.add_path("/users/".to_string(), path2);

    let yaml = schema.to_yaml().expect("Failed to render YAML");

    // YAML should not contain aliases (like &id001 or *id001)
    assert!(!yaml.contains("&"));
    assert!(!yaml.contains("*id"));
}

#[test]
fn test_json_pretty_formatting() {
    // Test that JSON is properly formatted
    let mut schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");
    schema.info.description = Some("Description".to_string());

    let json = schema.to_json().expect("Failed to render JSON");

    // JSON should be valid and parseable
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Invalid JSON");
    assert_eq!(parsed["info"]["title"].as_str(), Some("Test API"));
    assert_eq!(parsed["info"]["description"].as_str(), Some("Description"));
}

#[test]
fn test_yaml_valid_format() {
    // Test that YAML output is valid
    let mut schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");
    schema.info.description = Some("A test API description".to_string());

    let yaml = schema.to_yaml().expect("Failed to render YAML");

    // Parse it back to verify it's valid YAML
    let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("Invalid YAML");

    assert_eq!(parsed["openapi"].as_str(), Some("3.1.0"));
    assert_eq!(parsed["info"]["title"].as_str(), Some("Test API"));
}

#[test]
fn test_json_valid_format() {
    // Test that JSON output is valid
    let schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");
    let json = schema.to_json().expect("Failed to render JSON");

    // Parse it back to verify it's valid JSON
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("Invalid JSON");

    assert_eq!(parsed["openapi"].as_str(), Some("3.1.0"));
    assert_eq!(parsed["info"]["title"].as_str(), Some("Test API"));
}

#[test]
fn test_empty_optional_fields_omitted_in_json() {
    // Test that empty optional fields are omitted from JSON
    let schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");
    let json = schema.to_json().expect("Failed to render JSON");

    // Optional fields like description, servers, security should be omitted
    assert!(!json.contains("\"description\""));
    assert!(!json.contains("\"servers\""));
    assert!(!json.contains("\"security\""));
}
