//! Static File URL Resolver
//!
//! This module provides a global static file URL resolver that integrates with
//! the Reinhardt static files system. It allows components to resolve static
//! file paths to their final URLs, supporting manifest-based hashing for
//! cache busting.
//!
//! ## Usage
//!
//! ### Initialization
//!
//! The static resolver must be initialized before use, typically in your
//! server startup code:
//!
//! ```ignore
//! use reinhardt_pages::static_resolver::init_static_resolver;
//! use reinhardt_static::TemplateStaticConfig;
//!
//! // Initialize with basic configuration
//! init_static_resolver(TemplateStaticConfig::new("/static/".to_string()));
//!
//! // Or with manifest for cache-busted filenames
//! let manifest = load_manifest("staticfiles.json").await?;
//! init_static_resolver(
//!     TemplateStaticConfig::new("/static/".to_string())
//!         .with_manifest(manifest)
//! );
//! ```
//!
//! ### Resolving URLs
//!
//! Once initialized, use `resolve_static` to get the final URL:
//!
//! ```ignore
//! use reinhardt_pages::static_resolver::resolve_static;
//!
//! let css_url = resolve_static("css/style.css");
//! // Returns: "/static/css/style.css"
//! // Or with manifest: "/static/css/style.abc123.css"
//!
//! let js_url = resolve_static("js/app.js");
//! // Returns: "/static/js/app.js"
//! ```
//!
//! ### Using with head! macro
//!
//! ```ignore
//! use reinhardt_pages::{head, static_resolver::resolve_static};
//!
//! let my_head = head!(|| {
//!     link { rel: "stylesheet", href: resolve_static("css/style.css") }
//!     script { src: resolve_static("js/app.js"), defer }
//! });
//! ```
//!
//! ## Thread Safety
//!
//! The static resolver uses `OnceLock` for thread-safe lazy initialization.
//! It can only be initialized once per application lifecycle.

use std::sync::OnceLock;

#[cfg(not(target_arch = "wasm32"))]
use reinhardt_static::TemplateStaticConfig;

/// Global static configuration storage.
///
/// This is initialized once at application startup and provides
/// thread-safe access to the static URL resolver.
#[cfg(not(target_arch = "wasm32"))]
static STATIC_CONFIG: OnceLock<TemplateStaticConfig> = OnceLock::new();

/// WASM-specific static URL prefix.
///
/// In WASM environments, we use a simple prefix since there's no
/// server-side manifest processing.
#[cfg(target_arch = "wasm32")]
static STATIC_URL_PREFIX: OnceLock<String> = OnceLock::new();

/// Initializes the static resolver with the given configuration.
///
/// This function should be called once during application startup,
/// before any calls to `resolve_static`.
///
/// ## Example
///
/// ```ignore
/// use reinhardt_pages::static_resolver::init_static_resolver;
/// use reinhardt_static::TemplateStaticConfig;
///
/// // Basic initialization
/// init_static_resolver(TemplateStaticConfig::new("/static/".to_string()));
/// ```
///
/// ## Panics
///
/// This function does not panic if called multiple times; subsequent
/// calls are silently ignored.
#[cfg(not(target_arch = "wasm32"))]
pub fn init_static_resolver(config: TemplateStaticConfig) {
	// OnceLock::set returns Err if already set, but we ignore this
	// to allow for idempotent initialization
	let _ = STATIC_CONFIG.set(config);
}

/// Initializes the static resolver with a URL prefix (WASM version).
///
/// In WASM environments, static files are typically served from a
/// fixed prefix without manifest-based hashing.
///
/// ## Example
///
/// ```ignore
/// use reinhardt_pages::static_resolver::init_static_resolver;
///
/// init_static_resolver("/static/".to_string());
/// ```
#[cfg(target_arch = "wasm32")]
pub fn init_static_resolver(static_url: String) {
	let _ = STATIC_URL_PREFIX.set(static_url);
}

