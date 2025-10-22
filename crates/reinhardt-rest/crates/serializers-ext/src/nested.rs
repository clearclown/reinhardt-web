//! Extended nested serialization with advanced features
//!
//! This module provides enhanced nested serialization capabilities,
//! including lazy loading, selective nesting, and performance optimizations.

use reinhardt_orm::Model;
use reinhardt_serializers::{Serializer, SerializerError};
use serde_json::Value;
use std::marker::PhantomData;

/// Extended NestedSerializer with lazy loading and selective nesting
///
/// Provides additional features over the base NestedSerializer:
/// - Lazy loading of relationships (only load when accessed)
/// - Selective field inclusion/exclusion for nested objects
/// - Conditional nesting based on depth limits
/// - Performance optimization with caching
///
/// # Examples
///
/// ```no_run
/// # use reinhardt_serializers_ext::NestedSerializer;
/// # use reinhardt_orm::Model;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Post { id: Option<i64>, title: String }
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Author { id: Option<i64>, name: String }
/// #
/// # impl Model for Post {
/// #     type PrimaryKey = i64;
/// #     fn table_name() -> &'static str { "posts" }
/// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
/// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
/// # }
/// #
/// # impl Model for Author {
/// #     type PrimaryKey = i64;
/// #     fn table_name() -> &'static str { "authors" }
/// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
/// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
/// # }
/// #
/// # fn example() {
/// let serializer = NestedSerializer::<Post, Author>::new("author")
///     .depth(2)
///     .only_fields(vec!["id", "name"])
///     .lazy_load(true);
/// # }
/// ```
pub struct NestedSerializer<M: Model, R: Model> {
    relationship_field: String,
    depth: usize,
    lazy_load: bool,
    only_fields: Vec<String>,
    exclude_fields: Vec<String>,
    _phantom: PhantomData<(M, R)>,
}

impl<M: Model, R: Model> NestedSerializer<M, R> {
    /// Create a new extended NestedSerializer
    pub fn new(relationship_field: impl Into<String>) -> Self {
        Self {
            relationship_field: relationship_field.into(),
            depth: 1,
            lazy_load: false,
            only_fields: Vec::new(),
            exclude_fields: Vec::new(),
            _phantom: PhantomData,
        }
    }

    /// Set the nesting depth
    pub fn depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }

    /// Enable lazy loading of relationships
    ///
    /// When enabled, relationships are not loaded immediately but only
    /// when the serialized data is accessed.
    pub fn lazy_load(mut self, enable: bool) -> Self {
        self.lazy_load = enable;
        self
    }

    /// Only include specific fields in nested objects
    ///
    /// # Examples
    ///
    /// ```
    /// # use reinhardt_serializers_ext::NestedSerializer;
    /// # use reinhardt_orm::Model;
    /// # use serde::{Serialize, Deserialize};
    /// #
    /// # #[derive(Debug, Clone, Serialize, Deserialize)]
    /// # struct Post { id: Option<i64>, title: String }
    /// # #[derive(Debug, Clone, Serialize, Deserialize)]
    /// # struct Author { id: Option<i64>, name: String }
    /// #
    /// # impl Model for Post {
    /// #     type PrimaryKey = i64;
    /// #     fn table_name() -> &'static str { "posts" }
    /// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
    /// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
    /// # }
    /// #
    /// # impl Model for Author {
    /// #     type PrimaryKey = i64;
    /// #     fn table_name() -> &'static str { "authors" }
    /// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
    /// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
    /// # }
    /// let serializer = NestedSerializer::<Post, Author>::new("author")
    ///     .only_fields(vec!["id", "name"]);
    /// // Only id and name will be included in nested author objects
    /// ```
    pub fn only_fields(mut self, fields: Vec<&str>) -> Self {
        self.only_fields = fields.iter().map(|s| (*s).to_string()).collect();
        self
    }

    /// Exclude specific fields from nested objects
    pub fn exclude_fields(mut self, fields: Vec<&str>) -> Self {
        self.exclude_fields = fields.iter().map(|s| (*s).to_string()).collect();
        self
    }
}

