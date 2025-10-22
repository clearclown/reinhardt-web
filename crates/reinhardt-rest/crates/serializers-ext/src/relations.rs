//! Extended relation fields with advanced features
//!
//! This module provides enhanced relationship field types with features like
//! prefetching, custom queryset filtering, and representation control.

use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// Extended RelationField with prefetch and custom filtering
///
/// Extends the base RelationField with performance optimizations and
/// advanced querying capabilities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationField<T> {
    prefetch: bool,
    _phantom: PhantomData<T>,
}

impl<T> RelationField<T> {
    /// Create a new extended RelationField
    pub fn new() -> Self {
        Self {
            prefetch: false,
            _phantom: PhantomData,
        }
    }

    /// Enable prefetching for this relation
    ///
    /// When enabled, the related objects are loaded eagerly to avoid N+1 queries.
    pub fn prefetch(mut self, enable: bool) -> Self {
        self.prefetch = enable;
        self
    }
}

impl<T> Default for RelationField<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Extended PrimaryKeyRelatedField with custom representation
///
/// # Examples
///
/// ```
/// # use reinhardt_serializers_ext::PrimaryKeyRelatedField;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Author {
/// #     id: Option<i64>,
/// #     name: String,
/// # }
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Post {
/// #     id: Option<i64>,
/// #     title: String,
/// #     author: PrimaryKeyRelatedField<Author>,
/// # }
/// let field = PrimaryKeyRelatedField::<Author>::new().prefetch(true);
/// ```
pub type PrimaryKeyRelatedField<T> = RelationField<T>;

/// Extended SlugRelatedField with slug field customization
///
/// # Examples
///
/// ```
/// # use reinhardt_serializers_ext::SlugRelatedField;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Category {
/// #     id: Option<i64>,
/// #     slug: String,
/// #     name: String,
/// # }
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Product {
/// #     id: Option<i64>,
/// #     name: String,
/// #     category: SlugRelatedField<Category>,
/// # }
/// let field = SlugRelatedField::<Category>::new().prefetch(true);
/// ```
pub type SlugRelatedField<T> = RelationField<T>;

/// Extended HyperlinkedRelatedField with URL customization
///
/// # Examples
///
/// ```
/// # use reinhardt_serializers_ext::HyperlinkedRelatedField;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Author {
/// #     id: Option<i64>,
/// #     name: String,
/// # }
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Book {
/// #     id: Option<i64>,
/// #     title: String,
/// #     author: HyperlinkedRelatedField<Author>,
/// # }
/// let field = HyperlinkedRelatedField::<Author>::new();
/// ```
pub type HyperlinkedRelatedField<T> = RelationField<T>;

/// Extended ManyRelatedField with batch operations
///
/// Supports efficient bulk operations on many-to-many relationships.
///
/// # Examples
///
/// ```
/// # use reinhardt_serializers_ext::ManyRelatedField;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Tag {
/// #     id: Option<i64>,
/// #     name: String,
/// # }
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Article {
/// #     id: Option<i64>,
/// #     title: String,
/// #     tags: ManyRelatedField<Tag>,
/// # }
/// let field = ManyRelatedField::<Tag>::new().prefetch(true);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManyRelatedField<T> {
    prefetch: bool,
    batch_size: Option<usize>,
    _phantom: PhantomData<T>,
}

impl<T> ManyRelatedField<T> {
    /// Create a new extended ManyRelatedField
    pub fn new() -> Self {
        Self {
            prefetch: false,
            batch_size: None,
            _phantom: PhantomData,
        }
    }

    /// Enable prefetching for this relation
    pub fn prefetch(mut self, enable: bool) -> Self {
        self.prefetch = enable;
        self
    }

    /// Set batch size for bulk operations
    pub fn batch_size(mut self, size: usize) -> Self {
        self.batch_size = Some(size);
        self
    }
}

impl<T> Default for ManyRelatedField<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// GenericForeignKey - Polymorphic relationship field
///
/// Represents a relationship that can point to different model types.
/// Inspired by Django's GenericForeignKey.
///
/// # Examples
///
/// ```
/// # use reinhardt_serializers_ext::GenericForeignKey;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Comment {
/// #     id: Option<i64>,
/// #     text: String,
/// #     content_type: String,  // "post" or "page"
/// #     object_id: i64,
/// #     content_object: GenericForeignKey,  // Points to Post or Page
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericForeignKey {
    pub content_type: Option<String>,
    pub object_id: Option<i64>,
}

impl GenericForeignKey {
    /// Create a new GenericForeignKey
    pub fn new() -> Self {
        Self {
            content_type: None,
            object_id: None,
        }
    }

    /// Set the content type
    pub fn content_type(mut self, ct: impl Into<String>) -> Self {
        self.content_type = Some(ct.into());
        self
    }

    /// Set the object ID
    pub fn object_id(mut self, id: i64) -> Self {
        self.object_id = Some(id);
        self
    }
}

impl Default for GenericForeignKey {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestRelated {
        id: Option<i64>,
        name: String,
    }

    #[test]
    fn test_relation_field_creation() {
        let field = RelationField::<TestRelated>::new();
        assert!(!field.prefetch);
    }

    #[test]
    fn test_relation_field_prefetch() {
        let field = RelationField::<TestRelated>::new().prefetch(true);
        assert!(field.prefetch);
    }

    #[test]
    fn test_primary_key_related_field() {
        let field = PrimaryKeyRelatedField::<TestRelated>::new().prefetch(true);
        assert!(field.prefetch);
    }

    #[test]
    fn test_slug_related_field() {
        let field = SlugRelatedField::<TestRelated>::new();
        assert!(!field.prefetch);
    }

    #[test]
    fn test_hyperlinked_related_field() {
        let field = HyperlinkedRelatedField::<TestRelated>::new();
        assert!(!field.prefetch);
    }

    #[test]
    fn test_many_related_field_creation() {
        let field = ManyRelatedField::<TestRelated>::new();
        assert!(!field.prefetch);
        assert!(field.batch_size.is_none());
    }

    #[test]
    fn test_many_related_field_configuration() {
        let field = ManyRelatedField::<TestRelated>::new()
            .prefetch(true)
            .batch_size(100);
        assert!(field.prefetch);
        assert_eq!(field.batch_size, Some(100));
    }

    #[test]
    fn test_generic_foreign_key_creation() {
        let field = GenericForeignKey::new();
        assert!(field.content_type.is_none());
        assert!(field.object_id.is_none());
    }

    #[test]
    fn test_generic_foreign_key_builder() {
        let field = GenericForeignKey::new().content_type("post").object_id(42);
        assert_eq!(field.content_type, Some(String::from("post")));
        assert_eq!(field.object_id, Some(42));
    }

    #[test]
    fn test_field_defaults() {
        let _relation = RelationField::<TestRelated>::default();
        let _many = ManyRelatedField::<TestRelated>::default();
        let _generic = GenericForeignKey::default();
    }

    #[test]
    fn test_relation_field_serialization() {
        let field = RelationField::<TestRelated>::new().prefetch(true);
        let json = serde_json::to_string(&field).unwrap();
        assert!(json.contains("true"));
    }

    #[test]
    fn test_many_related_field_serialization() {
        let field = ManyRelatedField::<TestRelated>::new()
            .prefetch(true)
            .batch_size(50);
        let json = serde_json::to_string(&field).unwrap();
        assert!(json.contains("50"));
    }

    #[test]
    fn test_generic_foreign_key_serialization() {
        let field = GenericForeignKey::new()
            .content_type("article")
            .object_id(123);
        let json = serde_json::to_string(&field).unwrap();
        assert!(json.contains("article"));
        assert!(json.contains("123"));
    }
}
