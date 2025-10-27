# REST API Example

このexampleは、ReinhardtフレームワークでRESTful APIを構築する方法を示します。

## 特徴

- **Django風のプロジェクト構造**: config/, settings/, apps.rsを使用
- **環境別設定**: local, staging, production環境の設定分離
- **manage CLI**: Django風の管理コマンド（`cargo run --bin manage`）
- **URL routing**: シンプルなAPI endpointsの定義

## プロジェクト構造

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
    └── manage.rs            # 管理CLIツール
```

## セットアップ

### 前提条件

- Rust 2024 edition以降
- Cargo

### ビルド

```bash
# プロジェクトルートから
cargo build --package example-rest-api
```

**注**: このexampleはreinhardtが crates.ioに公開された後にビルド可能になります（version ^0.1）。

## 使用方法

### 開発サーバーの起動

```bash
# デフォルト（127.0.0.1:8000）
cargo run --bin manage runserver

# カスタムアドレス
cargo run --bin manage runserver 0.0.0.0:3000
```

### 管理コマンド

```bash
# データベースマイグレーション作成
cargo run --bin manage makemigrations

# マイグレーション適用
cargo run --bin manage migrate

# 対話型シェル起動
cargo run --bin manage shell

# プロジェクトチェック
cargo run --bin manage check

# 静的ファイル収集
cargo run --bin manage collectstatic

# URL一覧表示
cargo run --bin manage showurls
```

## APIエンドポイント

### GET /api/users

ユーザーリストを返すサンプルエンドポイント

**レスポンス例:**
```json
[
  {
    "id": 1,
    "name": "Alice",
    "email": "alice@example.com"
  },
  {
    "id": 2,
    "name": "Bob",
    "email": "bob@example.com"
  }
]
```

## 環境設定

環境変数 `REINHARDT_ENV` で使用する設定を切り替えます:

```bash
# ローカル開発（デフォルト）
export REINHARDT_ENV=local
cargo run --bin manage runserver

# ステージング
export REINHARDT_ENV=staging
export SECRET_KEY=your_secret_key
export ALLOWED_HOSTS=stage.example.com
cargo run --bin manage runserver

# 本番
export REINHARDT_ENV=production
export SECRET_KEY=your_secret_key
export ALLOWED_HOSTS=example.com,www.example.com
cargo run --bin manage runserver
```

## カスタマイズ

### 新しいエンドポイントの追加

1. `src/config/urls.rs` にハンドラー関数を定義
2. `url_patterns()` でrouterに登録

```rust
async fn new_endpoint() -> Json<MyData> {
    Json(MyData { /* ... */ })
}

pub fn url_patterns() -> Arc<UnifiedRouter> {
    let router = UnifiedRouter::builder().build();
    router.add_function_route("/api/new", Method::GET, new_endpoint);
    Arc::new(router)
}
```

### アプリの追加

1. `src/config/apps.rs` の `installed_apps!` マクロに追加
2. `src/config/settings/base.rs` で必要な設定を追加

## 参考

- [Reinhardt Documentation](https://docs.rs/reinhardt)
- [Django Settings Best Practices](https://docs.djangoproject.com/en/stable/topics/settings/)
- [12 Factor App](https://12factor.net/)

## ライセンス

このexampleはReinhardtプロジェクトの一部として、MIT/Apache-2.0ライセンスの下で提供されています。
