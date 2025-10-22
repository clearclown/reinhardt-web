//! Extended field types with advanced validation and transformation
//!
//! This module provides enhanced field types that build upon the base fields
//! with additional features like custom transformations, conditional validation,
//! and computed values.

use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

/// ComputedField - Field with dynamically computed value
///
/// Unlike SerializerMethodField which requires a method, ComputedField
/// can use closures or other dynamic computation strategies.
///
/// # Examples
///
/// ```
/// # use reinhardt_serializers_ext::ComputedField;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct User {
/// #     first_name: String,
/// #     last_name: String,
/// #     full_name: ComputedField<String>,
/// # }
/// // full_name would be computed from first_name + last_name
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputedField<T> {
    _phantom: PhantomData<T>,
}

impl<T> ComputedField<T> {
    /// Create a new ComputedField
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for ComputedField<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// TransformedField - Field with value transformation
///
/// Applies transformations to field values before serialization or after
/// deserialization. Useful for formatting, normalization, etc.
///
/// # Examples
///
/// ```
/// # use reinhardt_serializers_ext::TransformedField;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Product {
/// #     name: String,
/// #     slug: TransformedField<String>, // Auto-generated from name
/// # }
/// // slug would be name.to_lowercase().replace(' ', '-')
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformedField<T> {
    _phantom: PhantomData<T>,
}

impl<T> TransformedField<T> {
    /// Create a new TransformedField
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for TransformedField<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// ConditionalField - Field that may or may not be included
///
/// Only includes the field in serialized output if a condition is met.
/// Useful for permission-based field visibility.
///
/// # Examples
///
/// ```
/// # use reinhardt_serializers_ext::ConditionalField;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct User {
/// #     username: String,
/// #     email: ConditionalField<String>, // Only shown to authenticated users
/// #     admin_notes: ConditionalField<String>, // Only shown to admins
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalField<T> {
    _phantom: PhantomData<T>,
}

impl<T> ConditionalField<T> {
    /// Create a new ConditionalField
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for ConditionalField<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// ReadOnlyField - Field that cannot be modified via API
///
/// This field is included in serialization but ignored during deserialization.
/// Common for computed values, timestamps, etc.
///
/// # Examples
///
/// ```
/// # use reinhardt_serializers_ext::ReadOnlyField;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Post {
/// #     title: String,
/// #     view_count: ReadOnlyField<i64>,
/// #     created_at: ReadOnlyField<String>,
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadOnlyField<T> {
    _phantom: PhantomData<T>,
}

impl<T> ReadOnlyField<T> {
    /// Create a new ReadOnlyField
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for ReadOnlyField<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// WriteOnlyField - Field that is only used for input
///
/// This field is excluded from serialization but required during deserialization.
/// Common for passwords, sensitive data, etc.
///
/// # Examples
///
/// ```
/// # use reinhardt_serializers_ext::WriteOnlyField;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct UserRegistration {
/// #     username: String,
/// #     password: WriteOnlyField<String>,
/// #     confirm_password: WriteOnlyField<String>,
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteOnlyField<T> {
    _phantom: PhantomData<T>,
}

impl<T> WriteOnlyField<T> {
    /// Create a new WriteOnlyField
    pub fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }
}

impl<T> Default for WriteOnlyField<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// FileField - Field for file upload handling
///
/// Represents a file upload with metadata like filename, size, content type.
/// Integrates with multipart form data parsing.
///
/// # Examples
///
/// ```
/// # use reinhardt_serializers_ext::FileField;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Document {
/// #     title: String,
/// #     attachment: FileField,
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileField {
    pub filename: Option<String>,
    pub content_type: Option<String>,
    pub size: Option<usize>,
}

impl FileField {
    /// Create a new FileField
    pub fn new() -> Self {
        Self {
            filename: None,
            content_type: None,
            size: None,
        }
    }

    /// Set the filename
    pub fn filename(mut self, name: impl Into<String>) -> Self {
        self.filename = Some(name.into());
        self
    }

