//! Schema registry for managing reusable component schemas
//!
//! This module provides a centralized registry for managing OpenAPI schemas
//! with automatic deduplication and $ref reference generation.

use crate::openapi::{Components, ComponentsBuilder, RefOr, Schema};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A registry for managing reusable OpenAPI schemas
///
/// The `SchemaRegistry` provides centralized schema management with automatic
/// deduplication using `$ref` references. When a schema with the same name
/// is registered multiple times, the registry ensures only one definition
/// exists in the `components/schemas` section.
///
/// # Example
///
/// ```rust
/// use reinhardt_openapi::registry::SchemaRegistry;
/// use reinhardt_openapi::{Schema, SchemaExt};
///
/// let registry = SchemaRegistry::new();
///
/// // Register a schema
/// let user_schema = Schema::object_with_properties(
///     vec![
///         ("id", Schema::integer()),
///         ("name", Schema::string()),
///     ],
///     vec!["id", "name"],
/// );
/// registry.register("User", user_schema);
///
/// // Get a $ref to the schema
/// let user_ref = registry.get_ref("User");
/// assert!(user_ref.is_some());
///
/// // Export to components
/// let components = registry.to_components();
/// assert!(components.schemas.contains_key("User"));
/// ```
#[derive(Clone)]
pub struct SchemaRegistry {
    schemas: Arc<Mutex<HashMap<String, Schema>>>,
    references: Arc<Mutex<HashMap<String, usize>>>, // Track reference counts for circular detection
}

impl SchemaRegistry {
    /// Create a new empty schema registry
    pub fn new() -> Self {
        Self {
            schemas: Arc::new(Mutex::new(HashMap::new())),
            references: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register a schema with a given name
    ///
    /// If a schema with the same name already exists, it will be replaced.
    ///
    /// # Example
    ///
    /// ```rust
    /// use reinhardt_openapi::registry::SchemaRegistry;
    /// use reinhardt_openapi::{Schema, SchemaExt};
    ///
    /// let registry = SchemaRegistry::new();
    /// registry.register("User", Schema::object());
    /// ```
    pub fn register(&self, name: impl Into<String>, schema: Schema) {
        let name = name.into();
        let mut schemas = self.schemas.lock().unwrap();
        schemas.insert(name, schema);
    }

    /// Get a schema by name
    ///
    /// Returns `None` if the schema is not registered.
    ///
    /// # Example
    ///
    /// ```rust
    /// use reinhardt_openapi::registry::SchemaRegistry;
    /// use reinhardt_openapi::{Schema, SchemaExt};
    ///
    /// let registry = SchemaRegistry::new();
    /// registry.register("User", Schema::object());
    ///
    /// let schema = registry.get_schema("User");
    /// assert!(schema.is_some());
    /// ```
    pub fn get_schema(&self, name: &str) -> Option<Schema> {
        let schemas = self.schemas.lock().unwrap();
        schemas.get(name).cloned()
    }

    /// Get a $ref reference to a schema
    ///
    /// Returns a `RefOr::Ref` variant pointing to `#/components/schemas/{name}`.
    /// If the schema is not registered, returns `None`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use reinhardt_openapi::registry::SchemaRegistry;
    /// use reinhardt_openapi::{Schema, SchemaExt, RefOr};
    ///
    /// let registry = SchemaRegistry::new();
    /// registry.register("User", Schema::object());
    ///
    /// let user_ref = registry.get_ref("User");
    /// assert!(user_ref.is_some());
    ///
    /// match user_ref.unwrap() {
    ///     RefOr::Ref(ref_obj) => {
    ///         assert_eq!(ref_obj.ref_location, "#/components/schemas/User");
    ///     }
    ///     _ => panic!("Expected Ref variant"),
    /// }
    /// ```
    pub fn get_ref(&self, name: &str) -> Option<RefOr<Schema>> {
        let schemas = self.schemas.lock().unwrap();
        if schemas.contains_key(name) {
            // Increment reference count for circular detection
            let mut references = self.references.lock().unwrap();
            *references.entry(name.to_string()).or_insert(0) += 1;

            Some(RefOr::Ref(utoipa::openapi::Ref::new(format!(
                "#/components/schemas/{}",
                name
            ))))
        } else {
            None
        }
    }

    /// Check if a schema is registered
    ///
    /// # Example
    ///
    /// ```rust
    /// use reinhardt_openapi::registry::SchemaRegistry;
    /// use reinhardt_openapi::{Schema, SchemaExt};
    ///
    /// let registry = SchemaRegistry::new();
    /// assert!(!registry.contains("User"));
    ///
    /// registry.register("User", Schema::object());
    /// assert!(registry.contains("User"));
    /// ```
    pub fn contains(&self, name: &str) -> bool {
        let schemas = self.schemas.lock().unwrap();
        schemas.contains_key(name)
    }

    /// Get the number of registered schemas
    ///
    /// # Example
    ///
    /// ```rust
    /// use reinhardt_openapi::registry::SchemaRegistry;
    /// use reinhardt_openapi::{Schema, SchemaExt};
    ///
    /// let registry = SchemaRegistry::new();
    /// assert_eq!(registry.len(), 0);
    ///
    /// registry.register("User", Schema::object());
    /// assert_eq!(registry.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        let schemas = self.schemas.lock().unwrap();
        schemas.len()
    }

    /// Check if the registry is empty
    ///
    /// # Example
    ///
    /// ```rust
    /// use reinhardt_openapi::registry::SchemaRegistry;
    /// use reinhardt_openapi::{Schema, SchemaExt};
    ///
    /// let registry = SchemaRegistry::new();
    /// assert!(registry.is_empty());
    ///
    /// registry.register("User", Schema::object());
    /// assert!(!registry.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        let schemas = self.schemas.lock().unwrap();
        schemas.is_empty()
    }

