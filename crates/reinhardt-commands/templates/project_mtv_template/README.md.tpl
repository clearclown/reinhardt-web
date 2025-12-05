# {{ project_name }}

A Reinhardt web application using MVC architecture.

## Getting Started

### Using cargo-make (Recommended)

Install cargo-make:
```bash
cargo install cargo-make
```

Run the development server:
```bash
cargo make runserver
```

### Using manage command

```bash
cargo run --bin manage runserver
```

Visit http://127.0.0.1:8000 to see your application.

## Structure

- `src/config/` - Configuration files
- `src/apps/` - Application modules
- `templates/` - HTML templates
- `Makefile.toml` - Task runner configuration

## Environment

Set `REINHARDT_ENV` to switch between environments:
- `local` (default)
- `staging`
- `production`

## Common Tasks

### Development

```bash
cargo make dev              # Run checks + build + start server
cargo make runserver-watch  # Start server with auto-reload
```

### Database

```bash
cargo make makemigrations   # Create new migrations
cargo make migrate          # Apply migrations
```

### Testing

```bash
cargo make test             # Run all tests
cargo make test-watch       # Run tests with auto-reload
```

### Code Quality

```bash
cargo make quality          # Run all checks (format + lint)
cargo make quality-fix      # Fix all issues automatically
```

### Help

```bash
cargo make help             # Show all available tasks
```
