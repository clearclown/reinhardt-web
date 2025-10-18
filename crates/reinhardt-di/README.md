# reinhardt-di

FastAPI-inspired dependency injection system for Reinhardt.

## Overview

FastAPI-styleの依存性注入システムを提供します。リクエストスコープ、シングルトンスコープの依存性キャッシング、ネストされた依存性の自動解決、認証やデータベース接続との統合をサポートします。

型安全で非同期ファーストな設計により、FastAPIの開発体験をRustで実現します。

## Core Concepts

### Dependency Scopes

- **Request Scope**: リクエストごとにキャッシュされる依存性（デフォルト）
- **Singleton Scope**: アプリケーション全体で共有される依存性

### Automatic Injection

`Default + Clone + Send + Sync + 'static`を実装する型は、自動的に`Injectable`トレイトが実装され、依存性として使用できます。

## Implemented Features ✓

### Core Dependency Injection

- ✓ **`Depends<T>` Wrapper**: FastAPI-styleの依存性注入ラッパー
  - `Depends::<T>::new()` - キャッシュ有効（デフォルト）
  - `Depends::<T>::no_cache()` - キャッシュ無効
  - `resolve(&ctx)` - 依存性の解決
  - `from_value(value)` - テスト用の値からの生成

- ✓ **Injectable Trait**: 依存性として注入可能な型を定義
  - 自動実装: `Default + Clone + Send + Sync + 'static`型に対して
  - カスタム実装: 複雑な初期化ロジックが必要な場合

- ✓ **InjectionContext**: 依存性解決のためのコンテキスト
  - `get_request<T>()` / `set_request<T>()` - リクエストスコープ
  - `get_singleton<T>()` / `set_singleton<T>()` - シングルトンスコープ
  - リクエストごとに新しいコンテキストを生成

- ✓ **RequestScope**: リクエスト内でのキャッシング
  - 型ベースのキャッシュ（`TypeId`をキーとして使用）
  - スレッドセーフな実装（`Arc<RwLock<HashMap>>`）

- ✓ **SingletonScope**: アプリケーション全体でのキャッシング
  - すべてのリクエスト間で共有される依存性
  - スレッドセーフな実装

### Advanced Features

- ✓ **Dependency Caching**: リクエストスコープ内での自動キャッシング
  - 同じ依存性を複数回要求しても1回だけ生成される
  - ネストされた依存性間でキャッシュが共有される
  - キャッシュの有効/無効を制御可能

- ✓ **Nested Dependencies**: 依存性が他の依存性に依存できる
  - 依存性グラフの自動解決
  - 循環依存の検出とエラー処理

- ✓ **Dependency Overrides**: テスト用の依存性オーバーライド
  - 本番とテストで異なる実装を使用可能
  - アプリケーションレベルでのオーバーライド管理
  - サブ依存性を持つオーバーライドのサポート

- ✓ **Provider System**: 非同期ファクトリーパターン
  - `Provider` trait - 依存性を提供するための汎用インターフェース
  - `ProviderFn` - 関数ベースのプロバイダー
  - 任意の非同期クロージャをプロバイダーとして使用可能

### Error Handling

- ✓ **DiError**: 包括的なエラー型
  - `NotFound` - 依存性が見つからない
  - `CircularDependency` - 循環依存の検出
  - `ProviderError` - プロバイダーのエラー
  - `TypeMismatch` - 型の不一致
  - `ScopeError` - スコープ関連のエラー

### Integration Support

- ✓ **HTTP Integration**: HTTPリクエスト/レスポンスとの統合
  - リクエストからの依存性注入
  - 接続情報の注入サポート

- ✓ **WebSocket Support**: WebSocket接続への依存性注入
  - WebSocketハンドラーでの`Depends<T>`使用

## Planned Features

### Lifecycle Management

- ⏳ **Generator-based Dependencies** (`yield`パターン)
  - セットアップ/ティアダウンロジックのサポート
  - コンテキストマネージャーパターン
  - リソースの自動クリーンアップ

### Advanced Patterns

- ⏳ **Dependency Classes**: クラスベースの依存性
  - より複雑な依存性の定義
  - 状態を持つ依存性のサポート

- ⏳ **Parametrized Dependencies**: パラメーター化された依存性
  - パスパラメーター、クエリパラメーターとの統合
  - 動的な依存性の生成

- ⏳ **Schema Generation**: 依存性のスキーマ自動生成
  - OpenAPI仕様への統合
  - 依存性の重複検出と最適化

### Security

- ⏳ **Security Overrides**: セキュリティ関連の依存性オーバーライド
  - 認証/認可の依存性
  - セキュリティスキームの統合

## Usage Examples

### Basic Usage

