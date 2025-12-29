# reinhardt-pages

WASM-based reactive frontend framework for Reinhardt with Django-like API.

## Features

- **Fine-grained Reactivity**: Leptos/Solid.js-style Signal system with automatic dependency tracking
- **Hybrid Rendering**: SSR + Client-side Hydration for optimal performance and SEO
- **Django-like API**: Familiar patterns for Reinhardt developers
- **Low-level Only**: Built on wasm-bindgen, web-sys, and js-sys (no high-level framework dependencies)
- **Security First**: Built-in CSRF protection, XSS prevention, and session management
- **Simplified Conditional Compilation**: `cfg_aliases` integration and automatic event handler handling

## Quick Start

### Using the Prelude (Recommended)

The prelude provides all commonly used types with a single import:

```rust
// Instead of multiple scattered imports:
// use reinhardt_pages::{Signal, View, use_state, ...};
// use reinhardt_pages::component::{ElementView, IntoView};
// use reinhardt_pages::reactive::{Effect, Memo};

// Use the unified prelude:
use reinhardt_pages::prelude::*;
// or via reinhardt crate:
use reinhardt::pages::prelude::*;
```

### Platform-Agnostic Event Type

The `platform` module provides unified types that work across both WASM and native:

```rust
use reinhardt_pages::platform::Event;

// Works on both WASM and native targets
fn handle_click(_event: Event) {
    // Event handling logic
}
```

### Simplified cfg Attributes with cfg_aliases

Configure `cfg_aliases` in your project's `build.rs`:

```rust
// build.rs
use cfg_aliases::cfg_aliases;

fn main() {
    // Rust 2024 edition requires explicit check-cfg declarations
    println!("cargo::rustc-check-cfg=cfg(wasm)");
    println!("cargo::rustc-check-cfg=cfg(native)");

    cfg_aliases! {
        wasm: { target_arch = "wasm32" },
        native: { not(target_arch = "wasm32") },
    }
}
```

Add to `Cargo.toml`:

```toml
[build-dependencies]
cfg_aliases = "0.2"
```

Now you can use shorter cfg attributes:

```rust
// Before:
#[cfg(target_arch = "wasm32")]
// After:
#[cfg(wasm)]

// Before:
#[cfg(not(target_arch = "wasm32"))]
// After:
#[cfg(native)]
```

### Automatic Event Handler Handling

The `page!` macro automatically handles event handlers for server-side rendering. You no longer need to write duplicate conditional blocks:

```rust
use reinhardt_pages::prelude::*;

// This works on both WASM and native targets!
// On WASM: Event handlers are bound to DOM events
// On native: Event handlers are automatically ignored
fn my_button(on_click: Signal<bool>) -> View {
    page!(|| {
        button {
            @click: move |_| { on_click.set(true); },
            "Click me"
        }
    })
}
```

**Before** (manual conditional compilation):
```rust
#[cfg(target_arch = "wasm32")]
{
    page!(|| {
        button {
            @click: move |_| { on_click.set(true); },
            "Click me"
        }
    })
}
#[cfg(not(target_arch = "wasm32"))]
{
    let _ = on_click; // suppress warning
    page!(|| {
        button { "Click me" }
    })
}
```

**After** (automatic handling):
```rust
// Just write once - the macro handles everything!
page!(|| {
    button {
        @click: move |_| { on_click.set(true); },
        "Click me"
    }
})
```

## Architecture

This framework consists of several key modules:

- **`reactive`**: Fine-grained reactivity system (Signal, Effect, Memo)
- **`dom`**: DOM abstraction layer
- **`builder`**: HTML element builder API
- **`component`**: Component system with IntoView trait
- **`form`**: Django Form integration (native only)
- **`csrf`**: CSRF protection
- **`auth`**: Authentication integration
- **`api`**: API client with Django QuerySet-like interface
- **`server_fn`**: Server Functions (RPC)
- **`ssr`**: Server-side rendering
- **`hydration`**: Client-side hydration
- **`router`**: Client-side routing (reinhardt-urls compatible)
- **`platform`**: Platform abstraction types
- **`prelude`**: Unified imports

## Prelude Contents

The prelude includes:

### Reactive System
- `Signal`, `Effect`, `Memo`, `Resource`, `ResourceState`
- Context: `Context`, `ContextGuard`, `create_context`, `get_context`, `provide_context`, `remove_context`

### Hooks
- `use_state`, `use_effect`, `use_memo`, `use_callback`, `use_context`
- `use_ref`, `use_reducer`, `use_transition`, `use_deferred_value`
- `use_id`, `use_layout_effect`, `use_effect_event`, `use_debug_value`
- `use_optimistic`, `use_action_state`, `use_shared_state`, `use_sync_external_store`

### Component System
- `Component`, `ElementView`, `IntoView`, `View`, `Props`, `ViewEventHandler`

### Events and Callbacks
- `Callback`, `IntoEventHandler`, `into_event_handler`
- `Event` (platform-agnostic via `platform` module)

### DOM
- `Document`, `Element`, `EventHandle`, `EventType`, `document`

### Routing
- `Link`, `Router`, `Route`, `RouterOutlet`, `PathPattern`

### API and Server Functions
- `ApiModel`, `ApiQuerySet`, `Filter`, `FilterOp`
- `ServerFn`, `ServerFnError`

### Authentication and Security
- `AuthData`, `AuthError`, `AuthState`, `auth_state`
- `CsrfManager`, `get_csrf_token`

### SSR and Hydration
- `HydrationContext`, `HydrationError`, `hydrate`
- `SsrOptions`, `SsrRenderer`, `SsrState`

### Forms (native only)
- `FormBinding`, `FormComponent`
- `Widget`, `FieldMetadata`, `FormMetadata`

### Macros
- `page!`

### WASM-specific
- `spawn_local` (re-exported from wasm_bindgen_futures)
- `create_resource`, `create_resource_with_deps`

## Example

```rust
use reinhardt_pages::prelude::*;

fn counter() -> View {
    let (count, set_count) = use_state(|| 0);

    page!(|| {
        div {
            p { format!("Count: {}", count.get()) }
            button {
                @click: move |_| set_count.update(|n| *n + 1),
                "Increment"
            }
        }
    })
}
```

## Feature Flags

| Feature | Description |
|---------|-------------|
| `msgpack` | MessagePack serialization support |
| `pages-full` | All features enabled |
| `static` | Static file serving |
| `urls` | URL routing integration |

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
