//! Build script for hello-world example (remote mode)
//!
//! This example uses reinhardt from crates.io.
//! Tests are enabled when reinhardt is available on crates.io.
//! Version specification is controlled by Cargo.toml.

fn main() {
	// Remote mode: enable with-reinhardt feature
	// If reinhardt is not available on crates.io, dependency resolution will fail
	println!("cargo:rustc-cfg=feature=\"with-reinhardt\"");
	println!("cargo:warning=Using reinhardt from crates.io (examples/remote)");
	println!("cargo:rerun-if-changed=build.rs");
}
