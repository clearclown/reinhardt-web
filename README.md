<div align="center">
  <img src="branding/logo.png" alt="Reinhardt Logo" width="200"/>

  <h1>Reinhardt</h1>

  <h3>🦀 Polylithic Batteries Included</h3>

  <p><strong>A composable full-stack API framework for Rust</strong></p>
  <p>Build with <em>all</em> the power of Django's batteries-included philosophy,<br/>
  or compose <em>only</em> what you need—your choice, your way.</p>

[![Crates.io](https://img.shields.io/crates/v/reinhardt.svg)](https://crates.io/crates/reinhardt)
[![Documentation](https://docs.rs/reinhardt/badge.svg)](https://docs.rs/reinhardt)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

</div>

---

## 📍 Quick Navigation

You may be looking for:

- 🚀 [Quick Start](#quick-start) - Get up and running in 5 minutes
- 📦 [Installation Options](#installation) - Choose your flavor: Micro, Standard, or Full
- 📚 [Getting Started Guide](docs/GETTING_STARTED.md) - Step-by-step tutorial
- 🎛️ [Feature Flags](docs/FEATURE_FLAGS.md) - Fine-tune your build
- 📖 [API Documentation](https://docs.rs/reinhardt) - Complete API reference
- 💬 [Community & Support](#getting-help) - Get help from the community

## Why Reinhardt?

**Polylithic = Poly (many) + Lithic (building blocks)**
Unlike monolithic frameworks that force you to use everything, Reinhardt lets you compose your perfect stack from independent, well-tested components.

Reinhardt brings together the best of three worlds:

| Inspiration        | What We Borrowed                                       | What We Improved                                     |
|--------------------|--------------------------------------------------------|------------------------------------------------------|
| 🐍 **Django**      | Batteries-included philosophy, ORM design, admin panel | Feature flags for composable builds, Rust's type safety |
| 🎯 **Django REST** | Serializers, ViewSets, permissions                     | Compile-time validation, zero-cost abstractions      |
| ⚡ **FastAPI**      | DI system, automatic OpenAPI                           | Native Rust performance, no runtime overhead         |
| 🗄️ **SQLAlchemy** | QuerySet patterns, relationship handling               | Type-safe query builder, compile-time validation     |

**Result**: A framework that's familiar to Python developers, but with Rust's performance and safety guarantees.

## ✨ Features

### 🎯 Core Framework

- **Type-Safe ORM**: QuerySet API with compile-time query validation
- **Powerful Serializers**: Automatic validation and transformation
- **Flexible ViewSets**: DRY principle for CRUD operations
- **Smart Routing**: Automatic URL configuration from ViewSets
- **Multi-Auth Support**: JWT, Token, Session, and Basic authentication

### 🚀 FastAPI-Inspired Ergonomics

- **Parameter Extraction**: Type-safe `Path<T>`, `Query<T>`, `Header<T>`, `Cookie<T>`, `Json<T>`, `Form<T>` extractors
- **Dependency Injection**: FastAPI-style DI system with `Depends<T>`, request scoping, and caching
- **Auto OpenAPI**: Generate OpenAPI 3.0 schemas from Rust types with `#[derive(Schema)]`
- **Function-based Endpoints**: Ergonomic `#[endpoint]` macro for defining API routes (coming soon)
- **Background Tasks**: Simple async task execution

### 🔋 Batteries Included

- **Admin Panel**: Django-style auto-generated admin interface (coming soon)
- **Middleware System**: Request/response processing pipeline
- **Management Commands**: CLI tools for migrations, static files, and more
- **Pagination**: PageNumber, LimitOffset, and Cursor strategies
- **Filtering & Search**: Built-in SearchFilter and OrderingFilter for querysets
- **Rate Limiting**: Flexible throttling (AnonRateThrottle, UserRateThrottle, ScopedRateThrottle)
- **Signals**: Event-driven hooks (pre_save, post_save, pre_delete, post_delete, m2m_changed)

### 🌍 Advanced Features

- **GraphQL Support**: Build GraphQL APIs alongside REST (coming soon)
- **WebSocket Support**: Real-time bidirectional communication (coming soon)
- **Internationalization**: Multi-language support
- **Static Files**: CDN integration, hashed storage, and compression
- **Browsable API**: HTML interface for API exploration

## Installation

Compose your perfect framework—Reinhardt offers three ready-made flavors:

### Reinhardt Micro - For Microservices

Lightweight and fast, perfect for simple APIs and microservices:

```toml
[dependencies]
reinhardt-micro = "0.1.0"
```

### Reinhardt Standard - Balanced Approach

The default configuration, suitable for most projects:

```toml
[dependencies]
reinhardt = "0.1.0"
# Equivalent to: reinhardt = { version = "0.1.0", features = ["standard"] }
```

### Reinhardt Full - Everything Included

All features enabled, Django-style batteries-included:

```toml
[dependencies]
reinhardt = { version = "0.1.0", features = ["full"] }
```

### Compose Your Own Configuration

Mix and match features to build your ideal framework:

```toml
[dependencies]
# Minimal setup with just routing and params
reinhardt = { version = "0.1.0", default-features = false, features = ["minimal"] }

# Add database support
reinhardt = { version = "0.1.0", default-features = false, features = ["minimal", "database"] }

# Standard with extra features
reinhardt = { version = "0.1.0", features = ["standard", "websockets", "graphql"] }
```

## Quick Start

### 1. Install Reinhardt Admin Tool

```bash
cargo install reinhardt-admin
```

### 2. Create a New Project

```bash
# Create a RESTful API project
reinhardt-admin startproject my-api --template-type restful
cd my-api

# Or create a Model-Template-View (MTV) project
reinhardt-admin startproject my-web --template-type mtv
```

This generates a complete project structure:

```
my-api/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── config.rs
│   ├── apps.rs
│   ├── config/
│   │   ├── settings.rs
│   │   ├── settings/
│   │   │   ├── base.rs
│   │   │   ├── local.rs
│   │   │   ├── staging.rs
│   │   │   └── production.rs
│   │   ├── urls.rs
│   │   └── apps.rs
│   └── bin/
│       ├── runserver.rs
│       └── manage.rs
└── README.md
```

### 3. Run the Development Server

```bash
# Using the runserver binary (recommended)
cargo run --bin runserver

# Or using manage command
cargo run --bin manage runserver

# Server will start at http://127.0.0.1:8000
```

### 4. Create Your First App

```bash
# Create a new app
cargo run --bin manage startapp users --template-type restful
```

This creates an app structure:

```
users/
├── lib.rs
├── models.rs
├── models/
├── views.rs
├── views/
├── serializers.rs
├── serializers/
├── admin.rs
├── urls.rs
└── tests.rs
```

### 5. Register ViewSets

Edit your app's `urls.rs`:

```rust
use reinhardt_routers::UnifiedRouter;
use crate::views::UserViewSet;
use std::sync::Arc;

pub fn url_patterns() -> UnifiedRouter {
    UnifiedRouter::new()
        .viewset("/users", Arc::new(UserViewSet::new()))
}
```

Include in `src/config/urls.rs`:

```rust
use reinhardt::prelude::*;
use std::sync::Arc;

pub fn url_patterns() -> Arc<UnifiedRouter> {
    let router = UnifiedRouter::new()
        .mount("/api/", users::urls::url_patterns());

    Arc::new(router)
}
```

For a complete step-by-step guide, see [Getting Started](docs/GETTING_STARTED.md).

## 🎓 Learn by Example

### With Database

Configure database in `settings/base.toml`:

```toml
debug = true
secret_key = "your-secret-key-for-development"

[database]
engine = "postgresql"
host = "localhost"
port = 5432
name = "mydb"
user = "postgres"
password = "postgres"
```

Settings are automatically loaded in `src/config/settings.rs`:

```rust
use reinhardt_settings::prelude::*;
use reinhardt_core::Settings;

pub fn get_settings() -> Settings {
    let profile_str = env::var("REINHARDT_ENV").unwrap_or_else(|_| "local".to_string());
    let profile = Profile::from_str(&profile_str).unwrap_or(Profile::Development);

    let settings_dir = PathBuf::from("settings");

    SettingsBuilder::new()
        .profile(profile)
        .add_source(LowPriorityEnvSource::new().with_prefix("REINHARDT_"))
        .add_source(TomlFileSource::new(settings_dir.join("base.toml")))
        .add_source(TomlFileSource::new(settings_dir.join(format!("{}.toml", profile_str))))
        .build()
        .expect("Failed to build settings")
        .into_typed()
        .expect("Failed to convert settings")
}
```

**Priority Order**: `{profile}.toml` > `base.toml` > Environment Variables > Defaults

See [Settings Documentation](docs/SETTINGS_DOCUMENT.md) for more details.

Define models in your app (e.g., `users/models.rs`):

```rust
use reinhardt::prelude::*;

#[derive(Model, Serialize, Deserialize)]
#[reinhardt(table_name = "users")]
pub struct User {
    #[reinhardt(primary_key)]
    pub id: i64,
    pub email: String,
    pub name: String,
}
```

Register in `src/config/apps.rs`:

```rust
use reinhardt_macros::installed_apps;

installed_apps! {
    auth: "reinhardt.contrib.auth",
    contenttypes: "reinhardt.contrib.contenttypes",
    users: "users",
}

pub fn get_installed_apps() -> Vec<String> {
    InstalledApp::all_apps()
}
```

### With Authentication

Add to your app's `views/profile.rs`:

```rust
use reinhardt::prelude::*;
use crate::models::User;

pub async fn get_profile(req: Request) -> Result<Json<UserProfile>> {
    // Extract authenticated user from request
    let user: Authenticated<User> = req.extensions().get().cloned()
        .ok_or_else(|| Error::Unauthorized("Authentication required".to_string()))?;

    Ok(Json(user.to_profile()))
}
```

Re-export in your app's `views.rs`:

```rust
mod profile;
pub use profile::*;
```

Register URL in your app's `urls.rs`:

```rust
use reinhardt_routers::UnifiedRouter;
use hyper::Method;
use crate::views;

pub fn url_patterns() -> UnifiedRouter {
    UnifiedRouter::new()
        .function("/profile", Method::GET, views::get_profile)
}
```

### With Dependency Injection

In your app's `views/user.rs`:

```rust
use reinhardt::prelude::*;
use crate::models::User;

pub async fn get_user(req: Request) -> Result<Json<User>> {
    // Extract path parameter
    let id: i64 = req.path_param("id")
        .ok_or_else(|| Error::BadRequest("Missing id parameter".to_string()))?
        .parse()
        .map_err(|_| Error::BadRequest("Invalid id format".to_string()))?;

    // Get database from DI context
    let db = req.di_context()
        .ok_or_else(|| Error::InternalServerError("DI context not available".to_string()))?
        .resolve::<Database>()
        .await?;

    let user = User::find_by_id(id, &db).await?;
    Ok(Json(user))
}
```

Re-export in your app's `views.rs`:

```rust
mod user;
pub use user::*;
```

### With Serializers and Validation

In your app's `serializers/user.rs`:

```rust
use reinhardt::prelude::*;

#[derive(Serialize, Deserialize, Validate)]
pub struct CreateUserRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 3, max = 50))]
    pub name: String,
}

#[derive(Serializer)]
#[serializer(model = "User")]
pub struct UserSerializer {
    pub id: i64,
    pub email: String,
    pub name: String,
}
```

Re-export in your app's `serializers.rs`:

```rust
mod user;
pub use user::*;
```

In your app's `views/user.rs`:

```rust
use reinhardt::prelude::*;
use crate::models::User;
use crate::serializers::{CreateUserRequest, UserSerializer};

pub async fn create_user(req: Request) -> Result<Json<UserSerializer>> {
    // Parse and validate request body
    let create_req: CreateUserRequest = req.json().await?;
    create_req.validate()?;

    // Get database from DI context
    let db = req.di_context()
        .ok_or_else(|| Error::InternalServerError("DI context not available".to_string()))?
        .resolve::<Database>()
        .await?;

    let user = User::create(&create_req, &db).await?;
    Ok(Json(UserSerializer::from(user)))
}
```

## Choosing the Right Flavor

| Feature      | Micro    | Standard  | Full    |
|--------------|----------|-----------|---------|
| Binary Size  | ~5-10 MB | ~20-30 MB | ~50+ MB |
| Compile Time | Fast     | Medium    | Slower  |
| **Core Features**     |
| Routing               | ✅       | ✅        | ✅      |
| Parameter Extraction  | ✅       | ✅        | ✅      |
| Dependency Injection  | ✅       | ✅        | ✅      |
| **Standard Features** |
| ORM                   | Optional | ✅        | ✅      |
| Serializers           | ❌       | ✅        | ✅      |
| ViewSets              | ❌       | ✅        | ✅      |
| Authentication        | ❌       | ✅        | ✅      |
| Pagination            | ❌       | ✅        | ✅      |
| **Advanced Features** |
| Admin Panel           | ❌       | ❌        | ✅      |
| GraphQL               | ❌       | ❌        | ✅      |
| WebSockets            | ❌       | ❌        | ✅      |
| i18n                  | ❌       | ❌        | ✅      |
| **Use Case**          |
| Microservices         | ✅       | ⚠️        | ❌      |
| REST APIs             | ✅       | ✅        | ✅      |
| Full Applications     | ❌       | ✅        | ✅      |
| Complex Systems       | ❌       | ⚠️        | ✅      |

**Legend**: ✅ Recommended • ⚠️ Possible but not optimal • ❌ Not recommended

## Components

Reinhardt includes the following core components:

### Core Framework

- **ORM**: Database abstraction layer with QuerySet API
- **Serializers**: Type-safe data serialization and validation
- **ViewSets**: Composable views for API endpoints
- **Routers**: Automatic URL routing configuration
- **Authentication**: JWT auth, permissions system
- **Middleware**: Request/response processing pipeline
- **Management Commands**: Django-style CLI for project management (`reinhardt-commands`)

### REST API Features (reinhardt-rest)

- **Authentication**: JWT, Token, Session, and Basic authentication
- **Routing**: Automatic URL routing for ViewSets
- **Browsable API**: HTML interface for API exploration
- **Schema Generation**: OpenAPI/Swagger documentation
- **Pagination**: PageNumber, LimitOffset, and Cursor pagination
- **Filtering**: SearchFilter and OrderingFilter for querysets
- **Throttling**: Rate limiting (AnonRateThrottle, UserRateThrottle, ScopedRateThrottle)
- **Signals**: Event-driven hooks (pre_save, post_save, pre_delete, post_delete, m2m_changed)

### FastAPI Inspired Features

- **Parameter Extraction**: Type-safe `Path<T>`, `Query<T>`, `Header<T>`, `Cookie<T>`, `Json<T>`, `Form<T>` extractors
- **Dependency Injection**: FastAPI-style DI system with `Depends<T>`, request scoping, and caching
- **Auto Schema Generation**: Derive OpenAPI schemas from Rust types with `#[derive(Schema)]`
- **Function-based Endpoints**: Ergonomic `#[endpoint]` macro for defining API routes (coming soon)
- **Background Tasks**: Simple background task execution

## Documentation

- 📚 [Getting Started Guide](docs/GETTING_STARTED.md) - Step-by-step tutorial for beginners
- 🎛️ [Feature Flags Guide](docs/FEATURE_FLAGS.md) - Optimize your build with granular feature control
- 🐳 [Podman Setup Guide](docs/PODMAN_SETUP.md) - Use Podman as a Docker Desktop alternative
- 📖 [API Reference](https://docs.rs/reinhardt) (Coming soon)
- 📝 [Tutorials](docs/tutorials/) - Learn by building real applications

**For AI Assistants**: See [CLAUDE.md](CLAUDE.md) for project-specific coding standards, testing guidelines, and development conventions.

## 💬 Getting Help

Reinhardt is a community-driven project. Here's where you can get help:

- 💬 **Discord**: Join our Discord server for real-time chat (coming soon)
- 💭 **GitHub Discussions**: [Ask questions and share ideas](https://github.com/kent8192/reinhardt-rs/discussions)
- 🐛 **Issues**: [Report bugs](https://github.com/kent8192/reinhardt-rs/issues)
- 📖 **Documentation**: [Read the guides](docs/)

Before asking, please check:

- ✅ [Getting Started Guide](docs/GETTING_STARTED.md)
- ✅ [Examples](examples/)
- ✅ Existing GitHub Issues and Discussions

## 🤝 Contributing

We love contributions! Please read our [Contributing Guide](CONTRIBUTING.md) to get started.

**Quick links**:

- [Development Setup](CONTRIBUTING.md#development-setup)
- [Testing Guidelines](CONTRIBUTING.md#testing-guidelines)
- [Commit Guidelines](CONTRIBUTING.md#commit-guidelines)

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Third-Party Attribution

This project is inspired by:

- [Django](https://www.djangoproject.com/) (BSD 3-Clause License)
- [Django REST Framework](https://www.django-rest-framework.org/) (BSD 3-Clause License)
- [FastAPI](https://fastapi.tiangolo.com/) (MIT License)
- [SQLAlchemy](https://www.sqlalchemy.org/) (MIT License)

See [THIRD-PARTY-NOTICES](THIRD-PARTY-NOTICES) for full attribution.

**Note:** This project is not affiliated with or endorsed by the Django Software Foundation, Encode OSS Ltd., Sebastián Ramírez (FastAPI author), or Michael Bayer (SQLAlchemy author).