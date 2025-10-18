//! Views core functionality

pub mod admin;
pub mod browsable_api;
pub mod generic;
pub mod openapi;
pub mod test_utils;
// Templates are embedded resources, not a Rust module
// pub mod templates;

pub use admin::*;
pub use browsable_api::*;
pub use generic::*;
pub use openapi::*;
pub use test_utils::*;
