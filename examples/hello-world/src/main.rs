// Hello World Example
//
// This example demonstrates the simplest possible reinhardt application.
// It only compiles when reinhardt is available from crates.io (any version).

#[cfg(not(any(reinhardt_unavailable, reinhardt_version_mismatch)))]
mod app {
	use reinhardt::prelude::*;

	pub fn run() {
		println!("Hello, Reinhardt!");
		println!("✅ Reinhardt is available and working");
	}
}

#[cfg(any(reinhardt_unavailable, reinhardt_version_mismatch))]
mod app {
	pub fn run() {
		eprintln!("⚠️  Hello World Example");
		eprintln!();
		eprintln!("This example requires reinhardt from crates.io.");
		eprintln!();
		eprintln!("Current status:");
		#[cfg(reinhardt_unavailable)]
		eprintln!("  ❌ reinhardt is not available from crates.io");
		#[cfg(reinhardt_version_mismatch)]
		eprintln!("  ❌ reinhardt version check failed");
		eprintln!();
		eprintln!("This example will be available once reinhardt is published.");
		eprintln!();
		eprintln!("For development, use the integration tests in tests/ directory instead.");
	}
}

fn main() {
	app::run();
}
