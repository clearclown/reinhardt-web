//! Endpoint Inspector for Function-Based Routes
//!
//! Extracts endpoint metadata from HTTP method decorator macros
//! (#[get], #[post], etc.) using the inventory crate.

use crate::SchemaError;
use indexmap::IndexMap;
use regex::Regex;
use reinhardt_core::endpoint::EndpointMetadata;
use utoipa::openapi::{
	HttpMethod, PathItem, ResponseBuilder,
	path::{Operation, OperationBuilder, Parameter, ParameterBuilder, ParameterIn},
	schema::{ObjectBuilder, Schema, SchemaFormat, Type},
};

/// Configuration for endpoint inspection
#[derive(Debug, Clone)]
pub struct InspectorConfig {
	/// Whether to include function names in summaries
	pub include_function_names: bool,
	/// Default tag when module path inference fails
	pub default_tag: String,
}

impl Default for InspectorConfig {
	fn default() -> Self {
		Self {
			include_function_names: true,
			default_tag: "Default".to_string(),
		}
	}
}

/// Endpoint inspector for function-based routes
///
/// Extracts endpoint information from HTTP method decorator macros
/// using the inventory crate to collect metadata at compile time.
pub struct EndpointInspector {
	config: InspectorConfig,
}

impl EndpointInspector {
	/// Create a new endpoint inspector with default configuration
	pub fn new() -> Self {
		Self {
			config: InspectorConfig::default(),
		}
	}

	/// Create a new endpoint inspector with custom configuration
	pub fn with_config(config: InspectorConfig) -> Self {
		Self { config }
	}

	/// Extract all registered endpoints as OpenAPI paths
	///
	/// Collects endpoint metadata from the global inventory and
	/// generates OpenAPI path items for each endpoint.
	pub fn extract_paths(&self) -> Result<IndexMap<String, PathItem>, SchemaError> {
		let mut paths = IndexMap::new();

		for metadata in inventory::iter::<EndpointMetadata>() {
			let normalized_path = self.normalize_path(metadata.path);
			let path_item = self.create_path_item(metadata)?;

			// Merge with existing path item if the path already exists
			if let Some(existing) = paths.get_mut(&normalized_path) {
				self.merge_path_items(existing, path_item, metadata.method);
			} else {
				paths.insert(normalized_path, path_item);
			}
		}

		Ok(paths)
	}

	/// Normalize Django-style path to OpenAPI format
	///
	/// Converts patterns like:
	/// - `{<uuid:user_id>}` → `{user_id}`
	/// - `{<int:id>}` → `{id}`
	/// - `{<str:slug>}` → `{slug}`
	fn normalize_path(&self, path: &str) -> String {
		// Regex to match Django-style path parameters: {<type:name>}
		let re = Regex::new(r"\{<[^:]+:([^>]+)>\}").unwrap();
		re.replace_all(path, "{$1}").to_string()
	}

	/// Create a PathItem for a single endpoint
	fn create_path_item(&self, metadata: &EndpointMetadata) -> Result<PathItem, SchemaError> {
		let parameters = self.extract_path_parameters(metadata.path)?;
		let operation = self.create_operation(metadata, parameters);

		let http_method = match metadata.method {
			"GET" => HttpMethod::Get,
			"POST" => HttpMethod::Post,
			"PUT" => HttpMethod::Put,
			"PATCH" => HttpMethod::Patch,
			"DELETE" => HttpMethod::Delete,
			_ => {
				return Err(SchemaError::InspectorError(format!(
					"Unsupported HTTP method: {}",
					metadata.method
				)));
			}
		};

		Ok(PathItem::new(http_method, operation))
	}

	/// Merge a new operation into an existing PathItem
	fn merge_path_items(&self, _existing: &mut PathItem, _new: PathItem, method: &str) {
		// PathItem in utoipa 5.x doesn't support direct field assignment
		// We need to create a new PathItem with both operations
		// For now, we'll just warn about duplicate paths
		eprintln!(
			"Warning: Duplicate path with different method {}. Only the last one will be kept.",
			method
		);
	}

	/// Create an Operation object for an endpoint
	fn create_operation(
		&self,
		metadata: &EndpointMetadata,
		parameters: Vec<Parameter>,
	) -> Operation {
		let operation_id = metadata.name.unwrap_or(metadata.function_name).to_string();
		let summary = self.generate_summary(metadata);
		let tags = self.infer_tags(metadata.module_path);

		let mut builder = OperationBuilder::new()
			.operation_id(Some(operation_id))
			.summary(Some(summary))
			.tags(Some(tags));

		// Add parameters
		for param in parameters {
			builder = builder.parameter(param);
		}

		// Add default response
		builder = builder.response(
			"200",
			ResponseBuilder::new()
				.description("Successful response")
				.build(),
		);

		builder.build()
	}

