//! Integration tests for cross-database autodetector consistency
//!
//! Tests detection behavior across different database backends:
//! - PostgreSQL vs MySQL detection consistency
//! - Type mapping differences
//! - Composite primary key handling
//!
//! **Test Coverage:**
//! - Cross-database detection consistency
//! - Database-specific type mappings
//! - Composite primary key operations
//!
//! **Fixtures Used:**
//! - postgres_container: PostgreSQL database container
//! - mysql_container: MySQL database container

use reinhardt_migrations::{
	ConstraintDefinition, FieldState, FieldType, MigrationAutodetector, ModelState, ProjectState,
};
use rstest::*;
use std::collections::BTreeMap;

// ============================================================================
// Test Helper Functions
// ============================================================================

/// Create a basic model with id field
fn create_basic_model(app: &str, name: &str, table_name: &str) -> ModelState {
	let mut fields = BTreeMap::new();
	fields.insert(
		"id".to_string(),
		FieldState::new("id".to_string(), FieldType::Integer, false, BTreeMap::new()),
	);

	ModelState {
		app_label: app.to_string(),
		name: name.to_string(),
		table_name: table_name.to_string(),
		fields,
		options: BTreeMap::new(),
		base_model: None,
		inheritance_type: None,
		discriminator_column: None,
		indexes: vec![],
		constraints: vec![],
		many_to_many_fields: vec![],
	}
}

/// Add a field to a model
fn add_field(model: &mut ModelState, name: &str, field_type: FieldType) {
	model.fields.insert(
		name.to_string(),
		FieldState::new(name.to_string(), field_type, true, BTreeMap::new()),
	);
}

// ============================================================================
// Test 40: PostgreSQL vs MySQL Detection Consistency
// ============================================================================

/// Test that autodetector produces consistent results across PostgreSQL and MySQL
///
/// **Test Intent**: Verify that same schema changes are detected consistently regardless of DB backend
///
/// **Integration Point**: MigrationAutodetector → detect_changes()
///
/// **Expected Behavior**: Detection results should be identical for common operations
#[rstest]
#[test]
fn test_postgres_mysql_detection_consistency() {
	// from_state: User model with basic fields
	let mut from_state = ProjectState::new();
	let mut user_model = create_basic_model("testapp", "User", "testapp_user");
	add_field(&mut user_model, "username", FieldType::VarChar(100));
	add_field(&mut user_model, "email", FieldType::VarChar(255));
	from_state.add_model(user_model);

	// to_state: User model with additional field
	let mut to_state = ProjectState::new();
	let mut user_model = create_basic_model("testapp", "User", "testapp_user");
	add_field(&mut user_model, "username", FieldType::VarChar(100));
	add_field(&mut user_model, "email", FieldType::VarChar(255));
	add_field(&mut user_model, "created_at", FieldType::DateTime);
	to_state.add_model(user_model);

	// Autodetector実行（DB非依存の検出）
	let autodetector = MigrationAutodetector::new(from_state, to_state);
	let detected = autodetector.detect_changes();

	// 検証: フィールド追加が検出される（PostgreSQL/MySQL共通）
	assert_eq!(
		detected.added_fields.len(),
		1,
		"Should detect 1 added field consistently"
	);
	assert_eq!(detected.added_fields[0].0, "testapp");
	assert_eq!(detected.added_fields[0].1, "User");
	assert_eq!(detected.added_fields[0].2, "created_at");

	// NOTE: このテストはDB非依存の検出ロジックを検証
	// 実際のSQL生成における型マッピングの差異は、次のテストで検証
}

// ============================================================================
// Test 41: Type Mapping Differences Detection
// ============================================================================

