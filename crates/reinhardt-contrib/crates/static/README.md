# reinhardt-static

Static file serving and production utilities for Reinhardt

## Overview

Static file handling for serving CSS, JavaScript, images, and other static assets. Includes file collection, URL generation, storage backends, health checks, and metrics collection for production deployments.

## Features

### Core Functionality

#### ✓ Implemented

- **Static File Configuration** (`StaticFilesConfig`)
  - Configurable static root directory for collected files
  - Static URL path configuration with validation
  - Multiple source directories support via `STATICFILES_DIRS`
  - Media URL configuration and conflict detection

- **Storage Backends** (`Storage` trait)
  - `FileSystemStorage` - Local file system storage
  - `MemoryStorage` - In-memory storage for testing
  - Extensible storage backend system

- **Static File Finder** (`StaticFilesFinder`)
  - Locate files across multiple static directories
  - Support for collecting files from various sources
  - `find_all()` - Recursively discover all static files across configured directories
  - Efficient directory tree traversal with proper error handling

- **Hashed File Storage** (`HashedFileStorage`)
  - File hashing for cache busting
  - Configurable hashing algorithms (MD5, SHA-256)
  - Automatic hash calculation and filename generation
  - Integration with manifest system

- **Manifest System** (`ManifestStaticFilesStorage`)
  - JSON manifest for mapping original filenames to hashed versions
  - Versioned manifest format (currently V1)
  - Enables efficient static file lookup in production
  - Supports deployment workflows with pre-collected assets

- **Media Asset Management** (`Media`, `HasMedia`)
  - CSS and JavaScript dependency declaration for forms and widgets
  - Media type organization (e.g., "all", "screen", "print")
  - HTML rendering for `<link>` and `<script>` tags
  - Dependency merging with duplicate prevention
  - Trait-based system for components to declare their assets

- **Static File Handler** (`StaticFileHandler`)
  - HTTP request handling for static files
  - MIME type detection via `mime_guess`
  - Error handling with `StaticError` and `StaticResult` types
  - File serving with proper content types
  - Directory serving with automatic index file detection
  - Configurable index files (default: `["index.html"]`) via `with_index_files()`
  - Serves index.html when accessing directories directly

- **Configuration Validation** (`checks` module)
  - Django-style system checks for static files configuration
  - Multiple check levels: Debug, Info, Warning, Error, Critical
  - Comprehensive validation rules:
    - `static.E001` - STATIC_ROOT not set
    - `static.E002` - STATIC_ROOT in STATICFILES_DIRS
    - `static.E003` - STATIC_URL is empty
    - `static.E004` - STATICFILES_DIRS entry is not a directory
    - `static.W001` - STATIC_ROOT is subdirectory of STATICFILES_DIRS
    - `static.W002` - STATIC_URL doesn't start with '/'
    - `static.W003` - STATIC_URL doesn't end with '/'
    - `static.W004` - STATICFILES_DIRS is empty
    - `static.W005` - Directory doesn't exist
    - `static.W006` - Duplicate STATICFILES_DIRS entries
    - `static.W007` - MEDIA_URL doesn't start with '/'
    - `static.W008` - MEDIA_URL doesn't end with '/'
    - `static.W009` - MEDIA_URL prefix conflict with STATIC_URL
  - Helpful hints for fixing configuration issues

- **Health Check System** (`health` module)
  - Health status monitoring (Healthy, Degraded, Unhealthy)
  - Async health check trait with `async_trait`
  - Health check manager for centralized monitoring
  - Detailed health reports with metadata support
  - Marker traits for specialized checks:
    - `CacheHealthCheck` - Cache-related health checks
    - `DatabaseHealthCheck` - Database-related health checks
  - Component-level health status tracking
  - Production-ready monitoring integration

- **Metrics Collection** (`metrics` module)
  - Performance metrics tracking
  - Request timing and profiling (`RequestTimer`)
  - Request-specific metrics (`RequestMetrics`)
  - Centralized metrics collection (`MetricsCollector`)
  - Generic metric types for custom measurements

- **Middleware** (`StaticFilesMiddleware`)
  - Request/response processing for static files
  - Integration with HTTP pipeline
  - Automatic static file serving in development

- **Dependency Resolution** (`DependencyGraph`)
  - Track dependencies between static assets
  - Resolve asset loading order
  - Support for complex asset dependency chains

