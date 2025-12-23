//! Reinhardt Project Management CLI for examples-twitter
//!
//! This is the project-specific management command interface (equivalent to Django's manage.py).

use examples_twitter::config; // Explicitly reference config module
use reinhardt::commands::execute_from_command_line;
use reinhardt::core::tokio;
use std::process;

#[tokio::main]
async fn main() {
	// Set settings module environment variable
	// SAFETY: This is safe because we're setting it before any other code runs
	unsafe {
		std::env::set_var(
			"REINHARDT_SETTINGS_MODULE",
			"examples_twitter.config.settings",
		);
	}

	// Ensure config module is loaded (triggers register_url_patterns! macro)
	let _ = &config::urls::url_patterns;

	// Router registration and database connection are now automatic
	// via register_url_patterns!() macro in src/config/urls.rs

	// Execute command from command line
	if let Err(e) = execute_from_command_line().await {
		eprintln!("Error: {}", e);
		process::exit(1);
	}
}
