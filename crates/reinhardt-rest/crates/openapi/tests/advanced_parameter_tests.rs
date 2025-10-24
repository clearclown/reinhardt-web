//! Advanced Parameter Tests
//!
//! Tests for primary key related fields and parser mapping.

use openapi::openapi::RequestBodyBuilder;
use openapi::{
    MediaType, Operation, OperationExt, Parameter, ParameterExt, ParameterLocation, Required,
    Schema, SchemaExt,
};

#[test]
fn test_primary_key_related_field() {
    // Test primary key field as path parameter
    let mut operation = <Operation as OperationExt>::new();

    let id_param = <Parameter as ParameterExt>::new_with_description(
        "id",
        ParameterLocation::Path,
        Schema::integer(),
        true,
        "Primary key identifier",
    );

    operation.add_parameter(id_param);

    let params = operation.parameters.as_ref().unwrap();
    assert_eq!(params.len(), 1);

    // Verify via JSON serialization
    let param_json = serde_json::to_value(&params[0]).unwrap();
    assert_eq!(param_json["name"], "id");
    assert_eq!(param_json["in"], "path");
    assert_eq!(param_json["required"], true);
    assert_eq!(param_json["description"], "Primary key identifier");

    // Verify schema is integer type
    assert_eq!(param_json["schema"]["type"], "integer");
}

#[test]
fn test_parser_mapping() {
    // Test request body with different Content-Type parsers
    let mut operation = <Operation as OperationExt>::new();

    // Create MediaType instances
    let mut json_media = MediaType::new(Some(Schema::object()));
    json_media.example = Some(serde_json::json!({"key": "value"}));

    let form_media = MediaType::new(Some(Schema::object()));
    let multipart_media = MediaType::new(Some(Schema::object()));

    // Build request body with multiple content types
    let request_body = RequestBodyBuilder::new()
        .description(Some("Request with multiple parsers"))
        .content("application/json", json_media)
        .content("application/x-www-form-urlencoded", form_media)
        .content("multipart/form-data", multipart_media)
        .required(Some(Required::True))
        .build();

    operation.request_body = Some(request_body);

    let request_body_ref = operation.request_body.as_ref().unwrap();
    assert!(matches!(request_body_ref.required, Some(Required::True)));

    let content_map = &request_body_ref.content;
    assert_eq!(content_map.len(), 3);
    assert!(content_map.contains_key("application/json"));
    assert!(content_map.contains_key("application/x-www-form-urlencoded"));
    assert!(content_map.contains_key("multipart/form-data"));

    // Verify JSON parser has example
    let json_media_ref = &content_map["application/json"];
    assert!(json_media_ref.example.is_some());

    // Verify all parsers have object schema via JSON serialization
    for (content_type, media_type) in content_map {
        let media_json = serde_json::to_value(media_type).unwrap();
        assert_eq!(
            media_json["schema"]["type"], "object",
            "Content-Type {} should have object schema",
            content_type
        );
    }
}
