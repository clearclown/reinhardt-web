//! Helper functions for dynamic crate path resolution using proc_macro_crate

use proc_macro2::TokenStream;
use quote::quote;

/// Resolves the path to the reinhardt_di crate dynamically.
pub fn get_reinhardt_di_crate() -> TokenStream {
	use proc_macro_crate::{FoundCrate, crate_name};

	// Try direct crate first
	match crate_name("reinhardt-di") {
		Ok(FoundCrate::Itself) => return quote!(crate),
		Ok(FoundCrate::Name(name)) => {
			let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
			return quote!(::#ident);
		}
		Err(_) => {
			// Try via reinhardt crate
			match crate_name("reinhardt") {
				Ok(FoundCrate::Itself) => return quote!(crate::reinhardt_di),
				Ok(FoundCrate::Name(name)) => {
					let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
					return quote!(::#ident::reinhardt_di);
				}
				Err(_) => {}
			}

			// Try via reinhardt-web (published package name)
			match crate_name("reinhardt-web") {
				Ok(FoundCrate::Itself) => return quote!(crate::reinhardt_di),
				Ok(FoundCrate::Name(name)) => {
					let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
					return quote!(::#ident::reinhardt_di);
				}
				Err(_) => {}
			}
		}
	}

	// Final fallback
	quote!(::reinhardt_di)
}

/// Resolves the path to the reinhardt_http crate dynamically.
pub fn get_reinhardt_http_crate() -> TokenStream {
	use proc_macro_crate::{FoundCrate, crate_name};

	// Try direct crate first
	match crate_name("reinhardt-http") {
		Ok(FoundCrate::Itself) => return quote!(crate),
		Ok(FoundCrate::Name(name)) => {
			let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
			return quote!(::#ident);
		}
		Err(_) => {
			// Try via reinhardt crate (when used with `package = "reinhardt-web"`)
			match crate_name("reinhardt") {
				Ok(FoundCrate::Itself) => return quote!(crate::reinhardt_http),
				Ok(FoundCrate::Name(name)) => {
					let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
					return quote!(::#ident::reinhardt_http);
				}
				Err(_) => {}
			}

			// Try via reinhardt-web (published package name)
			match crate_name("reinhardt-web") {
				Ok(FoundCrate::Itself) => return quote!(crate::reinhardt_http),
				Ok(FoundCrate::Name(name)) => {
					let ident = syn::Ident::new(&name, proc_macro2::Span::call_site());
					return quote!(::#ident::reinhardt_http);
				}
				Err(_) => {}
			}
		}
	}

	// Final fallback
	quote!(::reinhardt_http)
}
