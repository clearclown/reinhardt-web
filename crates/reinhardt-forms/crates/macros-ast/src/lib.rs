//! AST definitions for the `form!` macro.
//!
//! This crate provides the Abstract Syntax Tree (AST) structures used by the
//! `form!` procedural macro. It defines both untyped (raw parse output) and
//! typed (validated) AST nodes.
//!
//! ## Architecture
//!
//! The macro processing pipeline:
//! 1. **Parse**: TokenStream → Untyped AST ([`FormMacro`])
//! 2. **Validate**: Untyped AST → Typed AST ([`TypedFormMacro`])
//! 3. **Codegen**: Typed AST → Rust code
//!
//! ## Example
//!
//! ```ignore
//! form! {
//!     fields: {
//!         username: CharField {
//!             required,
//!             max_length: 100,
//!         },
//!     },
//! }
//! ```

mod node;
mod parser;
mod typed_node;

pub use node::*;
pub use typed_node::*;
