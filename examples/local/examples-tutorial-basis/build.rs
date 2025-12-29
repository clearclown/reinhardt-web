use cfg_aliases::cfg_aliases;

fn main() {
	// Declare custom cfg to avoid warnings in Rust 2024 edition
	println!("cargo::rustc-check-cfg=cfg(with_reinhardt)");
	println!("cargo::rustc-check-cfg=cfg(wasm)");
	println!("cargo::rustc-check-cfg=cfg(native)");

	// Local examples always enable with-reinhardt feature
	println!("cargo:rustc-cfg=with_reinhardt");

	cfg_aliases! {
		// Platform aliases for simpler conditional compilation
		// Use `#[cfg(wasm)]` instead of `#[cfg(target_arch = "wasm32")]`
		wasm: { target_arch = "wasm32" },
		// Use `#[cfg(native)]` instead of `#[cfg(not(target_arch = "wasm32"))]`
		native: { not(target_arch = "wasm32") },
	}
}
