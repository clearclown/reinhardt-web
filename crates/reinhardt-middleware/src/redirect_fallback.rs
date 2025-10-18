//! Redirect fallback middleware

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedirectResponseConfig {
    pub fallback_url: String,
}

impl RedirectResponseConfig {
    pub fn new(fallback_url: String) -> Self {
        Self { fallback_url }
    }
}

pub struct RedirectFallbackMiddleware;

impl RedirectFallbackMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RedirectFallbackMiddleware {
    fn default() -> Self {
        Self::new()
    }
}