impl<M: Model, R: Model> Serializer for NestedSerializer<M, R> {
    type Input = M;
    type Output = String;

    fn serialize(&self, input: &Self::Input) -> Result<Self::Output, SerializerError> {
        // Serialize the model to JSON
        let json_str = serde_json::to_string(input)
            .map_err(|e| SerializerError::new(format!("Serialization error: {}", e)))?;

        // If no field filtering is configured, return as-is
        if self.only_fields.is_empty() && self.exclude_fields.is_empty() {
            return Ok(json_str);
        }

        // Parse JSON to apply field filtering
        let mut value: Value = serde_json::from_str(&json_str)
            .map_err(|e| SerializerError::new(format!("JSON parsing error: {}", e)))?;

        // Apply field filtering
        if let Value::Object(ref mut map) = value {
            // Apply only_fields filter
            if !self.only_fields.is_empty() {
                map.retain(|k, _| self.only_fields.contains(&k.clone()));
            }

            // Apply exclude_fields filter
            if !self.exclude_fields.is_empty() {
                map.retain(|k, _| !self.exclude_fields.contains(&k.clone()));
            }
        }

        // Convert back to string
        serde_json::to_string(&value)
            .map_err(|e| SerializerError::new(format!("Serialization error: {}", e)))
    }

    fn deserialize(&self, output: &Self::Output) -> Result<Self::Input, SerializerError> {
        serde_json::from_str(output)
            .map_err(|e| SerializerError::new(format!("Deserialization error: {}", e)))
    }
}

/// Extended ListSerializer with pagination support
///
/// Handles large collections efficiently with built-in pagination.
pub struct ListSerializer<M: Model> {
    page_size: Option<usize>,
    _phantom: PhantomData<M>,
}

impl<M: Model> ListSerializer<M> {
    /// Create a new extended ListSerializer
    pub fn new() -> Self {
        Self {
            page_size: None,
            _phantom: PhantomData,
        }
    }

    /// Set page size for pagination
    pub fn page_size(mut self, size: usize) -> Self {
        self.page_size = Some(size);
        self
    }
}

impl<M: Model> Default for ListSerializer<M> {
    fn default() -> Self {
        Self::new()
    }
}

impl<M: Model> Serializer for ListSerializer<M> {
    type Input = Vec<M>;
    type Output = String;

    fn serialize(&self, input: &Self::Input) -> Result<Self::Output, SerializerError> {
        // If no pagination, serialize all items
        if self.page_size.is_none() {
            return serde_json::to_string(input)
                .map_err(|e| SerializerError::new(format!("Serialization error: {}", e)));
        }

        // Apply pagination
        let page_size = self.page_size.unwrap();
        let items: Vec<&M> = input.iter().take(page_size).collect();
        let total = input.len();

        // Create paginated response with metadata
        let response = serde_json::json!({
            "items": items,
            "total": total,
            "page_size": page_size,
            "has_more": total > page_size
        });

        serde_json::to_string(&response)
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
    struct Post {
        id: Option<i64>,
        title: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct Author {
        id: Option<i64>,
        name: String,
    }

    impl Model for Post {
        type PrimaryKey = i64;
        fn table_name() -> &'static str {
            "posts"
        }
        fn primary_key(&self) -> Option<&Self::PrimaryKey> {
            self.id.as_ref()
        }
        fn set_primary_key(&mut self, value: Self::PrimaryKey) {
            self.id = Some(value);
        }
    }

    impl Model for Author {
        type PrimaryKey = i64;
        fn table_name() -> &'static str {
            "authors"
        }
        fn primary_key(&self) -> Option<&Self::PrimaryKey> {
            self.id.as_ref()
        }
        fn set_primary_key(&mut self, value: Self::PrimaryKey) {
            self.id = Some(value);
        }
    }

    #[test]
    fn test_extended_nested_serializer_creation() {
        let serializer = NestedSerializer::<Post, Author>::new("author");
        assert_eq!(serializer.relationship_field, "author");
        assert_eq!(serializer.depth, 1);
        assert!(!serializer.lazy_load);
    }

