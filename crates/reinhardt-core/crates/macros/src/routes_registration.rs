//! Routes attribute macro implementation
//!
//! This module implements the `#[routes]` attribute macro that allows
//! functions to be registered as URL pattern providers for automatic
//! discovery by the framework.
//!
//! # Macro Syntax
//!
//! ```rust,ignore
//! #[routes]
//! pub fn routes() -> UnifiedRouter {
//!     UnifiedRouter::new()
//!         .endpoint(views::index)
//!         .endpoint(views::about)
//! }
//! ```
//!
//! # Generated Code
//!
//! The macro preserves the original function and adds `inventory::submit!`
//! registration code:
//!
//! ```rust,ignore
//! // Input:
//! #[routes]
//! pub fn routes() -> UnifiedRouter {
//!     UnifiedRouter::new()
//! }
//!
//! // Generated output:
//! pub fn routes() -> UnifiedRouter {
//!     UnifiedRouter::new()
//! }
//!
//! ::reinhardt::inventory::submit! {
//!     ::reinhardt::UrlPatternsRegistration {
//!         get_router: || ::std::sync::Arc::new(routes()),
//!     }
//! }
//! ```

use crate::crate_paths::get_reinhardt_crate;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Result};

/// Implementation of the `#[routes]` attribute macro
///
/// This function generates code that:
/// 1. Preserves the original function definition
/// 2. Adds `inventory::submit!` to register the function with the framework
///
/// # Parameters
///
/// * `_args` - Attribute arguments (currently unused, reserved for future use)
/// * `input` - The function to annotate
///
/// # Returns
///
/// Generated code as a `TokenStream`
///
/// # Errors
///
/// Returns an error if the function signature is invalid (e.g., missing return type)
pub(crate) fn routes_impl(_args: TokenStream, input: ItemFn) -> Result<TokenStream> {
	let reinhardt = get_reinhardt_crate();

	let fn_name = &input.sig.ident;
	let fn_vis = &input.vis;
	let fn_attrs = &input.attrs;
	let fn_sig = &input.sig;
	let fn_block = &input.block;

	// Validate that the function has a return type
	if matches!(input.sig.output, syn::ReturnType::Default) {
		return Err(syn::Error::new_spanned(
			&input.sig,
			"#[routes] function must have a return type (e.g., -> UnifiedRouter)",
		));
	}

	// Generate the original function and the inventory registration
	let expanded = quote! {
		#(#fn_attrs)*
		#fn_vis #fn_sig #fn_block

		#reinhardt::inventory::submit! {
			#reinhardt::UrlPatternsRegistration {
				get_router: || ::std::sync::Arc::new(#fn_name()),
			}
		}
	};

	Ok(expanded)
}
