//! # Reinhardt ContentTypes
//!
//! Django-style content types framework for polymorphic relationships.
//!
//! This crate provides a content type system that enables generic relations
//! similar to Django's contenttypes framework.
//!
//! ## Planned Features
//! TODO: Add multi-database support for content types
//! TODO: Complete ORM integration

pub mod contenttypes;
pub mod generic_fk;
pub mod persistence;

pub use contenttypes::{
    ContentType, ContentTypeRegistry, GenericForeignKey, GenericRelatable, GenericRelationQuery,
    ModelType, CONTENT_TYPE_REGISTRY,
};

pub use generic_fk::GenericForeignKeyField;

#[cfg(feature = "database")]
pub use generic_fk::constraints;

#[cfg(feature = "database")]
pub use persistence::{
    ContentTypeModel, ContentTypePersistence, ContentTypePersistenceBackend, PersistenceError,
};

#[cfg(not(feature = "database"))]
pub use persistence::PersistenceError;
