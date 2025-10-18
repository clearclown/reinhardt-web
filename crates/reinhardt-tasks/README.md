# reinhardt-tasks

Background task processing

## Overview

Background task queue for executing long-running or scheduled tasks asynchronously.

Supports task scheduling, retries, task priorities, and multiple worker processes.

## Features

### Implemented ✓

#### Core Task System
- **Task Trait**: 基本的なタスクインターフェース
  - タスクID (`TaskId`): UUID ベースの一意識別子
  - タスク名とタスク優先度の管理
  - 優先度範囲: 0-9 (デフォルト: 5)
- **TaskExecutor Trait**: 非同期タスク実行インターフェース
- **TaskStatus**: タスクのライフサイクル管理
  - `Pending`: 待機中
  - `Running`: 実行中
  - `Success`: 成功
  - `Failure`: 失敗
  - `Retry`: リトライ中

#### Task Backends
- **TaskBackend Trait**: タスクバックエンドの抽象化インターフェース
  - タスクのエンキュー
  - タスクステータスの取得
- **DummyBackend**: テスト用ダミーバックエンド
  - 常に成功を返すシンプルな実装
- **ImmediateBackend**: 即座に実行するバックエンド
  - 同期的なタスク実行用

#### Task Queue
- **TaskQueue**: タスクキュー管理
  - 設定可能なキュー名
  - リトライ回数の設定 (デフォルト: 3回)
  - バックエンドを介したタスクのエンキュー
- **QueueConfig**: キュー設定
  - カスタマイズ可能なキュー名
  - 最大リトライ回数の設定

#### Task Scheduling
- **Scheduler**: タスクスケジューラー
  - タスクとスケジュールの登録
  - スケジュールに基づいたタスク実行の基盤
- **Schedule Trait**: スケジュールインターフェース
  - 次回実行時刻の計算
- **CronSchedule**: Cron式ベースのスケジュール
  - Cron式の保持と管理

#### Worker System
- **Worker**: タスクワーカー
  - 並行実行数の設定 (デフォルト: 4)
  - バックエンドからのタスク取得と実行
  - グレースフルシャットダウン
- **WorkerConfig**: ワーカー設定
  - ワーカー名の設定
  - 並行実行数のカスタマイズ

#### Result Handling
- **TaskOutput**: タスク実行結果
  - タスクIDと結果の文字列表現
- **TaskResult**: タスク結果型
  - Result型によるエラーハンドリング

#### Error Handling
- **TaskError**: タスク関連エラー
  - 実行失敗 (`ExecutionFailed`)
  - タスク未発見 (`TaskNotFound`)
  - キューエラー (`QueueError`)
  - シリアライゼーションエラー (`SerializationFailed`)
  - タイムアウト (`Timeout`)
  - 最大リトライ超過 (`MaxRetriesExceeded`)
- **TaskExecutionError**: バックエンド実行エラー
  - 実行失敗、タスク未発見、バックエンドエラー

### Planned

- **Cron式パーサー**: CronScheduleの`next_run()`メソッドの実装
- **実際のタスク実行ロジック**: Scheduler、Worker、TaskQueueの実行ロジック
- **永続化バックエンド**: Redis、Database等のバックエンド実装
- **タスクチェーン**: 複数タスクの連鎖実行
- **エクスポネンシャルバックオフ**: リトライ時の待機時間制御
- **タスク結果の永続化**: 実行結果の保存と取得
- **分散タスク実行**: 複数ワーカーでのタスク分散処理
