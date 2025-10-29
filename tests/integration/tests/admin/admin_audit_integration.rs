//! Admin Audit統合テスト
//!
//! DatabaseAuditLoggerを使用した実運用シナリオのテスト。
//! PostgreSQLコンテナを使用して、実際のデータベースでのログ記録、
//! クエリ、集計をテストします。

use chrono::{Duration, Utc};
use reinhardt_admin::audit::{
    AuditAction, AuditLog, AuditLogQuery, AuditLogger, DatabaseAuditLogger,
};
use reinhardt_admin::AdminDatabase;
use reinhardt_orm::DatabaseConnection;
use serde_json::json;
use std::net::IpAddr;
use std::sync::Arc;
use testcontainers::{core::WaitFor, runners::AsyncRunner, GenericImage};

/// テスト用データベースのセットアップとaudit_logsテーブルの作成
async fn setup_test_db() -> (
    testcontainers::ContainerAsync<GenericImage>,
    DatabaseAuditLogger,
) {
    // PostgreSQLコンテナを起動
    let postgres = GenericImage::new("postgres", "16-alpine")
        .with_env_var("POSTGRES_PASSWORD", "test")
        .with_env_var("POSTGRES_DB", "test_db")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .start()
        .await
        .expect("Failed to start PostgreSQL container");

    let port = postgres
        .get_host_port_ipv4(5432)
        .await
        .expect("Failed to get PostgreSQL port");

    let database_url = format!("postgres://postgres:test@localhost:{}/test_db", port);

    // DatabaseConnectionを使用して接続を作成
    let conn = DatabaseConnection::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // audit_logsテーブルを作成
    conn.execute(
        "CREATE TABLE IF NOT EXISTS audit_logs (
            id SERIAL PRIMARY KEY,
            user_id TEXT NOT NULL,
            model_name TEXT NOT NULL,
            object_id TEXT NOT NULL,
            action TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            changes TEXT,
            ip_address TEXT,
            user_agent TEXT
        )",
    )
    .await
    .expect("Failed to create audit_logs table");

    // AdminDatabaseとDatabaseAuditLoggerを作成
    let admin_db = Arc::new(AdminDatabase::new(Arc::new(conn)));
    let logger = DatabaseAuditLogger::new(admin_db, "audit_logs".to_string());

    (postgres, logger)
}

#[tokio::test]
async fn test_admin_audit_create_action() {
    let (_container, logger) = setup_test_db().await;

    // Createアクションのログを作成
    let changes = json!({
        "name": "John Doe",
        "email": "john@example.com",
        "role": "admin"
    });

    let log = AuditLog::builder()
        .user_id("admin_user".to_string())
        .model_name("User".to_string())
        .object_id("123".to_string())
        .action(AuditAction::Create)
        .changes(changes.clone())
        .ip_address("192.168.1.100".parse::<IpAddr>().unwrap())
        .user_agent("Mozilla/5.0 (Admin Client)".to_string())
        .build();

    // ログを記録
    let result = logger.log(log).await;
    assert!(result.is_ok(), "Create action logging should succeed");

    let inserted_log = result.unwrap();

    // 監査ログの内容を検証
    assert!(inserted_log.id().is_some(), "ID should be assigned");
    assert_eq!(inserted_log.user_id(), "admin_user");
    assert_eq!(inserted_log.model_name(), "User");
    assert_eq!(inserted_log.object_id(), "123");
    assert_eq!(inserted_log.action(), AuditAction::Create);
    assert_eq!(inserted_log.changes(), Some(&changes));
    assert_eq!(
        inserted_log.ip_address(),
        Some("192.168.1.100".parse::<IpAddr>().unwrap())
    );
    assert_eq!(
        inserted_log.user_agent(),
        Some("Mozilla/5.0 (Admin Client)")
    );
}

