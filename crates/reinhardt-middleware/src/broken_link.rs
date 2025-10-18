//! Broken link detection middleware

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokenLinkConfig {
    pub enabled: bool,
}

impl BrokenLinkConfig {
    pub fn new() -> Self {
        Self { enabled: false }
    }
}

impl Default for BrokenLinkConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BrokenLinkEmailsMiddleware;

impl BrokenLinkEmailsMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl Default for BrokenLinkEmailsMiddleware {
    fn default() -> Self {
        Self::new()
    }
}
