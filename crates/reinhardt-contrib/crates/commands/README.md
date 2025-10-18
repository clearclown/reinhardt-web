# reinhardt-commands

CLI command framework for Reinhardt

## Overview

Django-style management command framework for creating custom CLI tools for tasks like data migration, maintenance, and administrative operations. Provides a structured way to create custom commands with argument parsing, interactive prompts, and rich terminal output.

## Features

### Core Framework (Implemented ✓)

- **BaseCommand Trait**: Define custom management commands with a trait-based interface
  - Command lifecycle hooks: `before_execute`, `execute`, `after_execute`
  - Argument and option definitions with `CommandArgument` and `CommandOption`
  - Built-in help text and description support

- **Command Context**: Execution context for commands
  - Argument and option parsing
  - Colored output methods: `info`, `success`, `warning`, `verbose`
  - Access to command-line arguments and flags

- **Command Registry**: Automatic command discovery and registration
  - Register custom commands dynamically
  - List all available commands
  - Lookup commands by name

### Built-in Commands (Implemented ✓)

#### Database Management
- **migrate**: Run database migrations with support for app-specific migrations
  - Options: `--fake`, `--fake-initial`, `--database`
  - Arguments: `app`, `migration`

- **makemigrations**: Create new migrations based on model changes
  - Options: `--dry-run`, `--empty`, `--name`
  - Arguments: `app`

#### Development Server
- **runserver**: Start the development server
  - Default address: `127.0.0.1:8000`
  - Options: `--noreload`, `--nothreading`, `--insecure`
  - HTTPS support with TLS/SSL
  - Auto-reload on file changes
  - Arguments: `address`

#### Interactive Tools
- **shell**: Start an interactive Rust REPL
  - Options: `--command` for executing commands directly
  - Interactive mode support

#### Project/App Scaffolding
- **startproject**: Create a Reinhardt project directory structure
  - Options: `--template`, `--extension`, `--mtv`, `--restful`
  - Arguments: `name`, `directory`
  - Support for MTV (Model-Template-View) or RESTful API project styles
  - Template-based code generation with embedded templates
  - Automatic secret key generation

- **startapp**: Create a Reinhardt app directory structure
  - Options: `--template`, `--extension`, `--mtv`, `--restful`, `--workspace`
  - Arguments: `name`, `directory`
  - Create as module (default) or workspace crate
  - Automatic integration with apps.rs and Cargo.toml
  - Template-based code generation

#### Static Files
- **collectstatic**: Collect static files into a single location
  - Options: `--clear`, `--no-input`, `--dry-run`
  - Statistics tracking: copied, skipped, deleted files

#### Internationalization (i18n)
- **makemessages**: Extract translatable strings from source files
  - Options: `--locale`, `--all`, `--extension`, `--ignore`, `--no-default-ignore`, `--no-wrap`, `--no-location`, `--add-location`, `--keep-pot`
  - Support for multiple file extensions: `.html`, `.txt`, `.py`, `.rs`
  - Pattern matching for: `gettext!()`, `_()`, `t!()`, `{% trans "" %}`
  - Creates/updates `.po` files in `locale/{locale}/LC_MESSAGES/`
  - Locale validation and merging existing translations

- **compilemessages**: Compile `.po` message files to `.mo` binary format
  - Options: `--locale`, `--exclude`, `--ignore`, `--use-fuzzy`
  - GNU gettext-compatible MO file format
  - Automatic locale discovery
  - Skip untranslated messages

#### Email
- **sendtestemail**: Send a test email to verify email configuration
  - Basic implementation for email testing

#### System Utilities
- **showurls**: Display all registered URL patterns
  - Options: `--names` for showing only named URLs

- **check**: Check for common problems
  - Options: `--deploy` for checking deployment settings
  - System-wide health checks

### Template System (Implemented ✓)

- **TemplateCommand**: Base command for template-based code generation
  - Template context with variable substitution
  - Support for custom template directories
  - File rendering with Askama template engine

- **Template Utilities**:
  - `generate_secret_key()`: Generate Django-compatible secret keys
  - `to_camel_case()`: Convert strings to CamelCase for Rust naming conventions
  - Embedded templates via `rust-embed`

### Advanced Features (Implemented ✓)

- **Argument Parsing**: Clap-based argument handling with typed arguments and options
- **Interactive Mode**: Dialoguer-based interactive prompts for user input
- **Colored Output**: Colored terminal output for better readability
- **Error Handling**: Comprehensive error types with `thiserror`
  - `CommandError::NotFound`, `InvalidArguments`, `ExecutionError`, `IoError`, `ParseError`
- **Async Support**: Full async/await support with `async-trait`

## Planned Features

### Additional Built-in Commands
- **createsuperuser**: Create a superuser account interactively
- **changepassword**: Change a user's password
- **dumpdata**: Serialize database data to JSON/YAML
- **loaddata**: Load fixtures into the database
- **inspectdb**: Generate models from existing database schema
- **sqlmigrate**: Display SQL for a migration
- **showmigrations**: Show all migrations and their status
- **flush**: Remove all data from the database
- **dbshell**: Open database shell

### Template Enhancements
- Custom template tags and filters
- Template inheritance and includes
- More project/app templates (GraphQL, gRPC, etc.)

### Testing Utilities
- **test**: Run tests with custom test runner
- **testserver**: Run development server with test fixtures

### Performance Tools
- **benchmark**: Benchmark view performance
- **profile**: Profile application performance

### Deployment Utilities
- **compilestatic**: Optimize and minify static files
- **clearsessions**: Clear expired sessions from database

## Usage Example

```rust
use reinhardt_commands::{BaseCommand, CommandContext, CommandResult};
use async_trait::async_trait;

struct MyCommand;

#[async_trait]
impl BaseCommand for MyCommand {
    fn name(&self) -> &str {
        "mycommand"
    }

    fn description(&self) -> &str {
        "My custom management command"
    }

    async fn execute(&self, ctx: &CommandContext) -> CommandResult<()> {
        ctx.info("Executing my command...");
        ctx.success("Command completed successfully!");
        Ok(())
    }
}
```

## Architecture

```
reinhardt-commands/
├── src/
│   ├── base.rs              # BaseCommand trait
│   ├── builtin.rs           # Built-in commands
│   ├── collectstatic.rs     # Static file collection
│   ├── context.rs           # Command execution context
│   ├── embedded_templates.rs # Embedded template resources
│   ├── i18n_commands.rs     # i18n commands
│   ├── mail_commands.rs     # Email commands
│   ├── registry.rs          # Command registry
│   ├── start_commands.rs    # Project/app scaffolding
│   └── template.rs          # Template utilities
└── templates/               # Built-in templates
    ├── project/             # Project templates
    └── app/                 # App templates
```

## Dependencies

- `clap`: Command-line argument parsing
- `tokio`: Async runtime
- `async-trait`: Async trait support
- `colored`: Terminal colored output
- `dialoguer`: Interactive prompts
- `askama`: Template engine
- `walkdir`: Directory traversal
- `regex`: Regular expression support
- `rust-embed`: Embedded template resources
- `rustls`, `tokio-rustls`, `hyper-rustls`: TLS/HTTPS support
- `rcgen`: Certificate generation for development HTTPS

## Integration

This crate integrates with:
- `reinhardt-apps`: App configuration
- `reinhardt-mail`: Email sending functionality
- `reinhardt-settings`: Application settings
- `reinhardt-static`: Static file handling