#[tokio::test]
async fn test_admin_audit_update_action() {
    let (_container, logger) = setup_test_db().await;

    // Updateアクションのログを作成（変更前後の値を記録）
    let changes = json!({
        "email": {
            "old": "old@example.com",
            "new": "new@example.com"
        },
        "role": {
            "old": "user",
            "new": "admin"
        }
    });

    let log = AuditLog::builder()
        .user_id("admin_user".to_string())
        .model_name("User".to_string())
        .object_id("456".to_string())
        .action(AuditAction::Update)
        .changes(changes.clone())
        .build();

    // ログを記録
    let result = logger.log(log).await;
    assert!(result.is_ok(), "Update action logging should succeed");

    let inserted_log = result.unwrap();

    // 監査ログの検証
    assert_eq!(inserted_log.action(), AuditAction::Update);
    assert_eq!(inserted_log.object_id(), "456");
    assert_eq!(inserted_log.changes(), Some(&changes));
}

#[tokio::test]
async fn test_admin_audit_delete_action() {
    let (_container, logger) = setup_test_db().await;

    // Deleteアクションのログを作成（削除されたデータの情報を記録）
    let changes = json!({
        "deleted_record": {
            "name": "Jane Smith",
            "email": "jane@example.com"
        }
    });

    let log = AuditLog::builder()
        .user_id("admin_user".to_string())
        .model_name("User".to_string())
        .object_id("789".to_string())
        .action(AuditAction::Delete)
        .changes(changes.clone())
        .build();

    // ログを記録
    let result = logger.log(log).await;
    assert!(result.is_ok(), "Delete action logging should succeed");

    let inserted_log = result.unwrap();

    // 監査ログの検証
    assert_eq!(inserted_log.action(), AuditAction::Delete);
    assert_eq!(inserted_log.object_id(), "789");
    assert_eq!(inserted_log.changes(), Some(&changes));
}

#[tokio::test]
async fn test_admin_audit_query_by_user() {
    let (_container, logger) = setup_test_db().await;

    // 複数ユーザーのログを記録
    for i in 1..=5 {
        let user_id = if i % 2 == 0 { "user_a" } else { "user_b" };
        let log = AuditLog::builder()
            .user_id(user_id.to_string())
            .model_name("Article".to_string())
            .object_id(i.to_string())
            .action(AuditAction::Create)
            .build();
        logger.log(log).await.unwrap();
    }

    // user_aでクエリ
    let query = AuditLogQuery::builder()
        .user_id("user_a".to_string())
        .build();

    let results = logger.query(&query).await.expect("Query should succeed");

    // user_aのログのみ取得されることを検証
    assert_eq!(results.len(), 2, "Should return 2 logs for user_a");
    for log in &results {
        assert_eq!(log.user_id(), "user_a");
    }
}

#[tokio::test]
async fn test_admin_audit_query_by_model() {
    let (_container, logger) = setup_test_db().await;

    // 異なるモデルのログを記録
    let models = vec!["User", "Article", "Comment"];
    for (i, model) in models.iter().enumerate() {
        let log = AuditLog::builder()
            .user_id("admin".to_string())
            .model_name(model.to_string())
            .object_id((i + 1).to_string())
            .action(AuditAction::Create)
            .build();
        logger.log(log).await.unwrap();
    }

    // Articleモデルでクエリ
    let query = AuditLogQuery::builder()
        .model_name("Article".to_string())
        .build();

    let results = logger.query(&query).await.expect("Query should succeed");

    // Articleのログのみ取得されることを検証
    assert_eq!(results.len(), 1, "Should return 1 log for Article model");
    assert_eq!(results[0].model_name(), "Article");
}

#[tokio::test]
async fn test_admin_audit_query_by_date_range() {
    let (_container, logger) = setup_test_db().await;

    // 過去、現在、未来のタイムスタンプでログを記録
    let now = Utc::now();
    let past = now - Duration::hours(2);
    let future = now + Duration::hours(2);

    let log_now = AuditLog::builder()
        .user_id("admin".to_string())
        .model_name("Post".to_string())
        .object_id("1".to_string())
        .action(AuditAction::Create)
        .build();

    logger.log(log_now).await.unwrap();

    // 日時範囲でクエリ（過去から未来まで - 含まれる）
    let query_include = AuditLogQuery::builder()
        .start_date(past)
        .end_date(future)
        .build();

    let results_include = logger
        .query(&query_include)
        .await
        .expect("Query should succeed");

    assert_eq!(
        results_include.len(),
        1,
        "Should return 1 log within date range"
    );

    // 日時範囲でクエリ（範囲外 - 含まれない）
    let query_exclude = AuditLogQuery::builder()
        .start_date(past - Duration::hours(10))
        .end_date(past - Duration::hours(5))
        .build();

    let results_exclude = logger
        .query(&query_exclude)
        .await
        .expect("Query should succeed");

    assert_eq!(
        results_exclude.len(),
        0,
        "Should return 0 logs outside date range"
    );
}

