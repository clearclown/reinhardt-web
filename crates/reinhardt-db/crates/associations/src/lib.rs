//! Association proxies for Reinhardt
//!
//! This crate provides SQLAlchemy-style association proxies for simplifying
//! access to related objects through associations.
//!
//! ## Planned Features
//! TODO: Implement One-to-One, One-to-Many (ForeignKey), and Many-to-Many relationship definitions
//! TODO: Add automatic reverse relationship accessor generation
//! TODO: Implement custom naming for reverse relationships (related_name)
//! TODO: Add lazy loading and eager loading strategies
//! TODO: Implement cascade deletion and update options
//! TODO: Add support for polymorphic associations

pub mod collection;
pub mod proxy;

pub use collection::AssociationCollection;
pub use proxy::AssociationProxy;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::collection::*;
    pub use crate::proxy::*;
}
