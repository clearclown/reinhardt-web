# reinhardt-admin-cli

Global command-line tool for Reinhardt project management.

## Overview

`reinhardt-admin-cli` is the Django's `django-admin` equivalent for Reinhardt. It provides utilities for creating new projects and applications.

## Installation

Install globally using cargo:

```bash
cargo install reinhardt-admin-cli
```

This installs the `reinhardt-admin` command.

## Usage

### Create a New Project

```bash
# Create a RESTful API project (default)
reinhardt-admin startproject myproject

# Create an MTV-style project
reinhardt-admin startproject myproject --template-type mtv

# Create project in a specific directory
reinhardt-admin startproject myproject /path/to/directory
```

### Create a New App

```bash
# Create a RESTful app (default)
reinhardt-admin startapp myapp

# Create an MTV-style app
reinhardt-admin startapp myapp --template-type mtv

# Create app in a specific directory
reinhardt-admin startapp myapp /path/to/directory
```

### Other Commands

```bash
# Display help
reinhardt-admin help

# Display version
reinhardt-admin --version
```

### Format page! Macro DSL

Format `page!` macro DSL in your source files:

```bash
# Format all Rust files in the current directory
reinhardt-admin fmt .

# Format a specific file
reinhardt-admin fmt src/main.rs

# Check formatting without modifying files
reinhardt-admin fmt --check .

# Show all files (including unchanged)
reinhardt-admin fmt -v .
```

#### Verbosity Levels

- **Default**: Show formatted files, errors, and summary (with color output)
- **`-v`**: Also show unchanged files
- **`-vv`**: Show all file processing status (deprecated, same as `-v`)

#### Output Format

- **Progress display**: Shows current processing position in `[1/50]` format
- **Color output**:
  - Success (Formatted): Green
  - Error: Red
  - Unchanged: Gray (dimmed)
  - Progress counter: Blue

#### Example Output

```bash
$ reinhardt-admin fmt .
[1/47] Formatted: src/main.rs
[2/47] Formatted: src/config/settings.rs
[3/47] Error src/broken.rs: Parse error

Summary: 2 formatted, 45 unchanged, 1 errors
```

## Django Equivalents

| Django                                | Reinhardt                                |
|---------------------------------------|------------------------------------------|
| `django-admin startproject myproject` | `reinhardt-admin startproject myproject` |
| `django-admin startapp myapp`         | `reinhardt-admin startapp myapp`         |

## Project Templates

`reinhardt-admin-cli` includes two project templates:

- **RESTful** (default): API-focused applications
- **MTV**: Traditional server-rendered web applications (Model-Template-View)

## App Templates

Apps can be created in two forms:

- **Module** (default): Created in `apps/` directory
- **Workspace**: Separate crate in workspace

## Features

- **Embedded Templates**: Templates are compiled into the binary using `rust-embed`
- **No External Dependencies**: Works without internet connection
- **Django-Compatible**: Familiar interface for Django developers

## Architecture

`reinhardt-admin-cli` depends on `reinhardt-commands` for its core functionality:

```
reinhardt-admin-cli (CLI binary)
    ↓
reinhardt-commands (Library)
    ↓
StartProjectCommand / StartAppCommand
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.