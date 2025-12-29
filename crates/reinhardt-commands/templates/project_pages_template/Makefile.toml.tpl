# Makefile.toml for Reinhardt Project
#
# This file provides cargo-make task definitions for common development operations.
# Install cargo-make: `cargo install cargo-make`
# Usage: `cargo make <task>`

[config]
default_to_workspace = false
skip_core_tasks = true

[env]
WASM_TARGET = "wasm32-unknown-unknown"
WASM_BINDGEN_VERSION = "0.2.100"

# ============================================================================
# Development Server
# ============================================================================

[tasks.runserver]
description = "Start the development server with static files"
command = "cargo"
args = ["run", "--bin", "manage", "runserver", "--with-pages"]
dependencies = ["wasm-build-dev"]

[tasks.runserver-watch]
description = "Start the development server with auto-reload (requires bacon)"
command = "bacon"
args = ["runserver"]
dependencies = ["install-bacon"]

# ============================================================================
# WASM Build
# ============================================================================

[tasks.wasm-build-dev]
description = "Build WASM in debug mode"
script = '''
#!/bin/bash
set -e
echo "Building WASM (debug mode)..."
cargo build --target ${WASM_TARGET} --lib
WASM_FILE=$(ls target/${WASM_TARGET}/debug/*.wasm 2>/dev/null | head -1)
if [ -n "$WASM_FILE" ]; then
	mkdir -p dist
	wasm-bindgen --target web "$WASM_FILE" --out-dir dist --debug
	echo "WASM build complete: dist/"
else
	echo "No WASM file found to process"
fi
'''
dependencies = ["install-wasm-tools"]

[tasks.wasm-build-release]
description = "Build WASM in release mode with optimization"
script = '''
#!/bin/bash
set -e
echo "Building WASM (release mode)..."
cargo build --target ${WASM_TARGET} --lib --release
WASM_FILE=$(ls target/${WASM_TARGET}/release/*.wasm 2>/dev/null | head -1)
if [ -n "$WASM_FILE" ]; then
	mkdir -p dist
	wasm-bindgen --target web "$WASM_FILE" --out-dir dist
	if command -v wasm-opt &> /dev/null; then
		WASM_BG=$(ls dist/*_bg.wasm 2>/dev/null | head -1)
		if [ -n "$WASM_BG" ]; then
			wasm-opt -O3 "$WASM_BG" -o "$WASM_BG"
			echo "WASM optimized with wasm-opt"
		fi
	fi
	echo "WASM build complete: dist/"
else
	echo "No WASM file found to process"
fi
'''
dependencies = ["install-wasm-tools"]

[tasks.wasm-watch]
description = "Watch and rebuild WASM on changes"
command = "cargo"
args = ["watch", "-w", "src/client", "-s", "cargo make wasm-build-dev"]
dependencies = ["install-cargo-watch"]

[tasks.wasm-clean]
description = "Clean WASM build artifacts"
script = '''
rm -rf dist/
echo "WASM build artifacts cleaned"
'''

# ============================================================================
# Database Migrations
# ============================================================================

[tasks.makemigrations]
description = "Create new migrations based on model changes"
command = "cargo"
args = ["run", "--bin", "manage", "makemigrations"]

[tasks.makemigrations-app]
description = "Create new migrations for a specific app (usage: cargo make makemigrations-app -- <app_label>)"
command = "cargo"
args = ["run", "--bin", "manage", "makemigrations", "${@}"]

[tasks.migrate]
description = "Apply database migrations"
command = "cargo"
args = ["run", "--bin", "manage", "migrate"]

# ============================================================================
# Static Files
# ============================================================================

[tasks.collectstatic]
description = "Collect static files into STATIC_ROOT"
command = "cargo"
args = ["run", "--bin", "manage", "collectstatic"]

# ============================================================================
# Project Management
# ============================================================================

[tasks.check]
description = "Check the project for common issues"
command = "cargo"
args = ["run", "--bin", "manage", "check"]

[tasks.showurls]
description = "Display all registered URL patterns"
command = "cargo"
args = ["run", "--bin", "manage", "showurls"]

[tasks.shell]
description = "Run an interactive Rust shell (REPL)"
command = "cargo"
args = ["run", "--bin", "manage", "shell"]

# ============================================================================
# Testing
# ============================================================================

[tasks.test]
description = "Run all tests"
command = "cargo"
args = ["nextest", "run", "--all-features"]
dependencies = ["install-nextest"]

[tasks.test-unit]
description = "Run unit tests only"
command = "cargo"
args = ["nextest", "run", "--lib", "--all-features"]
dependencies = ["install-nextest"]

[tasks.test-integration]
description = "Run integration tests only"
command = "cargo"
args = ["nextest", "run", "--test", "*", "--all-features"]
dependencies = ["install-nextest"]

[tasks.test-watch]
description = "Run tests with auto-reload (requires bacon)"
command = "bacon"
args = ["test"]
dependencies = ["install-bacon", "install-nextest"]

# ============================================================================
# Code Quality
# ============================================================================

[tasks.fmt-check]
description = "Check code formatting"
command = "cargo"
args = ["fmt", "--all", "--", "--check"]

[tasks.fmt-fix]
description = "Fix code formatting"
command = "cargo"
args = ["fmt", "--all"]

[tasks.clippy-check]
description = "Check linting rules"
command = "cargo"
args = ["clippy", "--all-features", "--", "-D", "warnings"]

[tasks.clippy-fix]
description = "Fix linting issues automatically"
command = "cargo"
args = ["clippy", "--all-features", "--fix", "--allow-dirty", "--allow-staged"]

[tasks.quality]
description = "Run all code quality checks (format + lint)"
dependencies = ["fmt-check", "clippy-check"]

