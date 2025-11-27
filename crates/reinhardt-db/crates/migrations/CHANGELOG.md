# Changelog

All notable changes to the `reinhardt-migrations` crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Migration registry hybrid architecture with `GlobalRegistry` and `LocalRegistry`
- `MigrationRegistry` trait providing unified interface for both registry types
- `LocalRegistry` for test isolation without linkme's global state
- `migration_registry` fixture in `reinhardt-test` for convenient testing
- Comprehensive test coverage for both global and local registries
- Migration registry testing guidelines in TESTING_STANDARDS.md

### Changed
- Refactored `registry.rs` into modular structure: `traits`, `global`, `local`
- Updated `collect_migrations!` macro to use new `registry::global::MIGRATION_PROVIDERS` path
- Deprecated old registry functions (`all_migrations()`, `migrations_for_app()`, etc.)
  - Use `global_registry().all_migrations()` instead

### Fixed
- Resolved "duplicate distributed_slice" errors in test environments
  - Introduced `LocalRegistry` for test isolation without linkme's global state
  - Added `#[cfg(test)]` guard in `GlobalRegistry::collect_compile_time_migrations()` to prevent accessing `MIGRATION_PROVIDERS` during tests
- Fixed SQL split test assertion to correctly validate FOREIGN KEY constraints
  - FOREIGN KEY constraints necessarily include `REFERENCES "table_name"`

### Removed
- Removed `preloaded_migration_registry` test fixture (not needed - examples should use `collect_migrations!`)

## Migration Guide

### For Test Code

**Before:**
```rust
use reinhardt_migrations::registry::all_migrations;

#[test]
fn test_migrations() {
    let migrations = all_migrations();  // ❌ Causes duplicate errors
    assert!(!migrations.is_empty());
}
```

**After:**
```rust
use reinhardt_migrations::registry::{LocalRegistry, MigrationRegistry};

#[test]
fn test_migrations() {
    let registry = LocalRegistry::new();  // ✅ Isolated
    registry.register(migration).unwrap();
    assert_eq!(registry.all_migrations().len(), 1);
}
```

**Or use fixtures:**
```rust
use reinhardt_test::fixtures::*;
use rstest::*;

#[rstest]
fn test_migrations(migration_registry: LocalRegistry) {
    migration_registry.register(migration).unwrap();
    assert_eq!(migration_registry.all_migrations().len(), 1);
}
```

### For Production Code

**Before:**
```rust
use reinhardt_migrations::registry::all_migrations;

let migrations = all_migrations();
```

**After:**
```rust
use reinhardt_migrations::registry::{global_registry, MigrationRegistry};

let registry = global_registry();
let migrations = registry.all_migrations();
```

**Note:** Old functions still work but are deprecated and will be removed in the next major version.

## [0.1.0-alpha.1] - YYYY-MM-DD

Initial alpha release.

### Added
- Migration framework with operation definitions
- SQL statement splitting and execution
- Migration dependency resolution
- Global migration registry using `linkme::distributed_slice`
- `collect_migrations!` macro for automatic registration

[Unreleased]: https://github.com/kent8192/reinhardt-rs/compare/reinhardt-migrations@v0.1.0-alpha.1...HEAD
[0.1.0-alpha.1]: https://github.com/kent8192/reinhardt-rs/releases/tag/reinhardt-migrations@v0.1.0-alpha.1
