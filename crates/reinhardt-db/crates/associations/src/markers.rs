//! Marker types for relationship field annotations.
//!
//! These types are used as field types to indicate relationship definitions
//! when using the `#[rel]` attribute macro.
//!
//! # Example
//!
//! ```ignore
//! use reinhardt::prelude::*;
//! use reinhardt::db::associations::ManyToManyField;
//!
//! #[model(app_label = "users")]
//! pub struct User {
//!     #[field(primary_key = true)]
//!     pub id: Uuid,
//!
//!     #[rel(many_to_many, to = User, related_name = "followers")]
//!     pub following: ManyToManyField<User>,
//! }
//! ```

use std::marker::PhantomData;

/// Marker type for ManyToMany relationship fields.
///
/// This type is used as the field type for `#[rel(many_to_many, ...)]` attributes.
/// It indicates that the field represents a many-to-many relationship.
///
/// The type parameter `T` specifies the target model type.
///
/// # Example
///
/// ```ignore
/// #[rel(many_to_many, to = Tag, related_name = "articles")]
/// pub tags: ManyToManyField<Tag>,
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ManyToManyField<T>(PhantomData<T>);

impl<T> ManyToManyField<T> {
	/// Creates a new ManyToManyField marker.
	#[inline]
	pub const fn new() -> Self {
		Self(PhantomData)
	}
}

/// Marker type for OneToMany relationship fields (reverse of ForeignKey).
///
/// This type is used as the field type for `#[rel(one_to_many, ...)]` attributes.
/// It represents the reverse side of a ForeignKey relationship.
///
/// The type parameter `T` specifies the related model type.
///
/// # Example
///
/// ```ignore
/// // On User model
/// #[rel(one_to_many, to = Post, foreign_key = "author_id")]
/// pub posts: OneToManyField<Post>,
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct OneToManyField<T>(PhantomData<T>);

impl<T> OneToManyField<T> {
	/// Creates a new OneToManyField marker.
	#[inline]
	pub const fn new() -> Self {
		Self(PhantomData)
	}
}

/// Marker type for PolymorphicManyToMany relationship fields.
///
/// This type is used as the field type for `#[rel(polymorphic_many_to_many, ...)]` attributes.
/// It indicates a polymorphic many-to-many relationship.
///
/// The type parameter `K` specifies the key type (usually `i64` or `Uuid`).
///
/// # Example
///
/// ```ignore
/// #[rel(polymorphic_many_to_many, name = "taggable")]
/// pub tags: PolymorphicManyToManyField<Uuid>,
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct PolymorphicManyToManyField<K>(PhantomData<K>);

impl<K> PolymorphicManyToManyField<K> {
	/// Creates a new PolymorphicManyToManyField marker.
	#[inline]
	pub const fn new() -> Self {
		Self(PhantomData)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_many_to_many_field_creation() {
		struct User;
		let _field: ManyToManyField<User> = ManyToManyField::new();
	}

	#[test]
	fn test_one_to_many_field_creation() {
		struct Post;
		let _field: OneToManyField<Post> = OneToManyField::new();
	}

	#[test]
	fn test_polymorphic_many_to_many_field_creation() {
		let _field: PolymorphicManyToManyField<i64> = PolymorphicManyToManyField::new();
	}

	#[test]
	fn test_default_impl() {
		#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
		struct Tag;
		let field1: ManyToManyField<Tag> = ManyToManyField::default();
		let field2: ManyToManyField<Tag> = ManyToManyField::new();
		assert_eq!(field1, field2);
	}
}