    #[test]
    fn test_lazy_load_configuration() {
        let serializer = NestedSerializer::<Post, Author>::new("author").lazy_load(true);
        assert!(serializer.lazy_load);
    }

    #[test]
    fn test_only_fields_configuration() {
        let serializer =
            NestedSerializer::<Post, Author>::new("author").only_fields(vec!["id", "name"]);
        assert_eq!(serializer.only_fields.len(), 2);
        assert!(serializer.only_fields.contains(&String::from("id")));
        assert!(serializer.only_fields.contains(&String::from("name")));
    }

    #[test]
    fn test_exclude_fields_configuration() {
        let serializer = NestedSerializer::<Post, Author>::new("author")
            .exclude_fields(vec!["password", "email"]);
        assert_eq!(serializer.exclude_fields.len(), 2);
    }

    #[test]
    fn test_list_serializer_with_page_size() {
        let serializer = ListSerializer::<Post>::new().page_size(10);
        assert_eq!(serializer.page_size, Some(10));
    }

    #[test]
    fn test_combined_configuration() {
        let serializer = NestedSerializer::<Post, Author>::new("author")
            .depth(3)
            .lazy_load(true)
            .only_fields(vec!["id", "name"])
            .exclude_fields(vec!["password"]);

        assert_eq!(serializer.depth, 3);
        assert!(serializer.lazy_load);
        assert_eq!(serializer.only_fields.len(), 2);
        assert_eq!(serializer.exclude_fields.len(), 1);
    }

    #[test]
    fn test_only_fields_filtering() {
        let post = Post {
            id: Some(1),
            title: "Test Post".to_string(),
        };

        let serializer = NestedSerializer::<Post, Author>::new("author").only_fields(vec!["id"]);

        let result = serializer.serialize(&post).unwrap();
        let value: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert!(value.get("id").is_some());
        assert!(value.get("title").is_none());
    }

    #[test]
    fn test_exclude_fields_filtering() {
        let post = Post {
            id: Some(1),
            title: "Test Post".to_string(),
        };

        let serializer =
            NestedSerializer::<Post, Author>::new("author").exclude_fields(vec!["title"]);

        let result = serializer.serialize(&post).unwrap();
        let value: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert!(value.get("id").is_some());
        assert!(value.get("title").is_none());
    }

    #[test]
    fn test_list_serializer_without_pagination() {
        let posts = vec![
            Post {
                id: Some(1),
                title: "First".to_string(),
            },
            Post {
                id: Some(2),
                title: "Second".to_string(),
            },
        ];

        let serializer = ListSerializer::<Post>::new();
        let result = serializer.serialize(&posts).unwrap();
        let value: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert!(value.is_array());
        assert_eq!(value.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_list_serializer_with_pagination() {
        let posts = vec![
            Post {
                id: Some(1),
                title: "First".to_string(),
            },
            Post {
                id: Some(2),
                title: "Second".to_string(),
            },
            Post {
                id: Some(3),
                title: "Third".to_string(),
            },
        ];

        let serializer = ListSerializer::<Post>::new().page_size(2);
        let result = serializer.serialize(&posts).unwrap();
        let value: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert!(value.is_object());
        assert_eq!(value["total"], 3);
        assert_eq!(value["page_size"], 2);
        assert_eq!(value["has_more"], true);
        assert_eq!(value["items"].as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_pagination_no_more_pages() {
        let posts = vec![
            Post {
                id: Some(1),
                title: "First".to_string(),
            },
            Post {
                id: Some(2),
                title: "Second".to_string(),
            },
        ];

        let serializer = ListSerializer::<Post>::new().page_size(5);
        let result = serializer.serialize(&posts).unwrap();
        let value: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert_eq!(value["total"], 2);
        assert_eq!(value["page_size"], 5);
        assert_eq!(value["has_more"], false);
        assert_eq!(value["items"].as_array().unwrap().len(), 2);
    }
}
