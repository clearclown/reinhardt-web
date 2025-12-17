//! Application configuration for examples-github-issues
//!
//! This module defines the installed applications using compile-time validation.

use reinhardt::installed_apps;

// Define installed applications with compile-time validation
// The macro will fail to compile if any referenced reinhardt.contrib.* app doesn't exist
installed_apps! {
	// Reinhardt contrib apps
	reinhardt_auth: "reinhardt.contrib.auth",
	contenttypes: "reinhardt.contrib.contenttypes",
	sessions: "reinhardt.contrib.sessions",
	drf: "reinhardt.drf",

	// Custom apps
	auth: "auth",
	projects: "projects",
	issues: "issues",
}

/// Get the list of installed applications
pub fn get_installed_apps() -> Vec<String> {
    InstalledApp::all_apps()
}
