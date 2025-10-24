//! Parameter Tests
//!
//! Tests for path parameters, query parameters, and parameter inclusion.

use openapi::{
    OpenApiSchema, OpenApiSchemaExt, Operation, OperationExt, Parameter, ParameterExt,
    ParameterLocation, PathItem, PathItemExt, Schema, SchemaExt,
};

#[test]
fn test_path_without_parameters() {
    // Test paths without parameters
    let mut schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");

    let path_item = <PathItem as PathItemExt>::new();
    schema.add_path("/items/".to_string(), path_item);

    let path = &schema.paths.paths["/items/"];
    assert!(path.parameters.is_none() || path.parameters.as_ref().unwrap().is_empty());
}

#[test]
fn test_path_with_id_parameter() {
    // Test ID parameters in path
    let mut schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");

    let mut path_item = <PathItem as PathItemExt>::new();
    let id_param = <Parameter as ParameterExt>::new_with_description(
        "id",
        ParameterLocation::Path,
        Schema::integer(),
        true,
        "Item ID",
    );
    path_item.parameters = Some(vec![id_param.into()]);

    schema.add_path("/items/{id}/".to_string(), path_item);

    let path = &schema.paths.paths["/items/{id}/"];
    assert!(path.parameters.is_some());
    let params = path.parameters.as_ref().unwrap();
    assert_eq!(params.len(), 1);
}

#[test]
fn test_param_include_in_schema() {
    // Test parameter inclusion control
    let mut operation = <Operation as OperationExt>::new();

    // Add a query parameter
    let search_param = <Parameter as ParameterExt>::new_with_description(
        "search",
        ParameterLocation::Query,
        Schema::string(),
        false,
        "Search query",
    );

    operation.add_parameter(search_param);

    let params = operation.parameters.as_ref().unwrap();
    assert_eq!(params.len(), 1);
}

#[test]
fn test_parameter_locations() {
    // Test all parameter locations
    let path_param = <Parameter as ParameterExt>::new_simple(
        "id",
        ParameterLocation::Path,
        Schema::integer(),
        true,
    );

    let query_param = <Parameter as ParameterExt>::new_simple(
        "filter",
        ParameterLocation::Query,
        Schema::string(),
        false,
    );

    let header_param = <Parameter as ParameterExt>::new_simple(
        "X-Api-Key",
        ParameterLocation::Header,
        Schema::string(),
        true,
    );

    let cookie_param = <Parameter as ParameterExt>::new_simple(
        "session",
        ParameterLocation::Cookie,
        Schema::string(),
        false,
    );

    // Verify they serialize correctly
    let serialized = serde_json::to_value(&path_param).unwrap();
    assert_eq!(serialized["in"], "path");

    let serialized = serde_json::to_value(&query_param).unwrap();
    assert_eq!(serialized["in"], "query");

    let serialized = serde_json::to_value(&header_param).unwrap();
    assert_eq!(serialized["in"], "header");

    let serialized = serde_json::to_value(&cookie_param).unwrap();
    assert_eq!(serialized["in"], "cookie");
}

#[test]
fn test_openapi_multiple_parameters() {
    // Test multiple parameters in a single operation
    let mut operation = <Operation as OperationExt>::new();

    let params = vec![
        <Parameter as ParameterExt>::new_with_description(
            "id",
            ParameterLocation::Path,
            Schema::integer(),
            true,
            "Resource ID",
        ),
        <Parameter as ParameterExt>::new_with_description(
            "include",
            ParameterLocation::Query,
            Schema::string(),
            false,
            "Related resources to include",
        ),
        <Parameter as ParameterExt>::new_with_description(
            "Authorization",
            ParameterLocation::Header,
            Schema::string(),
            true,
            "Bearer token",
        ),
    ];

    for param in params {
        operation.add_parameter(param);
    }

    let op_params = operation.parameters.as_ref().unwrap();
    assert_eq!(op_params.len(), 3);
}

#[test]
fn test_parameter_required_optional() {
    // Test required and optional parameters
    let required_param = <Parameter as ParameterExt>::new_simple(
        "required_field",
        ParameterLocation::Query,
        Schema::string(),
        true,
    );

    let optional_param = <Parameter as ParameterExt>::new_simple(
        "optional_field",
        ParameterLocation::Query,
        Schema::string(),
        false,
    );

    // Verify via JSON serialization
    let json = serde_json::to_value(&required_param).unwrap();
    assert_eq!(json["required"], true);

    let json = serde_json::to_value(&optional_param).unwrap();
    assert_eq!(json["required"], false);
}

#[test]
fn test_parameter_with_description() {
    // Test parameters with descriptions
    let param = <Parameter as ParameterExt>::new_with_description(
        "page",
        ParameterLocation::Query,
        Schema::integer(),
        false,
        "Page number for pagination",
    );

    // Verify via JSON serialization
    let json = serde_json::to_value(&param).unwrap();
    assert_eq!(json["description"], "Page number for pagination");
}

#[test]
fn test_parameter_schema_types() {
    // Test different parameter schema types
    let int_param = <Parameter as ParameterExt>::new_simple(
        "count",
        ParameterLocation::Query,
        Schema::integer(),
        false,
    );

    let str_param = <Parameter as ParameterExt>::new_simple(
        "name",
        ParameterLocation::Query,
        Schema::string(),
        false,
    );

    // Verify via JSON serialization
    let int_json = serde_json::to_value(&int_param).unwrap();
    assert_eq!(int_json["schema"]["type"], "integer");

    let str_json = serde_json::to_value(&str_param).unwrap();
    assert_eq!(str_json["schema"]["type"], "string");
}