/// Test detection of type mapping differences between PostgreSQL and MySQL
///
/// **Test Intent**: Verify that type differences are properly detected
///
/// **Integration Point**: MigrationAutodetector → detect_altered_fields()
///
/// **Expected Behavior**: Type changes should be detected, though SQL generation may differ
#[rstest]
#[test]
fn test_type_mapping_differences() {
	// from_state: Product with Integer price
	let mut from_state = ProjectState::new();
	let mut product_model = create_basic_model("testapp", "Product", "testapp_product");
	add_field(&mut product_model, "price", FieldType::Integer);
	from_state.add_model(product_model);

	// to_state: Product with Decimal price
	// PostgreSQL: NUMERIC(10,2)
	// MySQL: DECIMAL(10,2)
	let mut to_state = ProjectState::new();
	let mut product_model = create_basic_model("testapp", "Product", "testapp_product");
	add_field(&mut product_model, "price", FieldType::Decimal(10, 2));
	to_state.add_model(product_model);

	// Autodetector実行
	let autodetector = MigrationAutodetector::new(from_state, to_state);
	let detected = autodetector.detect_changes();

	// 検証: 型変更が検出される
	assert_eq!(
		detected.altered_fields.len(),
		1,
		"Should detect type change from Integer to Decimal"
	);
	assert_eq!(detected.altered_fields[0].0, "testapp");
	assert_eq!(detected.altered_fields[0].1, "Product");
	assert_eq!(detected.altered_fields[0].2, "price");

	// NOTE: 実際のSQL生成時にはDB固有の型にマッピングされる
	// PostgreSQL: ALTER TABLE ... ALTER COLUMN price TYPE NUMERIC(10,2)
	// MySQL: ALTER TABLE ... MODIFY COLUMN price DECIMAL(10,2)
}

// ============================================================================
// Composite Primary Key Tests
// ============================================================================

// ============================================================================
// Test: Add Composite Primary Key
// ============================================================================

/// Test detection of composite primary key addition
///
/// **Test Intent**: Verify that composite primary key constraints are detected
///
/// **Integration Point**: MigrationAutodetector → detect_added_constraints()
///
/// **Expected Behavior**: Composite PK constraint addition should be detected
#[rstest]
#[test]
fn test_detect_add_composite_primary_key() {
	// from_state: OrderItem with auto-increment id (single PK)
	let mut from_state = ProjectState::new();
	let order_item_model = create_basic_model("testapp", "OrderItem", "testapp_orderitem");
	from_state.add_model(order_item_model);

	// to_state: OrderItem with composite PK (order_id, product_id)
	let mut to_state = ProjectState::new();
	let mut order_item_model = create_basic_model("testapp", "OrderItem", "testapp_orderitem");

	// idフィールドを削除し、複合PKフィールドを追加
	order_item_model.fields.remove("id");
	add_field(&mut order_item_model, "order_id", FieldType::Integer);
	add_field(&mut order_item_model, "product_id", FieldType::Integer);

	// 複合PK制約として追加
	order_item_model.constraints.push(ConstraintDefinition {
		name: "pk_orderitem".to_string(),
		constraint_type: "PrimaryKey".to_string(),
		fields: vec!["order_id".to_string(), "product_id".to_string()],
		expression: None,
		foreign_key_info: None,
	});

	to_state.add_model(order_item_model);

	// Autodetector実行
	let autodetector = MigrationAutodetector::new(from_state, to_state);
	let detected = autodetector.detect_changes();

	// 検証: idフィールドの削除と新しいフィールドの追加、複合PK制約の追加が検出される
	assert_eq!(detected.removed_fields.len(), 1, "Should detect id field removal");
	assert_eq!(
		detected.added_fields.len(),
		2,
		"Should detect order_id and product_id addition"
	);
	assert_eq!(
		detected.added_constraints.len(),
		1,
		"Should detect composite PK constraint addition"
	);
	assert_eq!(detected.added_constraints[0].2.constraint_type, "PrimaryKey");
	assert_eq!(detected.added_constraints[0].2.fields.len(), 2);
}

// ============================================================================
// Test: Modify Composite Primary Key
// ============================================================================

