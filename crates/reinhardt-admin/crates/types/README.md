# reinhardt-panel-types

Shared type definitions for Reinhardt admin panel API.

## Overview

This crate provides common type definitions used by both the admin panel API (backend) and UI (frontend) components. It contains request/response types, error types, and model metadata types that are shared across the admin panel architecture.

## Purpose

- **Type Safety**: Ensure type consistency between frontend and backend
- **Minimal Dependencies**: Only depends on `serde`, `serde_json`, `thiserror`, and `chrono`
- **Reusability**: Can be used by any Reinhardt crate or external application

## Main Modules

### `requests`

Request types for admin panel API endpoints:

- `ListQueryParams`: Query parameters for list endpoints (pagination, search, filters)
- `MutationRequest`: Request body for create/update operations
- `BulkDeleteRequest`: Request body for bulk delete operations
- `ExportFormat`: Export format enumeration (JSON, CSV, TSV)

### `responses`

Response types for admin panel API endpoints:

- `DashboardResponse`: Dashboard page data
- `ListResponse`: Paginated list of model instances
- `DetailResponse`: Single model instance detail
- `MutationResponse`: Result of create/update/delete operations
- `BulkDeleteResponse`: Result of bulk delete operations
- `ImportResponse`: Result of import operations

### `models`

Model metadata types:

- `ModelInfo`: Model metadata for dashboard (name, list URL)

### `errors`

Error types and result type alias:

- `AdminError`: Admin panel error enumeration
- `AdminResult<T>`: Convenient result type alias

## Installation

Add `reinhardt` to your `Cargo.toml`:

```toml
[dependencies]
reinhardt = { version = "0.1.0-alpha.1", features = ["admin"] }

# Or use a preset:
# reinhardt = { version = "0.1.0-alpha.1", features = ["standard"] }  # Recommended
# reinhardt = { version = "0.1.0-alpha.1", features = ["full"] }      # All features
```

Then import admin panel types:

```rust
use reinhardt::admin::types::{ListQueryParams, MutationRequest};
use reinhardt::admin::types::{DashboardResponse, ListResponse};
```

**Note:** Admin features are included in the `standard` and `full` feature presets.

## Usage Example

```rust
use reinhardt::admin::types::{
    ListQueryParams, ListResponse, AdminError, AdminResult,
};

fn list_models(params: ListQueryParams) -> AdminResult<ListResponse> {
    // ... implementation
    Ok(ListResponse {
        model_name: "User".to_string(),
        count: 100,
        page: params.page.unwrap_or(1),
        page_size: params.page_size.unwrap_or(25),
        total_pages: 4,
        results: vec![],
    })
}
```

## Architecture

This crate is part of the Reinhardt admin panel 3-crate architecture:

1. **reinhardt-panel-types** (this crate): Shared type definitions
2. **reinhardt-panel-api**: Backend JSON API implementation
3. **reinhardt-panel-ui**: Leptos CSR frontend implementation

The types crate is used by both api and ui crates to ensure type consistency.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
