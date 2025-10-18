//! Association proxies for Reinhardt
//!
//! This crate provides SQLAlchemy-style association proxies for simplifying
//! access to related objects through associations.

pub mod collection;
pub mod proxy;

pub use collection::AssociationCollection;
pub use proxy::AssociationProxy;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::collection::*;
    pub use crate::proxy::*;
}
