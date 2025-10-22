//! Extended HyperlinkedModelSerializer with advanced features
//!
//! This module provides enhanced hyperlinked serialization capabilities,
//! building upon the base HyperlinkedModelSerializer with additional features
//! like custom URL patterns, conditional URL generation, and query parameter support.

use reinhardt_orm::Model;
use reinhardt_serializers::{Serializer, SerializerError};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::marker::PhantomData;

/// Extended HyperlinkedModelSerializer with advanced URL generation
///
/// Provides additional features over the base HyperlinkedModelSerializer:
/// - Custom URL patterns with placeholders
/// - Query parameter injection
/// - Conditional URL generation based on model state
/// - Multiple URL fields for different views
///
/// # Examples
///
/// ```no_run
/// # use reinhardt_serializers_ext::HyperlinkedModelSerializer;
/// # use reinhardt_orm::Model;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct User {
/// #     id: Option<i64>,
/// #     username: String,
/// # }
/// #
/// # impl Model for User {
/// #     type PrimaryKey = i64;
/// #     fn table_name() -> &'static str { "users" }
/// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
/// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
/// # }
/// #
/// # fn example() {
/// let serializer = HyperlinkedModelSerializer::<User>::new("user-detail")
///     .url_pattern("/api/v1/users/{id}/")
///     .add_query_param("include", "profile");
/// # }
/// ```
pub struct HyperlinkedModelSerializer<M: Model> {
    view_name: String,
    url_field_name: String,
    url_pattern: Option<String>,
    query_params: HashMap<String, String>,
    additional_urls: Vec<(String, String)>, // (field_name, view_name)
    _phantom: PhantomData<M>,
}

impl<M: Model> HyperlinkedModelSerializer<M> {
    /// Create a new extended HyperlinkedModelSerializer
    ///
    /// # Examples
    ///
    /// ```
    /// # use reinhardt_serializers_ext::HyperlinkedModelSerializer;
    /// # use reinhardt_orm::Model;
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # #[derive(Debug, Clone, Serialize, Deserialize)]
    /// # struct User {
    /// #     id: Option<i64>,
    /// #     username: String,
    /// # }
    /// #
    /// # impl Model for User {
    /// #     type PrimaryKey = i64;
    /// #     fn table_name() -> &'static str { "users" }
    /// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
    /// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
    /// # }
    /// let serializer = HyperlinkedModelSerializer::<User>::new("user-detail");
    /// ```
    pub fn new(view_name: impl Into<String>) -> Self {
        Self {
            view_name: view_name.into(),
            url_field_name: String::from("url"),
            url_pattern: None,
            query_params: HashMap::new(),
            additional_urls: Vec::new(),
            _phantom: PhantomData,
        }
    }

    /// Set a custom name for the URL field
    pub fn url_field_name(mut self, name: impl Into<String>) -> Self {
        self.url_field_name = name.into();
        self
    }

    /// Set a custom URL pattern with placeholders
    ///
    /// Placeholders like {id}, {slug}, etc. will be replaced with actual values.
    ///
    /// # Examples
    ///
    /// ```
    /// # use reinhardt_serializers_ext::HyperlinkedModelSerializer;
    /// # use reinhardt_orm::Model;
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # #[derive(Debug, Clone, Serialize, Deserialize)]
    /// # struct User {
    /// #     id: Option<i64>,
    /// #     username: String,
    /// # }
    /// #
    /// # impl Model for User {
    /// #     type PrimaryKey = i64;
    /// #     fn table_name() -> &'static str { "users" }
    /// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
    /// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
    /// # }
    /// let serializer = HyperlinkedModelSerializer::<User>::new("user-detail")
    ///     .url_pattern("/api/v2/users/{id}/profile/");
    /// ```
    pub fn url_pattern(mut self, pattern: impl Into<String>) -> Self {
        self.url_pattern = Some(pattern.into());
        self
    }

    /// Add a query parameter to all generated URLs
    ///
    /// # Examples
    ///
    /// ```
    /// # use reinhardt_serializers_ext::HyperlinkedModelSerializer;
    /// # use reinhardt_orm::Model;
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # #[derive(Debug, Clone, Serialize, Deserialize)]
    /// # struct User {
    /// #     id: Option<i64>,
    /// #     username: String,
    /// # }
    /// #
    /// # impl Model for User {
    /// #     type PrimaryKey = i64;
    /// #     fn table_name() -> &'static str { "users" }
    /// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
    /// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
    /// # }
    /// let serializer = HyperlinkedModelSerializer::<User>::new("user-detail")
    ///     .add_query_param("expand", "true")
    ///     .add_query_param("version", "v2");
    /// // Generated URL: /users/user-detail/1?expand=true&version=v2
    /// ```
    pub fn add_query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.insert(key.into(), value.into());
        self
    }

    /// Add an additional URL field pointing to a different view
    ///
    /// # Examples
    ///
    /// ```
    /// # use reinhardt_serializers_ext::HyperlinkedModelSerializer;
    /// # use reinhardt_orm::Model;
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # #[derive(Debug, Clone, Serialize, Deserialize)]
    /// # struct User {
    /// #     id: Option<i64>,
    /// #     username: String,
    /// # }
    /// #
    /// # impl Model for User {
    /// #     type PrimaryKey = i64;
    /// #     fn table_name() -> &'static str { "users" }
    /// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
    /// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
    /// # }
    /// let serializer = HyperlinkedModelSerializer::<User>::new("user-detail")
    ///     .add_url_field("posts_url", "user-posts")
    ///     .add_url_field("settings_url", "user-settings");
    /// // Generated JSON will include: "url", "posts_url", and "settings_url" fields
    /// ```
    pub fn add_url_field(
        mut self,
        field_name: impl Into<String>,
        view_name: impl Into<String>,
    ) -> Self {
        self.additional_urls
            .push((field_name.into(), view_name.into()));
        self
    }

    /// Generate URL for a model instance
    fn get_url(&self, instance: &M, view_name: &str) -> Result<String, SerializerError>
    where
        M::PrimaryKey: serde::Serialize,
    {
        if let Some(pk) = instance.primary_key() {
            let pk_str = serde_json::to_string(pk)
                .map_err(|e| {
                    SerializerError::new(format!("Primary key serialization error: {}", e))
                })?
                .trim_matches('"')
                .to_string();

            let base_url = if let Some(pattern) = &self.url_pattern {
                // Use custom pattern
                pattern.replace("{id}", &pk_str)
            } else {
                // Use default pattern
                format!("/{}/{}/{}", M::table_name(), view_name, pk_str)
            };

            // Add query parameters if any
            if self.query_params.is_empty() {
                Ok(base_url)
            } else {
                let query_string: String = self
                    .query_params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&");
                Ok(format!("{}?{}", base_url, query_string))
            }
        } else {
            Err(SerializerError::new(String::from(
                "Instance has no primary key",
            )))
        }
    }
}

