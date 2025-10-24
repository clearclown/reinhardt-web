//! OpenAPI 3.0 types with Reinhardt extensions
//!
//! This module re-exports utoipa's OpenAPI types and provides
//! convenient helper functions and extension traits for easier usage.

// Re-export core utoipa types as Reinhardt's OpenAPI types
pub use utoipa::openapi::{
    Components, Contact, Header, Info, License, OpenApi as OpenApiSchema, PathItem, Paths, RefOr,
    Required, Schema, Server, Tag,
};

// Re-export request/response types
pub use utoipa::openapi::request_body::RequestBody;
pub use utoipa::openapi::response::{Response, Responses};

// Re-export path operation types
pub use utoipa::openapi::path::{Operation, Parameter, ParameterIn};

// Re-export content-related types (MediaType)
pub use utoipa::openapi::Content as MediaType;

// Re-export path-related types
pub use utoipa::openapi::path::ParameterIn as ParameterLocation;

// Re-export security-related types
pub use utoipa::openapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme};

// Provide convenient type alias for API key location
pub type ApiKeyLocation = utoipa::openapi::security::ApiKeyValue;

// Re-export HttpScheme for convenience
pub type HttpScheme = HttpAuthScheme;

// Re-export builders
pub use utoipa::openapi::path::{OperationBuilder, ParameterBuilder, PathItemBuilder};
pub use utoipa::openapi::request_body::RequestBodyBuilder;
pub use utoipa::openapi::response::{ResponseBuilder, ResponsesBuilder};
pub use utoipa::openapi::schema::{ArrayBuilder, ObjectBuilder, SchemaType};
pub use utoipa::openapi::tag::TagBuilder;
pub use utoipa::openapi::{
    ComponentsBuilder, ContactBuilder, InfoBuilder, OpenApiBuilder, PathsBuilder, ServerBuilder,
};

/// Extension trait for Schema to provide convenient constructor methods
pub trait SchemaExt {
    /// Create a string schema
    fn string() -> Schema;

    /// Create an integer schema
    fn integer() -> Schema;

    /// Create a number (float) schema
    fn number() -> Schema;

    /// Create a boolean schema
    fn boolean() -> Schema;

    /// Create an empty object schema
    fn object() -> Schema;

    /// Create a date schema (string with format: "date")
    fn date() -> Schema;

    /// Create a datetime schema (string with format: "date-time")
    fn datetime() -> Schema;

    /// Create an array schema with the given item schema
    fn array(items: Schema) -> Schema;

    /// Create an object schema with properties and required fields
    fn object_with_properties(
        properties: Vec<(impl Into<String>, Schema)>,
        required: Vec<impl Into<String>>,
    ) -> Schema;

    /// Create an object schema with properties, required fields, and description
    fn object_with_description(
        properties: Vec<(impl Into<String>, Schema)>,
        required: Vec<impl Into<String>>,
        description: impl Into<String>,
    ) -> Schema;
}

impl SchemaExt for Schema {
    fn string() -> Schema {
        Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::String))
                .build(),
        )
    }

    fn integer() -> Schema {
        Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::Integer))
                .build(),
        )
    }

    fn number() -> Schema {
        Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::Number))
                .build(),
        )
    }

    fn boolean() -> Schema {
        Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::Boolean))
                .build(),
        )
    }

    fn object() -> Schema {
        Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::Object))
                .build(),
        )
    }

    fn date() -> Schema {
        Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::String))
                .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                    utoipa::openapi::KnownFormat::Date,
                )))
                .build(),
        )
    }

    fn datetime() -> Schema {
        Schema::Object(
            ObjectBuilder::new()
                .schema_type(SchemaType::Type(utoipa::openapi::Type::String))
                .format(Some(utoipa::openapi::SchemaFormat::KnownFormat(
                    utoipa::openapi::KnownFormat::DateTime,
                )))
                .build(),
        )
    }

    fn array(items: Schema) -> Schema {
        Schema::Array(ArrayBuilder::new().items(RefOr::T(items)).build())
    }

    fn object_with_properties(
        properties: Vec<(impl Into<String>, Schema)>,
        required: Vec<impl Into<String>>,
    ) -> Schema {
        let mut builder =
            ObjectBuilder::new().schema_type(SchemaType::Type(utoipa::openapi::Type::Object));

        for (name, schema) in properties {
            builder = builder.property(name, schema);
        }

        for req in required {
            builder = builder.required(req);
        }

        Schema::Object(builder.build())
    }

    fn object_with_description(
        properties: Vec<(impl Into<String>, Schema)>,
        required: Vec<impl Into<String>>,
        description: impl Into<String>,
    ) -> Schema {
        let mut builder = ObjectBuilder::new()
            .schema_type(SchemaType::Type(utoipa::openapi::Type::Object))
            .description(Some(description.into()));

        for (name, schema) in properties {
            builder = builder.property(name, schema);
        }

        for req in required {
            builder = builder.required(req);
        }

        Schema::Object(builder.build())
    }
}

/// Extension trait for OpenApiSchema to provide convenient methods
pub trait OpenApiSchemaExt {
    /// Create a new OpenApiSchema with title and version
    fn new(title: impl Into<String>, version: impl Into<String>) -> OpenApiSchema;

    /// Add a path to the schema
    fn add_path(&mut self, path: String, item: PathItem);

    /// Add a tag to the schema
    fn add_tag(&mut self, name: String, description: Option<String>);
}