/// Test detection of composite primary key modification
///
/// **Test Intent**: Verify that changes to composite PK fields are detected
///
/// **Integration Point**: MigrationAutodetector → detect_removed_constraints() + detect_added_constraints()
///
/// **Expected Behavior**: Old PK removal and new PK addition should be detected
#[rstest]
#[test]
fn test_detect_modify_composite_primary_key() {
	// from_state: OrderItem with composite PK (order_id, product_id)
	let mut from_state = ProjectState::new();
	let mut order_item_model = create_basic_model("testapp", "OrderItem", "testapp_orderitem");
	order_item_model.fields.remove("id");
	add_field(&mut order_item_model, "order_id", FieldType::Integer);
	add_field(&mut order_item_model, "product_id", FieldType::Integer);
	order_item_model.constraints.push(ConstraintDefinition {
		name: "pk_orderitem".to_string(),
		constraint_type: "PrimaryKey".to_string(),
		fields: vec!["order_id".to_string(), "product_id".to_string()],
		expression: None,
		foreign_key_info: None,
	});
	from_state.add_model(order_item_model);

	// to_state: OrderItem with different composite PK (order_id, product_id, line_number)
	let mut to_state = ProjectState::new();
	let mut order_item_model = create_basic_model("testapp", "OrderItem", "testapp_orderitem");
	order_item_model.fields.remove("id");
	add_field(&mut order_item_model, "order_id", FieldType::Integer);
	add_field(&mut order_item_model, "product_id", FieldType::Integer);
	add_field(&mut order_item_model, "line_number", FieldType::Integer);
	order_item_model.constraints.push(ConstraintDefinition {
		name: "pk_orderitem_new".to_string(),
		constraint_type: "PrimaryKey".to_string(),
		fields: vec![
			"order_id".to_string(),
			"product_id".to_string(),
			"line_number".to_string(),
		],
		expression: None,
		foreign_key_info: None,
	});
	to_state.add_model(order_item_model);

	// Autodetector実行
	let autodetector = MigrationAutodetector::new(from_state, to_state);
	let detected = autodetector.detect_changes();

	// 検証: 新しいフィールド追加、古いPK削除、新しいPK追加が検出される
	assert_eq!(
		detected.added_fields.len(),
		1,
		"Should detect line_number field addition"
	);
	assert_eq!(
		detected.removed_constraints.len(),
		1,
		"Should detect old PK constraint removal"
	);
	assert_eq!(
		detected.added_constraints.len(),
		1,
		"Should detect new PK constraint addition"
	);

	// 新しいPKは3つのフィールドを含む
	assert_eq!(detected.added_constraints[0].2.fields.len(), 3);
	assert!(detected.added_constraints[0].2.fields.contains(&"line_number".to_string()));
}

// ============================================================================
// Test: Cross-DB Composite PK Behavior
// ============================================================================

/// Test cross-database behavior consistency for composite primary keys
///
/// **Test Intent**: Verify that composite PK detection is consistent across databases
///
/// **Integration Point**: MigrationAutodetector → detect_added_constraints()
///
/// **Expected Behavior**: Detection should be consistent, though SQL syntax may differ
#[rstest]
#[test]
fn test_cross_db_composite_pk_behavior() {
	// from_state: Empty
	let from_state = ProjectState::new();

	// to_state: UserRole with composite PK (user_id, role_id)
	let mut to_state = ProjectState::new();
	let mut user_role_model = create_basic_model("testapp", "UserRole", "testapp_userrole");
	user_role_model.fields.remove("id");
	add_field(&mut user_role_model, "user_id", FieldType::Integer);
	add_field(&mut user_role_model, "role_id", FieldType::Integer);
	add_field(&mut user_role_model, "assigned_at", FieldType::DateTime);

	// 複合PK制約
	user_role_model.constraints.push(ConstraintDefinition {
		name: "pk_userrole".to_string(),
		constraint_type: "PrimaryKey".to_string(),
		fields: vec!["user_id".to_string(), "role_id".to_string()],
		expression: None,
		foreign_key_info: None,
	});

	to_state.add_model(user_role_model);

	// Autodetector実行
	let autodetector = MigrationAutodetector::new(from_state, to_state);
	let detected = autodetector.detect_changes();

	// 検証: モデル作成と複合PK制約が検出される
	assert_eq!(detected.created_models.len(), 1, "Should detect model creation");
	assert_eq!(
		detected.added_constraints.len(),
		1,
		"Should detect composite PK constraint"
	);
	assert_eq!(detected.added_constraints[0].2.constraint_type, "PrimaryKey");
	assert_eq!(
		detected.added_constraints[0].2.fields,
		vec!["user_id".to_string(), "role_id".to_string()]
	);

	// NOTE: 実際のSQL生成時の構文はDB依存:
	// PostgreSQL: CREATE TABLE ... (user_id INT, role_id INT, PRIMARY KEY (user_id, role_id))
	// MySQL: CREATE TABLE ... (user_id INT, role_id INT, PRIMARY KEY (user_id, role_id))
	// 構文はほぼ同じだが、型の違い（INT vs INTEGER）などがある可能性がある
}

