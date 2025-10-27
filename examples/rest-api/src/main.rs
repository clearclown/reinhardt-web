//! REST API Example
//!
//! This example demonstrates a RESTful API with reinhardt.
//! It only compiles when reinhardt is available from crates.io with version ^0.1.
//!
//! To run this example:
//! - Use `cargo run --bin manage runserver` to start the development server
//! - Use `cargo run` to see the basic app initialization

mod config;
mod apps;

// This example only compiles when reinhardt is available from crates.io with version ^0.1.
#[cfg(not(any(reinhardt_unavailable, reinhardt_version_mismatch)))]
pub use available::*;

#[cfg(not(any(reinhardt_unavailable, reinhardt_version_mismatch)))]
mod available {
    use crate::config::settings::get_settings;
    use crate::config::apps::get_installed_apps;
    use crate::config::urls::url_patterns;

    pub fn run() {
        println!("REST API Example");

        // Load settings based on environment (REINHARDT_ENV)
        let settings = get_settings();
        println!("Settings loaded: debug={}", settings.debug);

        // Initialize app registry with compile-time validated apps
        match reinhardt_core::init_apps_checked(get_installed_apps) {
            Ok(_) => println!("✅ Apps initialized"),
            Err(e) => {
                eprintln!("❌ Failed to initialize apps: {}", e);
                return;
            }
        }

        // Get URL configuration and register globally
        let router = url_patterns();
        reinhardt_routers::register_router(router.clone());
        println!("✅ URL patterns registered");

        println!();
        println!("Application initialized successfully!");
        println!();
        println!("To start the development server, run:");
        println!("  cargo run --bin manage runserver");
        println!();
        println!("Available management commands:");
        println!("  cargo run --bin manage runserver    - Start development server");
        println!("  cargo run --bin manage showurls     - Display URL patterns");
        println!("  cargo run --bin manage check        - Check project configuration");
    }
}

#[cfg(any(reinhardt_unavailable, reinhardt_version_mismatch))]
pub use unavailable::*;

#[cfg(any(reinhardt_unavailable, reinhardt_version_mismatch))]
mod unavailable {
    pub fn run() {
        eprintln!("⚠️  REST API Example");
        eprintln!();
        eprintln!("This example requires reinhardt from crates.io (version ^0.1).");
        eprintln!();
        eprintln!("Current status:");
        #[cfg(reinhardt_unavailable)]
        eprintln!("  ❌ reinhardt is not available from crates.io");
        #[cfg(reinhardt_version_mismatch)]
        eprintln!("  ❌ reinhardt version does not match requirement ^0.1");
        eprintln!();
        eprintln!("This example will be available once reinhardt 0.1.x is published.");
        eprintln!();
        eprintln!("For development, use the integration tests in tests/ directory instead.");
    }
}

fn main() {
    run();
}
