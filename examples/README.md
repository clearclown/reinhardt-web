# Reinhardt Examples Tests

このディレクトリには、**crates.io から公開された reinhardt** を使用した実際のアプリケーション例が含まれています。

## 🎯 Purpose

- **crates.io からの取得**: ローカルの実装ではなく、公開版を使用
- **バージョン検証**: 各 example が特定のバージョンで動作することを保証
- **エンドツーエンドテスト**: 実際のユーザー環境での動作を検証
- **インフラストラクチャ**: Podman + docker-compose で再現可能な環境

## 📋 Prerequisites

### Required
- **Rust**: 1.85+ (Rust 2024 Edition)
- **Podman**: Container management
- **podman-compose**: docker-compose compatible tool

### Optional
- **cargo-make**: For convenient commands (`cargo install cargo-make`)

### Installation Check

```bash
# Check Podman installation
podman --version
podman-compose --version

# Or use cargo-make
cargo make check-podman
```

## 🚀 Quick Start

### 1. Initial Setup

```bash
cd examples

# Create .env file
cargo make setup

# Or manually
cp .env.example .env
```

### 2. Start Infrastructure

```bash
# Start PostgreSQL only
cargo make up

# Start all services (including MySQL, Redis)
cargo make up-all

# Check status
cargo make status
```

### 3. Run Tests

```bash
# Test all examples
cargo make test

# Keep infrastructure running after tests
cargo make test-keep

# Or run directly
cargo test --workspace
```

### 4. Stop Infrastructure

```bash
# Stop
cargo make down

# Stop and remove volumes
cargo make down-volumes
```

## 📝 Version Specification (Cargo Compatible)

Each test can specify version requirements using `#[example_test(version = "...")]` attribute with **the same syntax as Cargo.toml**.

### Supported Version Specifiers

```rust
// 1. Exact version
#[example_test(version = "0.1.0")]
fn test_exact() { }

// 2. Caret requirement (^)
#[example_test(version = "^0.1")]
fn test_caret() { }  // 0.1.x only

// 3. Tilde requirement (~)
#[example_test(version = "~0.1.2")]
fn test_tilde() { }  // 0.1.2 <= version < 0.2.0

// 4. Range specification
#[example_test(version = ">=0.1.0, <0.2.0")]
fn test_range() { }

// 5. Wildcard
#[example_test(version = "*")]
fn test_latest() { }  // Latest version
```

## 📂 Examples List

| Example | Version Requirement | Database | Description | README |
|---------|---------------------|----------|-------------|--------|
| `hello-world` | `*` (latest) | Not required | Minimal application | - |
| `rest-api` | `^0.1` (0.1.x) | Not required | RESTful API with Django-style structure | [README](rest-api/README.md) |
| `database-integration` | `^0.1` (0.1.x) | Required | PostgreSQL integration with migrations | [README](database-integration/README.md) |

### Example Features

#### hello-world
- 最小限の構成
- シンプルなエントリーポイント
- Reinhardtの基本的な使い方

#### rest-api ([詳細](rest-api/README.md))
- **Django風のプロジェクト構造**: config/, settings/, apps.rs
- **環境別設定**: local, staging, production
- **manage CLI**: `cargo run --bin manage` でDjango風の管理コマンド
- **URL routing**: RESTful API endpoints

#### database-integration ([詳細](database-integration/README.md))
- **Django風のプロジェクト構造**: config/, settings/, apps.rs
- **データベース設定管理**: 環境別のDB接続設定
- **マイグレーションシステム**: スキーマのバージョン管理
- **manage CLI**: makemigrations, migrateコマンド

## 🏗️ Workspace Structure

```
examples/                    # Independent workspace
├── Cargo.toml              # Workspace configuration
├── test-macros/            # Custom test macros
├── common/                 # Common utilities
│   └── src/
│       └── manage_cli.rs   # 共通manage CLI実装
├── hello-world/            # Example 1 (minimal structure)
├── rest-api/               # Example 2 (full structure)
│   └── src/
│       ├── config/         # Django-style config
│       ├── bin/
│       │   └── manage.rs   # Management CLI
│       └── main.rs
└── database-integration/   # Example 3 (full structure)
    └── src/
        ├── config/         # Django-style config
        ├── bin/
        │   └── manage.rs   # Management CLI
        └── main.rs
```

Each example is a **workspace member**, managed in `examples/Cargo.toml`.

### Project Structure

Examples (`rest-api`, `database-integration`) use **Django-style project structure**:

```
src/
├── config/
│   ├── apps.rs              # インストール済みアプリの定義
│   ├── settings.rs          # 環境に応じた設定ローダー
│   ├── settings/
│   │   ├── base.rs          # 全環境共通の基本設定
│   │   ├── local.rs         # ローカル開発環境設定
│   │   ├── staging.rs       # ステージング環境設定
│   │   └── production.rs    # 本番環境設定
│   └── urls.rs              # URLルーティング設定
├── apps.rs                  # アプリレジストリ
├── config.rs                # configモジュール宣言
├── main.rs                  # アプリケーションエントリーポイント
└── bin/
    └── manage.rs            # 管理CLIツール (Django's manage.py)
```

