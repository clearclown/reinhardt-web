# チュートリアル 2: リクエストとレスポンス

ここから、Reinhardtの中核を本格的に説明していきます。いくつかの重要な構成要素を紹介しましょう。

## Requestオブジェクト

Reinhardtの`Request`オブジェクトは、HTTPリクエストをカプセル化します。`reinhardt-core`クレートで定義されています。

```rust
use reinhardt_core::Request;

// Requestオブジェクトには以下の情報が含まれます:
// - method: HTTPメソッド (GET, POST, PUT, DELETE等)
// - uri: リクエストURI
// - headers: HTTPヘッダー
// - body: リクエストボディ
// - path_params: URLパスパラメータ
```

### リクエストボディの解析

JSONボディを解析する例:

```rust
use reinhardt_core::{Request, Response, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CreateSnippet {
    code: String,
    language: Option<String>,
}

async fn create_snippet(request: Request) -> Result<Response> {
    // JSONボディをデシリアライズ
    let body_bytes = request.body_bytes();
    let data: CreateSnippet = serde_json::from_slice(&body_bytes)
        .map_err(|e| reinhardt_core::Error::Validation(format!("Invalid JSON: {}", e)))?;

    println!("Received code: {}", data.code);

    Response::ok().with_json(&data)
}
```

## Responseオブジェクト

`Response`オブジェクトは、HTTPレスポンスを構築するための便利なビルダーパターンを提供します。

```rust
use reinhardt_core::Response;

// 基本的なレスポンス
let response = Response::ok();

// JSONレスポンス
let data = serde_json::json!({"message": "Success"});
let response = Response::ok().with_json(&data)?;

// カスタムステータスコード
let response = Response::new(201);  // Created

// ヘッダー付きレスポンス
let response = Response::ok()
    .with_header("Content-Type", "application/json")
    .with_header("X-Custom-Header", "value");
```

### 便利なレスポンスビルダー

```rust
use reinhardt_core::Response;

// 200 OK
Response::ok()

// 201 Created
Response::created()

// 204 No Content
Response::no_content()

// 400 Bad Request
Response::bad_request()

// 404 Not Found
Response::not_found()

// カスタムステータス
Response::new(418)  // I'm a teapot
```

## HTTPステータスコード

明示的なステータスコードの使用:

```rust
use reinhardt_core::Response;

// 成功レスポンス
let response = Response::new(200);  // OK
let response = Response::new(201);  // Created
let response = Response::new(204);  // No Content

// クライアントエラー
let response = Response::new(400);  // Bad Request
let response = Response::new(404);  // Not Found
let response = Response::new(403);  // Forbidden

// サーバーエラー
let response = Response::new(500);  // Internal Server Error
```

## APIハンドラの実装

前のチュートリアルのスニペットAPIを、Request/Responseを使用して実装しましょう。

次のチュートリアル: [チュートリアル 3: 構造体ベースのビュー](3-class-based-views.md)
