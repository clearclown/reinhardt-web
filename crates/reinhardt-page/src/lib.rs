//! Reinhardt Page - WASM-based Frontend Framework
//!
//! A Django-inspired frontend framework for Reinhardt that preserves the benefits of
//! Django templates while leveraging WebAssembly for modern interactivity.
//!
//! ## Features
//!
//! - **Fine-grained Reactivity**: Leptos/Solid.js-style Signal system with automatic dependency tracking
//! - **Hybrid Rendering**: SSR + Client-side Hydration for optimal performance and SEO
//! - **Django-like API**: Familiar patterns for Reinhardt developers
//! - **Low-level Only**: Built on wasm-bindgen, web-sys, and js-sys (no high-level framework dependencies)
//! - **Security First**: Built-in CSRF protection, XSS prevention, and session management
//!
//! ## Architecture
//!
//! This framework consists of several key modules:
//!
//! - `reactive`: Fine-grained reactivity system (Signal, Effect, Memo)
//! - `dom`: DOM abstraction layer
//! - `component`: Component system
//! - `form`: Django Form integration
//! - `csrf`: CSRF protection
//! - `auth`: Authentication integration
//! - `api`: API client with Django QuerySet-like interface
//! - `server_fn`: Server Functions (RPC)
//! - `ssr`: Server-side rendering and hydration
//! - `router`: Client-side routing
//!
//! ## Current Implementation Status
//!
//! ### Phase 1: Core Reactive System ✓ (COMPLETED)
//! - Fine-grained Signal<T> with automatic dependency tracking
//! - Effect for reactive side effects
//! - Memo<T> for cached computations
//! - Global reactive runtime with micro-task batching
//! - 33 tests passing (24 unit + 9 integration)
//!
//! ### Phase 2: DOM Abstraction & Builder API ✓ (COMPLETED Week 2)
//! - Element wrapper with RAII event listeners
//! - Document wrapper for DOM creation
//! - Reactive attribute binding
//! - Fluent Builder API (ElementBuilder)
//! - Type-safe attribute operations (BooleanAttributes, AriaAttributes)
//! - Event shortcuts (.on_click(), .on_input(), etc.)
//! - 19 tests passing (Week 1 DOM integration tests)
//!
//! ### Phase 5.5: Server Functions (RPC) ✓ (COMPLETED Week 2-4)
//! - #[server_fn] macro infrastructure and conditional compilation
//! - ServerFnOptions parsing (use_inject, endpoint, codec)
//! - ServerFn trait and ServerFnError types
//! - Client stub and server handler generation
//! - DI support (use_inject = true)
//! - Codec abstraction (JSON, URL encoding, MessagePack)
//! - 47 tests passing (macro tests + codec integration tests)
//!
//! ### Phase 3: Django Form Integration ✓ (Week 5 Day 1-3 COMPLETED)
//! - FormMetadata and FieldMetadata structs (serializable)
//! - FormExt trait for Form::to_metadata() conversion
//! - FormComponent with web-sys-based rendering
//! - Client-side validation (required field checking)
//! - AJAX form submission with automatic CSRF token injection
//! - Reactive field values (Signal-based)
//! - 11 tests passing (7 wasm_compat + 4 FormComponent)
//!
//! ### Planned Features
//!
//! - Week 5 Day 4: FormBinding (two-way Signal binding)
//! - Phase 4: CSRF & Auth Integration
//! - Phase 5: API Client with QuerySet
//! - Phase 6: SSR Infrastructure (Complete Tera replacement)
//! - Phase 7: Hydration
//! - Phase 8: Component System & Macros
//! - Phase 9: Router
//! - Phase 10: Testing & Documentation
//!
//! ## Example (Future API)
//!
//! ```ignore
//! use reinhardt_page::prelude::*;
//!
//! #[component]
//! fn Counter() -> impl IntoView {
//!     let count = Signal::new(0);
//!
//!     view! {
//!         <div>
//!             <p>"Count: " {count}</p>
//!             <button on:click=move |_| count.update(|n| *n += 1)>
//!                 "Increment"
//!             </button>
//!         </div>
//!     }
//! }
//! ```

#![warn(missing_docs)]
#![cfg_attr(target_arch = "wasm32", no_std)]

// Phase 1: Core Reactive System
pub mod reactive;

// Phase 2: DOM Abstraction & Builder API
pub mod builder;
pub mod dom;

// Phase 5.5: Server Functions (RPC)
pub mod server_fn;

// Phase 3: Form Integration (Week 5-6)
pub mod form;

// Re-export commonly used types
pub use builder::{
	attributes::{AriaAttributes, BooleanAttributes},
	html::{
		a, button, div, form, h1, h2, h3, img, input, li, ol, option, p, select, span, textarea, ul,
	},
};
pub use dom::{Document, Element, EventHandle, EventType, document};
pub use form::{FormBinding, FormComponent};
pub use reactive::{Effect, Memo, Signal};
pub use server_fn::{ServerFn, ServerFnError};
