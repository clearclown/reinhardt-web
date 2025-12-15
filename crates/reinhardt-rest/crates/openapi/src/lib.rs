//! # Reinhardt Schema Generation
//!
//! OpenAPI 3.0 schema generation for Reinhardt REST APIs.
//!
//! ## Features
//!
//! - **OpenAPI 3.0**: Full OpenAPI 3.0 specification support
//! - **Auto-generation**: Automatic schema generation from ViewSets
//! - **Customization**: Override and extend generated schemas
//! - **Swagger UI**: Built-in Swagger UI integration
//! - **YAML/JSON**: Export schemas in both formats
//! - **Schema Registry**: Centralized schema management with $ref references
//! - **Enum Support**: Tagged, adjacently tagged, and untagged enum handling
//! - **Serde Integration**: Support for `#[serde(rename)]`, `#[serde(skip)]`, and more
//!
//! ## Example
//!
//! ```rust,no_run
//! use reinhardt_openapi::{SchemaGenerator, OpenApiSchema};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Generate schema from ViewSets
//! let generator = SchemaGenerator::new()
//!     .title("My API")
//!     .version("1.0.0")
//!     .description("API documentation");
//!
//! let schema = generator.generate()?;
//! let json = schema.to_json()?;
//! # Ok(())
//! # }
//! ```

pub mod auto_schema;
pub mod config;
pub mod endpoint_inspector;
pub mod endpoints;
pub mod enum_schema;
pub mod generator;
pub mod openapi;
pub mod param_metadata;
pub mod registry;
pub mod router_wrapper;
pub mod schema_registration;
pub mod serde_attrs;
pub mod swagger;
pub mod viewset_inspector;

use thiserror::Error;

pub use auto_schema::{SchemaObject, ToSchema};
pub use config::OpenApiConfig;
pub use endpoint_inspector::EndpointInspector;
pub use enum_schema::{EnumSchemaBuilder, EnumTagging};
pub use generator::SchemaGenerator;
pub use openapi::{
	ArrayBuilder, Components, ComponentsExt, Header, Info, MediaType, ObjectBuilder, OpenApiSchema,
	OpenApiSchemaExt, Operation, OperationExt, Parameter, ParameterExt,
	ParameterIn as ParameterLocation, PathItem, PathItemExt, RefOr, RequestBody, Required,
	Response, ResponsesExt, Schema, SchemaExt, Server, Tag,
};
pub use param_metadata::{CookieParam, HeaderParam, ParameterMetadata, PathParam, QueryParam};
pub use registry::SchemaRegistry;
pub use reinhardt_openapi_macros::Schema;
pub use router_wrapper::OpenApiRouter;
pub use schema_registration::SchemaRegistration;
pub use serde_attrs::{FieldMetadata, RenameAll, SchemaBuilderExt};
pub use swagger::{RedocUI, SwaggerUI};
pub use utoipa::Number;
pub use viewset_inspector::{InspectorConfig, ViewSetInspector};

// Re-export utoipa and inventory for macro-generated code
pub use inventory;
pub use utoipa;

#[derive(Debug, Error)]
pub enum SchemaError {
	#[error("Invalid schema: {0}")]
	InvalidSchema(String),

	#[error("Serialization error: {0}")]
	SerializationError(String),

	#[error("Inspector error: {0}")]
	InspectorError(String),

	#[error("JSON error: {0}")]
	JsonError(#[from] serde_json::Error),
}

pub type SchemaResult<T> = std::result::Result<T, SchemaError>;
