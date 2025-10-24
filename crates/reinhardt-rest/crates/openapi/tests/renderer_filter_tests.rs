//! Renderer and Filter Tests
//!
//! Tests for response renderer mapping and query parameter filter schemas.

use openapi::openapi::{ObjectBuilder, ResponseBuilder, SchemaType};
use openapi::{
    MediaType, Operation, OperationExt, Parameter, ParameterExt, ParameterLocation, Schema,
    SchemaExt,
};

#[test]
fn test_renderer_mapping() {
    // Test response with different Content-Type renderers
    let mut operation = <Operation as OperationExt>::new();

    // Create response with multiple content types using ResponseBuilder
    let mut json_media = MediaType::new(Some(Schema::object()));
    json_media.example = Some(serde_json::json!({"status": "success"}));

    let response = ResponseBuilder::new()
        .description("Response with multiple renderers")
        .content("application/json", json_media)
        .content("application/xml", MediaType::new(Some(Schema::object())))
        .content("text/plain", MediaType::new(Some(Schema::string())))
        .build();

    operation.add_response("200", response);

    // Extract the Response from RefOr
    let response_ref = &operation.responses.responses["200"];
    if let openapi::openapi::RefOr::T(response) = response_ref {
        assert_eq!(response.description, "Response with multiple renderers");

        let content_map = &response.content;
        assert_eq!(content_map.len(), 3);
        assert!(content_map.contains_key("application/json"));
        assert!(content_map.contains_key("application/xml"));
        assert!(content_map.contains_key("text/plain"));

        // Verify JSON renderer has example
        let json_media = &content_map["application/json"];
        assert!(json_media.example.is_some());

        // Verify text/plain uses string schema
        let text_media = &content_map["text/plain"];
        let text_schema = text_media.schema.as_ref().unwrap();

        // Use JSON serialization to verify schema type
        let schema_json = serde_json::to_value(text_schema).unwrap();
        assert_eq!(schema_json["type"], "string");
    } else {
        panic!("Expected direct Response, not a reference");
    }
}

#[test]
fn test_filters() {
    // Test query parameter filter schemas
    let mut operation = <Operation as OperationExt>::new();

    // Create parameters using ParameterExt
    let search_param = <Parameter as ParameterExt>::new_simple(
        "search",
        ParameterLocation::Query,
        Schema::string(),
        false,
    );

    // Create ordering schema with enum values
    let ordering_schema = Schema::Object(
        ObjectBuilder::new()
            .schema_type(SchemaType::Type(utoipa::openapi::Type::String))
            .enum_values(Some(vec![
                serde_json::json!("name"),
                serde_json::json!("-name"),
                serde_json::json!("created_at"),
                serde_json::json!("-created_at"),
            ]))
            .build(),
    );

    let ordering_param = <Parameter as ParameterExt>::new_simple(
        "ordering",
        ParameterLocation::Query,
        ordering_schema,
        false,
    );

    // Create page_size schema with constraints
    let page_size_schema = Schema::Object(
        ObjectBuilder::new()
            .schema_type(SchemaType::Type(utoipa::openapi::Type::Integer))
            .minimum(Some(utoipa::Number::from(1.0)))
            .maximum(Some(utoipa::Number::from(100.0)))
            .build(),
    );

    let page_size_param = <Parameter as ParameterExt>::new_simple(
        "page_size",
        ParameterLocation::Query,
        page_size_schema,
        false,
    );

    // Create status schema with enum values
    let status_schema = Schema::Object(
        ObjectBuilder::new()
            .schema_type(SchemaType::Type(utoipa::openapi::Type::String))
            .enum_values(Some(vec![
                serde_json::json!("active"),
                serde_json::json!("inactive"),
                serde_json::json!("pending"),
            ]))
            .build(),
    );

    let status_param = <Parameter as ParameterExt>::new_simple(
        "status",
        ParameterLocation::Query,
        status_schema,
        false,
    );

    operation.parameters = Some(vec![
        search_param.into(),
        ordering_param.into(),
        page_size_param.into(),
        status_param.into(),
    ]);

    // Verify all parameters
    let params = operation.parameters.as_ref().unwrap();
    assert_eq!(params.len(), 4);

    // Verify search filter
    let search = &params[0];
    assert_eq!(search.name, "search");
    let search_json = serde_json::to_value(search).unwrap();
    assert_eq!(search_json["in"], "query");

    // Verify ordering filter has enum
    let ordering = &params[1];
    assert_eq!(ordering.name, "ordering");

    // Access enum_values through the Schema enum
    if let Some(ref schema_ref) = ordering.schema {
        match schema_ref {
            openapi::openapi::RefOr::T(Schema::Object(obj)) => {
                assert!(obj.enum_values.is_some());
                assert_eq!(obj.enum_values.as_ref().unwrap().len(), 4);
            }
            _ => panic!("Expected Schema::Object with enum values"),
        }
    }

    // Verify page_size has constraints - use JSON serialization
    let page_size = &params[2];
    assert_eq!(page_size.name, "page_size");

    if let Some(ref schema_ref) = page_size.schema {
        let json = serde_json::to_value(schema_ref).unwrap();
        assert_eq!(json["minimum"], 1);
        assert_eq!(json["maximum"], 100);
    }

    // Verify status filter has enum
    let status = &params[3];
    assert_eq!(status.name, "status");

    if let Some(ref schema_ref) = status.schema {
        match schema_ref {
            openapi::openapi::RefOr::T(Schema::Object(obj)) => {
                assert!(obj.enum_values.is_some());
                assert_eq!(obj.enum_values.as_ref().unwrap().len(), 3);
            }
            _ => panic!("Expected Schema::Object with enum values"),
        }
    }
}
