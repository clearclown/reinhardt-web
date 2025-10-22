//! OpenAPI/Swagger schema generation
//!
//! Re-exports schema types from reinhardt-openapi.
//!
//! NOTE: This module is temporarily disabled pending reinhardt-openapi refactoring.
//!
//! The reinhardt-openapi crate is undergoing significant refactoring to improve
//! its utoipa integration and auto-schema generation capabilities. This module
//! will be re-enabled once the following conditions are met:
//!
//! 1. reinhardt-openapi provides stable public API exports
//! 2. Auto-schema generation is fully implemented (see reinhardt-openapi/src/auto_schema.rs)
//! 3. Integration tests validate the schema generation functionality
//!
//! For current schema generation needs, use reinhardt-openapi directly.

/*
// Re-export all schema types from reinhardt-openapi
pub use reinhardt_openapi::{
    auto_schema::{SchemaObject, ToSchema},
    generator::SchemaGenerator,
    inspector::ViewSetInspector,
    openapi::{
        Components, Contact, Info, License, MediaType, OpenApiSchema, Operation, Parameter,
        ParameterLocation, PathItem, RequestBody, Response, Schema, SecurityRequirement,
        SecurityScheme, Server, ServerVariable, Tag,
    },
    swagger::SwaggerUI,
    SchemaError, SchemaResult,
};

/// OpenAPI version constant
pub const OPENAPI_VERSION: &str = "3.0.3";
*/
