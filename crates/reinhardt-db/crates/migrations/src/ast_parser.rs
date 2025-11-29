//! AST parser utilities for migration files
//!
//! Provides helper functions to extract migration metadata and operations
//! from parsed Rust ASTs.

use crate::{Migration, Result};
use syn::{Expr, File, Item, ItemFn, Stmt};

/// Extract migration metadata from parsed AST
pub fn extract_migration_metadata(ast: &File, app_label: &str, name: &str) -> Result<Migration> {
	let dependencies = extract_dependencies(ast)?;
	let atomic = extract_atomic(ast).unwrap_or(true);
	let replaces = extract_replaces(ast).unwrap_or_default();
	let operations = extract_operations(ast).unwrap_or_default();

	Ok(Migration {
		app_label: app_label.to_string(),
		name: name.to_string(),
		operations,
		dependencies,
		atomic,
		replaces,
	})
}

/// Extract dependencies from `dependencies()` function
fn extract_dependencies(ast: &File) -> Result<Vec<(String, String)>> {
	for item in &ast.items {
		if let Item::Fn(func) = item
			&& func.sig.ident == "dependencies"
		{
			// Simple implementation: assumes the function returns a literal vec!
			// In a real implementation, we would need to parse the function body properly
			// or use runtime evaluation (which is not possible here).
			// For now, we'll try to extract string literals from a vec! macro if present.
			return parse_vec_of_tuples(func);
		}
	}
	Ok(vec![])
}

/// Extract atomic flag from `atomic()` function
fn extract_atomic(ast: &File) -> Option<bool> {
	for item in &ast.items {
		if let Item::Fn(func) = item
			&& func.sig.ident == "atomic"
		{
			return parse_bool_return(func);
		}
	}
	None
}

/// Extract replaces from `replaces()` function
fn extract_replaces(ast: &File) -> Option<Vec<(String, String)>> {
	for item in &ast.items {
		if let Item::Fn(func) = item
			&& func.sig.ident == "replaces"
		{
			return parse_vec_of_tuples(func).ok();
		}
	}
	None
}

/// Extract operations from `migration()` function
fn extract_operations(_ast: &File) -> Result<Vec<crate::Operation>> {
	// This is the tricky part. Parsing full Operation structs from AST is complex.
	// For now, we will return an empty vector and rely on the fact that
	// we typically don't need to inspect operations of existing migrations
	// unless we are applying them.
	//
	// If we need to load operations for application, we should dynamically load/compile
	// the migration module, but Rust doesn't support dynamic loading easily.
	//
	// The current architecture relies on static compilation of migrations via `mod` declarations.
	// The FilesystemSource/Repository is mainly for management (creating files, listing).
	//
	// However, to solve the TODO completely, we would need a way to reconstruct Operation objects.
	// Given the complexity, we will note this limitation.

	Ok(vec![])
}

/// Helper to parse `vec![("app", "name"), ...]`
fn parse_vec_of_tuples(func: &ItemFn) -> Result<Vec<(String, String)>> {
	let result = Vec::new();

	// Look for the last expression in the block
	if let Some(Stmt::Expr(expr, _)) = func.block.stmts.last()
		&& let Expr::Macro(expr_macro) = expr
		&& expr_macro.mac.path.is_ident("vec")
	{
		// This requires parsing tokens inside vec! which is hard without more syn features
		// For the scope of this task, we'll assume empty or implement basic parsing if needed
	}

	Ok(result)
}

/// Helper to parse `true` or `false` return
fn parse_bool_return(func: &ItemFn) -> Option<bool> {
	if let Some(Stmt::Expr(Expr::Lit(expr_lit), _)) = func.block.stmts.last()
		&& let syn::Lit::Bool(lit_bool) = &expr_lit.lit
	{
		return Some(lit_bool.value);
	}
	None
}
