//! HTTP method route macros

use crate::path_macro;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
	Error, Expr, ExprLit, FnArg, ItemFn, Lit, LitStr, Meta, Pat, Result, Token, Type,
	parse::Parser, punctuated::Punctuated, spanned::Spanned,
};

/// パラメータエクストラクタの情報
#[derive(Clone)]
struct ExtractorInfo {
	pat: Box<Pat>,
	ty: Box<Type>,
	extractor_name: String,
}

/// Validate a route path at compile time
fn validate_route_path(path: &str, span: Span) -> Result<()> {
	path_macro::parse_and_validate(path)
		.map(|_| ())
		.map_err(|e| Error::new(span, format!("Invalid route path: {}", e)))
}

/// パラメータにエクストラクタが含まれているか検出
fn detect_extractors(inputs: &Punctuated<FnArg, Token![,]>) -> Vec<ExtractorInfo> {
	let mut extractors = Vec::new();

	for input in inputs {
		if let FnArg::Typed(pat_type) = input {
			// Path(id): Path<i64>のようなパターンを検出
			if let Pat::TupleStruct(_) = &*pat_type.pat
				&& let Type::Path(type_path) = &*pat_type.ty
				&& let Some(segment) = type_path.path.segments.last()
			{
				let type_name = segment.ident.to_string();
				if matches!(
					type_name.as_str(),
					"Path" | "Json" | "Query" | "Header" | "Cookie" | "Form" | "Body"
				) {
					extractors.push(ExtractorInfo {
						pat: pat_type.pat.clone(),
						ty: pat_type.ty.clone(),
						extractor_name: type_name,
					});
				}
			}
		}
	}

	extractors
}

/// Body消費型エクストラクタの重複をバリデーション
fn validate_extractors(extractors: &[ExtractorInfo]) -> Result<()> {
	let body_consuming_types = ["Json", "Form", "Body"];
	let body_extractors: Vec<_> = extractors
		.iter()
		.filter(|ext| body_consuming_types.contains(&ext.extractor_name.as_str()))
		.collect();

	if body_extractors.len() > 1 {
		let names: Vec<_> = body_extractors
			.iter()
			.map(|e| e.extractor_name.as_str())
			.collect();
		return Err(Error::new(
			Span::call_site(),
			format!(
				"Cannot use multiple body-consuming extractors: {}. Request body can only be read once.",
				names.join(", ")
			),
		));
	}

	Ok(())
}

/// ラッパー関数を生成（エクストラクタあり）
fn generate_wrapper_with_extractors(
	original_fn: &ItemFn,
	extractors: &[ExtractorInfo],
) -> TokenStream {
	let fn_name = &original_fn.sig.ident;
	let original_fn_name = quote::format_ident!("{}_original", fn_name);
	let fn_vis = &original_fn.vis;
	let fn_attrs = &original_fn.attrs;
	let output = &original_fn.sig.output;
	let fn_block = &original_fn.block;
	let asyncness = &original_fn.sig.asyncness;
	let generics = &original_fn.sig.generics;
	let where_clause = &original_fn.sig.generics.where_clause;

	// 元の関数のパラメータリスト
	let original_params = &original_fn.sig.inputs;

	// エクストラクタ呼び出しコード生成
	let extractor_calls: Vec<_> = extractors
		.iter()
		.map(|ext| {
			let pat = &ext.pat;
			let ty = &ext.ty;
			quote! {
				let #pat = <#ty as ::reinhardt::reinhardt_params::FromRequest>::from_request(&req, &ctx)
					.await
					.map_err(|e| ::reinhardt::reinhardt_core::exception::Error::Validation(
						format!("Parameter extraction failed: {:?}", e)
					))?;
			}
		})
		.collect();

	// 元の関数への引数（エクストラクタのパターンのみ）
	let call_args: Vec<_> = extractors.iter().map(|ext| &ext.pat).collect();

	// 生成コード
	quote! {
		// 元の関数（リネーム）
		#(#fn_attrs)*
		#fn_vis #asyncness fn #original_fn_name #generics (#original_params) #output #where_clause {
			#fn_block
		}

		// ラッパー関数
		#fn_vis #asyncness fn #fn_name(req: ::reinhardt::reinhardt_http::Request) #output {
			// ParamContext構築
			let ctx = ::reinhardt::reinhardt_params::ParamContext::with_path_params(req.path_params.clone());

			// エクストラクタ実行
			#(#extractor_calls)*

			// 元の関数呼び出し
			#original_fn_name(#(#call_args),*).await
		}
	}
}

