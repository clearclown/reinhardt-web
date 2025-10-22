//! Type-safe filtering backends for Reinhardt framework
//!
//! Provides compile-time type-safe filtering using reinhardt-orm's Field<M, T> system.

// Core filter trait
pub mod filter;

// Type-safe filtering system
pub mod field_extensions;
pub mod multi_term;
pub mod ordering_field;
pub mod query_filter;
pub mod searchable;

// Core exports
pub use filter::{FilterBackend, FilterError, FilterResult};

// Type-safe exports
pub use field_extensions::FieldOrderingExt;
pub use multi_term::{MultiTermSearch, Operator, SearchTerm, TermType};
pub use ordering_field::{OrderDirection, OrderingField};
pub use query_filter::QueryFilter;
pub use searchable::SearchableModel;