#### Implemented in Related Crates

- **collectstatic Command** (implemented in `reinhardt-commands`)
  - ✓ CLI command for collecting static files from all sources
  - ✓ Copy files to STATIC_ROOT with optional processing
  - ✓ Integration with deployment workflows
  - ✓ Progress reporting and verbose output
  - See [reinhardt-commands](../../commands/README.md) for details

#### Planned

<!-- TODO: The following features are not yet implemented -->

- **Advanced Storage Backends**
  - S3-compatible storage backend
  - Azure Blob Storage backend
  - Google Cloud Storage backend
  - Custom storage backend registration

- **File Processing Pipeline**
  - CSS/JavaScript minification
  - Image optimization
  - Source map generation
  - Asset bundling and concatenation

- **Advanced Caching**
  - ETags and conditional requests
  - Compression (gzip, brotli)
  - Cache-Control header management
  - CDN integration helpers

- **Template Integration**
  - `{% static %}` template tag equivalent
  - Automatic URL generation in templates
  - Static file versioning in rendered HTML

- **Development Server Features**
  - Auto-reload on file changes
  - Source file watching
  - Development-only error pages

## Architecture

### Storage System

The storage system is built around the `Storage` trait, allowing multiple backend implementations:

- **FileSystemStorage**: Default storage using local filesystem
- **MemoryStorage**: In-memory storage for testing
- **HashedFileStorage**: Wraps other storage backends to add content-based hashing
- **ManifestStaticFilesStorage**: Production storage with manifest for efficient lookups

### Health Checks

The health check system provides:
- Async health check execution
- Component-level status tracking
- Aggregated health reports
- Extensible check registration
- Integration with monitoring systems

### Metrics

The metrics system enables:
- Request-level timing
- Custom metric collection
- Performance profiling
- Production monitoring integration

## Usage Examples

### Basic Configuration

```rust
use reinhardt_static::StaticFilesConfig;
use std::path::PathBuf;

let config = StaticFilesConfig {
    static_root: PathBuf::from("/var/www/static"),
    static_url: "/static/".to_string(),
    staticfiles_dirs: vec![
        PathBuf::from("app/static"),
        PathBuf::from("vendor/static"),
    ],
    media_url: Some("/media/".to_string()),
};
```

### Configuration Validation

```rust
use reinhardt_static::checks::check_static_files_config;

let messages = check_static_files_config(&config);
for message in messages {
    println!("[{}] {}", message.id, message.message);
    if let Some(hint) = message.hint {
        println!("  Hint: {}", hint);
    }
}
```

### Finding All Static Files

```rust
use reinhardt_static::StaticFilesFinder;
use std::path::PathBuf;

let mut finder = StaticFilesFinder::new();
finder.add_directory(PathBuf::from("app/static"));
finder.add_directory(PathBuf::from("vendor/static"));

// Recursively find all static files
let all_files = finder.find_all();
for file in all_files {
    println!("Found: {}", file);
}
```

### Directory Serving with Index Files

```rust
use reinhardt_static::StaticFileHandler;
use std::path::PathBuf;

let handler = StaticFileHandler::new(PathBuf::from("/var/www/static"))
    .with_index_files(vec![
        "index.html".to_string(),
        "index.htm".to_string(),
        "default.html".to_string(),
    ]);

// Accessing /docs/ will serve /docs/index.html if it exists
```

### Media Assets for Forms

```rust
use reinhardt_static::media::{Media, HasMedia};

let mut media = Media::new();
media.add_css("all", "/static/css/forms.css");
media.add_js("/static/js/widgets.js");

// Render in templates
let css_html = media.render_css();
let js_html = media.render_js();
```

### Health Checks

```rust
use reinhardt_static::health::{HealthCheckManager, HealthCheck, HealthCheckResult};
use async_trait::async_trait;
use std::sync::Arc;

struct StaticFilesHealthCheck;

#[async_trait]
impl HealthCheck for StaticFilesHealthCheck {
    async fn check(&self) -> HealthCheckResult {
        // Check if static files are accessible
        HealthCheckResult::healthy("static_files")
            .with_metadata("static_root_exists", "true")
    }
}

let mut manager = HealthCheckManager::new();
manager.register("static", Arc::new(StaticFilesHealthCheck));

let report = manager.run_checks().await;
if report.is_healthy() {
    println!("All systems operational");
}
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
