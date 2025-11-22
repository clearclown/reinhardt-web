//! Reinhardt Project Management CLI for {{ project_name }}
//!
//! This is the project-specific management command interface (equivalent to Django's manage.py).

use reinhardt_commands::execute_from_command_line;
use std::process;

#[tokio::main]
async fn main() {
	// Set settings module environment variable
	// SAFETY: This is safe because we're setting it before any other code runs
	unsafe {
		std::env::set_var("REINHARDT_SETTINGS_MODULE", "{{ project_name }}.config.settings");
	}

	// Execute command from command line
	if let Err(e) = execute_from_command_line().await {
		eprintln!("Error: {}", e);
		process::exit(1);
	}
}