```rust
use reinhardt_di::{Depends, Injectable, InjectionContext, SingletonScope};
use std::sync::Arc;

#[derive(Clone, Default)]
struct Config {
    api_key: String,
    database_url: String,
}

#[tokio::main]
async fn main() {
    // シングルトンスコープの作成
    let singleton = Arc::new(SingletonScope::new());

    // リクエストコンテキストの作成
    let ctx = InjectionContext::new(singleton);

    // 依存性の解決（キャッシュ有効）
    let config = Depends::<Config>::new()
        .resolve(&ctx)
        .await
        .unwrap();

    println!("API Key: {}", config.api_key);
}
```

### Custom Injectable Implementation

```rust
use reinhardt_di::{Injectable, InjectionContext, DiResult};

struct Database {
    pool: DbPool,
}

#[async_trait::async_trait]
impl Injectable for Database {
    async fn inject(ctx: &InjectionContext) -> DiResult<Self> {
        // カスタム初期化ロジック
        let config = Config::inject(ctx).await?;
        let pool = create_pool(&config.database_url).await?;

        Ok(Database { pool })
    }
}
```

### Nested Dependencies

```rust
#[derive(Clone)]
struct ServiceA {
    db: Arc<Database>,
}

#[async_trait::async_trait]
impl Injectable for ServiceA {
    async fn inject(ctx: &InjectionContext) -> DiResult<Self> {
        // Databaseに依存
        let db = Database::inject(ctx).await?;
        Ok(ServiceA { db: Arc::new(db) })
    }
}

#[derive(Clone)]
struct ServiceB {
    service_a: Arc<ServiceA>,
    config: Config,
}

#[async_trait::async_trait]
impl Injectable for ServiceB {
    async fn inject(ctx: &InjectionContext) -> DiResult<Self> {
        // ServiceAとConfigに依存（ネストされた依存性）
        let service_a = ServiceA::inject(ctx).await?;
        let config = Config::inject(ctx).await?;

        Ok(ServiceB {
            service_a: Arc::new(service_a),
            config,
        })
    }
}
```

### Dependency Overrides for Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct MockDatabase {
        // テスト用のモック実装
    }

    #[tokio::test]
    async fn test_with_mock_database() {
        let singleton = Arc::new(SingletonScope::new());
        let ctx = InjectionContext::new(singleton);

        // テスト用のモックをセット
        let mock_db = MockDatabase { /* ... */ };
        ctx.set_request(mock_db);

        // テストコード
    }
}
```

### Cache Control

```rust
// キャッシュ有効（デフォルト） - 同じインスタンスを返す
let config1 = Depends::<Config>::new().resolve(&ctx).await?;
let config2 = Depends::<Config>::new().resolve(&ctx).await?;
// config1とconfig2は同じインスタンス

// キャッシュ無効 - 毎回新しいインスタンスを作成
let config3 = Depends::<Config>::no_cache().resolve(&ctx).await?;
let config4 = Depends::<Config>::no_cache().resolve(&ctx).await?;
// config3とconfig4は異なるインスタンス
```

## Architecture

### Type-Based Caching

依存性のキャッシュは型（`TypeId`）をキーとして管理されます。これにより、同じ型の依存性は自動的にキャッシュされます。

### Scope Hierarchy

```
SingletonScope (アプリケーションレベル)
    ↓ 共有
InjectionContext (リクエストレベル)
    ↓ 保持
RequestScope (リクエスト内キャッシュ)
```

### Thread Safety

- すべてのスコープは`Arc<RwLock<HashMap>>`を使用してスレッドセーフ
- `Injectable` traitは`Send + Sync`を要求
- 非同期コードで安全に使用可能

## Testing Support

テストフレームワークには包括的なテストスイートが含まれています：

- **Unit Tests**: 各コンポーネントの単体テスト
- **Integration Tests**: FastAPIのテストケースを移植した統合テスト
- **Feature Tests**:
  - 自動Injectable実装のテスト
  - 循環依存の検出テスト
  - キャッシュ動作のテスト
  - 依存性オーバーライドのテスト
  - ネストされた依存性のテスト

## Performance Considerations

- **Lazy Initialization**: 依存性は必要になるまで生成されません
- **Cache Efficiency**: リクエストスコープ内で同じ依存性は1回だけ生成されます
- **Zero-Cost Abstractions**: Rustの型システムを活用したオーバーヘッドの少ない設計
- **Arc-based Sharing**: `Arc`を使用した効率的なインスタンス共有

## Comparison with FastAPI

| Feature | FastAPI (Python) | reinhardt-di (Rust) |
|---------|-----------------|---------------------|
| 基本的なDI | ✓ | ✓ |
| リクエストスコープ | ✓ | ✓ |
| シングルトンスコープ | ✓ | ✓ |
| 依存性キャッシング | ✓ | ✓ |
| ネストされた依存性 | ✓ | ✓ |
| 依存性オーバーライド | ✓ | ✓ |
| `yield`パターン | ✓ | ⏳ Planned |
| 型安全性 | Runtime | **Compile-time** |
| パフォーマンス | 動的 | **静的・高速** |

## License

This crate is part of the Reinhardt project and follows the same dual-license structure (MIT or Apache-2.0).
