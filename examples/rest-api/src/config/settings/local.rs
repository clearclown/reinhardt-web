//! Local development settings for example-rest-api

#[cfg(not(any(reinhardt_unavailable, reinhardt_version_mismatch)))]
pub use available::*;

#[cfg(not(any(reinhardt_unavailable, reinhardt_version_mismatch)))]
mod available {
    use reinhardt_core::Settings;
    use super::base::get_base_settings;

    /// Get local development settings
    pub fn get_settings() -> Settings {
        // Use a simple secret key for development
        // WARNING: Never use this in production!
        let secret_key = "dev-secret-key-not-for-production".to_string();

        let mut settings = get_base_settings(secret_key, true);

        // Development-specific overrides
        settings.debug = true;

        // Allow all hosts in development
        settings.allowed_hosts = vec!["*".to_string()];

        settings
    }
}

#[cfg(any(reinhardt_unavailable, reinhardt_version_mismatch))]
pub use unavailable::*;

#[cfg(any(reinhardt_unavailable, reinhardt_version_mismatch))]
mod unavailable {
    pub fn get_settings() -> () {
        ()
    }
}
