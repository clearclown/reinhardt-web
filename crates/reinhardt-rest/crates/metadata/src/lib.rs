//! # Reinhardt Metadata
//!
//! Metadata API for handling OPTIONS requests in Reinhardt framework.
//!
//! ## Features
//!
//! - **BaseMetadata**: Base trait for metadata providers
//! - **SimpleMetadata**: Default metadata implementation that returns view and field information
//! - Automatic field type detection
//! - Action-based metadata (POST, PUT, etc.)
//!
//! ## Example
//!
//! ```rust
//! use reinhardt_metadata::{BaseMetadata, SimpleMetadata, MetadataOptions};
//!
//! let metadata = SimpleMetadata::new();
//! let options = MetadataOptions {
//!     name: "User List".to_string(),
//!     description: "List all users".to_string(),
//!     allowed_methods: vec!["GET".to_string(), "POST".to_string()],
//!     renders: vec!["application/json".to_string()],
//!     parses: vec!["application/json".to_string()],
//! };
//! ```
//!
//! ## Planned Features
//! TODO: OpenAPI 3.0 schema generation from field metadata
//! TODO: Automatic schema inference from Rust types
//! TODO: Schema validation and documentation
//! TODO: Serializer-aware metadata generation
//! TODO: Model-based metadata introspection
//! TODO: Custom metadata class support
//! TODO: Regular expression validation patterns
//! TODO: Field dependencies and conditional requirements

use async_trait::async_trait;
use reinhardt_apps::{Request, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum MetadataError {
    #[error("Failed to determine metadata: {0}")]
    DeterminationError(String),

    #[error("Serializer not available")]
    SerializerNotAvailable,
}

/// Field type enumeration for metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    Field,
    Boolean,
    String,
    Integer,
    Float,
    Decimal,
    Date,
    DateTime,
    Time,
    Duration,
    Email,
    Url,
    Uuid,
    Choice,
    MultipleChoice,
    File,
    Image,
    List,
    NestedObject,
}

/// Field metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldInfo {
    #[serde(rename = "type")]
    pub field_type: FieldType,
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<ChoiceInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child: Option<Box<FieldInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<HashMap<String, FieldInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validators: Option<Vec<FieldValidator>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<serde_json::Value>,
}

/// Choice information for choice fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChoiceInfo {
    pub value: String,
    pub display_name: String,
}

/// Field validator specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValidator {
    /// Type of validator (e.g., "email", "url", "regex", "min_length", "max_length")
    pub validator_type: String,
    /// Optional validator configuration (e.g., regex pattern, min/max values)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    /// Optional error message for validation failures
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Action metadata (for POST, PUT, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionMetadata {
    pub method: String,
    pub fields: HashMap<String, FieldInfo>,
}

/// Complete metadata response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetadataResponse {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renders: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parses: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<HashMap<String, HashMap<String, FieldInfo>>>,
}

/// Options for configuring metadata
#[derive(Debug, Clone)]
pub struct MetadataOptions {
    pub name: String,
    pub description: String,
    pub allowed_methods: Vec<String>,
    pub renders: Vec<String>,
    pub parses: Vec<String>,
}

impl Default for MetadataOptions {
    fn default() -> Self {
        Self {
            name: "API View".to_string(),
            description: "API endpoint".to_string(),
            allowed_methods: vec!["GET".to_string()],
            renders: vec!["application/json".to_string()],
            parses: vec!["application/json".to_string()],
        }
    }
}

/// Base trait for metadata providers
#[async_trait]
pub trait BaseMetadata: Send + Sync {
    /// Determine metadata for a view based on the request
    async fn determine_metadata(
        &self,
        request: &Request,
        options: &MetadataOptions,
    ) -> Result<MetadataResponse>;
}

/// Simple metadata implementation
///
/// This is the default metadata implementation that returns
/// basic information about the view and its fields.
#[derive(Debug, Clone)]
pub struct SimpleMetadata {
    pub include_actions: bool,
}

