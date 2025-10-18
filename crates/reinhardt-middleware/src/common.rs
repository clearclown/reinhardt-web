//! Common middleware utilities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonConfig {
    pub append_slash: bool,
    pub prepend_www: bool,
}

impl CommonConfig {
    pub fn new() -> Self {
        Self {
            append_slash: true,
            prepend_www: false,
        }
    }
}

impl Default for CommonConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CommonMiddleware;

impl CommonMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CommonMiddleware {
    fn default() -> Self {
        Self::new()
    }
}
