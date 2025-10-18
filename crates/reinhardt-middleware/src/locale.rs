//! Locale detection middleware

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleConfig {
    pub default_locale: String,
}

impl LocaleConfig {
    pub fn new() -> Self {
        Self {
            default_locale: "en".to_string(),
        }
    }
}

impl Default for LocaleConfig {
    fn default() -> Self {
        Self::new()
    }
}

pub struct LocaleMiddleware;

impl LocaleMiddleware {
    pub fn new() -> Self {
        Self
    }
}

impl Default for LocaleMiddleware {
    fn default() -> Self {
        Self::new()
    }
}