fn route_impl(method: &str, args: TokenStream, input: ItemFn) -> Result<TokenStream> {
	let mut path: Option<(String, Span)> = None;

	// Handle the common case: #[get("/users/{id}")]
	// Try to parse as a single string literal first
	if let Ok(lit) = syn::parse2::<LitStr>(args.clone()) {
		let path_str = lit.value();
		validate_route_path(&path_str, lit.span())?;
		path = Some((path_str, lit.span()));
	} else {
		// Parse path argument for other formats
		let meta_list = Punctuated::<Meta, Token![,]>::parse_terminated.parse2(args)?;

		for meta in meta_list {
			match meta {
				Meta::Path(p) => {
					if let Some(ident) = p.get_ident() {
						let path_str = ident.to_string();
						validate_route_path(&path_str, p.span())?;
						path = Some((path_str, p.span()));
					}
				}
				Meta::NameValue(nv) if nv.path.is_ident("path") => {
					if let Expr::Lit(ExprLit {
						lit: Lit::Str(lit), ..
					}) = &nv.value
					{
						let path_str = lit.value();
						validate_route_path(&path_str, lit.span())?;
						path = Some((path_str, lit.span()));
					}
				}
				_ => {}
			}
		}
	}

	// エクストラクタ検出
	let extractors = detect_extractors(&input.sig.inputs);

	// エクストラクタがある場合はバリデーションとラッパー生成
	if !extractors.is_empty() {
		validate_extractors(&extractors)?;
		return Ok(generate_wrapper_with_extractors(&input, &extractors));
	}

	// エクストラクタがない場合は従来通り（ドキュメント追加のみ）
	let fn_name = &input.sig.ident;
	let fn_block = &input.block;
	let fn_inputs = &input.sig.inputs;
	let fn_output = &input.sig.output;
	let fn_vis = &input.vis;
	let fn_attrs = &input.attrs;
	let asyncness = &input.sig.asyncness;
	let generics = &input.sig.generics;
	let where_clause = &input.sig.generics.where_clause;

	let route_doc = if let Some((p, _)) = &path {
		format!("Route: {} {}", method, p)
	} else {
		format!("HTTP Method: {}", method)
	};

	Ok(quote! {
		#(#fn_attrs)*
		#[doc = #route_doc]
		#fn_vis #asyncness fn #fn_name #generics (#fn_inputs) #fn_output #where_clause {
			#fn_block
		}
	})
}
/// Implementation of GET route macro
pub fn get_impl(args: TokenStream, input: ItemFn) -> Result<TokenStream> {
	route_impl("GET", args, input)
}
/// Implementation of POST route macro
pub fn post_impl(args: TokenStream, input: ItemFn) -> Result<TokenStream> {
	route_impl("POST", args, input)
}
/// Implementation of PUT route macro
pub fn put_impl(args: TokenStream, input: ItemFn) -> Result<TokenStream> {
	route_impl("PUT", args, input)
}
/// Implementation of PATCH route macro
pub fn patch_impl(args: TokenStream, input: ItemFn) -> Result<TokenStream> {
	route_impl("PATCH", args, input)
}
/// Implementation of DELETE route macro
pub fn delete_impl(args: TokenStream, input: ItemFn) -> Result<TokenStream> {
	route_impl("DELETE", args, input)
}
