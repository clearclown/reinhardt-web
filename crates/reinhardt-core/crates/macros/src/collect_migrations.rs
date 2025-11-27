//! Implementation of the `collect_migrations!` macro
//!
//! This macro generates a `MigrationProvider` implementation and registers it
//! with the global migration registry using `linkme::distributed_slice`.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{
	Ident, LitStr, Token,
	parse::{Parse, ParseStream},
	punctuated::Punctuated,
};

/// Input structure for the `collect_migrations!` macro
///
/// Parses:
/// ```text
/// collect_migrations!(
///     app_label = "polls",
///     _0001_initial,
///     _0002_add_fields,
/// );
/// ```
struct CollectMigrationsInput {
	/// The app label (e.g., "polls")
	app_label: String,
	/// List of migration module names
	migrations: Vec<Ident>,
}

impl Parse for CollectMigrationsInput {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		// Parse: app_label = "polls"
		let app_label_ident: Ident = input.parse()?;
		if app_label_ident != "app_label" {
			return Err(syn::Error::new(
				app_label_ident.span(),
				"expected `app_label`",
			));
		}
		let _eq: Token![=] = input.parse()?;
		let app_label_lit: LitStr = input.parse()?;
		let app_label = app_label_lit.value();

		// Parse comma after app_label
		let _comma: Token![,] = input.parse()?;

		// Parse migration module names
		let migrations: Punctuated<Ident, Token![,]> = Punctuated::parse_terminated(input)?;
		let migrations: Vec<Ident> = migrations.into_iter().collect();

		if migrations.is_empty() {
			return Err(syn::Error::new(
				input.span(),
				"at least one migration module is required",
			));
		}

		Ok(CollectMigrationsInput {
			app_label,
			migrations,
		})
	}
}

/// Implementation of the `collect_migrations!` macro
pub fn collect_migrations_impl(input: TokenStream) -> Result<TokenStream, syn::Error> {
	let input: CollectMigrationsInput = syn::parse2(input)?;

	let app_label = &input.app_label;
	let migrations = &input.migrations;

	// Generate struct name from app_label (e.g., "polls" -> "PollsMigrations")
	let struct_name = format_ident!("{}Migrations", to_pascal_case(app_label));

	// Generate static name for distributed_slice (e.g., "__POLLS_MIGRATIONS_PROVIDER")
	let static_name = format_ident!("__{}_MIGRATIONS_PROVIDER", app_label.to_uppercase());

	// Build the migrations vector with each module's `migration()` function
	let migration_calls = migrations.iter().map(|m| {
		quote! { #m::migration() }
	});

	let expanded = quote! {
		/// Auto-generated migration provider struct for app `#app_label`
		pub struct #struct_name;

		impl ::reinhardt_migrations::MigrationProvider for #struct_name {
			fn migrations() -> Vec<::reinhardt_migrations::Migration> {
				vec![
					#(#migration_calls),*
				]
			}
		}

		impl #struct_name {
			/// Returns all migrations for this app
			pub fn all() -> Vec<::reinhardt_migrations::Migration> {
				<Self as ::reinhardt_migrations::MigrationProvider>::migrations()
			}
		}

		#[::linkme::distributed_slice(::reinhardt_migrations::registry::MIGRATION_PROVIDERS)]
		static #static_name: ::reinhardt_migrations::registry::MigrationProviderFn =
			<#struct_name as ::reinhardt_migrations::MigrationProvider>::migrations;
	};

	Ok(expanded)
}

/// Convert a string to PascalCase
///
/// Examples:
/// - "polls" -> "Polls"
/// - "user_profile" -> "UserProfile"
/// - "my-app" -> "MyApp"
fn to_pascal_case(s: &str) -> String {
	let mut result = String::new();
	let mut capitalize_next = true;

	for c in s.chars() {
		if c == '_' || c == '-' {
			capitalize_next = true;
		} else if capitalize_next {
			result.push(c.to_ascii_uppercase());
			capitalize_next = false;
		} else {
			result.push(c);
		}
	}

	result
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_to_pascal_case() {
		assert_eq!(to_pascal_case("polls"), "Polls");
		assert_eq!(to_pascal_case("user_profile"), "UserProfile");
		assert_eq!(to_pascal_case("my-app"), "MyApp");
		assert_eq!(to_pascal_case("UPPER"), "UPPER");
		assert_eq!(to_pascal_case("camelCase"), "CamelCase");
	}
}
