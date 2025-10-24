# reinhardt-commands

Reinhardt用のDjangoスタイル管理コマンドフレームワーク。

## 概要

`reinhardt-commands`は、Reinhardtプロジェクトを管理するためのDjangoにインスパイアされたコマンドラインインターフェースを提供します。データベースマイグレーション、静的ファイルの収集、開発サーバーなどのための組み込みコマンドが含まれています。

## インストール

### ライブラリとして(`manage.rs`で使用)

プロジェクトの`Cargo.toml`に追加:

```toml
[dependencies]
reinhardt-commands = "0.1.0"
```

### グローバルCLIツールとして

新しいプロジェクトやアプリを作成するには、別パッケージの`reinhardt-admin`を使用します:

```bash
cargo install reinhardt-admin
```

これにより`reinhardt-admin`コマンドがインストールされます:

```bash
reinhardt-admin startproject myproject
reinhardt-admin startapp myapp
```

詳細は[reinhardt-adminドキュメント](../reinhardt-admin/README.ja.md)を参照してください。

## 機能

### 組み込みコマンド

- **makemigrations** - モデルの変更に基づいて新しいデータベースマイグレーションを作成
- **migrate** - データベースマイグレーションを適用
- **runserver** - 開発サーバーを起動
- **shell** - 対話型REPLを実行
- **check** - プロジェクトの一般的な問題をチェック
- **collectstatic** - 静的ファイルを`STATIC_ROOT`に収集
- **showurls** - 登録されているすべてのURLパターンを表示(`routers`機能が必要)

### 機能フラグ

- `migrations` - マイグレーション関連コマンドを有効化(`reinhardt-migrations`が必要)
- `routers` - URL関連コマンドを有効化(`reinhardt-routers`が必要)

## 使用方法

### プロジェクト内(`manage.rs`)

プロジェクトの`src/bin/`ディレクトリに`manage.rs`を作成:

```rust
use clap::{Parser, Subcommand};
use reinhardt_commands::{
    CheckCommand, CommandContext, MakeMigrationsCommand,
    MigrateCommand, RunServerCommand,
};

#[derive(Parser)]
#[command(name = "manage")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Makemigrations {
        #[arg(value_name = "APP_LABEL")]
        app_labels: Vec<String>,

        #[arg(long)]
        dry_run: bool,
    },
    Migrate {
        #[arg(value_name = "APP_LABEL")]
        app_label: Option<String>,
    },
    // ... その他のコマンド
}

#[tokio::main]
async fn main() {
    // CLIを解析してコマンドを実行
    // 完全な例はtemplates/project/src/bin/manage.rsを参照
}
```

その後、次のコマンドで実行:

```bash
cargo run --bin manage makemigrations
cargo run --bin manage migrate
cargo run --bin manage runserver
```

### Djangoとの対応

| Django                     | Reinhardt                             |
|---------------------------|---------------------------------------|
| `python manage.py makemigrations` | `cargo run --bin manage makemigrations` |
| `python manage.py migrate`        | `cargo run --bin manage migrate`        |
| `python manage.py runserver`      | `cargo run --bin manage runserver`      |
| `python manage.py shell`          | `cargo run --bin manage shell`          |
| `python manage.py check`          | `cargo run --bin manage check`          |
| `python manage.py collectstatic`  | `cargo run --bin manage collectstatic`  |
| `django-admin startproject`       | `reinhardt-admin startproject`          |
| `django-admin startapp`           | `reinhardt-admin startapp`              |

## カスタムコマンド

`BaseCommand`トレイトを実装してカスタムコマンドを作成:

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
        "カスタムコマンド"
    }

    async fn execute(&self, ctx: &CommandContext) -> CommandResult<()> {
        ctx.info("コマンドを実行中!");
        Ok(())
    }
}
```

`manage.rs`でコマンドを登録:

```rust
use reinhardt_commands::CommandRegistry;

let mut registry = CommandRegistry::new();
registry.register(Box::new(MyCommand));
```

## プロジェクトテンプレート

`reinhardt-commands`にはプロジェクトとアプリのテンプレートが含まれています:

### プロジェクトテンプレート

- **MTV** (Model-Template-View) - 伝統的なサーバーレンダリングWebアプリケーション
- **RESTful** - API中心のアプリケーション

```bash
reinhardt-admin startproject myproject --template-type restful
```

### アプリテンプレート

```bash
reinhardt-admin startapp myapp --template-type restful
```

テンプレートは`rust-embed`を使用してバイナリに埋め込まれているため、高速で依存関係のないプロジェクト生成が可能です。

## アーキテクチャ

`reinhardt-commands`は以下の設計原則に従っています:

- **独立性** - スタンドアロンでインストールして使用可能
- **組み合わせ可能** - コマンドを組み合わせて拡張可能
- **機能ゲート** - オプション依存関係によりコンパイル時間を短縮
- **Django互換** - Django開発者にとって馴染みのあるインターフェース

## ライセンス

以下のいずれかのライセンスで提供:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) または http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](../../LICENSE-MIT) または http://opensource.org/licenses/MIT)

お好きな方を選択してください。
