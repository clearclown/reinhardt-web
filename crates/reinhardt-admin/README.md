# reinhardt-admin

Django-style admin panel functionality for Reinhardt framework.

## Overview

This crate provides two main components:
- **Panel**: Web-based admin interface for managing database models
- **CLI**: Command-line tool for project management (available as `reinhardt-admin-cli`)

## Features

### Admin Panel (`reinhardt-panel`)

- ✅ **Model Management Interface**: Web-based CRUD operations for database models
- ✅ **Automatic Admin Discovery**: Auto-generate admin interfaces from model definitions
- ✅ **Customizable Admin Actions**: Bulk operations and custom actions
- ✅ **Search and Filtering**: Advanced search capabilities with multiple filter types
- ✅ **Permissions Integration**: Role-based access control for admin operations
- ✅ **Change Logging**: Audit trail for all admin actions
- ✅ **Inline Editing**: Edit related models inline
- ✅ **Responsive Design**: Mobile-friendly admin interface with customizable templates

### Command-Line Interface (`reinhardt-admin-cli`)

For project management commands (`startproject`, `startapp`), please use [`reinhardt-admin-cli`](../reinhardt-admin-cli).

## Quick Start

### Using the Admin Panel

```rust
use reinhardt_panel::{AdminSite, ModelAdmin};

#[tokio::main]
async fn main() {
    let mut admin = AdminSite::new("My Admin");

    // Register your models
    admin.register::<User>(UserAdmin::default()).await;

    // Start admin server
    admin.serve("127.0.0.1:8001").await.unwrap();
}
```

### Customizing the Admin

```rust
use reinhardt_panel::ModelAdmin;

struct UserAdmin {
    list_display: Vec<String>,
    list_filter: Vec<String>,
    search_fields: Vec<String>,
}

impl Default for UserAdmin {
    fn default() -> Self {
        Self {
            list_display: vec!["username".to_string(), "email".to_string(), "is_active".to_string()],
            list_filter: vec!["is_active".to_string()],
            search_fields: vec!["username".to_string(), "email".to_string()],
        }
    }
}
```

## Feature Flags

- `panel` (default): Web admin panel
- `cli`: Command-line interface
- `all`: All admin functionality

## Documentation

- [API Documentation](https://docs.rs/reinhardt-panel) (coming soon)
- [Panel Module Documentation](crates/panel/src/lib.rs)

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
