# reinhardt-graphql-macros

ReinhardtフレームワークのGraphQL-gRPC統合用Deriveマクロ

## 概要

`reinhardt-graphql-macros`は、ReinhardtフレームワークにおけるgRPCとGraphQLの統合を簡素化するための手続き型マクロを提供します。これらのマクロは、ボイラープレートを削減するために、変換コードとサブスクリプション実装を自動生成します。

## 機能

- **GrpcGraphQLConvert**: ProtobufとGraphQL型間の自動型変換
  - `From<proto::T> for T`と`From<T> for proto::T`を導出
  - `#[graphql(rename_all = "camelCase")]`によるフィールドリネーム
  - `#[graphql(skip_if = "...")]`による条件付きフィールド含有
  - `#[proto(...)]`属性によるカスタムprotobuf型マッピング

- **GrpcSubscription**: gRPCストリームからの自動GraphQLサブスクリプション
  - gRPCストリーミングメソッドをGraphQLサブスクリプションにマッピング
  - `#[grpc(service = "...", method = "...")]`によるサービスとメソッドの指定
  - `#[graphql(filter = "...")]`によるオプショナルフィルタリング
  - Rust 2024ライフタイム互換性