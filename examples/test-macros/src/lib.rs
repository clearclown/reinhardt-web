//! Custom Test Macros
//!
//! Provides test macros with version verification using Cargo version specifiers.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

/// Test macro using Cargo version specifiers
///
/// # Examples
///
/// ```rust
/// #[example_test(version = "0.1.0")]
/// fn test_exact_version() {
///     // Run only on reinhardt 0.1.0
/// }
///
/// #[example_test(version = "^0.1")]
/// fn test_caret_requirement() {
///     // Run only on reinhardt ^0.1 (0.1.x)
/// }
///
/// #[example_test(version = ">=0.1.0, <0.2.0")]
/// fn test_version_range() {
///     // Run only on reinhardt 0.1.x
/// }
///
/// #[example_test(version = "*")]
/// fn test_latest() {
///     // Run on latest version
/// }
/// ```
///
/// # Supported Version Specifiers
///
/// - `"0.1.0"` - Exact version
/// - `"^0.1"` - Caret requirement (0.1.x)
/// - `"~0.1.2"` - Tilde requirement (0.1.2 <= version < 0.2.0)
/// - `">=0.1, <0.2"` - Range specification
/// - `"*"` - Wildcard (latest)
#[proc_macro_attribute]
pub fn example_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Extract version specifier
    let version_spec = parse_macro_input!(attr as LitStr).value();
    let test_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &test_fn.sig.ident;
    let fn_block = &test_fn.block;
    let fn_attrs = &test_fn.attrs;
    let fn_async = &test_fn.sig.asyncness;

    // Generate code differently based on async/sync
    let expanded = if fn_async.is_some() {
        quote! {
            #(#fn_attrs)*
            #[tokio::test]
            async fn #fn_name() {
                // Version check
                if !example_common::version::check_version(#version_spec) {
                    eprintln!(
                        "⏭️  Skipping test '{}': version mismatch",
                        stringify!(#fn_name)
                    );
                    eprintln!(
                        "   Required: {}, Actual: {}",
                        #version_spec,
                        example_common::version::get_reinhardt_version()
                    );
                    return; // Skip test
                }

                // crates.io availability check
                if !example_common::availability::is_reinhardt_available() {
                    eprintln!(
                        "⏭️  Skipping test '{}': reinhardt not available from crates.io",
                        stringify!(#fn_name)
                    );
                    return; // Skip test
                }

                // Execute actual test
                #fn_block
            }
        }
    } else {
        quote! {
            #(#fn_attrs)*
            #[test]
            fn #fn_name() {
                // Version check
                if !example_common::version::check_version(#version_spec) {
                    eprintln!(
                        "⏭️  Skipping test '{}': version mismatch",
                        stringify!(#fn_name)
                    );
                    eprintln!(
                        "   Required: {}, Actual: {}",
                        #version_spec,
                        example_common::version::get_reinhardt_version()
                    );
                    return; // Skip test
                }

                // crates.io availability check
                if !example_common::availability::is_reinhardt_available() {
                    eprintln!(
                        "⏭️  Skipping test '{}': reinhardt not available from crates.io",
                        stringify!(#fn_name)
                    );
                    return; // Skip test
                }

                // Execute actual test
                #fn_block
            }
        }
    };

    TokenStream::from(expanded)
}