	/// Extract path parameters from a path pattern
	///
	/// Parses Django-style path parameters and converts them to OpenAPI parameters.
	///
	/// Supported patterns:
	/// - `{<uuid:user_id>}` → name="user_id", type="string", format="uuid"
	/// - `{<int:id>}` → name="id", type="integer"
	/// - `{<str:slug>}` → name="slug", type="string"
	fn extract_path_parameters(&self, path: &str) -> Result<Vec<Parameter>, SchemaError> {
		let mut parameters = Vec::new();

		// Regex to match Django-style path parameters: {<type:name>}
		let re = Regex::new(r"\{<([^:]+):([^>]+)>\}").unwrap();

		for caps in re.captures_iter(path) {
			let type_str = caps.get(1).unwrap().as_str();
			let name = caps.get(2).unwrap().as_str();

			let (schema_type, format) = self.django_type_to_openapi(type_str);

			// Build schema based on type
			let mut schema_builder = ObjectBuilder::new().schema_type(schema_type);
			if let Some(fmt) = format {
				schema_builder = schema_builder.format(Some(fmt));
			}
			let schema = Schema::Object(schema_builder.build());

			let parameter = ParameterBuilder::new()
				.name(name)
				.parameter_in(ParameterIn::Path)
				.required(utoipa::openapi::Required::True)
				.schema(Some(schema))
				.build();

			parameters.push(parameter);
		}

		Ok(parameters)
	}

	/// Convert Django type specifier to OpenAPI type and format
	///
	/// Mappings:
	/// - `uuid` → (string, uuid)
	/// - `int` → (integer, None)
	/// - `str` → (string, None)
	/// - `slug` → (string, None)
	/// - `path` → (string, None)
	fn django_type_to_openapi(&self, django_type: &str) -> (Type, Option<SchemaFormat>) {
		match django_type {
			"uuid" => (Type::String, Some(SchemaFormat::Custom("uuid".to_string()))),
			"int" => (Type::Integer, None),
			"str" | "slug" | "path" => (Type::String, None),
			_ => (Type::String, None), // Default to string
		}
	}

	/// Generate a summary from endpoint metadata
	fn generate_summary(&self, metadata: &EndpointMetadata) -> String {
		if let Some(name) = metadata.name {
			if self.config.include_function_names {
				format!("{} ({})", name, metadata.function_name)
			} else {
				name.to_string()
			}
		} else if self.config.include_function_names {
			metadata.function_name.to_string()
		} else {
			format!("{} endpoint", metadata.method)
		}
	}

	/// Infer tags from module path
	///
	/// Examples:
	/// - `examples_twitter::apps::auth::views::register` → ["Auth"]
	/// - `examples_twitter::apps::profile::views::fetch_profile` → ["Profile"]
	/// - `examples_twitter::apps::dm::views::messages` → ["Dm"]
	fn infer_tags(&self, module_path: &str) -> Vec<String> {
		// Split module path by ::
		let parts: Vec<&str> = module_path.split("::").collect();

		// Look for pattern: *::apps::<app_name>::*
		for (i, part) in parts.iter().enumerate() {
			if *part == "apps" && i + 1 < parts.len() {
				let app_name = parts[i + 1];
				// Capitalize first letter
				let tag = app_name
					.chars()
					.enumerate()
					.map(|(idx, c)| {
						if idx == 0 {
							c.to_uppercase().collect::<String>()
						} else {
							c.to_string()
						}
					})
					.collect::<String>();
				return vec![tag];
			}
		}

		// Fallback to default tag
		vec![self.config.default_tag.clone()]
	}
}

impl Default for EndpointInspector {
	fn default() -> Self {
		Self::new()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_normalize_path() {
		let inspector = EndpointInspector::new();

		assert_eq!(
			inspector.normalize_path("/users/{<uuid:user_id>}/"),
			"/users/{user_id}/"
		);
		assert_eq!(
			inspector.normalize_path("/posts/{<int:id>}/"),
			"/posts/{id}/"
		);
		assert_eq!(
			inspector.normalize_path("/articles/{<str:slug>}/"),
			"/articles/{slug}/"
		);
	}

	#[test]
	fn test_django_type_to_openapi() {
		let inspector = EndpointInspector::new();

		let (uuid_type, uuid_format) = inspector.django_type_to_openapi("uuid");
		assert!(matches!(uuid_type, Type::String));
		assert!(matches!(uuid_format, Some(SchemaFormat::Custom(_))));

		let (int_type, int_format) = inspector.django_type_to_openapi("int");
		assert!(matches!(int_type, Type::Integer));
		assert!(int_format.is_none());

		let (str_type, str_format) = inspector.django_type_to_openapi("str");
		assert!(matches!(str_type, Type::String));
		assert!(str_format.is_none());
	}

	#[test]
	fn test_infer_tags() {
		let inspector = EndpointInspector::new();

		assert_eq!(
			inspector.infer_tags("examples_twitter::apps::auth::views::register"),
			vec!["Auth".to_string()]
		);
		assert_eq!(
			inspector.infer_tags("examples_twitter::apps::profile::views::fetch_profile"),
			vec!["Profile".to_string()]
		);
		assert_eq!(
			inspector.infer_tags("my_project::some_module::function"),
			vec!["Default".to_string()]
		);
	}
}