#[tokio::test]
async fn test_admin_audit_count() {
    let (_container, logger) = setup_test_db().await;

    // 複数のログを記録
    for i in 1..=10 {
        let log = AuditLog::builder()
            .user_id("admin".to_string())
            .model_name("Product".to_string())
            .object_id(i.to_string())
            .action(AuditAction::Create)
            .build();
        logger.log(log).await.unwrap();
    }

    // 全ログのカウント
    let query_all = AuditLogQuery::builder().build();
    let count_all = logger.count(&query_all).await.expect("Count should succeed");

    assert_eq!(count_all, 10, "Should count 10 total logs");
}

#[tokio::test]
async fn test_admin_audit_complex_query() {
    let (_container, logger) = setup_test_db().await;

    // 多様なログを記録
    let scenarios = vec![
        ("user1", "User", "1", AuditAction::Create),
        ("user1", "User", "1", AuditAction::Update),
        ("user1", "Article", "1", AuditAction::Create),
        ("user2", "User", "2", AuditAction::Create),
        ("user2", "Article", "2", AuditAction::Delete),
    ];

    for (user, model, obj_id, action) in scenarios {
        let log = AuditLog::builder()
            .user_id(user.to_string())
            .model_name(model.to_string())
            .object_id(obj_id.to_string())
            .action(action)
            .build();
        logger.log(log).await.unwrap();
    }

    // 複合クエリ: user1 + User model + Update action
    let query = AuditLogQuery::builder()
        .user_id("user1".to_string())
        .model_name("User".to_string())
        .action(AuditAction::Update)
        .build();

    let results = logger.query(&query).await.expect("Query should succeed");

    assert_eq!(
        results.len(),
        1,
        "Should return 1 log matching all filters"
    );
    assert_eq!(results[0].user_id(), "user1");
    assert_eq!(results[0].model_name(), "User");
    assert_eq!(results[0].action(), AuditAction::Update);
}

#[tokio::test]
async fn test_admin_audit_pagination() {
    let (_container, logger) = setup_test_db().await;

    // 20件のログを記録
    for i in 1..=20 {
        let log = AuditLog::builder()
            .user_id("admin".to_string())
            .model_name("Item".to_string())
            .object_id(i.to_string())
            .action(AuditAction::Create)
            .build();
        logger.log(log).await.unwrap();
        // タイムスタンプの違いを確保
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
    }

    // 1ページ目（最初の10件）
    let query_page1 = AuditLogQuery::builder().limit(10).offset(0).build();

    let results_page1 = logger.query(&query_page1).await.expect("Query should succeed");

    assert_eq!(results_page1.len(), 10, "Should return 10 logs on page 1");

    // 2ページ目（次の10件）
    let query_page2 = AuditLogQuery::builder().limit(10).offset(10).build();

    let results_page2 = logger.query(&query_page2).await.expect("Query should succeed");

    assert_eq!(results_page2.len(), 10, "Should return 10 logs on page 2");

    // ページ間でログが重複しないことを検証
    let ids_page1: Vec<_> = results_page1.iter().filter_map(|l| l.id()).collect();
    let ids_page2: Vec<_> = results_page2.iter().filter_map(|l| l.id()).collect();

    for id in &ids_page1 {
        assert!(!ids_page2.contains(id), "Pages should not overlap");
    }
}

