# reinhardt-apps

Application configuration and registry for Reinhardt framework.

## Overview

`reinhardt-apps` provides the application configuration system inspired by Django's `INSTALLED_APPS`. It enables:

- Application discovery and registration
- App-specific configuration
- Integration with migrations, admin panel, and other framework features

## Installation

Add `reinhardt` to your `Cargo.toml`:

```toml
[dependencies]
reinhardt = { version = "0.1.0-alpha.1", features = ["apps"] }

# Or use a preset:
# reinhardt = { version = "0.1.0-alpha.1", features = ["standard"] }  # Recommended
# reinhardt = { version = "0.1.0-alpha.1", features = ["full"] }      # All features
```

Then import app features:

```rust
use reinhardt::apps::{AppConfig, installed_apps};
```

**Note:** App features are included in the `standard` and `full` feature presets.

## Usage

Define installed apps using the `installed_apps!` macro:

```rust
use reinhardt::apps::installed_apps;

installed_apps! {
	auth: "reinhardt.contrib.auth",
	contenttypes: "reinhardt.contrib.contenttypes",
	sessions: "reinhardt.contrib.sessions",
	myapp: "myapp",
}
```

This generates:
- `InstalledApp` enum with variants for each app
- Conversion traits (`From`, `Into`, `Display`)
- App registry for framework integration

## What the Macro Generates

The `installed_apps!` macro automatically generates:

```rust
// Generated enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InstalledApp {
	Auth,
	Contenttypes,
	Sessions,
	Myapp,
}

// Display implementation
impl std::fmt::Display for InstalledApp {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Auth => write!(f, "reinhardt.contrib.auth"),
			Self::Contenttypes => write!(f, "reinhardt.contrib.contenttypes"),
			Self::Sessions => write!(f, "reinhardt.contrib.sessions"),
			Self::Myapp => write!(f, "myapp"),
		}
	}
}

// Helper methods
impl InstalledApp {
	pub fn all_apps() -> Vec<String> {
		vec![
			"reinhardt.contrib.auth".to_string(),
			"reinhardt.contrib.contenttypes".to_string(),
			"reinhardt.contrib.sessions".to_string(),
			"myapp".to_string(),
		]
	}
}
```

## Features

### Type-safe App References

Use the generated enum for type-safe app references:

```rust
// Type-safe reference
let app = InstalledApp::Myapp;
println!("App path: {}", app);  // "myapp"

// List all apps
let all = InstalledApp::all_apps();
```

### Automatic Discovery

The app registry enables automatic discovery for:

- **Migrations**: Discover migration files for each app
- **Admin Panel**: Auto-register models from each app
- **Static Files**: Collect static files from app directories
- **Templates**: Load templates from app template directories

### Framework Integration

The `installed_apps!` macro integrates with:

```rust
// src/config/apps.rs
use reinhardt::apps::installed_apps;

installed_apps! {
	// Framework apps
	auth: "reinhardt.contrib.auth",
	contenttypes: "reinhardt.contrib.contenttypes",
	sessions: "reinhardt.contrib.sessions",
	messages: "reinhardt.contrib.messages",
	static_files: "reinhardt.contrib.static",

	// Your apps
	users: "users",
	posts: "posts",
}

pub fn get_installed_apps() -> Vec<String> {
	InstalledApp::all_apps()
}
```

## App Naming Conventions

### Framework Apps

Framework-provided apps use the `reinhardt.contrib.*` namespace:

```rust
auth: "reinhardt.contrib.auth",
contenttypes: "reinhardt.contrib.contenttypes",
sessions: "reinhardt.contrib.sessions",
messages: "reinhardt.contrib.messages",
static_files: "reinhardt.contrib.static",
admin: "reinhardt.contrib.admin",
```

### User Apps

User-defined apps use simple names matching their directory:

```rust
users: "users",
blog: "blog",
api: "api",
```

## Compile-time Validation

The macro performs compile-time validation:

- **Path Format**: Validates `reinhardt.contrib.*` format for framework apps
- **Module Existence**: Checks that `reinhardt.contrib.*` modules exist
- **Unique Names**: Ensures app names are unique

```rust
// Compile error: Invalid path format
installed_apps! {
	bad: "invalid.path.format",  // ❌ Error
}

// Compile error: Non-existent framework module
installed_apps! {
	nonexistent: "reinhardt.contrib.nonexistent",  // ❌ Error
}
```

## Example Project Structure

```
my-project/
├── src/
│   ├── config/
│   │   ├── apps.rs           # installed_apps! definition
│   │   ├── settings.rs
│   │   └── urls.rs
│   └── apps/
│       ├── users/
│       │   ├── lib.rs
│       │   ├── models.rs
│       │   └── views.rs
│       └── posts/
│           ├── lib.rs
│           ├── models.rs
│           └── views.rs
└── Cargo.toml
```

```rust
// src/config/apps.rs
use reinhardt::apps::installed_apps;

installed_apps! {
	auth: "reinhardt.contrib.auth",
	contenttypes: "reinhardt.contrib.contenttypes",
	users: "users",
	posts: "posts",
}

pub fn get_installed_apps() -> Vec<String> {
	InstalledApp::all_apps()
}
```

## Integration with Other Components

### Migrations

```rust
// Migrations automatically discover apps
use reinhardt::db::migrations::MigrationRunner;

let runner = MigrationRunner::new(db);
let installed = InstalledApp::all_apps();
runner.migrate(&installed).await?;
```

### Admin Panel

```rust
// Admin panel auto-discovers models from apps
use reinhardt::admin::AdminSite;

let admin = AdminSite::new();
admin.autodiscover(&InstalledApp::all_apps()).await?;
```

### Settings Integration

```rust
// src/config/settings.rs
use reinhardt::conf::Settings;

pub fn get_settings() -> Settings {
	Settings::builder()
		.installed_apps(crate::config::apps::get_installed_apps())
		.build()
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
