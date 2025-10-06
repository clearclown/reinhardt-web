# Reinhardt

A full-stack API framework for Rust, inspired by Django and Django REST Framework.

## Overview

Reinhardt combines the best practices from Django's robust web framework and Django REST Framework's powerful API capabilities, reimagined for the Rust ecosystem. It provides a complete, batteries-included solution for building production-ready REST APIs.

## Features

- **Full-stack API development**: Everything you need to build RESTful APIs in one framework
- **Django-inspired architecture**: Familiar patterns for developers coming from Python/Django
- **Type-safe**: Leverages Rust's type system for compile-time guarantees
- **Batteries included**: Authentication, serialization, ORM, routing, and more out of the box
- **High performance**: Built on Rust's zero-cost abstractions

## Installation

Add Reinhardt to your `Cargo.toml`:

```toml
[dependencies]
reinhardt = "0.0.1"
```

## Quick Start

```rust
// Example coming soon
```

## Components

Reinhardt includes the following core components:

- **ORM**: Database abstraction layer with migrations
- **Serializers**: Type-safe data serialization and validation
- **ViewSets**: Class-based views for API endpoints
- **Routers**: Automatic URL routing configuration
- **Authentication**: Built-in auth backends and permissions
- **Middleware**: Request/response processing pipeline

## Documentation

Full documentation is coming soon.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Third-Party Attribution

This project is inspired by [Django](https://www.djangoproject.com/) and [Django REST Framework](https://www.django-rest-framework.org/), both licensed under the BSD 3-Clause License. See [THIRD-PARTY-NOTICES](THIRD-PARTY-NOTICES) for full attribution.

**Note:** This project is not affiliated with or endorsed by the Django Software Foundation or Encode OSS Ltd.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
