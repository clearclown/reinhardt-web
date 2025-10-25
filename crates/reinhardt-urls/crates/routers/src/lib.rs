//! # Reinhardt Routers
//!
//! URL routing for Reinhardt framework.
//!
//! ## Planned Features
//! TODO: SimpleRouter - Lightweight router for basic routing needs with minimal overhead
//! TODO: Namespace-based URL reversal - Hierarchical route naming (`"v1:users:detail"`)
//! TODO: Nested namespace resolution
//! TODO: URL reversal with namespace support
//! TODO: Route caching - Performance optimization for large route tables
//! TODO: Custom converters - Type-specific path parameter converters (Integer, UUID, slug)
//! TODO: Custom validation rules for path parameters
//! TODO: Route introspection - Runtime route analysis and debugging
//! TODO: OpenAPI integration - Automatic OpenAPI schema generation from routes
//! TODO: Route visualization - Generate route maps for documentation

pub mod helpers;
pub mod pattern;
pub mod reverse;
pub mod route;
pub mod router;
pub mod script_prefix;
pub mod unified_router;

// Re-export the path! macro for compile-time path validation
pub use reinhardt_routers_macros::path;

pub use helpers::{include_routes, path, re_path, IncludedRouter};
pub use pattern::{PathMatcher, PathPattern};
pub use reverse::{
    reverse,
    reverse_typed,
    reverse_typed_with_params,
    ReverseError,
    ReverseResult,
    UrlParams,
    // Type-safe reversal
    UrlPattern,
    UrlPatternWithParams,
    UrlReverser,
};
pub use route::Route;
pub use router::{DefaultRouter, Router};
pub use script_prefix::{clear_script_prefix, get_script_prefix, set_script_prefix};
pub use unified_router::{
    clear_router, get_router, is_router_registered, register_router, UnifiedRouter,
};
