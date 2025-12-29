//! Build script for {{ project_name }}.
//!
//! Sets up cfg aliases for simplified conditional compilation.

use cfg_aliases::cfg_aliases;

fn main() {
	// Rust 2024 edition requires explicit check-cfg declarations
	println!("cargo::rustc-check-cfg=cfg(wasm)");
	println!("cargo::rustc-check-cfg=cfg(native)");

	cfg_aliases! {
		// Platform aliases for simpler conditional compilation
		// Use `#[cfg(wasm)]` instead of `#[cfg(target_arch = "wasm32")]`
		wasm: { target_arch = "wasm32" },
		// Use `#[cfg(native)]` instead of `#[cfg(not(target_arch = "wasm32"))]`
		native: { not(target_arch = "wasm32") },
	}
}
