//! Browsable API for Reinhardt
//!
//! This crate provides DRF-style browsable API interface for exploring APIs
//! through a web browser.

pub mod middleware;
pub mod renderer;
pub mod response;
pub mod template;

pub use middleware::{BrowsableApiConfig, BrowsableApiMiddleware};
pub use renderer::{ApiContext, BrowsableApiRenderer, FormContext, FormField, SelectOption};
pub use response::BrowsableResponse;
pub use template::ApiTemplate;

/// Re-export commonly used types
pub mod prelude {
    pub use crate::middleware::*;
    pub use crate::renderer::*;
    pub use crate::response::*;
    pub use crate::template::*;
}
