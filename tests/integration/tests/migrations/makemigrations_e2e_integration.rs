//! makemigrations Command E2E Integration Tests
//!
//! Tests the complete end-to-end workflow of makemigrations functionality,
//! including file generation and filesystem operations.
//!
//! **Test Coverage:**
//! - Normal Cases (NC-01 ~ NC-20): Basic to advanced migration generation
//! - Error Cases (EC-01 ~ EC-05): Error handling validation
//! - Edge Cases (EDG-01 ~ EDG-14): Special scenarios and options
//!
//! **Test Approach:**
//! - Uses FilesystemRepository for actual file generation
//! - TempDir for isolated filesystem operations
//! - Verifies generated migration files
//! - Tests migration executability on real databases

use reinhardt_backends::DatabaseConnection;
use reinhardt_migrations::{
	AutoMigrationGenerator, ColumnDefinition, FieldType, FilesystemRepository,
	FilesystemSource, Migration, MigrationNamer, MigrationNumbering, MigrationService, Operation,
	autodetector::ProjectState,
};
use reinhardt_migrations::schema_diff::{ColumnSchema, DatabaseSchema, TableSchema};
use reinhardt_test::fixtures::postgres_container;
use rstest::*;
use serial_test::serial;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tempfile::TempDir;
use testcontainers::ContainerAsync;
use testcontainers::core::WaitFor;
use testcontainers::GenericImage;
use tokio::sync::Mutex;

// ============================================================================
// Helper Functions
// ============================================================================

/// Helper to create a simple schema with a todos table
fn create_todos_schema() -> DatabaseSchema {
	let mut schema = DatabaseSchema::default();
	let mut table = TableSchema {
		name: "todos",
		columns: BTreeMap::new(),
		indexes: Vec::new(),
		constraints: Vec::new(),
	};

	table.columns.insert(
		"id".to_string(),
		ColumnSchema {
			name: "id",
			data_type: FieldType::Integer,
			nullable: false,
			default: None,
			primary_key: true,
			auto_increment: true,
		},
	);

	table.columns.insert(
		"title".to_string(),
		ColumnSchema {
			name: "title",
			data_type: FieldType::VarChar(255),
			nullable: false,
			default: None,
			primary_key: false,
			auto_increment: false,
		},
	);

	table.columns.insert(
		"completed".to_string(),
		ColumnSchema {
			name: "completed",
			data_type: FieldType::Boolean,
			nullable: false,
			default: Some("false".to_string()),
			primary_key: false,
			auto_increment: false,
		},
	);

	schema.tables.insert("todos".to_string(), table);
	schema
}

/// Verify that a migration file exists at the specified path
fn verify_migration_file_exists(migrations_dir: &Path, app_label: &str, expected_number: &str) -> bool {
	let app_dir = migrations_dir.join(app_label);
	if !app_dir.exists() {
		return false;
	}

	std::fs::read_dir(&app_dir)
		.ok()
		.and_then(|entries| {
			entries
				.filter_map(Result::ok)
				.find(|entry| {
					entry
						.file_name()
						.to_str()
						.map(|name| name.starts_with(expected_number) && name.ends_with(".rs"))
						.unwrap_or(false)
				})
		})
		.is_some()
}

/// Read and parse a generated migration file
fn read_migration_file(path: &Path) -> Result<String, std::io::Error> {
	std::fs::read_to_string(path)
}

