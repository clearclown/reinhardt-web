# Reinhardt

DjangoとDjango REST Framework、FastAPIにインスパイアされた、Rust用フルスタックAPIフレームワーク。

## 概要

ReinhardtはDjangoの堅牢なWebフレームワーク、Django REST Frameworkの強力なAPI機能、そしてFastAPIのモダンな開発体験のベストプラクティスを、Rustエコシステム向けに再構築したものです。本番環境向けのREST APIを構築するための完全なバッテリーインクルードソリューションを提供します。

## 特徴

- **フルスタックAPI開発**: RESTful APIを構築するために必要なすべてを1つのフレームワークで
- **Djangoインスパイアのアーキテクチャ**: Python/Djangoから移行する開発者にとって馴染み深いパターン
- **FastAPIインスパイアのエルゴノミクス**: 型安全なパラメータ抽出とdependency injectionによるモダンな開発体験
- **型安全**: Rustの型システムを活用したコンパイル時の保証
- **バッテリーインクルード**: 認証、シリアライゼーション、ORM、ルーティングなどを標準搭載
- **ハイパフォーマンス**: Rustのゼロコスト抽象化に基づいて構築
- **自動OpenAPI**: Rustの型から自動的にOpenAPI 3.0スキーマを生成

## インストール

Reinhardtは、プロジェクトの規模に合わせて3つのフレーバーを提供しています:

### Reinhardt Micro - マイクロサービス向け

軽量で高速、シンプルなAPIやマイクロサービスに最適:

```toml
[dependencies]
reinhardt-micro = "0.1.0"
```

### Reinhardt Standard - バランス型

ほとんどのプロジェクトに適したデフォルト構成:

```toml
[dependencies]
reinhardt = "0.1.0"
# 以下と同等: reinhardt = { version = "0.1.0", features = ["standard"] }
```

### Reinhardt Full - 全機能搭載

全ての機能を有効化、Djangoスタイルのバッテリーインクルード:

```toml
[dependencies]
reinhardt = { version = "0.1.0", features = ["full"] }
```

### カスタム構成

必要に応じて機能を組み合わせる:

```toml
[dependencies]
# ルーティングとパラメータのみの最小構成
reinhardt = { version = "0.1.0", default-features = false, features = ["minimal"] }

# データベースサポートを追加
reinhardt = { version = "0.1.0", default-features = false, features = ["minimal", "database"] }

# Standardに追加機能
reinhardt = { version = "0.1.0", features = ["standard", "websockets", "graphql"] }
```

## クイックスタート

```rust
// 例は近日公開予定
```

## 適切なフレーバーの選択

| 機能                 | Micro      | Standard  | Full    |
| -------------------- | ---------- | --------- | ------- |
| バイナリサイズ       | ~5-10 MB   | ~20-30 MB | ~50+ MB |
| コンパイル時間       | 高速       | 中程度    | 低速    |
| **コア機能**         |
| ルーティング         | ✅         | ✅        | ✅      |
| パラメータ抽出       | ✅         | ✅        | ✅      |
| Dependency Injection | ✅         | ✅        | ✅      |
| **標準機能**         |
| ORM                  | オプション | ✅        | ✅      |
| シリアライザ         | ❌         | ✅        | ✅      |
| ビューセット         | ❌         | ✅        | ✅      |
| 認証                 | ❌         | ✅        | ✅      |
| ページネーション     | ❌         | ✅        | ✅      |
| **高度な機能**       |
| 管理画面             | ❌         | ❌        | ✅      |
| GraphQL              | ❌         | ❌        | ✅      |
| WebSocket            | ❌         | ❌        | ✅      |
| 国際化               | ❌         | ❌        | ✅      |
| **ユースケース**     |
| マイクロサービス     | ✅         | ⚠️        | ❌      |
| REST API             | ✅         | ✅        | ✅      |
| フルアプリケーション | ❌         | ✅        | ✅      |
| 複雑なシステム       | ❌         | ⚠️        | ✅      |

**凡例**: ✅ 推奨 • ⚠️ 可能だが最適ではない • ❌ 非推奨

## コンポーネント

Reinhardtには以下のコアコンポーネントが含まれています:

### コアフレームワーク

- **ORM**: QuerySet APIを備えたデータベース抽象化レイヤー
- **シリアライザ**: 型安全なデータシリアライゼーションとバリデーション
- **ビューセット**: APIエンドポイント用の合成可能なビュー
- **ルーター**: 自動URL構成
- **認証**: JWT認証とパーミッションシステム
- **ミドルウェア**: リクエスト/レスポンス処理パイプライン

### REST API機能 (reinhardt-rest)

- **認証**: JWT、Token、Session、Basic認証
- **ルーティング**: ViewSetの自動URLルーティング
- **ブラウザブルAPI**: API探索用のHTMLインターフェース
- **スキーマ生成**: OpenAPI/Swaggerドキュメント
- **ページネーション**: PageNumber、LimitOffset、Cursorページネーション
- **フィルタリング**: クエリセット用のSearchFilterとOrderingFilter
- **スロットリング**: レート制限 (AnonRateThrottle、UserRateThrottle、ScopedRateThrottle)
- **シグナル**: イベント駆動フック (pre_save、post_save、pre_delete、post_delete、m2m_changed)

### FastAPIインスパイアの機能 (NEW!)

- **パラメータ抽出**: 型安全な`Path<T>`、`Query<T>`、`Header<T>`、`Cookie<T>`、`Json<T>`、`Form<T>`抽出器
- **Dependency Injection**: `Depends<T>`、リクエストスコープ、キャッシングを備えたFastAPIスタイルのDIシステム
- **自動スキーマ生成**: `#[derive(Schema)]`でRustの型からOpenAPIスキーマを導出
- **関数ベースのエンドポイント**: APIルートを定義するための人間工学的な`#[endpoint]`マクロ (近日公開)
- **バックグラウンドタスク**: シンプルなバックグラウンドタスク実行

## ドキュメント

- 📚 [Getting Started Guide](docs/GETTING_STARTED.md) - 初心者向けステップバイステップチュートリアル
- 🎛️ [Feature Flags Guide](docs/FEATURE_FLAGS.md) - きめ細かい機能制御でビルドを最適化
- 📖 [API Reference](https://docs.rs/reinhardt) (近日公開予定)

## ライセンス

以下のいずれかのライセンスで提供されます:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) または http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) または http://opensource.org/licenses/MIT)

お好きな方をお選びください。

### サードパーティの帰属表示

このプロジェクトは以下にインスパイアされています:

- [Django](https://www.djangoproject.com/) (BSD 3-Clause License)
- [Django REST Framework](https://www.django-rest-framework.org/) (BSD 3-Clause License)
- [FastAPI](https://fastapi.tiangolo.com/) (MIT License)

詳細な帰属表示については[THIRD-PARTY-NOTICES](THIRD-PARTY-NOTICES)を参照してください。

**注意:** このプロジェクトはDjango Software Foundation、Encode OSS Ltd.、またはSebastián Ramírez氏(FastAPI作者)と提携または承認されたものではありません。

## コントリビューション

コントリビューションを歓迎します！お気軽にプルリクエストを送信してください。
