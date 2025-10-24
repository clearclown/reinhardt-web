//! Schema Construction Tests
//!
//! Tests for basic OpenAPI schema structure, info objects, and empty handling.

use openapi::{openapi::ContactBuilder, OpenApiSchema, OpenApiSchemaExt};
use utoipa::openapi::{LicenseBuilder, OpenApiVersion};

#[test]
fn test_schema_construction() {
    // Test basic schema structure with openapi version and paths
    let schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");

    // OpenApiVersion is an enum, check for Version3.1 (utoipa v5.x uses 3.1.0)
    assert!(matches!(schema.openapi, OpenApiVersion::Version31));
    assert_eq!(schema.info.title, "Test API");
    assert_eq!(schema.info.version, "1.0.0");
    assert!(schema.paths.paths.is_empty());
}

#[test]
fn test_schema_information() {
    // Test info object with title, version, and description
    let mut schema = <OpenApiSchema as OpenApiSchemaExt>::new("My API", "1.2.3");
    schema.info.description = Some("My description".to_string());

    assert_eq!(schema.info.title, "My API");
    assert_eq!(schema.info.version, "1.2.3");
    assert_eq!(schema.info.description, Some("My description".to_string()));
}

#[test]
fn test_schema_information_empty() {
    // Test empty info defaults
    let schema = <OpenApiSchema as OpenApiSchemaExt>::new("", "");

    assert_eq!(schema.info.title, "");
    assert_eq!(schema.info.version, "");
    assert_eq!(schema.info.description, None);
}

#[test]
fn test_schema_with_no_paths() {
    // Test empty paths handling
    let schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");

    assert!(schema.paths.paths.is_empty());
}

#[test]
fn test_schema_info_contact() {
    // Test contact information
    let mut schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");
    schema.info.contact = Some(
        ContactBuilder::new()
            .name(Some("Support Team"))
            .url(Some("https://example.com"))
            .email(Some("support@example.com"))
            .build(),
    );

    let contact = schema.info.contact.as_ref().unwrap();
    assert_eq!(contact.name, Some("Support Team".to_string()));
    assert_eq!(contact.url, Some("https://example.com".to_string()));
    assert_eq!(contact.email, Some("support@example.com".to_string()));
}

#[test]
fn test_schema_info_license() {
    // Test license information
    let mut schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");
    schema.info.license = Some(
        LicenseBuilder::new()
            .name("MIT")
            .url(Some("https://opensource.org/licenses/MIT"))
            .build(),
    );

    let license = schema.info.license.as_ref().unwrap();
    assert_eq!(license.name, "MIT");
    assert_eq!(
        license.url,
        Some("https://opensource.org/licenses/MIT".to_string())
    );
}

#[test]
fn test_schema_openapi_version() {
    // Test that OpenAPI version is correctly set
    let schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");

    // OpenApiVersion is an enum in utoipa v5.x
    assert!(matches!(schema.openapi, OpenApiVersion::Version31));
}