impl OpenApiSchemaExt for OpenApiSchema {
    fn new(title: impl Into<String>, version: impl Into<String>) -> OpenApiSchema {
        OpenApiBuilder::new()
            .info(InfoBuilder::new().title(title).version(version).build())
            .build()
    }

    fn add_path(&mut self, path: String, item: PathItem) {
        self.paths.paths.insert(path, item);
    }

    fn add_tag(&mut self, name: String, description: Option<String>) {
        let mut builder = TagBuilder::new().name(name);
        if let Some(desc) = description {
            builder = builder.description(Some(desc));
        }
        let tag = builder.build();

        if self.tags.is_none() {
            self.tags = Some(Vec::new());
        }
        if let Some(tags) = &mut self.tags {
            tags.push(tag);
        }
    }
}

/// Extension trait for Operation to provide convenient methods
pub trait OperationExt {
    /// Create a new Operation with default values
    fn new() -> Operation;

    /// Add a parameter to the operation
    fn add_parameter(&mut self, parameter: Parameter);

    /// Add a response to the operation
    fn add_response(&mut self, status: impl Into<String>, response: Response);
}

impl OperationExt for Operation {
    fn new() -> Operation {
        // Operation is non-exhaustive, so we must use Default
        Default::default()
    }

    fn add_parameter(&mut self, parameter: Parameter) {
        if self.parameters.is_none() {
            self.parameters = Some(Vec::new());
        }
        if let Some(params) = &mut self.parameters {
            params.push(parameter.into());
        }
    }

    fn add_response(&mut self, status: impl Into<String>, response: Response) {
        self.responses
            .responses
            .insert(status.into(), response.into());
    }
}

/// Extension trait for Responses to provide collection methods
pub trait ResponsesExt {
    /// Get the number of responses
    fn len(&self) -> usize;

    /// Check if responses collection is empty
    fn is_empty(&self) -> bool;

    /// Check if a specific status code exists
    fn contains_key(&self, status: &str) -> bool;
}

impl ResponsesExt for Responses {
    fn len(&self) -> usize {
        self.responses.len()
    }

    fn is_empty(&self) -> bool {
        self.responses.is_empty()
    }

    fn contains_key(&self, status: &str) -> bool {
        self.responses.contains_key(status)
    }
}

/// Extension trait for Components to provide convenient methods
pub trait ComponentsExt {
    /// Add a schema to the components
    fn add_schema(&mut self, name: String, schema: Schema);
}

impl ComponentsExt for Components {
    fn add_schema(&mut self, name: String, schema: Schema) {
        self.schemas.insert(name, schema.into());
    }
}

/// Extension trait for PathItem to provide constructor
pub trait PathItemExt {
    /// Create a new PathItem
    fn new() -> PathItem;
}

impl PathItemExt for PathItem {
    fn new() -> PathItem {
        PathItem::default()
    }
}

/// Extension trait for Parameter to provide convenient constructors
pub trait ParameterExt {
    /// Create a new Parameter with ParameterBuilder
    fn new_simple(
        name: impl Into<String>,
        location: ParameterIn,
        schema: Schema,
        required: bool,
    ) -> Parameter;

    /// Create a new Parameter with description
    fn new_with_description(
        name: impl Into<String>,
        location: ParameterIn,
        schema: Schema,
        required: bool,
        description: impl Into<String>,
    ) -> Parameter;
}

impl ParameterExt for Parameter {
    fn new_simple(
        name: impl Into<String>,
        location: ParameterIn,
        schema: Schema,
        required: bool,
    ) -> Parameter {
        ParameterBuilder::new()
            .name(name)
            .parameter_in(location)
            .schema(Some(schema))
            .required(if required {
                utoipa::openapi::Required::True
            } else {
                utoipa::openapi::Required::False
            })
            .build()
    }

    fn new_with_description(
        name: impl Into<String>,
        location: ParameterIn,
        schema: Schema,
        required: bool,
        description: impl Into<String>,
    ) -> Parameter {
        ParameterBuilder::new()
            .name(name)
            .parameter_in(location)
            .schema(Some(schema))
            .required(if required {
                utoipa::openapi::Required::True
            } else {
                utoipa::openapi::Required::False
            })
            .description(Some(description.into()))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_helpers() {
        let string_schema = Schema::string();
        let integer_schema = Schema::integer();
        let boolean_schema = Schema::boolean();

        // These should not panic and should create valid schemas
        assert!(matches!(string_schema, Schema::Object(_)));
        assert!(matches!(integer_schema, Schema::Object(_)));
        assert!(matches!(boolean_schema, Schema::Object(_)));
    }

    #[test]
    fn test_openapi_schema_new() {
        let schema = <OpenApiSchema as OpenApiSchemaExt>::new("Test API", "1.0.0");

        assert_eq!(schema.info.title, "Test API");
        assert_eq!(schema.info.version, "1.0.0");
    }

    #[test]
    fn test_operation_ext() {
        let mut operation = <Operation as OperationExt>::new();
        let param = ParameterBuilder::new()
            .name("id")
            .parameter_in(ParameterIn::Path)
            .required(Required::True)
            .build();

        operation.add_parameter(param);

        assert!(operation.parameters.is_some());
        assert_eq!(operation.parameters.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_responses_ext() {
        let response = ResponseBuilder::new().description("Success").build();

        let mut responses = ResponsesBuilder::new().build();
        responses
            .responses
            .insert("200".to_string(), response.into());

        assert_eq!(responses.len(), 1);
        assert!(!responses.is_empty());
        assert!(responses.contains_key("200"));
    }
}
