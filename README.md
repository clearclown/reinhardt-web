# Reinhardt

A full-stack API framework for Rust, inspired by Django and Django REST Framework.

## Overview

Reinhardt combines the best practices from Django's robust web framework, Django REST Framework's powerful API capabilities, and FastAPI's modern developer experience, reimagined for the Rust ecosystem. It provides a complete, batteries-included solution for building production-ready REST APIs with the performance and safety of Rust.

## Features

- **Full-stack API development**: Everything you need to build RESTful APIs in one framework
- **Django-inspired architecture**: Familiar patterns for developers coming from Python/Django
- **FastAPI-inspired ergonomics**: Modern developer experience with type-safe parameter extraction and dependency injection
- **Type-safe**: Leverages Rust's type system for compile-time guarantees
- **Batteries included**: Authentication, serialization, ORM, routing, and more out of the box
- **High performance**: Built on Rust's zero-cost abstractions
- **Automatic OpenAPI**: Generate OpenAPI 3.0 schemas from your Rust types

## Installation

Reinhardt offers three flavors to match your project's scale:

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

### Custom Configuration

Mix and match features as needed:

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

```rust
// Example coming soon
```

## Choosing the Right Flavor

| Feature               | Micro    | Standard  | Full    |
| --------------------- | -------- | --------- | ------- |
| Binary Size           | ~5-10 MB | ~20-30 MB | ~50+ MB |
| Compile Time          | Fast     | Medium    | Slower  |
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

### REST API Features (reinhardt-rest)

- **Authentication**: JWT, Token, Session, and Basic authentication
- **Routing**: Automatic URL routing for ViewSets
- **Browsable API**: HTML interface for API exploration
- **Schema Generation**: OpenAPI/Swagger documentation
- **Pagination**: PageNumber, LimitOffset, and Cursor pagination
- **Filtering**: SearchFilter and OrderingFilter for querysets
- **Throttling**: Rate limiting (AnonRateThrottle, UserRateThrottle, ScopedRateThrottle)
- **Signals**: Event-driven hooks (pre_save, post_save, pre_delete, post_delete, m2m_changed)

### FastAPI Inspired Features (NEW!)

- **Parameter Extraction**: Type-safe `Path<T>`, `Query<T>`, `Header<T>`, `Cookie<T>`, `Json<T>`, `Form<T>` extractors
- **Dependency Injection**: FastAPI-style DI system with `Depends<T>`, request scoping, and caching
- **Auto Schema Generation**: Derive OpenAPI schemas from Rust types with `#[derive(Schema)]`
- **Function-based Endpoints**: Ergonomic `#[endpoint]` macro for defining API routes (coming soon)
- **Background Tasks**: Simple background task execution

## Documentation

- 📚 [Getting Started Guide](docs/GETTING_STARTED.md) - Step-by-step tutorial for beginners
- 🎛️ [Feature Flags Guide](docs/FEATURE_FLAGS.md) - Optimize your build with granular feature control
- 📖 [API Reference](https://docs.rs/reinhardt) (Coming soon)

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

See [THIRD-PARTY-NOTICES](THIRD-PARTY-NOTICES) for full attribution.

**Note:** This project is not affiliated with or endorsed by the Django Software Foundation, Encode OSS Ltd., or Sebastián Ramírez (FastAPI author).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
