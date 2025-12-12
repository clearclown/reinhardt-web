//! Helper functions for dynamic crate path resolution using proc_macro_crate

use proc_macro2::TokenStream;
use quote::quote;

/// Resolves the path to the reinhardt_openapi crate dynamically.
///
/// This supports different crate naming scenarios (reinhardt-openapi, renamed crates, etc.)
pub fn get_reinhardt_openapi_crate() -> TokenStream {
	use proc_macro_crate::{FoundCrate, crate_name};

	match crate_name("reinhardt-openapi") {
		Ok(FoundCrate::Itself) => quote!(crate),
		Ok(FoundCrate::Name(name)) => {
			let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
			quote!(::#ident)
		}
		Err(_) => quote!(::reinhardt_openapi), // Fallback
	}
}
