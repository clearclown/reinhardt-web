# チュートリアル 3: 構造体ベースのビュー

関数ベースのビューではなく、構造体ベースのビューを使用してAPIビューを記述することもできます。

## ジェネリックビューの使用

Reinhardtは、一般的なRESTパターン用のジェネリックビューを提供します。

### ListAPIView

オブジェクトのリストを表示するビュー:

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

let snippets = vec![
    Snippet { id: 1, code: "print('hello')".to_string(), language: "python".to_string() },
];

let view = ListAPIView::<Snippet, SnippetSerializer>::new()
    .with_objects(snippets)
    .with_paginate_by(10);
```

## 利用可能なジェネリックビュー

Reinhardtは以下のジェネリックビューを提供します:

### 単一操作ビュー

- `ListAPIView` - オブジェクトのリスト表示 (GET)
- `CreateAPIView` - オブジェクトの作成 (POST)
- `RetrieveAPIView` - 単一オブジェクトの取得 (GET)
- `UpdateAPIView` - オブジェクトの更新 (PUT/PATCH)
- `DestroyAPIView` - オブジェクトの削除 (DELETE)

### 複合操作ビュー

- `ListCreateAPIView` - リスト表示と作成 (GET, POST)
- `RetrieveUpdateAPIView` - 取得と更新 (GET, PUT, PATCH)
- `RetrieveDestroyAPIView` - 取得と削除 (GET, DELETE)
- `RetrieveUpdateDestroyAPIView` - 取得、更新、削除 (GET, PUT, PATCH, DELETE)

## ViewSetへの移行

より複雑なAPIには、ViewSetの使用を推奨します。ViewSetは複数のアクションを1つの構造体にまとめます:

```rust
use reinhardt::prelude::*;

// ViewSetは自動的に全てのCRUD操作を提供
let viewset = ModelViewSet::<Snippet, SnippetSerializer>::new("snippet");
```

ViewSetについての詳細は、[チュートリアル 6: ViewSetとRouter](6-viewsets-and-routers.md)を参照してください。

## まとめ

このチュートリアルで学んだこと:

1. ジェネリックビューの使用方法
2. 単一操作と複合操作ビューの違い
3. ViewSetへの移行

次のチュートリアル: [チュートリアル 4: 認証とパーミッション](4-authentication-and-permissions.md)
