//! Build script for hello-world example
//!
//! This build script controls the 'with-reinhardt' feature based on availability.
//! - Sets 'with-reinhardt' feature when reinhardt is available (local dev or crates.io)
//! - Tests with required-features = ["with-reinhardt"] are automatically enabled/disabled

/// Set to `true` to use local workspace crates, `false` to use crates.io
const USE_LOCAL_DEV: bool = true;

/// Required reinhardt version pattern (e.g., "^0.1", ">=0.1.0, <0.2.0", "*")
const REQUIRED_VERSION: &str = "*";

fn main() {
	if USE_LOCAL_DEV {
		// Local development mode: enable feature for tests
		println!("cargo:rustc-cfg=feature=\"with-reinhardt\"");
		println!("cargo:warning=Using local reinhardt workspace for examples");
		println!("cargo:rerun-if-changed=build.rs");
		println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");
		return;
	}

	// Production mode: check crates.io availability
	if !example_common::build_check::check_reinhardt_availability_at_build_time() {
		println!("cargo:warning=reinhardt not available from crates.io");
		println!("cargo:warning=Tests will be skipped (required-features not satisfied)");
		println!("cargo:rerun-if-changed=build.rs");
		return;
	}

	// Check version requirement
	if !example_common::build_check::check_version_requirement_at_build_time(REQUIRED_VERSION) {
		println!("cargo:warning=reinhardt version mismatch (requires {})", REQUIRED_VERSION);
		println!("cargo:warning=Tests will be skipped (required-features not satisfied)");
		println!("cargo:rerun-if-changed=build.rs");
		return;
	}

	// All checks passed: enable feature for tests
	println!("cargo:rustc-cfg=feature=\"with-reinhardt\"");
	println!("cargo:rerun-if-changed=build.rs");
	println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");
}
