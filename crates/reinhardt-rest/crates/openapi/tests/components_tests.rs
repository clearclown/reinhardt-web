//! Components/Schemas Tests
//!
//! Tests for component schemas, naming, and duplicate handling.

use openapi::openapi::ComponentsBuilder;
use openapi::{ComponentsExt, OpenApiSchema, OpenApiSchemaExt, Schema, SchemaExt};
use serde_json::Value;

#[test]
fn test_serializer_model() {
    // Test model serializer schemas in components
    let mut schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");

    let item_schema = Schema::object_with_description(
        vec![("id", Schema::integer()), ("name", Schema::string())],
        vec!["id"],
        "Item model",
    );

    let mut components = ComponentsBuilder::new().build();
    components.add_schema("Item".to_string(), item_schema);

    schema.components = Some(components);

    assert!(schema.components.is_some());
    let components = schema.components.as_ref().unwrap();
    assert!(components.schemas.contains_key("Item"));
}

#[test]
fn test_component_name() {
    // Test component naming
    let mut schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");

    let user_schema = Schema::object_with_description(
        Vec::<(&str, Schema)>::new(),
        Vec::<&str>::new(),
        "User component",
    );

    let mut components = ComponentsBuilder::new().build();
    components.add_schema("User".to_string(), user_schema);

    schema.components = Some(components);

    let components = schema.components.as_ref().unwrap();

    // Component name should be "User"
    assert!(components.schemas.contains_key("User"));
    assert!(!components.schemas.contains_key("user"));
}

#[test]
fn test_duplicate_component_name() {
    // Test duplicate handling - later one overwrites
    let mut components = ComponentsBuilder::new().build();

    let schema1 = Schema::object_with_description(
        Vec::<(&str, Schema)>::new(),
        Vec::<&str>::new(),
        "First version",
    );
    components.add_schema("Item".to_string(), schema1);

    let schema2 = Schema::object_with_description(
        Vec::<(&str, Schema)>::new(),
        Vec::<&str>::new(),
        "Second version",
    );
    components.add_schema("Item".to_string(), schema2);

    // Should only have one entry with the second description
    assert_eq!(components.schemas.len(), 1);

    // Serialize to JSON to check the description
    let json_str =
        serde_json::to_string(&components.schemas["Item"]).expect("Failed to serialize schema");
    assert!(json_str.contains("Second version"));
}

#[test]
fn test_multiple_components() {
    // Test multiple component schemas
    let mut schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");

    let mut components = ComponentsBuilder::new().build();
    components.add_schema("User".to_string(), Schema::string());
    components.add_schema("Post".to_string(), Schema::string());
    components.add_schema("Comment".to_string(), Schema::string());

    schema.components = Some(components);

    let components = schema.components.as_ref().unwrap();

    assert_eq!(components.schemas.len(), 3);
    assert!(components.schemas.contains_key("User"));
    assert!(components.schemas.contains_key("Post"));
    assert!(components.schemas.contains_key("Comment"));
}

#[test]
fn test_component_reference() {
    // Test referencing components
    let ref_schema = Schema::Object(
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::SchemaType::Type(
                utoipa::openapi::schema::Type::Object,
            ))
            .build(),
    );

    // In utoipa v5, we typically use RefOr for references
    // This test verifies schema object creation
    let json_str = serde_json::to_string(&ref_schema).expect("Failed to serialize");
    let json: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    // Verify it's an object type schema
    assert_eq!(json["type"], "object");
}

#[test]
fn test_nested_component_properties() {
    // Test nested properties in component schemas
    let address_ref = Schema::Object(
        utoipa::openapi::ObjectBuilder::new()
            .schema_type(utoipa::openapi::schema::SchemaType::Type(
                utoipa::openapi::schema::Type::Object,
            ))
            .build(),
    );

    let user_schema = Schema::object_with_properties(
        vec![
            ("id", Schema::integer()),
            ("name", Schema::string()),
            ("address", address_ref),
        ],
        vec!["id", "name"],
    );

    // Serialize to JSON to verify properties
    let json_str = serde_json::to_string(&user_schema).expect("Failed to serialize");
    let json: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    // Verify properties exist
    assert!(json["properties"]["id"].is_object());
    assert!(json["properties"]["name"].is_object());
    assert!(json["properties"]["address"].is_object());
}

#[test]
fn test_component_required_fields() {
    // Test required fields in component schemas
    let schema = Schema::object_with_properties(
        vec![
            ("id", Schema::integer()),
            ("name", Schema::string()),
            ("email", Schema::string()),
        ],
        vec!["id", "name"],
    );

    // Serialize to JSON to verify required fields
    let json_str = serde_json::to_string(&schema).expect("Failed to serialize");
    let json: Value = serde_json::from_str(&json_str).expect("Failed to parse JSON");

    // Verify required array
    let required = json["required"]
        .as_array()
        .expect("Required should be array");
    assert_eq!(required.len(), 2);
    assert!(required.iter().any(|v| v == "id"));
    assert!(required.iter().any(|v| v == "name"));
    assert!(!required.iter().any(|v| v == "email"));
}

#[test]
fn test_component_schema_in_json() {
    // Test that components serialize correctly in JSON
    let mut schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");

    let mut components = ComponentsBuilder::new().build();
    components.add_schema("User".to_string(), Schema::string());

    schema.components = Some(components);

    let json = schema.to_json().expect("Failed to serialize");

    assert!(json.contains("components"));
    assert!(json.contains("schemas"));
    assert!(json.contains("User"));
}
