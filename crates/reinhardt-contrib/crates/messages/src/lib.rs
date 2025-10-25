//! Message framework for Reinhardt
//!
//! This crate provides Django-style messaging for displaying one-time
//! notifications to users.
//!
//! ## Planned Features
//! TODO: Implement request/response middleware for automatic message handling
//! TODO: Add automatic message retrieval and storage during request lifecycle
//! TODO: Implement context processor for template integration
//! TODO: Add message filtering by level

pub mod levels;
pub mod message;
pub mod safedata;
pub mod storage;
pub mod utils;

pub use levels::Level;
pub use message::{Message, MessageConfig};
pub use safedata::SafeData;
pub use storage::{CookieStorage, FallbackStorage, MemoryStorage, MessageStorage, SessionStorage};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::levels::*;
    pub use crate::message::*;
    pub use crate::safedata::*;
    pub use crate::storage::*;
    pub use crate::utils::*;
}
