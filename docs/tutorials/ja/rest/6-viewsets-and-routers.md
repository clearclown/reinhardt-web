# チュートリアル 6: ViewSetとRouter

ViewSetとRouterを使用して、APIの構築に必要なコードの量を削減します。

## ViewSetの使用

ViewSetはRESTful APIの一般的なパターンを簡潔に実装できます。

### ModelViewSet

完全なCRUD操作を提供:

```rust
use reinhardt::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Snippet {
    id: i64,
    code: String,
    language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SnippetSerializer {
    id: i64,
    code: String,
    language: String,
}

// ViewSetを作成
let snippet_viewset = ModelViewSet::<Snippet, SnippetSerializer>::new("snippet");
```

### ReadOnlyModelViewSet

読み取り専用操作のみを提供:

```rust
use reinhardt::prelude::*;

let snippet_viewset = ReadOnlyModelViewSet::<Snippet, SnippetSerializer>::new("snippet");
```

## Routerの使用

ViewSetをルーターに登録して自動的にURLを生成:

```rust
use reinhardt::prelude::*;

#[tokio::main]
async fn main() {
    let mut router = DefaultRouter::new();

    // ViewSetを登録
    let snippet_viewset = ModelViewSet::<Snippet, SnippetSerializer>::new("snippet");
    let user_viewset = ReadOnlyModelViewSet::<User, UserSerializer>::new("user");

    router.register_viewset("snippets", snippet_viewset);
    router.register_viewset("users", user_viewset);

    // 以下のURLが自動生成されます:
    // GET/POST    /snippets/           - リスト/作成
    // GET/PUT/PATCH/DELETE /snippets/{id}/ - 詳細/更新/削除
    // GET         /users/              - リスト
    // GET         /users/{id}/         - 詳細
}
```

## 自動URL生成

ルーターはViewSetから自動的にURLパターンを生成します:

| HTTP Method | URL Pattern     | ViewSet Action | 説明                     |
| ----------- | --------------- | -------------- | ------------------------ |
| GET         | /{prefix}/      | list           | オブジェクトのリスト     |
| POST        | /{prefix}/      | create         | 新しいオブジェクトの作成 |
| GET         | /{prefix}/{id}/ | retrieve       | 特定のオブジェクトの取得 |
| PUT         | /{prefix}/{id}/ | update         | オブジェクトの更新       |
| PATCH       | /{prefix}/{id}/ | partial_update | オブジェクトの部分更新   |
| DELETE      | /{prefix}/{id}/ | destroy        | オブジェクトの削除       |

## ViewSetの利点

1. **少ないコード**: CRUD操作が自動的に実装される
2. **一貫性**: 標準的なRESTパターンに従う
3. **保守性**: ビジネスロジックに集中できる
4. **自動URL生成**: ルーティング設定が不要

## ビュー vs ViewSet

### ビューを使用する場合

- 単純なエンドポイント
- カスタムロジックが多い
- 標準的なCRUDパターンに従わない

### ViewSetを使用する場合

- 標準的なRESTful API
- 複数のエンドポイント（リスト、詳細等）
- コードの簡潔性が重要

## 完全な例

```rust
use reinhardt::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Snippet {
    id: i64,
    title: String,
    code: String,
    language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SnippetSerializer {
    id: i64,
    title: String,
    code: String,
    language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: i64,
    username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserSerializer {
    id: i64,
    username: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut router = DefaultRouter::new();

    // ViewSetを登録
    let snippet_viewset = ModelViewSet::<Snippet, SnippetSerializer>::new("snippet");
    let user_viewset = ReadOnlyModelViewSet::<User, UserSerializer>::new("user");

    router.register_viewset("snippets", snippet_viewset);
    router.register_viewset("users", user_viewset);

    println!("API endpoints:");
    println!("  GET/POST    /snippets/");
    println!("  GET/PUT/PATCH/DELETE /snippets/{{id}}/");
    println!("  GET         /users/");
    println!("  GET         /users/{{id}}/");

    Ok(())
}
```

## まとめ

このチュートリアルシリーズで学んだこと:

1. **シリアライゼーション** - データのシリアライズとバリデーション
2. **リクエストとレスポンス** - HTTP処理の基本
3. **構造体ベースのビュー** - ジェネリックビューの使用
4. **認証とパーミッション** - APIの保護
5. **ハイパーリンクAPI** - URLの逆引きとリレーションシップ
6. **ViewSetとRouter** - 効率的なAPI構築

これらの知識を使用して、本格的なRESTful APIを構築できます！

## 次のステップ

- より高度なトピックについては、[API Reference](../../../api/README.md)を参照
- [Dependency Injection](../../07-dependency-injection.md)を学ぶ
- [Feature Flags Guide](../../../FEATURE_FLAGS.md)でカスタマイズ方法を確認
- コミュニティに参加して質問する
