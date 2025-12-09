//! Views for auth app
//!
//! This module contains all view handlers for the auth application,
//! organized by functionality

pub mod change_password;
pub mod register;
pub mod reset_password;
pub mod signin;
pub mod signout;
pub mod verify_password;

// Re-export commonly used view handlers
pub use change_password::change_password;
pub use register::register;
pub use reset_password::{reset_password, reset_password_confirm};
pub use signin::signin;
pub use signout::signout;
pub use verify_password::verify_password;