/// Resolves a static file path to its final URL.
///
/// This function takes a relative path to a static file and returns
/// the full URL that should be used in HTML. If a manifest is configured,
/// it will use the hashed filename for cache busting.
///
/// ## Arguments
///
/// * `path` - The relative path to the static file (e.g., "css/style.css")
///
/// ## Returns
///
/// The resolved URL (e.g., "/static/css/style.abc123.css")
///
/// ## Example
///
/// ```ignore
/// use reinhardt_pages::static_resolver::resolve_static;
///
/// let css_url = resolve_static("css/style.css");
/// // Returns: "/static/css/style.css"
///
/// let favicon = resolve_static("favicon.png");
/// // Returns: "/static/favicon.png"
/// ```
///
/// ## Fallback Behavior
///
/// If the static resolver has not been initialized:
/// - A warning is logged to stderr
/// - The function returns a fallback URL using "/static/" prefix
///
/// This ensures the application continues to work even if initialization
/// was missed, though cache busting won't be available.
#[cfg(not(target_arch = "wasm32"))]
pub fn resolve_static(path: &str) -> String {
	STATIC_CONFIG
		.get()
		.map(|config| config.resolve_url(path))
		.unwrap_or_else(|| {
			// Only warn once to avoid log spam
			static WARNED: std::sync::atomic::AtomicBool =
				std::sync::atomic::AtomicBool::new(false);
			if !WARNED.swap(true, std::sync::atomic::Ordering::SeqCst) {
				eprintln!(
					"WARNING: Static resolver not initialized. Call init_static_resolver() at startup."
				);
			}

			// Fallback: use simple concatenation
			let path = path.trim_start_matches('/');
			format!("/static/{}", path)
		})
}

/// Resolves a static file path to its URL (WASM version).
///
/// In WASM environments, this simply concatenates the configured
/// prefix with the path.
#[cfg(target_arch = "wasm32")]
pub fn resolve_static(path: &str) -> String {
	let prefix = STATIC_URL_PREFIX
		.get()
		.map(|s| s.as_str())
		.unwrap_or("/static/");

	let prefix = prefix.trim_end_matches('/');
	let path = path.trim_start_matches('/');
	format!("{}/{}", prefix, path)
}

/// Checks if the static resolver has been initialized.
///
/// This can be useful for debugging or for conditional initialization.
///
/// ## Example
///
/// ```ignore
/// use reinhardt_pages::static_resolver::{is_initialized, init_static_resolver};
///
/// if !is_initialized() {
///     init_static_resolver(config);
/// }
/// ```
#[cfg(not(target_arch = "wasm32"))]
pub fn is_initialized() -> bool {
	STATIC_CONFIG.get().is_some()
}

/// Checks if the static resolver has been initialized (WASM version).
#[cfg(target_arch = "wasm32")]
pub fn is_initialized() -> bool {
	STATIC_URL_PREFIX.get().is_some()
}

#[cfg(test)]
mod tests {
	use super::*;
	use rstest::rstest;

	#[cfg(not(target_arch = "wasm32"))]
	mod native_tests {
		use super::*;

		#[rstest]
		fn test_resolve_static_fallback() {
			// This test relies on the resolver not being initialized in this test environment
			// Note: In a real app, we'd test with proper initialization
			let url = resolve_static("test.css");
			assert!(url.contains("test.css"));
		}

		#[rstest]
		fn test_resolve_static_strips_leading_slash() {
			let url = resolve_static("/test.css");
			assert!(url.contains("test.css"));
			// Should not have double slashes
			assert!(!url.contains("//static"));
		}
	}

	#[cfg(target_arch = "wasm32")]
	mod wasm_tests {
		use super::*;
		use wasm_bindgen_test::*;

		#[wasm_bindgen_test]
		fn test_resolve_static_wasm() {
			let url = resolve_static("test.css");
			assert!(url.contains("test.css"));
		}
	}
}
