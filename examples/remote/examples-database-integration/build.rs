//! Build script for database-integration example (remote mode)
//!
//! This example uses reinhardt from crates.io.
//! Tests are enabled when reinhardt is available on crates.io.
//! Version specification is controlled by Cargo.toml.

use example_common::availability;

fn main() {
	// Check if reinhardt is available on crates.io before enabling tests
	let required_version = "^0.1.0";

	match availability::verify_reinhardt_for_build(required_version) {
		Ok(_) => {
			// reinhardt is available on crates.io
			println!("cargo:rustc-cfg=feature=\"with-reinhardt\"");
			println!(
				"cargo:warning=✅ reinhardt {} is available on crates.io",
				required_version
			);
		}
		Err(e) => {
			// reinhardt is not available - skip tests
			println!(
				"cargo:warning=⚠️  reinhardt {} not available on crates.io",
				required_version
			);
			println!("cargo:warning=   {}", e);
			println!("cargo:warning=   Tests will be skipped (with-reinhardt feature disabled)");

			// Do NOT set with-reinhardt feature
			// Tests with #[cfg(feature = "with-reinhardt")] will be skipped
		}
	}

	println!("cargo:rerun-if-changed=build.rs");
}
