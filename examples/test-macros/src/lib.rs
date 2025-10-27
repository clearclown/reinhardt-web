//! カスタムテストマクロ
//!
//! Cargo バージョン指定子を使用したバージョン検証付きテストマクロを提供します。

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr};

/// Cargo バージョン指定子を使用したテストマクロ
///
/// # 使用例
///
/// ```rust
/// #[example_test(version = "0.1.0")]
/// fn test_exact_version() {
///     // reinhardt 0.1.0 でのみ実行
/// }
///
/// #[example_test(version = "^0.1")]
/// fn test_caret_requirement() {
///     // reinhardt ^0.1 (0.1.x) でのみ実行
/// }
///
/// #[example_test(version = ">=0.1.0, <0.2.0")]
/// fn test_version_range() {
///     // reinhardt 0.1.x でのみ実行
/// }
///
/// #[example_test(version = "*")]
/// fn test_latest() {
///     // 最新版で実行
/// }
/// ```
///
/// # サポートされるバージョン指定子
///
/// - `"0.1.0"` - 正確なバージョン
/// - `"^0.1"` - キャレット要件 (0.1.x)
/// - `"~0.1.2"` - チルダ要件 (0.1.2 <= version < 0.2.0)
/// - `">=0.1, <0.2"` - 範囲指定
/// - `"*"` - ワイルドカード (最新)
#[proc_macro_attribute]
pub fn example_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    // バージョン指定子を抽出
    let version_spec = parse_macro_input!(attr as LitStr).value();
    let test_fn = parse_macro_input!(item as ItemFn);

    let fn_name = &test_fn.sig.ident;
    let fn_block = &test_fn.block;
    let fn_attrs = &test_fn.attrs;
    let fn_async = &test_fn.sig.asyncness;

    // async/sync に応じてコード生成を変える
    let expanded = if fn_async.is_some() {
        quote! {
            #(#fn_attrs)*
            #[tokio::test]
            async fn #fn_name() {
                // バージョンチェック
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
                    return; // テストをスキップ
                }

                // crates.io 可用性チェック
                if !example_common::availability::is_reinhardt_available() {
                    eprintln!(
                        "⏭️  Skipping test '{}': reinhardt not available from crates.io",
                        stringify!(#fn_name)
                    );
                    return; // テストをスキップ
                }

                // 実際のテスト実行
                #fn_block
            }
        }
    } else {
        quote! {
            #(#fn_attrs)*
            #[test]
            fn #fn_name() {
                // バージョンチェック
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
                    return; // テストをスキップ
                }

                // crates.io 可用性チェック
                if !example_common::availability::is_reinhardt_available() {
                    eprintln!(
                        "⏭️  Skipping test '{}': reinhardt not available from crates.io",
                        stringify!(#fn_name)
                    );
                    return; // テストをスキップ
                }

                // 実際のテスト実行
                #fn_block
            }
        }
    };

    TokenStream::from(expanded)
}