    /// Detect potential circular references
    ///
    /// Returns a list of schema names that might be involved in circular references
    /// (referenced more than once).
    ///
    /// # Example
    ///
    /// ```rust
    /// use reinhardt_openapi::registry::SchemaRegistry;
    /// use reinhardt_openapi::{Schema, SchemaExt};
    ///
    /// let registry = SchemaRegistry::new();
    /// registry.register("User", Schema::object());
    ///
    /// // Get reference twice
    /// let _ = registry.get_ref("User");
    /// let _ = registry.get_ref("User");
    ///
    /// let circular = registry.detect_circular_references();
    /// assert!(circular.contains(&"User".to_string()));
    /// ```
    pub fn detect_circular_references(&self) -> Vec<String> {
        let references = self.references.lock().unwrap();
        references
            .iter()
            .filter(|(_, count)| **count > 1)
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Clear all registered schemas and reset reference counts
    ///
    /// # Example
    ///
    /// ```rust
    /// use reinhardt_openapi::registry::SchemaRegistry;
    /// use reinhardt_openapi::{Schema, SchemaExt};
    ///
    /// let registry = SchemaRegistry::new();
    /// registry.register("User", Schema::object());
    /// assert!(!registry.is_empty());
    ///
    /// registry.clear();
    /// assert!(registry.is_empty());
    /// ```
    pub fn clear(&self) {
        let mut schemas = self.schemas.lock().unwrap();
        schemas.clear();

        let mut references = self.references.lock().unwrap();
        references.clear();
    }

    /// Export all registered schemas to OpenAPI Components
    ///
    /// This creates a `Components` object with all registered schemas
    /// in the `schemas` section.
    ///
    /// # Example
    ///
    /// ```rust
    /// use reinhardt_openapi::registry::SchemaRegistry;
    /// use reinhardt_openapi::{Schema, SchemaExt};
    ///
    /// let registry = SchemaRegistry::new();
    /// registry.register("User", Schema::object());
    /// registry.register("Post", Schema::object());
    ///
    /// let components = registry.to_components();
    /// assert_eq!(components.schemas.len(), 2);
    /// assert!(components.schemas.contains_key("User"));
    /// assert!(components.schemas.contains_key("Post"));
    /// ```
    pub fn to_components(&self) -> Components {
        let schemas = self.schemas.lock().unwrap();
        let mut builder = ComponentsBuilder::new();

        for (name, schema) in schemas.iter() {
            builder = builder.schema(name, schema.clone());
        }

        builder.build()
    }

    /// Merge another registry into this one
    ///
    /// Schemas from the other registry will overwrite schemas with the same name
    /// in this registry.
    ///
    /// # Example
    ///
    /// ```rust
    /// use reinhardt_openapi::registry::SchemaRegistry;
    /// use reinhardt_openapi::{Schema, SchemaExt};
    ///
    /// let registry1 = SchemaRegistry::new();
    /// registry1.register("User", Schema::object());
    ///
    /// let registry2 = SchemaRegistry::new();
    /// registry2.register("Post", Schema::object());
    ///
    /// registry1.merge(&registry2);
    /// assert_eq!(registry1.len(), 2);
    /// assert!(registry1.contains("User"));
    /// assert!(registry1.contains("Post"));
    /// ```
    pub fn merge(&self, other: &SchemaRegistry) {
        let other_schemas = other.schemas.lock().unwrap();
        let mut schemas = self.schemas.lock().unwrap();

        for (name, schema) in other_schemas.iter() {
            schemas.insert(name.clone(), schema.clone());
        }
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openapi::SchemaExt;

    #[test]
    fn test_register_and_get_schema() {
        let registry = SchemaRegistry::new();
        let schema = Schema::object();

        registry.register("User", schema.clone());

        let retrieved = registry.get_schema("User");
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_get_ref() {
        let registry = SchemaRegistry::new();
        registry.register("User", Schema::object());

        let user_ref = registry.get_ref("User");
        assert!(user_ref.is_some());

        match user_ref.unwrap() {
            RefOr::Ref(ref_obj) => {
                assert_eq!(ref_obj.ref_location, "#/components/schemas/User");
            }
            _ => panic!("Expected Ref variant"),
        }
    }

    #[test]
    fn test_get_ref_nonexistent() {
        let registry = SchemaRegistry::new();
        let user_ref = registry.get_ref("User");
        assert!(user_ref.is_none());
    }

    #[test]
    fn test_contains() {
        let registry = SchemaRegistry::new();
        assert!(!registry.contains("User"));

        registry.register("User", Schema::object());
        assert!(registry.contains("User"));
    }

    #[test]
    fn test_len_and_is_empty() {
        let registry = SchemaRegistry::new();
        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());

        registry.register("User", Schema::object());
        assert_eq!(registry.len(), 1);
        assert!(!registry.is_empty());
    }

    #[test]
    fn test_circular_reference_detection() {
        let registry = SchemaRegistry::new();
        registry.register("User", Schema::object());

        // Get reference multiple times
        let _ = registry.get_ref("User");
        let _ = registry.get_ref("User");
        let _ = registry.get_ref("User");

        let circular = registry.detect_circular_references();
        assert!(circular.contains(&"User".to_string()));
    }

    #[test]
    fn test_clear() {
        let registry = SchemaRegistry::new();
        registry.register("User", Schema::object());
        registry.register("Post", Schema::object());

        assert_eq!(registry.len(), 2);

        registry.clear();
        assert_eq!(registry.len(), 0);
        assert!(registry.is_empty());
    }

    #[test]
    fn test_to_components() {
        let registry = SchemaRegistry::new();
        registry.register("User", Schema::object());
        registry.register("Post", Schema::object());

        let components = registry.to_components();
        assert_eq!(components.schemas.len(), 2);
        assert!(components.schemas.contains_key("User"));
        assert!(components.schemas.contains_key("Post"));
    }

    #[test]
    fn test_merge() {
        let registry1 = SchemaRegistry::new();
        registry1.register("User", Schema::object());

        let registry2 = SchemaRegistry::new();
        registry2.register("Post", Schema::object());

        registry1.merge(&registry2);
        assert_eq!(registry1.len(), 2);
        assert!(registry1.contains("User"));
        assert!(registry1.contains("Post"));
    }

    #[test]
    fn test_merge_overwrites() {
        let registry1 = SchemaRegistry::new();
        registry1.register("User", Schema::integer());

        let registry2 = SchemaRegistry::new();
        registry2.register("User", Schema::string());

        registry1.merge(&registry2);
        assert_eq!(registry1.len(), 1);

        let schema = registry1.get_schema("User").unwrap();
        match schema {
            Schema::Object(obj) => {
                assert!(matches!(
                    obj.schema_type,
                    utoipa::openapi::schema::SchemaType::Type(utoipa::openapi::Type::String)
                ));
            }
            _ => panic!("Expected Object schema"),
        }
    }

    #[test]
    fn test_replace_schema() {
        let registry = SchemaRegistry::new();
        registry.register("User", Schema::integer());

        let schema1 = registry.get_schema("User").unwrap();
        match schema1 {
            Schema::Object(obj) => {
                assert!(matches!(
                    obj.schema_type,
                    utoipa::openapi::schema::SchemaType::Type(utoipa::openapi::Type::Integer)
                ));
            }
            _ => panic!("Expected Object schema"),
        }

        // Replace with new schema
        registry.register("User", Schema::string());

        let schema2 = registry.get_schema("User").unwrap();
        match schema2 {
            Schema::Object(obj) => {
                assert!(matches!(
                    obj.schema_type,
                    utoipa::openapi::schema::SchemaType::Type(utoipa::openapi::Type::String)
                ));
            }
            _ => panic!("Expected Object schema"),
        }
    }
}
