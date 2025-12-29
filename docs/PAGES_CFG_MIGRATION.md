# reinhardt-pages cfg 簡略化マイグレーションガイド

このガイドでは、reinhardt-pages の cfg 簡略化機能を既存プロジェクトに導入する方法を説明します。

## 概要

### 変更内容

1. **prelude モジュール**: よく使う型を統一インポート
2. **platform モジュール**: WASM/native で共通の型エイリアス
3. **cfg_aliases**: `#[cfg(wasm)]` と `#[cfg(native)]` ショートカット
4. **page! マクロ改善**: サーバー側でイベントハンドラを自動的に無視

### メリット

- `#[cfg(target_arch = "wasm32")]` の記述が大幅に削減
- インポートの重複が解消
- コンポーネントコードの可読性が向上

---

## マイグレーション手順

### Step 1: Cargo.toml の更新

`[build-dependencies]` セクションに `cfg_aliases` を追加:

```toml
[build-dependencies]
cfg_aliases = "0.2"
```

### Step 2: build.rs の作成/更新

プロジェクトルートに `build.rs` がない場合は作成、ある場合は更新:

```rust
use cfg_aliases::cfg_aliases;

fn main() {
    // Rust 2024 edition requires explicit check-cfg declarations
    println!("cargo::rustc-check-cfg=cfg(wasm)");
    println!("cargo::rustc-check-cfg=cfg(native)");

    cfg_aliases! {
        wasm: { target_arch = "wasm32" },
        native: { not(target_arch = "wasm32") },
    }
}
```

### Step 3: インポートの更新

#### Before

```rust
use reinhardt_pages::{Signal, View, use_state};
use reinhardt_pages::component::{ElementView, IntoView};
use reinhardt_pages::reactive::{Effect, Memo};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;
```

#### After

```rust
use reinhardt_pages::prelude::*;
// spawn_local も prelude に含まれています (WASM のみ)
```

または reinhardt クレート経由:

```rust
use reinhardt::pages::prelude::*;
```

### Step 4: cfg 属性の短縮

#### Before

```rust
#[cfg(target_arch = "wasm32")]
mod client;

#[cfg(not(target_arch = "wasm32"))]
mod server;
```

#### After

```rust
#[cfg(wasm)]
mod client;

#[cfg(native)]
mod server;
```

### Step 5: イベントハンドラの簡略化

`page!` マクロ内のイベントハンドラは自動的に処理されるようになりました。

#### Before (手動で条件分岐)

```rust
pub fn button(on_click: Signal<bool>) -> View {
    #[cfg(target_arch = "wasm32")]
    {
        page!(|| {
            button {
                @click: move |_| { on_click.set(true); },
                "Click me"
            }
        })
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = on_click; // 未使用警告を抑制
        page!(|| {
            button { "Click me" }
        })
    }
}
```

#### After (自動処理)

```rust
pub fn button(on_click: Signal<bool>) -> View {
    // マクロが自動的にサーバー側でイベントハンドラを無視
    page!(|| {
        button {
            @click: move |_| { on_click.set(true); },
            "Click me"
        }
    })
}
```

### Step 6: platform モジュールの活用 (オプション)

プラットフォーム固有の型を抽象化したい場合:

```rust
use reinhardt_pages::platform::Event;

// WASM: web_sys::Event
// Native: DummyEvent
fn handle_event(_event: Event) {
    // 処理
}
```

---

## チェックリスト

- [ ] `Cargo.toml` に `cfg_aliases = "0.2"` を追加
- [ ] `build.rs` で `cfg_aliases!` を設定
- [ ] インポートを `prelude::*` に統一
- [ ] `#[cfg(target_arch = "wasm32")]` を `#[cfg(wasm)]` に変更
- [ ] `#[cfg(not(target_arch = "wasm32"))]` を `#[cfg(native)]` に変更
- [ ] コンポーネント内の重複イベントハンドラブロックを削除
- [ ] `cargo check` で動作確認

---

## トラブルシューティング

### `unknown cfg: wasm` 警告が出る

`build.rs` で `cargo::rustc-check-cfg` を設定してください:

```rust
println!("cargo::rustc-check-cfg=cfg(wasm)");
println!("cargo::rustc-check-cfg=cfg(native)");
```

### prelude のインポートでコンフリクトが発生

特定の型のみをインポートしたい場合は、明示的にインポートしてください:

```rust
use reinhardt_pages::prelude::{Signal, View, use_state};
// prelude の他の型は使用しない
```

### イベントハンドラのキャプチャ変数で警告が出る

`page!` マクロは自動的にキャプチャ変数の警告を抑制しますが、マクロ外で定義した変数については手動で対処が必要な場合があります:

```rust
#[cfg(wasm)]
let handler = move |_| { /* ... */ };

#[cfg(native)]
let handler = |_: reinhardt_pages::platform::Event| {};
```

---

## 関連ドキュメント

- [reinhardt-pages README](../crates/reinhardt-pages/README.md)
- [Feature Flags Guide](FEATURE_FLAGS.md)
- [Getting Started Guide](GETTING_STARTED.md)
