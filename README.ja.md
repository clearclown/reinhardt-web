# Reinhardt

DjangoとDjango REST Frameworkにインスパイアされた、Rust用フルスタックAPIフレームワーク。

## 概要

ReinhardtはDjangoの堅牢なWebフレームワークとDjango REST Frameworkの強力なAPI機能のベストプラクティスを、Rustエコシステム向けに再構築したものです。本番環境向けのREST APIを構築するための完全なバッテリーインクルードソリューションを提供します。

## 特徴

- **フルスタックAPI開発**: RESTful APIを構築するために必要なすべてを1つのフレームワークで
- **Djangoインスパイアのアーキテクチャ**: Python/Djangoから移行する開発者にとって馴染み深いパターン
- **型安全**: Rustの型システムを活用したコンパイル時の保証
- **バッテリーインクルード**: 認証、シリアライゼーション、ORM、ルーティングなどを標準搭載
- **ハイパフォーマンス**: Rustのゼロコスト抽象化に基づいて構築

## インストール

`Cargo.toml`にReinhardtを追加してください:

```toml
[dependencies]
reinhardt = "0.0.1"
```

## クイックスタート

```rust
// 例は近日公開予定
```

## コンポーネント

Reinhardtには以下のコアコンポーネントが含まれています:

- **ORM**: マイグレーション機能を備えたデータベース抽象化レイヤー
- **シリアライザ**: 型安全なデータシリアライゼーションとバリデーション
- **ビューセット**: APIエンドポイント用のクラスベースビュー
- **ルーター**: 自動URL構成
- **認証**: 組み込みの認証バックエンドとパーミッション
- **ミドルウェア**: リクエスト/レスポンス処理パイプライン

## ドキュメント

完全なドキュメントは近日公開予定です。

## ライセンス

以下のいずれかのライセンスで提供されます:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) または http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) または http://opensource.org/licenses/MIT)

お好きな方をお選びください。

### サードパーティの帰属表示

このプロジェクトは[Django](https://www.djangoproject.com/)と[Django REST Framework](https://www.django-rest-framework.org/)にインスパイアされています。両者はBSD 3-Clause Licenseでライセンスされています。詳細な帰属表示については[THIRD-PARTY-NOTICES](THIRD-PARTY-NOTICES)を参照してください。

**注意:** このプロジェクトはDjango Software FoundationやEncode OSS Ltd.と提携または承認されたものではありません。

## コントリビューション

コントリビューションを歓迎します！お気軽にプルリクエストを送信してください。