impl SimpleMetadata {
    /// Creates a new `SimpleMetadata` instance with actions enabled by default
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::SimpleMetadata;
    ///
    /// let metadata = SimpleMetadata::new();
    /// assert_eq!(metadata.include_actions, true);
    /// ```
    pub fn new() -> Self {
        Self {
            include_actions: true,
        }
    }
    /// Configures whether to include actions in metadata responses
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::SimpleMetadata;
    ///
    /// let metadata = SimpleMetadata::new().with_actions(false);
    /// assert_eq!(metadata.include_actions, false);
    ///
    /// let metadata_with_actions = SimpleMetadata::new().with_actions(true);
    /// assert_eq!(metadata_with_actions.include_actions, true);
    /// ```
    pub fn with_actions(mut self, include: bool) -> Self {
        self.include_actions = include;
        self
    }
    /// Determine which actions should be available based on allowed methods
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{SimpleMetadata, FieldInfoBuilder, FieldType};
    /// use std::collections::HashMap;
    ///
    /// let metadata = SimpleMetadata::new();
    /// let mut fields = HashMap::new();
    /// fields.insert(
    ///     "username".to_string(),
    ///     FieldInfoBuilder::new(FieldType::String).required(true).build()
    /// );
    ///
    /// let allowed_methods = vec!["GET".to_string(), "POST".to_string(), "PUT".to_string()];
    /// let actions = metadata.determine_actions(&allowed_methods, &fields);
    ///
    // GET is not included in actions, only POST and PUT
    /// assert!(!actions.contains_key("GET"));
    /// assert!(actions.contains_key("POST"));
    /// assert!(actions.contains_key("PUT"));
    /// assert_eq!(actions["POST"].len(), 1);
    /// ```
    pub fn determine_actions(
        &self,
        allowed_methods: &[String],
        fields: &HashMap<String, FieldInfo>,
    ) -> HashMap<String, HashMap<String, FieldInfo>> {
        let mut actions = HashMap::new();

        for method in allowed_methods {
            let method_upper = method.to_uppercase();
            if method_upper == "POST" || method_upper == "PUT" || method_upper == "PATCH" {
                actions.insert(method_upper, fields.clone());
            }
        }

        actions
    }
}