// ============================================================================
// Normal Cases (NC-01 ~ NC-20)
// ============================================================================

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_01_new_model_creates_create_table_migration() {
	// Test: 新規モデル作成からのCreateTable生成（E2E）
	// ファイルシステムへの書き込みまで検証

	let temp_dir = TempDir::new().unwrap();
	let migrations_dir = temp_dir.path().join("migrations");

	let app_label = "todos";
	let empty_schema = DatabaseSchema::default();
	let target_schema = create_todos_schema();

	// Setup FilesystemRepository and Service
	let source = Arc::new(FilesystemSource::new(migrations_dir.clone()));
	let repository = Arc::new(Mutex::new(FilesystemRepository::new(
		migrations_dir.clone(),
	)));
	let service = MigrationService::new(source.clone(), repository.clone());

	// Generate migration
	let generator = AutoMigrationGenerator::new(target_schema.clone(), repository.clone());
	let result = generator
		.generate(app_label, empty_schema.clone())
		.await
		.expect("First makemigrations should succeed");

	// Build migration name
	let migration_number = MigrationNumbering::next_number(&migrations_dir, app_label);
	assert_eq!(migration_number, "0001", "First migration should be 0001");

	let migration_name = format!("{}_{}", migration_number, "initial");

	// Create Migration struct
	let migration = Migration {
		app_label: Box::leak(app_label.to_string().into_boxed_str()),
		name: Box::leak(migration_name.clone().into_boxed_str()),
		operations: result.operations.clone(),
		dependencies: Vec::new(),
		atomic: true,
		replaces: Vec::new(),
		initial: Some(true),
	};

	// Save migration to filesystem
	service
		.save_migration(&migration)
		.await
		.expect("Failed to save migration");

	// Verify: File exists
	assert!(
		verify_migration_file_exists(&migrations_dir, app_label, "0001"),
		"Migration file should exist"
	);

	// Verify: File content contains CreateTable
	let migration_file_path = migrations_dir
		.join(app_label)
		.join(format!("{}.rs", migration_name));

	let file_content = read_migration_file(&migration_file_path)
		.expect("Failed to read migration file");

	assert!(
		file_content.contains("CreateTable"),
		"Migration file should contain CreateTable operation"
	);
	assert!(
		file_content.contains("todos"),
		"Migration file should reference 'todos' table"
	);
	assert!(
		file_content.contains("initial: Some(true)"),
		"Migration file should have initial flag set"
	);
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_02_field_addition_creates_add_column_migration() {
	// Test: フィールド追加からのAddColumn生成（E2E）
	todo!("Implement E2E field addition test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_03_field_deletion_creates_drop_column_migration() {
	// Test: フィールド削除からのDropColumn生成（E2E）
	todo!("Implement E2E field deletion test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_04_field_type_change_creates_alter_column_migration() {
	// Test: フィールド型変更からのAlterColumn生成（E2E）
	todo!("Implement E2E field type change test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_05_field_rename_creates_rename_column_migration() {
	// Test: フィールドリネームからのRenameColumn生成（E2E）
	todo!("Implement E2E field rename test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_06_index_addition_creates_create_index_migration() {
	// Test: インデックス追加からのCreateIndex生成（E2E）
	todo!("Implement E2E index addition test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_07_foreign_key_addition_creates_add_column_and_constraint() {
	// Test: ForeignKey追加からのAddColumn + AddConstraint生成（E2E）
	todo!("Implement E2E foreign key test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_08_many_to_many_creates_junction_table() {
	// Test: ManyToMany追加からのCreateTable（junction table）生成（E2E）
	todo!("Implement E2E many-to-many test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_09_initial_migration_correctness() {
	// Test: 初期マイグレーション（0001_initial）の正しい生成（E2E）
	todo!("Implement E2E initial migration test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_10_sequential_migrations_dependency_chain() {
	// Test: 連続マイグレーション（0001 → 0002 → 0003）の正しい生成（E2E）
	todo!("Implement E2E sequential migrations test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_11_generated_migration_executability() {
	// Test: 生成されたマイグレーションの実行可能性検証（E2E）
	todo!("Implement E2E executability test with real database")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_12_one_to_one_creates_unique_foreign_key() {
	// Test: OneToOne追加からの適切なマイグレーション生成（E2E）
	todo!("Implement E2E one-to-one test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_13_default_value_addition_creates_alter_column() {
	// Test: デフォルト値追加からのAlterColumn生成（E2E）
	todo!("Implement E2E default value test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_14_null_constraint_change_creates_alter_column() {
	// Test: NULL制約変更からのAlterColumn生成（E2E）
	todo!("Implement E2E null constraint test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_15_unique_constraint_addition_creates_add_constraint() {
	// Test: UNIQUE制約追加からのAddConstraint生成（E2E）
	todo!("Implement E2E unique constraint test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_16_index_deletion_creates_drop_index() {
	// Test: インデックス削除からのDropIndex生成（E2E）
	todo!("Implement E2E index deletion test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_17_constraint_deletion_creates_drop_constraint() {
	// Test: 制約削除からのDropConstraint生成（E2E）
	todo!("Implement E2E constraint deletion test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_18_multiple_changes_in_single_migration() {
	// Test: 複数の変更を含むマイグレーション生成（E2E）
	todo!("Implement E2E multiple changes test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_19_multi_app_migrations_generation() {
	// Test: 複数アプリの同時マイグレーション生成（E2E）
	todo!("Implement E2E multi-app test")
}

#[tokio::test]
#[serial(makemigrations_e2e)]
async fn nc_20_data_preservation_verification() {
	// Test: データ保持検証（既存データが失われない）（E2E）
	todo!("Implement E2E data preservation test")
}

// ============================================================================
// Error Cases (EC-01 ~ EC-05)
// ============================================================================

#[tokio::test]
#[serial(makemigrations_e2e_errors)]
async fn ec_01_no_models_error() {
	// Test: モデルが存在しない場合のエラー（E2E）
	todo!("Implement E2E no models error test")
}

#[tokio::test]
#[serial(makemigrations_e2e_errors)]
async fn ec_02_empty_flag_without_app_label_error() {
	// Test: --empty指定時にapp_labelがない場合のエラー（E2E）
	todo!("Implement E2E empty flag error test")
}

#[tokio::test]
#[serial(makemigrations_e2e_errors)]
async fn ec_03_from_state_construction_failure_error() {
	// Test: from_state構築失敗時のエラー（E2E）
	todo!("Implement E2E from_state failure test")
}

#[tokio::test]
#[serial(makemigrations_e2e_errors)]
async fn ec_04_invalid_field_definition_error() {
	// Test: 無効なフィールド定義のエラー（E2E）
	todo!("Implement E2E invalid field error test")
}

#[tokio::test]
#[serial(makemigrations_e2e_errors)]
async fn ec_05_file_write_permission_error() {
	// Test: ファイル書き込み権限エラー（E2E）

	let temp_dir = TempDir::new().unwrap();
	let migrations_dir = temp_dir.path().join("migrations");
	let app_dir = migrations_dir.join("todos");

	// Create directory
	std::fs::create_dir_all(&app_dir).unwrap();

	// Make directory read-only (remove write permission)
	#[cfg(unix)]
	{
		use std::os::unix::fs::PermissionsExt;
		let metadata = std::fs::metadata(&app_dir).unwrap();
		let mut permissions = metadata.permissions();
		permissions.set_mode(0o444); // Read-only
		std::fs::set_permissions(&app_dir, permissions).unwrap();
	}

	let target_schema = create_todos_schema();
	let empty_schema = DatabaseSchema::default();

	let repository = Arc::new(Mutex::new(FilesystemRepository::new(
		migrations_dir.clone(),
	)));
	let source = Arc::new(FilesystemSource::new(migrations_dir.clone()));
	let service = MigrationService::new(source, repository.clone());

	let generator = AutoMigrationGenerator::new(target_schema, repository);
	let result = generator.generate("todos", empty_schema).await.unwrap();

	let migration = Migration {
		app_label: "todos",
		name: "0001_initial",
		operations: result.operations,
		dependencies: Vec::new(),
		atomic: true,
		replaces: Vec::new(),
		initial: Some(true),
	};

	// Try to save migration (should fail with permission error)
	let save_result = service.save_migration(&migration).await;

	#[cfg(unix)]
	{
		assert!(
			save_result.is_err(),
			"Should fail with permission error"
		);
		// Cleanup: restore permissions
		let metadata = std::fs::metadata(&app_dir).unwrap();
		let mut permissions = metadata.permissions();
		permissions.set_mode(0o755);
		std::fs::set_permissions(&app_dir, permissions).unwrap();
	}
}

// ============================================================================
// Edge Cases (EDG-01 ~ EDG-14)
// ============================================================================

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_01_empty_migration_generation() {
	// Test: 空のマイグレーション（--empty）生成（E2E）
	todo!("Implement E2E empty migration test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_02_no_changes_detected() {
	// Test: 変更がない場合（No changes detected）（E2E）
	todo!("Implement E2E no changes test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_03_dry_run_mode() {
	// Test: --dry-run モード（E2E）
	todo!("Implement E2E dry-run test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_04_custom_name_specification() {
	// Test: --name カスタム名指定（E2E）
	todo!("Implement E2E custom name test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_05_verbose_mode() {
	// Test: --verbose モード（E2E）
	todo!("Implement E2E verbose mode test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_06_custom_migrations_directory() {
	// Test: --migrations-dir カスタムディレクトリ指定（E2E）
	todo!("Implement E2E custom directory test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_07_from_db_mode() {
	// Test: --from-db モード（E2E）
	todo!("Implement E2E from-db mode test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_08_long_model_field_names() {
	// Test: 長いモデル名/フィールド名（255文字）（E2E）
	todo!("Implement E2E long names test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_09_large_number_of_fields() {
	// Test: 大量のフィールド（100+）（E2E）
	todo!("Implement E2E many fields test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_10_deep_dependency_chain() {
	// Test: 深い依存チェーン（10段階）（E2E）
	todo!("Implement E2E deep dependency test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_11_unicode_in_names() {
	// Test: 特殊文字を含む名前（Unicode）（E2E）
	todo!("Implement E2E unicode test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_12_sql_reserved_words() {
	// Test: SQL予約語を含むテーブル名/カラム名（E2E）
	todo!("Implement E2E reserved words test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_13_same_name_different_apps() {
	// Test: 同一名の異なるアプリのモデル（E2E）
	todo!("Implement E2E same name test")
}

#[tokio::test]
#[serial(makemigrations_e2e_edge)]
async fn edg_14_cross_app_dependencies() {
	// Test: クロスアプリ依存関係（E2E）
	todo!("Implement E2E cross-app test")
}
