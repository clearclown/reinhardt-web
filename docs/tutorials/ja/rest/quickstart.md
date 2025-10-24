# クイックスタート

システム内のユーザーとグループを閲覧・編集できる、管理者向けのシンプルなAPIを作成します。

## プロジェクトのセットアップ

tutorialという名前の新しいReinhardtプロジェクトを作成します。

```bash
# プロジェクトディレクトリを作成
mkdir tutorial
cd tutorial

# 新しいプロジェクトをセットアップ
cargo init --name tutorial
```

`Cargo.toml`にReinhardtの依存関係を追加します:

```toml
[dependencies]
reinhardt = { version = "0.1.0", features = ["standard", "rest", "serializers", "viewsets", "routers", "auth"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
```

> **注記**: `reinhardt`クレートの`"standard"`フィーチャーフラグに加えて、REST API機能に必要な`"rest"`、`"serializers"`、`"viewsets"`、`"routers"`、`"auth"`を追加します。すべての機能は統合された`reinhardt`クレートから提供されるため、個別の`reinhardt-*`依存関係を追加する必要はありません。フィーチャーフラグの詳細については、[Feature Flags Guide](../../../FEATURE_FLAGS.md)を参照してください。

プロジェクトのレイアウトは以下のようになります:

```
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs
```

## モデル

このクイックスタートでは、Reinhardtの組み込み`User`と`Group`モデルを使用します。これらはauth機能から提供されます。

## シリアライザ

データ表現用のシリアライザを定義します。`src/main.rs`に以下を追加します:

```rust
use reinhardt::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSerializer {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupSerializer {
    pub id: i64,
    pub name: String,
}
```

この例ではシンプルなデータ構造を使用しています。実際のアプリケーションでは、`Serializer`トレイトを実装してバリデーションとデータ変換ロジックを追加できます。

## ViewSets

ViewSetを使用してCRUD操作を実装します。`src/main.rs`に追加:

```rust
use reinhardt::prelude::*;
use std::sync::Arc;

// UserViewSet - 完全なCRUD操作
let user_viewset = ModelViewSet::<User, UserSerializer>::new("user");

// GroupViewSet - 読み取り専用
let group_viewset = ReadOnlyModelViewSet::<Group, GroupSerializer>::new("group");
```

`ModelViewSet`はすべての標準的なCRUD操作（list、retrieve、create、update、delete）を提供します。`ReadOnlyModelViewSet`はlistとretrieve操作のみを提供します。

## ルーティング

ViewSetをルーターに登録してURLを自動生成します。`src/main.rs`の完全な例:

```rust
use reinhardt::prelude::*;
use serde::{Serialize, Deserialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Group {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSerializer {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupSerializer {
    pub id: i64,
    pub name: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // ルーターを作成
    let mut router = DefaultRouter::new();

    // ViewSetを登録
    let user_viewset = ModelViewSet::<User, UserSerializer>::new("user");
    let group_viewset = ReadOnlyModelViewSet::<Group, GroupSerializer>::new("group");

    router.register_viewset("users", user_viewset);
    router.register_viewset("groups", group_viewset);

    // 以下のURLが自動的に生成されます:
    // GET/POST    /users/      - ユーザーリスト/作成
    // GET/PUT/DELETE /users/{id}/ - ユーザー詳細/更新/削除
    // GET         /groups/     - グループリスト
    // GET         /groups/{id}/ - グループ詳細

    // サーバーを起動（実装は省略）
    // reinhardt::serve(router).await

    Ok(())
}
```

ルーターは以下のURLパターンを自動的に生成します:

- `GET /users/` - ユーザーのリスト
- `POST /users/` - 新しいユーザーの作成
- `GET /users/{id}/` - 特定のユーザーの取得
- `PUT /users/{id}/` - ユーザーの更新
- `PATCH /users/{id}/` - ユーザーの部分更新
- `DELETE /users/{id}/` - ユーザーの削除
- `GET /groups/` - グループのリスト
- `GET /groups/{id}/` - 特定のグループの取得

## パーミッション（オプション）

認証とパーミッションを追加するには、`reinhardt::auth::permissions`モジュールを使用します:

```rust
use reinhardt::auth::permissions::{IsAuthenticated, IsAuthenticatedOrReadOnly};

// ViewSet作成時にパーミッションを設定できます
// 注: 現在の実装では、カスタムViewSet実装が必要です
```

## ページネーション（オプション）

大量のデータを扱う場合は、ページネーションを実装できます:

```rust
use reinhardt::rest::pagination::PageNumberPagination;

// ページネーションの設定例
let pagination = PageNumberPagination::new(10); // 1ページあたり10件
```

## APIのテスト

APIをテストするには、curlまたはhttpieを使用します:

```bash
# ユーザーのリストを取得
curl http://127.0.0.1:8000/users/

# 新しいユーザーを作成
curl -X POST http://127.0.0.1:8000/users/ \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com"}'

# 特定のユーザーを取得
curl http://127.0.0.1:8000/users/1/

# ユーザーを更新
curl -X PUT http://127.0.0.1:8000/users/1/ \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"newemail@example.com"}'

# ユーザーを削除
curl -X DELETE http://127.0.0.1:8000/users/1/
```

## まとめ

このクイックスタートでは、以下を学びました:

1. Reinhardtプロジェクトのセットアップ
2. シリアライザの定義
3. ViewSetを使用したCRUD APIの作成
4. ルーターを使用した自動URL生成

より詳細な情報は、[チュートリアル](1-serialization.md)を参照してください。
