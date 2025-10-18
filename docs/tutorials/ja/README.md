# Reinhardt チュートリアル

Reinhardtフレームワークの使い方を学ぶためのチュートリアル集です。

## 基礎編

### [クイックスタート](rest/quickstart.md)

最初のReinhardt APIを5分で作成します。

### [1. シリアライゼーション](rest/1-serialization.md)

データのシリアライゼーションとバリデーションについて学びます。

### [2. リクエストとレスポンス](rest/2-requests-and-responses.md)

`Request`と`Response`オブジェクトの使い方を理解します。

### [3. クラスベースのビュー](rest/3-class-based-views.md)

ジェネリックビューを使って、よりクリーンなコードを書きます。

### [4. 認証とパーミッション](rest/4-authentication-and-permissions.md)

APIに認証とパーミッションを追加します。

### [5. リレーションシップとハイパーリンクAPI](rest/5-relationships-and-hyperlinked-apis.md)

エンティティ間の関係を扱い、ハイパーリンクAPIを構築します。

### [6. ViewSetとRouter](rest/6-viewsets-and-routers.md)

ViewSetとRouterを使って、効率的にAPIを構築します。

## 応用編

### [7. Dependency Injection](07-dependency-injection.md) ✨ 新規

FastAPI風のDependency Injectionシステムの使い方を学びます。

主なトピック:

- `Injectable`トレイトの実装
- ネストした依存性
- シングルトンとリクエストスコープ
- インフラストラクチャとの統合 (DB、キャッシュ、セッション)
- ViewSetとMiddlewareでのDI使用
- テストでのDI活用

## 学習の進め方

1. **初心者**: クイックスタートから順番に進むことをお勧めします
2. **経験者**: 興味のあるトピックから始めても構いません
3. **実践**: 各チュートリアルのコード例を実際に試してみてください

## 追加リソース

- [API リファレンス](../../api/README.md)
- [Getting Started Guide](../../GETTING_STARTED.md)
- [Feature Flags Guide](../../FEATURE_FLAGS.md)

## フィードバック

チュートリアルに関するフィードバックやご質問は、[GitHub Issues](https://github.com/your-org/reinhardt/issues)までお願いします。