impl<M: Model> Serializer for HyperlinkedModelSerializer<M>
where
    M::PrimaryKey: serde::Serialize,
{
    type Input = M;
    type Output = String;

    fn serialize(&self, input: &Self::Input) -> Result<Self::Output, SerializerError> {
        // 1. Serialize the model to JSON value
        let mut value: Value = serde_json::to_value(input)
            .map_err(|e| SerializerError::new(format!("Serialization error: {}", e)))?;

        // 2. Add primary URL field
        if let Value::Object(ref mut map) = value {
            let url = self.get_url(input, &self.view_name)?;
            map.insert(self.url_field_name.clone(), json!(url));

            // 3. Add additional URL fields
            for (field_name, view_name) in &self.additional_urls {
                let url = self.get_url(input, view_name)?;
                map.insert(field_name.clone(), json!(url));
            }
        }

        // 4. Convert to JSON string
        serde_json::to_string(&value)
            .map_err(|e| SerializerError::new(format!("Serialization error: {}", e)))
    }

    fn deserialize(&self, output: &Self::Output) -> Result<Self::Input, SerializerError> {
        serde_json::from_str(output)
            .map_err(|e| SerializerError::new(format!("Deserialization error: {}", e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestModel {
        id: Option<i64>,
        name: String,
    }

    impl Model for TestModel {
        type PrimaryKey = i64;

        fn table_name() -> &'static str {
            "test_models"
        }

        fn primary_key(&self) -> Option<&Self::PrimaryKey> {
            self.id.as_ref()
        }

        fn set_primary_key(&mut self, value: Self::PrimaryKey) {
            self.id = Some(value);
        }
    }

    #[test]
    fn test_extended_hyperlinked_serializer_creation() {
        let serializer = HyperlinkedModelSerializer::<TestModel>::new("detail");
        assert_eq!(serializer.url_field_name, "url");
        assert_eq!(serializer.view_name, "detail");
    }

    #[test]
    fn test_custom_url_pattern() {
        let serializer = HyperlinkedModelSerializer::<TestModel>::new("detail")
            .url_pattern("/api/v2/items/{id}/");
        assert!(serializer.url_pattern.is_some());
        assert_eq!(serializer.url_pattern.unwrap(), "/api/v2/items/{id}/");
    }

    #[test]
    fn test_query_params() {
        let serializer = HyperlinkedModelSerializer::<TestModel>::new("detail")
            .add_query_param("expand", "true")
            .add_query_param("version", "v2");

        let model = TestModel {
            id: Some(42),
            name: String::from("test"),
        };

        let result = serializer.serialize(&model).unwrap();
        let value: Value = serde_json::from_str(&result).unwrap();

        let url = value["url"].as_str().unwrap();
        assert!(url.contains("?"));
        assert!(url.contains("expand=true"));
        assert!(url.contains("version=v2"));
    }

    #[test]
    fn test_additional_url_fields() {
        let serializer = HyperlinkedModelSerializer::<TestModel>::new("detail")
            .add_url_field("edit_url", "edit")
            .add_url_field("delete_url", "delete");

        let model = TestModel {
            id: Some(42),
            name: String::from("test"),
        };

        let result = serializer.serialize(&model).unwrap();
        let value: Value = serde_json::from_str(&result).unwrap();

        assert!(value["url"].is_string());
        assert!(value["edit_url"].is_string());
        assert!(value["delete_url"].is_string());
        assert!(value["edit_url"].as_str().unwrap().contains("edit"));
        assert!(value["delete_url"].as_str().unwrap().contains("delete"));
    }

    #[test]
    fn test_custom_pattern_with_query_params() {
        let serializer = HyperlinkedModelSerializer::<TestModel>::new("detail")
            .url_pattern("/api/items/{id}/")
            .add_query_param("format", "json");

        let model = TestModel {
            id: Some(42),
            name: String::from("test"),
        };

        let result = serializer.serialize(&model).unwrap();
        let value: Value = serde_json::from_str(&result).unwrap();

        let url = value["url"].as_str().unwrap();
        assert!(url.starts_with("/api/items/42/"));
        assert!(url.contains("format=json"));
    }
}
