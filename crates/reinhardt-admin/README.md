# reinhardt-admin

Admin panel functionality for Reinhardt framework.

## Features

This crate provides the Django-style web admin panel for managing models.

For the command-line tool, see [`reinhardt-admin-cli`](../reinhardt-admin-cli).

## Usage

### Using the admin panel

Add to your `Cargo.toml`:

```toml
[dependencies]
reinhardt-admin = "0.1.0-alpha.1"
```

## Feature Flags

- `panel` (default): Web admin panel
- `all`: All admin panel functionality

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