### manage CLI

Django風の管理コマンドツール:

```bash
# 開発サーバー起動
cargo run --bin manage runserver [address]

# データベースマイグレーション
cargo run --bin manage makemigrations [app_labels...]
cargo run --bin manage migrate [app_label] [migration_name]

# 対話型シェル
cargo run --bin manage shell [-c command]

# プロジェクトチェック
cargo run --bin manage check [app_label]

# 静的ファイル収集
cargo run --bin manage collectstatic [options]

# URL一覧表示
cargo run --bin manage showurls [--names]
```

詳細は各exampleのREADMEを参照してください。

## 🐳 Infrastructure

### Available Services

```bash
# PostgreSQL (starts by default)
podman-compose up -d postgres

# MySQL (optional)
podman-compose --profile mysql up -d mysql

# Redis (optional)
podman-compose --profile cache up -d redis
```

### Connection Information

**PostgreSQL:**
```
Host: localhost
Port: 5432
User: reinhardt
Password: reinhardt_dev
Database: reinhardt_examples
URL: postgres://reinhardt:reinhardt_dev@localhost:5432/reinhardt_examples
```

**MySQL:**
```
Host: localhost
Port: 3306
User: reinhardt
Password: reinhardt_dev
Database: reinhardt_examples
URL: mysql://reinhardt:reinhardt_dev@localhost:3306/reinhardt_examples
```

**Redis:**
```
Host: localhost
Port: 6379
URL: redis://localhost:6379
```

### Database Migrations

Examples using databases utilize **reinhardt-migrations** for schema management:

- **No SQL Scripts**: Database initialization is handled through migrations
- **Automatic Application**: Migrations run on application startup
- **Version Control**: Migration history tracked in code

**Example Migration Structure:**
```
database-integration/
├── migrations/
│   ├── mod.rs
│   └── 0001_initial.rs
└── src/
    └── main.rs         # Applies migrations on startup
```

**Migration Example:**
```rust
use reinhardt_migrations::{Migration, Operation};

pub fn migration() -> Migration {
    Migration::new("0001_initial")
        .add_operation(Operation::CreateTable {
            name: "users".to_string(),
            columns: vec![
                ("id", "SERIAL PRIMARY KEY"),
                ("name", "VARCHAR(255) NOT NULL"),
                ("email", "VARCHAR(255) NOT NULL UNIQUE"),
            ],
        })
}
```

## 🔧 Development Workflow

### Adding a New Example

1. **Create directory**
   ```bash
   mkdir examples/my-example
   cd examples/my-example
   ```

2. **Create Cargo.toml**
   ```toml
   [package]
   name = "example-my-example"
   version = "0.1.0"
   edition = "2024"
   publish = false

   [dependencies]
   reinhardt = "^0.1"
   ```

3. **Add to workspace**
   ```toml
   # examples/Cargo.toml
   [workspace]
   members = [
       # ...
       "my-example",
   ]
   ```

4. **Create tests**
   ```rust
   // examples/my-example/tests/integration.rs
   use example_test_macros::example_test;

   #[example_test(version = "^0.1")]
   fn test_my_feature() {
       // Test code
   }
   ```

## ⚠️ Troubleshooting

### Podman Won't Start

```bash
# Start Podman service
podman machine start

# Or with systemd
systemctl --user start podman.socket
```

### Database Connection Error

```bash
# Check health
cargo make status

# Check logs
cargo make logs-postgres

# Restart database
cargo make down
cargo make up
```

### Port Conflict

```bash
# Change port numbers in .env file
POSTGRES_PORT=5433
MYSQL_PORT=3307
REDIS_PORT=6380
```

### Tests Are Skipped

```
⏭️  Skipping test: reinhardt not available from crates.io
```

**Cause**: reinhardt is not yet published to crates.io

**Solution**: Wait until published, or use local integration tests (`tests/`)

## 📚 Related Documentation

- [Reinhardt Main Tests](../tests/)
- [Project README](../README.md)
- [Contributing Guide](../CONTRIBUTING.md)
- [Podman Official Documentation](https://podman.io/)
- [docker-compose Specification](https://docs.docker.com/compose/compose-file/)

---

## 💡 Implementation Notes

### Why crates.io Only?

These examples test the **actual published version** that users will install. This ensures:

1. **Real User Experience**: Tests reflect what users will encounter
2. **Version Compatibility**: Verifies version claims are accurate
3. **Publication Validation**: Confirms published packages work correctly

### Why Version-Specific Tests?

Different versions may have different APIs or behaviors. Version-specific tests:

1. **Prevent Regressions**: Detect breaking changes
2. **Document Compatibility**: Show which features work with which versions
3. **Aid Migration**: Help users understand version differences

### Current Status

⚠️ **Note**: Since reinhardt is not yet published to crates.io, all tests will currently be skipped. This is **expected behavior**. Once published, tests will automatically begin running.

To test reinhardt before publication, use the main integration tests in `tests/` directory instead.