#[tokio::test]
async fn test_admin_audit_action_variety() {
    let (_container, logger) = setup_test_db().await;

    // 様々なアクションタイプのログを記録
    let actions = vec![
        AuditAction::Create,
        AuditAction::Update,
        AuditAction::Delete,
        AuditAction::View,
        AuditAction::BulkDelete,
        AuditAction::Export,
        AuditAction::Import,
    ];

    for (i, action) in actions.iter().enumerate() {
        let log = AuditLog::builder()
            .user_id("admin".to_string())
            .model_name("Resource".to_string())
            .object_id((i + 1).to_string())
            .action(*action)
            .build();
        logger.log(log).await.unwrap();
    }

    // 各アクションタイプでクエリして検証
    for action in actions {
        let query = AuditLogQuery::builder().action(action).build();

        let results = logger.query(&query).await.expect("Query should succeed");

        assert_eq!(results.len(), 1, "Should return 1 log for {:?}", action);
        assert_eq!(results[0].action(), action);
    }
}

#[tokio::test]
async fn test_admin_audit_bulk_operations() {
    let (_container, logger) = setup_test_db().await;

    // BulkDeleteアクションのログ（複数レコードの一括削除）
    let changes = json!({
        "deleted_count": 5,
        "deleted_ids": ["1", "2", "3", "4", "5"]
    });

    let log = AuditLog::builder()
        .user_id("admin".to_string())
        .model_name("User".to_string())
        .object_id("bulk_operation_1".to_string())
        .action(AuditAction::BulkDelete)
        .changes(changes.clone())
        .build();

    let result = logger.log(log).await;
    assert!(result.is_ok(), "Bulk delete logging should succeed");

    let inserted_log = result.unwrap();

    // BulkDeleteログの検証
    assert_eq!(inserted_log.action(), AuditAction::BulkDelete);
    assert_eq!(inserted_log.changes(), Some(&changes));

    // BulkDeleteアクションでクエリ
    let query = AuditLogQuery::builder()
        .action(AuditAction::BulkDelete)
        .build();

    let results = logger.query(&query).await.expect("Query should succeed");

    assert_eq!(results.len(), 1, "Should return 1 bulk delete log");
}

#[tokio::test]
async fn test_admin_audit_export_import_actions() {
    let (_container, logger) = setup_test_db().await;

    // Exportアクションのログ
    let export_changes = json!({
        "format": "csv",
        "record_count": 100,
        "exported_at": Utc::now().to_rfc3339()
    });

    let export_log = AuditLog::builder()
        .user_id("admin".to_string())
        .model_name("User".to_string())
        .object_id("export_1".to_string())
        .action(AuditAction::Export)
        .changes(export_changes)
        .build();

    logger.log(export_log).await.unwrap();

    // Importアクションのログ
    let import_changes = json!({
        "format": "json",
        "record_count": 50,
        "imported_at": Utc::now().to_rfc3339()
    });

    let import_log = AuditLog::builder()
        .user_id("admin".to_string())
        .model_name("User".to_string())
        .object_id("import_1".to_string())
        .action(AuditAction::Import)
        .changes(import_changes)
        .build();

    logger.log(import_log).await.unwrap();

    // ExportとImportアクションでそれぞれクエリ
    let export_query = AuditLogQuery::builder()
        .action(AuditAction::Export)
        .build();

    let export_results = logger
        .query(&export_query)
        .await
        .expect("Export query should succeed");

    assert_eq!(export_results.len(), 1, "Should return 1 export log");

    let import_query = AuditLogQuery::builder()
        .action(AuditAction::Import)
        .build();

    let import_results = logger
        .query(&import_query)
        .await
        .expect("Import query should succeed");

    assert_eq!(import_results.len(), 1, "Should return 1 import log");
}

#[tokio::test]
async fn test_admin_audit_ordering() {
    let (_container, logger) = setup_test_db().await;

    // タイムスタンプが異なるログを記録
    for i in 1..=5 {
        let log = AuditLog::builder()
            .user_id("admin".to_string())
            .model_name("Task".to_string())
            .object_id(i.to_string())
            .action(AuditAction::Create)
            .build();
        logger.log(log).await.unwrap();
        // タイムスタンプの違いを確保
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }

    // 全ログをクエリ
    let query = AuditLogQuery::builder().build();

    let results = logger.query(&query).await.expect("Query should succeed");

    // 降順（最新が最初）であることを検証
    for i in 0..results.len() - 1 {
        assert!(
            results[i].timestamp() >= results[i + 1].timestamp(),
            "Results should be ordered by timestamp DESC"
        );
    }
}
