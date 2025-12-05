//! Attribute macro implementation for `#[model(...)]`

use proc_macro2::TokenStream;
use quote::quote;
use syn::{Attribute, ItemStruct, Result};

pub(crate) fn model_attribute_impl(
	args: TokenStream,
	mut input: ItemStruct,
) -> Result<TokenStream> {
	// Check if #[derive(Model)] already exists (avoid double processing)
	let has_derive_model = input.attrs.iter().any(|attr| {
		if attr.path().is_ident("derive")
			&& let syn::Meta::List(meta_list) = &attr.meta
		{
			return meta_list.tokens.to_string().contains("Model");
		}
		false
	});

	if has_derive_model {
		// Already has #[derive(Model)], just return input unchanged
		// The derive macro will read #[model(...)] helper attribute
		return Ok(quote! { #input });
	}

	/// Check if a specific trait is already in #[derive(...)] attributes
	fn has_derive_trait(attrs: &[Attribute], trait_name: &str) -> bool {
		attrs.iter().any(|attr| {
			if attr.path().is_ident("derive")
				&& let syn::Meta::List(meta_list) = &attr.meta
			{
				// Parse tokens to extract individual trait names
				let tokens_str = meta_list.tokens.to_string();
				// Split by commas and check each identifier
				return tokens_str
					.split(',')
					.any(|s| s.trim().split("::").last().unwrap_or("") == trait_name);
			}
			false
		})
	}

	// Create a #[model_config(...)] helper attribute with the arguments
	// Using model_config instead of model to avoid name collision with the attribute macro
	let config_attr: Attribute = if args.is_empty() {
		syn::parse_quote! { #[model_config] }
	} else {
		syn::parse_quote! { #[model_config(#args)] }
	};

	// Build derive list with required traits
	// Model must be first for proper attribute processing
	let mut derive_traits = vec!["Model"];

	// Add common traits if not already present
	// Note: Eq and Hash are NOT included by default because:
	// - Not all types implement Eq/Hash (e.g., f64, f32)
	// - Most models don't need these traits for database operations
	// - Users can manually add them when needed
	// Note: Serialize and Deserialize are NOT included by default because:
	// - They require serde to be in scope at the call site
	// - The derive attribute's scope doesn't inherit the caller's imports
	// - Users should manually add #[derive(Serialize, Deserialize)] when needed
	let required_traits = ["Debug", "Clone", "PartialEq"];

	for trait_name in &required_traits {
		if !has_derive_trait(&input.attrs, trait_name) {
			derive_traits.push(trait_name);
		}
	}

	// Create derive attribute with all required traits
	let derive_attr: Attribute = if derive_traits.len() == 1 {
		// Only Model needed (user already has others)
		syn::parse_quote! { #[derive(Model)] }
	} else {
		// Build attribute with multiple traits
		let traits_str = derive_traits.join(", ");
		let tokens: TokenStream = traits_str
			.parse()
			.expect("Failed to parse derive traits - this is a bug");
		syn::parse_quote! { #[derive(#tokens)] }
	};

	// Insert at the beginning to ensure Model is processed first
	input.attrs.insert(0, derive_attr);

	// Add the helper attribute AFTER the derive
	input.attrs.insert(1, config_attr);

	// Note: We don't generate auto-imports here because:
	// 1. Each #[model] usage would generate duplicate imports in the same module
	// 2. The Model derive macro uses absolute paths (::reinhardt::db::orm::Model etc.)
	// 3. derive(Serialize, Deserialize) doesn't require explicit use statements
	// Users should import serde traits themselves if needed for non-derive usage

	Ok(quote! { #input })
}
