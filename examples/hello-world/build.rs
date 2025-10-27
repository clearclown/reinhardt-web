//! Build script for hello-world example
//!
//! This build script checks if reinhardt is available from crates.io
//! and whether the version matches the requirement (*).
//!
//! If either check fails, appropriate cfg flags are set to conditionally
//! exclude code compilation.

fn main() {
    // Check if reinhardt is available from crates.io
    if !example_common::build_check::check_reinhardt_availability_at_build_time() {
        println!("cargo:rustc-cfg=reinhardt_unavailable");
        println!("cargo:warning=hello-world example requires reinhardt from crates.io");
        println!("cargo:warning=Example code will be stubbed out");
        return;
    }

    // Check version requirement: * (any version)
    if !example_common::build_check::check_version_requirement_at_build_time("*") {
        println!("cargo:rustc-cfg=reinhardt_version_mismatch");
        println!("cargo:warning=hello-world example version check failed");
        println!("cargo:warning=Example code will be stubbed out");
        return;
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");
}