[tasks.quality-fix]
description = "Fix all code quality issues automatically"
dependencies = ["fmt-fix", "clippy-fix"]

# ============================================================================
# Build & Clean
# ============================================================================

[tasks.build]
description = "Build the project in debug mode"
command = "cargo"
args = ["build", "--all-features"]

[tasks.build-release]
description = "Build the project in release mode"
command = "cargo"
args = ["build", "--release", "--all-features"]

[tasks.clean]
description = "Clean build artifacts"
command = "cargo"
args = ["clean"]

# ============================================================================
# Dependencies Installation
# ============================================================================

[tasks.install-nextest]
description = "Install cargo-nextest if not already installed"
script = '''
if ! command -v cargo-nextest &> /dev/null
then
	echo "Installing cargo-nextest..."
	cargo install cargo-nextest --locked
else
	echo "cargo-nextest is already installed"
fi
'''

[tasks.install-bacon]
description = "Install bacon if not already installed"
script = '''
if ! command -v bacon &> /dev/null
then
	echo "Installing bacon..."
	cargo install --locked bacon
else
	echo "bacon is already installed"
fi
'''

[tasks.install-wasm-tools]
description = "Install WASM build tools"
script = '''
#!/bin/bash
set -e
echo "Installing WASM build tools..."

# Add wasm32 target
rustup target add wasm32-unknown-unknown

# Install wasm-bindgen-cli
if ! wasm-bindgen --version 2>/dev/null | grep -q "${WASM_BINDGEN_VERSION}"; then
	echo "Installing wasm-bindgen-cli..."
	cargo install wasm-bindgen-cli --version "${WASM_BINDGEN_VERSION}"
else
	echo "wasm-bindgen-cli is already installed"
fi

# Install wasm-opt (binaryen) - optional but recommended
if ! command -v wasm-opt &> /dev/null; then
	echo "wasm-opt not found. Install binaryen for WASM optimization:"
	echo "  macOS: brew install binaryen"
	echo "  Linux: apt install binaryen or download from GitHub"
else
	echo "wasm-opt is already installed"
fi
'''

[tasks.install-cargo-watch]
description = "Install cargo-watch for file watching"
script = '''
if ! command -v cargo-watch &> /dev/null; then
	echo "Installing cargo-watch..."
	cargo install cargo-watch
else
	echo "cargo-watch is already installed"
fi
'''

[tasks.install-tools]
description = "Install all required development tools"
dependencies = ["install-nextest", "install-bacon", "install-wasm-tools", "install-cargo-watch"]

# ============================================================================
# Development Workflow
# ============================================================================

[tasks.dev]
description = "Start development environment (checks, builds WASM, runs server)"
dependencies = ["quality", "wasm-build-dev", "runserver"]

[tasks.dev-watch]
description = "Start development with auto-reload"
dependencies = ["quality", "wasm-build-dev", "runserver-watch"]

# ============================================================================
# CI/CD Workflow
# ============================================================================

[tasks.ci]
description = "Run CI pipeline (format, lint, build, test)"
dependencies = ["fmt-check", "clippy-check", "build", "test"]

# ============================================================================
# Verbosity Control
# ============================================================================

[tasks.runserver-v]
description = "Start the development server with verbose output"
command = "cargo"
args = ["run", "--bin", "manage", "runserver", "-v"]

[tasks.runserver-vv]
description = "Start the development server with very verbose output"
command = "cargo"
args = ["run", "--bin", "manage", "runserver", "-vv"]

[tasks.runserver-vvv]
description = "Start the development server with maximum verbosity"
command = "cargo"
args = ["run", "--bin", "manage", "runserver", "-vvv"]

# ============================================================================
# Help
# ============================================================================

[tasks.help]
description = "Show available tasks"
script = '''
echo "Available tasks:"
echo "  Development:"
echo "    runserver          - Start the development server (with WASM)"
echo "    runserver-watch    - Start server with auto-reload"
echo "    dev                - Run checks + build WASM + start server"
echo "    dev-watch          - Development with auto-reload"
echo ""
echo "  WASM Build:"
echo "    wasm-build-dev     - Build WASM (debug mode)"
echo "    wasm-build-release - Build WASM (release + optimize)"
echo "    wasm-watch         - Watch and rebuild WASM on changes"
echo "    wasm-clean         - Clean WASM build artifacts"
echo ""
echo "  Database:"
echo "    makemigrations     - Create new migrations"
echo "    makemigrations-app - Create migrations for specific app"
echo "    migrate            - Apply migrations"
echo ""
echo "  Static Files:"
echo "    collectstatic      - Collect static files"
echo ""
echo "  Project Management:"
echo "    check              - Check project for issues"
echo "    showurls           - Show URL patterns"
echo "    shell              - Interactive REPL"
echo ""
echo "  Testing:"
echo "    test               - Run all tests"
echo "    test-unit          - Run unit tests"
echo "    test-integration   - Run integration tests"
echo "    test-watch         - Tests with auto-reload"
echo ""
echo "  Code Quality:"
echo "    fmt-check          - Check formatting"
echo "    fmt-fix            - Fix formatting"
echo "    clippy-check       - Check linting"
echo "    clippy-fix         - Fix linting issues"
echo "    quality            - Run all checks"
echo "    quality-fix        - Fix all issues"
echo ""
echo "  Build:"
echo "    build              - Build (debug)"
echo "    build-release      - Build (release)"
echo "    clean              - Clean artifacts"
echo ""
echo "  CI/CD:"
echo "    ci                 - Run CI pipeline"
echo ""
echo "  Tools:"
echo "    install-tools      - Install dev tools (incl. WASM tools)"
echo ""
echo "Usage: cargo make <task>"
'''