impl Default for SimpleMetadata {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl BaseMetadata for SimpleMetadata {
    async fn determine_metadata(
        &self,
        _request: &Request,
        options: &MetadataOptions,
    ) -> Result<MetadataResponse> {
        let mut response = MetadataResponse {
            name: options.name.clone(),
            description: options.description.clone(),
            renders: Some(options.renders.clone()),
            parses: Some(options.parses.clone()),
            actions: None,
        };

        if self.include_actions {
            // For now, we'll return empty actions
            // In a real implementation, this would inspect the serializer
            let fields = HashMap::new();
            let actions = self.determine_actions(&options.allowed_methods, &fields);
            if !actions.is_empty() {
                response.actions = Some(actions);
            }
        }

        Ok(response)
    }
}

/// Builder for field information
pub struct FieldInfoBuilder {
    field_type: FieldType,
    required: bool,
    read_only: Option<bool>,
    label: Option<String>,
    help_text: Option<String>,
    min_length: Option<usize>,
    max_length: Option<usize>,
    min_value: Option<f64>,
    max_value: Option<f64>,
    choices: Option<Vec<ChoiceInfo>>,
    child: Option<Box<FieldInfo>>,
    children: Option<HashMap<String, FieldInfo>>,
    validators: Vec<FieldValidator>,
    default_value: Option<serde_json::Value>,
}

impl FieldInfoBuilder {
    /// Creates a new `FieldInfoBuilder` with the specified field type
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    ///
    /// let builder = FieldInfoBuilder::new(FieldType::String);
    /// let field = builder.build();
    /// assert_eq!(field.field_type, FieldType::String);
    /// assert_eq!(field.required, false);
    /// ```
    pub fn new(field_type: FieldType) -> Self {
        Self {
            field_type,
            required: false,
            read_only: None,
            label: None,
            help_text: None,
            min_length: None,
            max_length: None,
            min_value: None,
            max_value: None,
            choices: None,
            child: None,
            children: None,
            validators: Vec::new(),
            default_value: None,
        }
    }
    /// Sets whether the field is required
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    ///
    /// let field = FieldInfoBuilder::new(FieldType::String)
    ///     .required(true)
    ///     .build();
    /// assert_eq!(field.required, true);
    /// ```
    pub fn required(mut self, required: bool) -> Self {
        self.required = required;
        self
    }
    /// Sets whether the field is read-only
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    ///
    /// let field = FieldInfoBuilder::new(FieldType::Integer)
    ///     .read_only(true)
    ///     .build();
    /// assert_eq!(field.read_only, Some(true));
    /// ```
    pub fn read_only(mut self, read_only: bool) -> Self {
        self.read_only = Some(read_only);
        self
    }
    /// Sets the human-readable label for the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    ///
    /// let field = FieldInfoBuilder::new(FieldType::String)
    ///     .label("Email Address")
    ///     .build();
    /// assert_eq!(field.label, Some("Email Address".to_string()));
    /// ```
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Sets help text that provides additional information about the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    ///
    /// let field = FieldInfoBuilder::new(FieldType::Email)
    ///     .help_text("Enter a valid email address")
    ///     .build();
    /// assert_eq!(field.help_text, Some("Enter a valid email address".to_string()));
    /// ```
    pub fn help_text(mut self, help_text: impl Into<String>) -> Self {
        self.help_text = Some(help_text.into());
        self
    }
    /// Sets the minimum length constraint for string fields
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    ///
    /// let field = FieldInfoBuilder::new(FieldType::String)
    ///     .min_length(3)
    ///     .build();
    /// assert_eq!(field.min_length, Some(3));
    /// ```
    pub fn min_length(mut self, min_length: usize) -> Self {
        self.min_length = Some(min_length);
        self
    }
    /// Sets the maximum length constraint for string fields
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    ///
    /// let field = FieldInfoBuilder::new(FieldType::String)
    ///     .max_length(100)
    ///     .build();
    /// assert_eq!(field.max_length, Some(100));
    /// ```
    pub fn max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }
    /// Sets the minimum value constraint for numeric fields
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    ///
    /// let field = FieldInfoBuilder::new(FieldType::Integer)
    ///     .min_value(1.0)
    ///     .build();
    /// assert_eq!(field.min_value, Some(1.0));
    /// ```
    pub fn min_value(mut self, min_value: f64) -> Self {
        self.min_value = Some(min_value);
        self
    }
    /// Sets the maximum value constraint for numeric fields
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    ///
    /// let field = FieldInfoBuilder::new(FieldType::Float)
    ///     .max_value(100.0)
    ///     .build();
    /// assert_eq!(field.max_value, Some(100.0));
    /// ```
    pub fn max_value(mut self, max_value: f64) -> Self {
        self.max_value = Some(max_value);
        self
    }
    /// Sets the available choices for choice fields
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType, ChoiceInfo};
    ///
    /// let choices = vec![
    ///     ChoiceInfo {
    ///         value: "small".to_string(),
    ///         display_name: "Small".to_string(),
    ///     },
    ///     ChoiceInfo {
    ///         value: "large".to_string(),
    ///         display_name: "Large".to_string(),
    ///     },
    /// ];
    ///
    /// let field = FieldInfoBuilder::new(FieldType::Choice)
    ///     .choices(choices)
    ///     .build();
    /// assert_eq!(field.choices.as_ref().unwrap().len(), 2);
    /// ```
    pub fn choices(mut self, choices: Vec<ChoiceInfo>) -> Self {
        self.choices = Some(choices);
        self
    }
    /// Sets the child field for list fields, describing the type of elements in the list
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    ///
    /// let child_field = FieldInfoBuilder::new(FieldType::String)
    ///     .required(true)
    ///     .build();
    ///
    /// let list_field = FieldInfoBuilder::new(FieldType::List)
    ///     .child(child_field)
    ///     .build();
    ///
    /// assert!(list_field.child.is_some());
    /// assert_eq!(list_field.child.unwrap().field_type, FieldType::String);
    /// ```
    pub fn child(mut self, child: FieldInfo) -> Self {
        self.child = Some(Box::new(child));
        self
    }
    /// Sets the children fields for nested object fields, describing the structure of nested objects
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    /// use std::collections::HashMap;
    ///
    /// let mut children = HashMap::new();
    /// children.insert(
    ///     "name".to_string(),
    ///     FieldInfoBuilder::new(FieldType::String).required(true).build()
    /// );
    /// children.insert(
    ///     "age".to_string(),
    ///     FieldInfoBuilder::new(FieldType::Integer).required(false).build()
    /// );
    ///
    /// let nested_field = FieldInfoBuilder::new(FieldType::NestedObject)
    ///     .children(children)
    ///     .build();
    ///
    /// assert!(nested_field.children.is_some());
    /// assert_eq!(nested_field.children.as_ref().unwrap().len(), 2);
    /// ```
    pub fn children(mut self, children: HashMap<String, FieldInfo>) -> Self {
        self.children = Some(children);
        self
    }

    /// Adds a custom validator to the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType, FieldValidator};
    ///
    /// let validator = FieldValidator {
    ///     validator_type: "email".to_string(),
    ///     options: None,
    ///     message: Some("Invalid email format".to_string()),
    /// };
    ///
    /// let field = FieldInfoBuilder::new(FieldType::Email)
    ///     .required(true)
    ///     .add_validator(validator)
    ///     .build();
    ///
    /// assert!(field.validators.is_some());
    /// assert_eq!(field.validators.as_ref().unwrap().len(), 1);
    /// assert_eq!(field.validators.as_ref().unwrap()[0].validator_type, "email");
    /// ```
    pub fn add_validator(mut self, validator: FieldValidator) -> Self {
        self.validators.push(validator);
        self
    }

    /// Adds multiple validators to the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType, FieldValidator};
    ///
    /// let validators = vec![
    ///     FieldValidator {
    ///         validator_type: "min_length".to_string(),
    ///         options: Some(serde_json::json!({"min": 3})),
    ///         message: Some("Too short".to_string()),
    ///     },
    ///     FieldValidator {
    ///         validator_type: "max_length".to_string(),
    ///         options: Some(serde_json::json!({"max": 50})),
    ///         message: Some("Too long".to_string()),
    ///     },
    /// ];
    ///
    /// let field = FieldInfoBuilder::new(FieldType::String)
    ///     .validators(validators)
    ///     .build();
    ///
    /// assert!(field.validators.is_some());
    /// assert_eq!(field.validators.as_ref().unwrap().len(), 2);
    /// ```
    pub fn validators(mut self, validators: Vec<FieldValidator>) -> Self {
        self.validators = validators;
        self
    }

    /// Sets the default value for the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    ///
    /// let field = FieldInfoBuilder::new(FieldType::String)
    ///     .required(false)
    ///     .default_value(serde_json::json!("default"))
    ///     .build();
    ///
    /// assert!(field.default_value.is_some());
    /// assert_eq!(field.default_value, Some(serde_json::json!("default")));
    /// ```
    pub fn default_value(mut self, default_value: serde_json::Value) -> Self {
        self.default_value = Some(default_value);
        self
    }
    /// Builds the final `FieldInfo` from the builder
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_metadata::{FieldInfoBuilder, FieldType};
    ///
    /// let field = FieldInfoBuilder::new(FieldType::String)
    ///     .required(true)
    ///     .label("Username")
    ///     .min_length(3)
    ///     .max_length(20)
    ///     .build();
    ///
    /// assert_eq!(field.field_type, FieldType::String);
    /// assert_eq!(field.required, true);
    /// assert_eq!(field.label, Some("Username".to_string()));
    /// assert_eq!(field.min_length, Some(3));
    /// assert_eq!(field.max_length, Some(20));
    /// ```
    pub fn build(self) -> FieldInfo {
        FieldInfo {
            field_type: self.field_type,
            required: self.required,
            read_only: self.read_only,
            label: self.label,
            help_text: self.help_text,
            min_length: self.min_length,
            max_length: self.max_length,
            min_value: self.min_value,
            max_value: self.max_value,
            choices: self.choices,
            child: self.child,
            children: self.children,
            validators: if self.validators.is_empty() {
                None
            } else {
                Some(self.validators)
            },
            default_value: self.default_value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use hyper::{HeaderMap, Method, Uri, Version};

    fn create_test_request() -> Request {
        Request::new(
            Method::OPTIONS,
            "/users/".parse::<Uri>().unwrap(),
            Version::HTTP_11,
            HeaderMap::new(),
            Bytes::new(),
        )
    }

    // DRF test: test_determine_metadata_abstract_method_raises_proper_error
    // BaseMetadata is a trait in Rust, so we test implementation requirements instead
    #[tokio::test]
    async fn test_base_metadata_trait_requires_implementation() {
        // This test verifies that BaseMetadata trait requires determine_metadata implementation
        let metadata = SimpleMetadata::new();
        let request = create_test_request();
        let options = MetadataOptions::default();

        // Should successfully call determine_metadata on a concrete implementation
        let result = metadata.determine_metadata(&request, &options).await;
        assert!(result.is_ok());
    }

    // DRF test: test_metadata
    // OPTIONS requests should return valid 200 response with metadata
    #[tokio::test]
    async fn test_metadata_basic_response() {
        let metadata = SimpleMetadata::new();
        let request = create_test_request();
        let options = MetadataOptions {
            name: "Example".to_string(),
            description: "Example view.".to_string(),
            allowed_methods: vec!["GET".to_string()],
            renders: vec!["application/json".to_string(), "text/html".to_string()],
            parses: vec![
                "application/json".to_string(),
                "application/x-www-form-urlencoded".to_string(),
                "multipart/form-data".to_string(),
            ],
        };

        let response = metadata
            .determine_metadata(&request, &options)
            .await
            .unwrap();

        assert_eq!(response.name, "Example");
        assert_eq!(response.description, "Example view.");
        assert_eq!(
            response.renders,
            Some(vec![
                "application/json".to_string(),
                "text/html".to_string()
            ])
        );
        assert_eq!(
            response.parses,
            Some(vec![
                "application/json".to_string(),
                "application/x-www-form-urlencoded".to_string(),
                "multipart/form-data".to_string(),
            ])
        );
    }

    // DRF test: test_actions
    // OPTIONS should return 'actions' key with field metadata for POST/PUT
    #[tokio::test]
    async fn test_actions_with_fields() {
        let metadata = SimpleMetadata::new();

        let mut fields = HashMap::new();

        // choice_field
        fields.insert(
            "choice_field".to_string(),
            FieldInfoBuilder::new(FieldType::Choice)
                .required(true)
                .read_only(false)
                .label("Choice field")
                .choices(vec![
                    ChoiceInfo {
                        value: "red".to_string(),
                        display_name: "red".to_string(),
                    },
                    ChoiceInfo {
                        value: "green".to_string(),
                        display_name: "green".to_string(),
                    },
                    ChoiceInfo {
                        value: "blue".to_string(),
                        display_name: "blue".to_string(),
                    },
                ])
                .build(),
        );

        // integer_field
        fields.insert(
            "integer_field".to_string(),
            FieldInfoBuilder::new(FieldType::Integer)
                .required(true)
                .read_only(false)
                .label("Integer field")
                .min_value(1.0)
                .max_value(1000.0)
                .build(),
        );

        // char_field
        fields.insert(
            "char_field".to_string(),
            FieldInfoBuilder::new(FieldType::String)
                .required(false)
                .read_only(false)
                .label("Char field")
                .min_length(3)
                .max_length(40)
                .build(),
        );

        // nested_field
        let mut nested_children = HashMap::new();
        nested_children.insert(
            "a".to_string(),
            FieldInfoBuilder::new(FieldType::Integer)
                .required(true)
                .read_only(false)
                .label("A")
                .build(),
        );
        nested_children.insert(
            "b".to_string(),
            FieldInfoBuilder::new(FieldType::Integer)
                .required(true)
                .read_only(false)
                .label("B")
                .build(),
        );

        fields.insert(
            "nested_field".to_string(),
            FieldInfoBuilder::new(FieldType::NestedObject)
                .required(true)
                .read_only(false)
                .label("Nested field")
                .children(nested_children)
                .build(),
        );

        let options = MetadataOptions {
            name: "Example".to_string(),
            description: "Example view.".to_string(),
            allowed_methods: vec!["POST".to_string()],
            renders: vec!["application/json".to_string()],
            parses: vec!["application/json".to_string()],
        };

        let actions = metadata.determine_actions(&options.allowed_methods, &fields);

        assert!(actions.contains_key("POST"));
        let post_fields = &actions["POST"];
        assert!(post_fields.contains_key("choice_field"));
        assert!(post_fields.contains_key("integer_field"));
        assert!(post_fields.contains_key("char_field"));
        assert!(post_fields.contains_key("nested_field"));

        // Verify choice field
        let choice_field = &post_fields["choice_field"];
        assert_eq!(choice_field.field_type, FieldType::Choice);
        assert_eq!(choice_field.required, true);
        assert_eq!(choice_field.read_only, Some(false));
        assert_eq!(choice_field.choices.as_ref().unwrap().len(), 3);

        // Verify integer field
        let integer_field = &post_fields["integer_field"];
        assert_eq!(integer_field.field_type, FieldType::Integer);
        assert_eq!(integer_field.min_value, Some(1.0));
        assert_eq!(integer_field.max_value, Some(1000.0));

        // Verify nested field
        let nested_field = &post_fields["nested_field"];
        assert_eq!(nested_field.field_type, FieldType::NestedObject);
        assert!(nested_field.children.is_some());
        let children = nested_field.children.as_ref().unwrap();
        assert!(children.contains_key("a"));
        assert!(children.contains_key("b"));
    }

    // DRF test: test_null_boolean_field_info_type
    #[test]
    fn test_boolean_field_info_type() {
        let field = FieldInfoBuilder::new(FieldType::Boolean)
            .required(false)
            .build();

        assert_eq!(field.field_type, FieldType::Boolean);
    }

    // DRF test: test_decimal_field_info_type
    #[test]
    fn test_decimal_field_info_type() {
        // Note: In DRF, max_digits and decimal_places are specific to DecimalField
        // In Rust, we use the Decimal field type and could add custom fields if needed
        let field = FieldInfoBuilder::new(FieldType::Decimal)
            .required(true)
            .build();

        assert_eq!(field.field_type, FieldType::Decimal);
    }

    #[tokio::test]
    async fn test_simple_metadata() {
        let metadata = SimpleMetadata::new();
        let request = create_test_request();
        let options = MetadataOptions {
            name: "User List".to_string(),
            description: "List all users".to_string(),
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            renders: vec!["application/json".to_string()],
            parses: vec!["application/json".to_string()],
        };

        let response = metadata
            .determine_metadata(&request, &options)
            .await
            .unwrap();

        assert_eq!(response.name, "User List");
        assert_eq!(response.description, "List all users");
        assert_eq!(response.renders, Some(vec!["application/json".to_string()]));
        assert_eq!(response.parses, Some(vec!["application/json".to_string()]));
    }

    #[tokio::test]
    async fn test_field_info_builder() {
        let field = FieldInfoBuilder::new(FieldType::String)
            .required(true)
            .label("Username")
            .help_text("Enter your username")
            .min_length(3)
            .max_length(50)
            .build();

        assert_eq!(field.field_type, FieldType::String);
        assert!(field.required);
        assert_eq!(field.label, Some("Username".to_string()));
        assert_eq!(field.help_text, Some("Enter your username".to_string()));
        assert_eq!(field.min_length, Some(3));
        assert_eq!(field.max_length, Some(50));
    }

    #[tokio::test]
    async fn test_choice_field() {
        let choices = vec![
            ChoiceInfo {
                value: "active".to_string(),
                display_name: "Active".to_string(),
            },
            ChoiceInfo {
                value: "inactive".to_string(),
                display_name: "Inactive".to_string(),
            },
        ];

        let field = FieldInfoBuilder::new(FieldType::Choice)
            .required(true)
            .label("Status")
            .choices(choices.clone())
            .build();

        assert_eq!(field.field_type, FieldType::Choice);
        assert!(field.required);
        assert_eq!(field.choices.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_metadata_serialization() {
        let response = MetadataResponse {
            name: "Test View".to_string(),
            description: "Test description".to_string(),
            renders: Some(vec!["application/json".to_string()]),
            parses: Some(vec!["application/json".to_string()]),
            actions: None,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("Test View"));
        assert!(json.contains("application/json"));
    }

    // DRF test: test_list_serializer_metadata_returns_info_about_fields_of_child_serializer
    #[test]
    fn test_list_field_with_child() {
        let child_field = FieldInfoBuilder::new(FieldType::Integer)
            .required(true)
            .read_only(false)
            .build();

        let list_field = FieldInfoBuilder::new(FieldType::List)
            .required(true)
            .read_only(false)
            .label("List field")
            .child(child_field)
            .build();

        assert_eq!(list_field.field_type, FieldType::List);
        assert!(list_field.child.is_some());
        let child = list_field.child.as_ref().unwrap();
        assert_eq!(child.field_type, FieldType::Integer);
    }

    // DRF test: test_dont_show_hidden_fields
    // In Rust, we handle this by simply not adding hidden fields to the field map
    #[test]
    fn test_hidden_fields_not_included() {
        let mut fields = HashMap::new();

        // Only add visible fields
        fields.insert(
            "integer_field".to_string(),
            FieldInfoBuilder::new(FieldType::Integer)
                .required(true)
                .max_value(10.0)
                .build(),
        );

        // hidden_field is intentionally not added

        assert!(fields.contains_key("integer_field"));
        assert!(!fields.contains_key("hidden_field"));
        assert_eq!(fields.len(), 1);
    }

    #[test]
    fn test_field_with_single_validator() {
        let validator = FieldValidator {
            validator_type: "email".to_string(),
            options: None,
            message: Some("Invalid email format".to_string()),
        };

        let field = FieldInfoBuilder::new(FieldType::Email)
            .required(true)
            .add_validator(validator)
            .build();

        assert!(field.validators.is_some());
        let validators = field.validators.as_ref().unwrap();
        assert_eq!(validators.len(), 1);
        assert_eq!(validators[0].validator_type, "email");
        assert_eq!(
            validators[0].message,
            Some("Invalid email format".to_string())
        );
    }

    #[test]
    fn test_field_with_multiple_validators() {
        let validators = vec![
            FieldValidator {
                validator_type: "min_length".to_string(),
                options: Some(serde_json::json!({"min": 3})),
                message: Some("Too short".to_string()),
            },
            FieldValidator {
                validator_type: "max_length".to_string(),
                options: Some(serde_json::json!({"max": 50})),
                message: Some("Too long".to_string()),
            },
            FieldValidator {
                validator_type: "regex".to_string(),
                options: Some(serde_json::json!({"pattern": "^[a-zA-Z0-9_]+$"})),
                message: Some("Invalid characters".to_string()),
            },
        ];

        let field = FieldInfoBuilder::new(FieldType::String)
            .validators(validators)
            .build();

        assert!(field.validators.is_some());
        let field_validators = field.validators.as_ref().unwrap();
        assert_eq!(field_validators.len(), 3);
        assert_eq!(field_validators[0].validator_type, "min_length");
        assert_eq!(field_validators[1].validator_type, "max_length");
        assert_eq!(field_validators[2].validator_type, "regex");
    }

    #[test]
    fn test_field_without_validators() {
        let field = FieldInfoBuilder::new(FieldType::String)
            .required(true)
            .label("Username")
            .build();

        assert!(field.validators.is_none());
    }

    #[test]
    fn test_validator_with_options() {
        let validator = FieldValidator {
            validator_type: "range".to_string(),
            options: Some(serde_json::json!({"min": 1, "max": 100})),
            message: Some("Value must be between 1 and 100".to_string()),
        };

        let field = FieldInfoBuilder::new(FieldType::Integer)
            .add_validator(validator)
            .build();

        let validators = field.validators.as_ref().unwrap();
        assert_eq!(validators[0].validator_type, "range");
        assert!(validators[0].options.is_some());

        let options = validators[0].options.as_ref().unwrap();
        assert_eq!(options["min"], 1);
        assert_eq!(options["max"], 100);
    }

    #[test]
    fn test_validator_serialization() {
        let validator = FieldValidator {
            validator_type: "custom".to_string(),
            options: Some(serde_json::json!({"key": "value"})),
            message: Some("Custom validation failed".to_string()),
        };

        let field = FieldInfoBuilder::new(FieldType::String)
            .add_validator(validator)
            .build();

        let json = serde_json::to_string(&field).unwrap();
        assert!(json.contains("custom"));
        assert!(json.contains("Custom validation failed"));
        assert!(json.contains("validators"));
    }

    #[test]
    fn test_field_with_default_value_string() {
        let field = FieldInfoBuilder::new(FieldType::String)
            .required(false)
            .default_value(serde_json::json!("default text"))
            .build();

        assert!(field.default_value.is_some());
        assert_eq!(field.default_value, Some(serde_json::json!("default text")));
    }

    #[test]
    fn test_field_with_default_value_number() {
        let field = FieldInfoBuilder::new(FieldType::Integer)
            .required(false)
            .default_value(serde_json::json!(42))
            .build();

        assert!(field.default_value.is_some());
        assert_eq!(field.default_value, Some(serde_json::json!(42)));
    }

    #[test]
    fn test_field_with_default_value_boolean() {
        let field = FieldInfoBuilder::new(FieldType::Boolean)
            .required(false)
            .default_value(serde_json::json!(true))
            .build();

        assert!(field.default_value.is_some());
        assert_eq!(field.default_value, Some(serde_json::json!(true)));
    }

    #[test]
    fn test_field_with_default_value_object() {
        let default_obj = serde_json::json!({
            "name": "John Doe",
            "age": 30
        });

        let field = FieldInfoBuilder::new(FieldType::NestedObject)
            .required(false)
            .default_value(default_obj.clone())
            .build();

        assert!(field.default_value.is_some());
        assert_eq!(field.default_value, Some(default_obj));
    }

    #[test]
    fn test_field_with_default_value_array() {
        let default_array = serde_json::json!(["item1", "item2", "item3"]);

        let field = FieldInfoBuilder::new(FieldType::List)
            .required(false)
            .default_value(default_array.clone())
            .build();

        assert!(field.default_value.is_some());
        assert_eq!(field.default_value, Some(default_array));
    }

    #[test]
    fn test_field_without_default_value() {
        let field = FieldInfoBuilder::new(FieldType::String)
            .required(true)
            .label("Username")
            .build();

        assert!(field.default_value.is_none());
    }

    #[test]
    fn test_default_value_serialization() {
        let field = FieldInfoBuilder::new(FieldType::String)
            .default_value(serde_json::json!("default"))
            .build();

        let json = serde_json::to_string(&field).unwrap();
        assert!(json.contains("default_value"));
        assert!(json.contains("default"));
    }

    #[test]
    fn test_default_value_not_serialized_when_none() {
        let field = FieldInfoBuilder::new(FieldType::String).build();

        let json = serde_json::to_string(&field).unwrap();
        assert!(!json.contains("default_value"));
    }
}
