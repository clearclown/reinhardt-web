//! Content negotiation for Reinhardt
//!
//! This crate provides DRF-style content negotiation for selecting
//! the appropriate renderer and parser based on Accept headers.
//!
//! ## Planned Features
//! TODO: Custom negotiation strategies - Pluggable negotiation logic
//! TODO: Content-Type detection - Automatic Content-Type detection for request body
//! TODO: Language negotiation - Support for Accept-Language header
//! TODO: Encoding negotiation - Support for Accept-Encoding header
//! TODO: Cache optimization - Caching of negotiation results
//! TODO: Detailed error information - Detailed feedback on negotiation failure

pub mod accept;
pub mod media_type;
pub mod negotiator;

pub use media_type::MediaType;
pub use negotiator::{
    BaseContentNegotiation, BaseNegotiator, ContentNegotiator, NegotiationError, RendererInfo,
};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::media_type::*;
    pub use crate::negotiator::*;
}