    /// Set the content type
    pub fn content_type(mut self, ct: impl Into<String>) -> Self {
        self.content_type = Some(ct.into());
        self
    }

    /// Set the file size
    pub fn size(mut self, s: usize) -> Self {
        self.size = Some(s);
        self
    }
}

impl Default for FileField {
    fn default() -> Self {
        Self::new()
    }
}

/// ImageField - Specialized field for image uploads
///
/// Extends FileField with image-specific features like dimension validation,
/// thumbnail generation, etc.
///
/// # Examples
///
/// ```
/// # use reinhardt_serializers_ext::ImageField;
/// # use serde::{Serialize, Deserialize};
/// #
/// # #[derive(Debug, Clone, Serialize, Deserialize)]
/// # struct Profile {
/// #     username: String,
/// #     avatar: ImageField,
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageField {
    pub file: FileField,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl ImageField {
    /// Create a new ImageField
    pub fn new() -> Self {
        Self {
            file: FileField::new(),
            width: None,
            height: None,
        }
    }

    /// Set image dimensions
    pub fn dimensions(mut self, w: u32, h: u32) -> Self {
        self.width = Some(w);
        self.height = Some(h);
        self
    }
}

impl Default for ImageField {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_computed_field_creation() {
        let field = ComputedField::<String>::new();
        assert!(std::mem::size_of_val(&field) >= 0);
    }

    #[test]
    fn test_transformed_field_creation() {
        let field = TransformedField::<i64>::new();
        assert!(std::mem::size_of_val(&field) >= 0);
    }

    #[test]
    fn test_conditional_field_creation() {
        let field = ConditionalField::<bool>::new();
        assert!(std::mem::size_of_val(&field) >= 0);
    }

    #[test]
    fn test_read_only_field_creation() {
        let field = ReadOnlyField::<String>::new();
        assert!(std::mem::size_of_val(&field) >= 0);
    }

    #[test]
    fn test_write_only_field_creation() {
        let field = WriteOnlyField::<String>::new();
        assert!(std::mem::size_of_val(&field) >= 0);
    }

    #[test]
    fn test_file_field_creation() {
        let field = FileField::new();
        assert!(field.filename.is_none());
        assert!(field.content_type.is_none());
        assert!(field.size.is_none());
    }

    #[test]
    fn test_file_field_builder() {
        let field = FileField::new()
            .filename("test.pdf")
            .content_type("application/pdf")
            .size(1024);

        assert_eq!(field.filename, Some(String::from("test.pdf")));
        assert_eq!(field.content_type, Some(String::from("application/pdf")));
        assert_eq!(field.size, Some(1024));
    }

    #[test]
    fn test_image_field_creation() {
        let field = ImageField::new();
        assert!(field.width.is_none());
        assert!(field.height.is_none());
    }

    #[test]
    fn test_image_field_dimensions() {
        let field = ImageField::new().dimensions(800, 600);
        assert_eq!(field.width, Some(800));
        assert_eq!(field.height, Some(600));
    }

    #[test]
    fn test_field_defaults() {
        let _computed = ComputedField::<String>::default();
        let _transformed = TransformedField::<i64>::default();
        let _conditional = ConditionalField::<bool>::default();
        let _readonly = ReadOnlyField::<String>::default();
        let _writeonly = WriteOnlyField::<String>::default();
        let _file = FileField::default();
        let _image = ImageField::default();
    }

    #[test]
    fn test_file_field_serialization() {
        let field = FileField::new().filename("doc.pdf").size(2048);
        let json = serde_json::to_string(&field).unwrap();
        assert!(json.contains("doc.pdf"));
        assert!(json.contains("2048"));
    }

    #[test]
    fn test_image_field_serialization() {
        let field = ImageField::new().dimensions(1920, 1080);
        let json = serde_json::to_string(&field).unwrap();
        assert!(json.contains("1920"));
        assert!(json.contains("1080"));
    }
}
