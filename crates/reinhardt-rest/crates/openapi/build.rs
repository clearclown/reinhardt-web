use std::env;
use std::path::PathBuf;

fn main() {
	// Traverse 4 levels up from CARGO_MANIFEST_DIR to get workspace root
	// crates/reinhardt-rest/crates/openapi -> crates/reinhardt-rest/crates
	//                                       -> crates/reinhardt-rest
	//                                       -> crates
	//                                       -> (workspace root)
	let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
	let workspace_root = PathBuf::from(manifest_dir)
		.parent() // crates/reinhardt-rest/crates
		.and_then(|p| p.parent()) // crates/reinhardt-rest
		.and_then(|p| p.parent()) // crates
		.and_then(|p| p.parent()) // workspace root
		.expect("Failed to determine workspace root")
		.to_path_buf();

	// Set workspace root as environment variable
	println!(
		"cargo:rustc-env=WORKSPACE_ROOT={}",
		workspace_root.display()
	);

	// Trigger rebuild when branding/thirdparty directory changes
	let branding_dir = workspace_root.join("branding/thirdparty");
	println!("cargo:rerun-if-changed={}", branding_dir.display());
}
