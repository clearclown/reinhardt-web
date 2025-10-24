//! Request/Response Body Tests
//!
//! Tests for request bodies, response bodies, and content types.

use openapi::openapi::{RefOr, RequestBodyBuilder, ResponseBuilder};
use openapi::{MediaType, OperationExt, Required, ResponsesExt, Schema, SchemaExt};

#[test]
fn test_request_body() {
    // Test request body schemas
    let request_body = RequestBodyBuilder::new()
        .description(Some("Request body"))
        .content("application/json", MediaType::new(Some(Schema::string())))
        .required(Some(Required::True))
        .build();

    assert_eq!(request_body.description, Some("Request body".to_string()));
    assert!(matches!(request_body.required, Some(Required::True)));
    assert!(request_body.content.contains_key("application/json"));
}

#[test]
fn test_response_body_generation() {
    // Test response body generation
    let schema = Schema::object_with_properties(
        vec![("id", Schema::integer()), ("name", Schema::string())],
        vec!["id", "name"],
    );

    let media_type = MediaType::new(Some(schema));

    let response = ResponseBuilder::new()
        .description("Success")
        .content("application/json", media_type)
        .build();

    assert_eq!(response.description, "Success");
    assert!(!response.content.is_empty());
    assert!(response.content.contains_key("application/json"));
}

#[test]
fn test_list_response_body_generation() {
    // Test list responses
    let list_schema = Schema::array(Schema::Object(Default::default()));

    let media_type = MediaType::new(Some(list_schema));

    let response = ResponseBuilder::new()
        .description("List of items")
        .content("application/json", media_type)
        .build();

    let media_type = &response.content["application/json"];
    let schema = media_type.schema.as_ref().unwrap();

    // Verify it's an array schema wrapped in RefOr::T
    assert!(matches!(schema, RefOr::T(Schema::Array(_))));
}

#[test]
fn test_paginated_list_response_body_generation() {
    // Test paginated responses
    let item_array = Schema::array(Schema::Object(Default::default()));

    let paginated_schema = Schema::object_with_properties(
        vec![
            ("count", Schema::integer()),
            ("next", Schema::string()),
            ("previous", Schema::string()),
            ("results", item_array),
        ],
        vec!["count", "results"],
    );

    let media_type = MediaType::new(Some(paginated_schema));

    let response = ResponseBuilder::new()
        .description("Paginated list")
        .content("application/json", media_type)
        .build();

    let media_type = &response.content["application/json"];
    let schema = media_type.schema.as_ref().unwrap();

    // Verify it's an object schema wrapped in RefOr::T
    assert!(matches!(schema, RefOr::T(Schema::Object(_))));

    // Extract properties if it's an Object variant
    if let RefOr::T(Schema::Object(obj)) = schema {
        let props = &obj.properties;
        assert!(props.contains_key("count"));
        assert!(props.contains_key("next"));
        assert!(props.contains_key("previous"));
        assert!(props.contains_key("results"));
    }
}

#[test]
fn test_multiple_content_types() {
    // Test multiple content types in response
    let response = ResponseBuilder::new()
        .description("Multi-format response")
        .content("application/json", MediaType::new(Some(Schema::string())))
        .content("application/xml", MediaType::new(Some(Schema::string())))
        .build();

    assert_eq!(response.content.len(), 2);
    assert!(response.content.contains_key("application/json"));
    assert!(response.content.contains_key("application/xml"));
}

#[test]
fn test_response_with_example() {
    // Test response with example value
    use serde_json::json;

    let example = json!({
        "id": 1,
        "name": "Test Item"
    });

    let mut media_type = MediaType::new(Some(Schema::string()));
    media_type.example = Some(example.clone());

    let response = ResponseBuilder::new()
        .description("Response with example")
        .content("application/json", media_type)
        .build();

    let media_type = &response.content["application/json"];
    assert!(media_type.example.is_some());
    assert_eq!(media_type.example, Some(example));
}

#[test]
fn test_empty_response() {
    // Test response without content (e.g., 204 No Content)
    let response = ResponseBuilder::new().description("No content").build();

    assert_eq!(response.description, "No content");
    assert!(response.content.is_empty());
}

#[test]
fn test_response_in_operation() {
    // Test adding responses to operations
    use openapi::Operation;

    let mut operation = <Operation as OperationExt>::new();

    let success_response = ResponseBuilder::new()
        .description("Successful operation")
        .build();
    operation.add_response("200", success_response);

    let error_response = ResponseBuilder::new().description("Not found").build();
    operation.add_response("404", error_response);

    assert_eq!(operation.responses.len(), 2);
    assert!(operation.responses.contains_key("200"));
    assert!(operation.responses.contains_key("404"));
}