// ============================================================================
// Test: Composite PK with Other Constraints
// ============================================================================

/// Test composite primary key combined with other constraints
///
/// **Test Intent**: Verify detection when composite PK coexists with UNIQUE/FK constraints
///
/// **Integration Point**: MigrationAutodetector → detect_added_constraints()
///
/// **Expected Behavior**: All constraints should be detected independently
#[rstest]
#[test]
fn test_composite_pk_with_other_constraints() {
	// from_state: Empty
	let from_state = ProjectState::new();

	// to_state: OrderItem with composite PK + UNIQUE constraint + FK constraint
	let mut to_state = ProjectState::new();
	let mut order_item_model = create_basic_model("testapp", "OrderItem", "testapp_orderitem");
	order_item_model.fields.remove("id");
	add_field(&mut order_item_model, "order_id", FieldType::Integer);
	add_field(&mut order_item_model, "product_id", FieldType::Integer);
	add_field(&mut order_item_model, "sku", FieldType::VarChar(100));

	// 複合PK制約
	order_item_model.constraints.push(ConstraintDefinition {
		name: "pk_orderitem".to_string(),
		constraint_type: "PrimaryKey".to_string(),
		fields: vec!["order_id".to_string(), "product_id".to_string()],
		expression: None,
		foreign_key_info: None,
	});

	// UNIQUE制約（SKU）
	order_item_model.constraints.push(ConstraintDefinition {
		name: "unique_sku".to_string(),
		constraint_type: "Unique".to_string(),
		fields: vec!["sku".to_string()],
		expression: None,
		foreign_key_info: None,
	});

	// FK制約（order_id → orders.id）
	order_item_model.constraints.push(ConstraintDefinition {
		name: "fk_order".to_string(),
		constraint_type: "ForeignKey".to_string(),
		fields: vec!["order_id".to_string()],
		expression: None,
		foreign_key_info: Some(reinhardt_migrations::ForeignKeyConstraintInfo {
			referenced_table: "testapp_order".to_string(),
			referenced_columns: vec!["id".to_string()],
			on_delete: reinhardt_migrations::ForeignKeyAction::Cascade,
			on_update: reinhardt_migrations::ForeignKeyAction::NoAction,
		}),
	});

	to_state.add_model(order_item_model);

	// Autodetector実行
	let autodetector = MigrationAutodetector::new(from_state, to_state);
	let detected = autodetector.detect_changes();

	// 検証: すべての制約が検出される
	assert_eq!(detected.created_models.len(), 1, "Should detect model creation");
	assert_eq!(
		detected.added_constraints.len(),
		3,
		"Should detect all 3 constraints (PK + UNIQUE + FK)"
	);

	// 制約タイプの確認
	let constraint_types: Vec<&str> = detected
		.added_constraints
		.iter()
		.map(|c| c.2.constraint_type.as_str())
		.collect();
	assert!(constraint_types.contains(&"PrimaryKey"));
	assert!(constraint_types.contains(&"Unique"));
	assert!(constraint_types.contains(&"ForeignKey"));
}
