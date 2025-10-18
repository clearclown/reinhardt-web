# チュートリアル 4: 認証とパーミッション

APIを保護するために、認証とパーミッションシステムを実装します。

## パーミッションシステム

Reinhardtは柔軟なパーミッションシステムを提供します。`reinhardt-auth`クレートで定義されています。

### 組み込みパーミッション

```rust
use reinhardt_auth::permissions::{
    Permission, PermissionContext,
    AllowAny, IsAuthenticated, IsAdminUser, IsActiveUser, IsAuthenticatedOrReadOnly
};

// 全てのリクエストを許可
let perm = AllowAny;

// 認証が必要
let perm = IsAuthenticated;

// 認証済みユーザーまたは読み取り専用
let perm = IsAuthenticatedOrReadOnly;

// 管理者のみ
let perm = IsAdminUser;

// アクティブなユーザーのみ
let perm = IsActiveUser;
```

### パーミッションの仕組み

パーミッションは`PermissionContext`を使用してチェックされます:

```rust
use reinhardt_auth::permissions::{Permission, PermissionContext};
use reinhardt_core::Request;
use async_trait::async_trait;

// カスタムパーミッションの例
struct IsOwner;

#[async_trait]
impl Permission for IsOwner {
    async fn has_permission(&self, context: &PermissionContext<'_>) -> bool {
        // カスタムロジックを実装
        context.is_authenticated && context.is_active
    }
}
```

### パーミッションの使用

ハンドラでパーミッションをチェック:

```rust
use reinhardt_core::{Request, Response, Result, Error};
use reinhardt_auth::permissions::{Permission, PermissionContext, IsAuthenticated};

async fn protected_handler(request: Request) -> Result<Response> {
    let permission = IsAuthenticated;

    // PermissionContextを作成（実際の実装では認証状態を確認）
    let context = PermissionContext {
        request: &request,
        is_authenticated: true,  // 実際の認証状態
        is_admin: false,
        is_active: true,
    };

    // パーミッションチェック
    if !permission.has_permission(&context).await {
        return Err(Error::Http("Forbidden".to_string()));
    }

    Response::ok().with_json(&serde_json::json!({"message": "Authorized"}))
}
```

## 複合パーミッション

複数のパーミッションを組み合わせる:

```rust
use reinhardt_auth::permissions::{AndPermission, OrPermission, NotPermission};

// 全てのパーミッションが必要
let and_perm = AndPermission::new()
    .add(Box::new(IsAuthenticated))
    .add(Box::new(IsActiveUser));

// いずれかのパーミッションが必要
let or_perm = OrPermission::new()
    .add(Box::new(IsAdminUser))
    .add(Box::new(IsOwner));

// パーミッションを反転
let not_perm = NotPermission::new(Box::new(IsAuthenticated));
```

## カスタムパーミッションの実装

オブジェクトレベルのパーミッション:

```rust
use reinhardt_auth::permissions::{Permission, PermissionContext};
use async_trait::async_trait;

struct IsOwnerOrReadOnly;

#[async_trait]
impl Permission for IsOwnerOrReadOnly {
    async fn has_permission(&self, context: &PermissionContext<'_>) -> bool {
        // 読み取りメソッドは誰でもOK
        if matches!(context.request.method.as_str(), "GET" | "HEAD" | "OPTIONS") {
            return true;
        }

        // 書き込みメソッドは認証済みかつアクティブなユーザーのみ
        context.is_authenticated && context.is_active
    }
}
```

## まとめ

このチュートリアルで学んだこと:

1. 組み込みパーミッションの使用
2. カスタムパーミッションの実装
3. PermissionContextの使用
4. 複合パーミッションの作成

次のチュートリアル: [チュートリアル 5: リレーションシップとハイパーリンクAPI](5-relationships-and-hyperlinked-apis.md)
