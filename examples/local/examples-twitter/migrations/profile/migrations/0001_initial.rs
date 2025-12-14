use reinhardt::db::migrations::FieldType;
use reinhardt::db::migrations::prelude::*;
pub fn migration() -> Migration {
	Migration {
		app_label: "profile",
		name: "0001_initial",
		operations: vec![
			Operation::AlterColumn {
				table: "profile_profile",
				column: "website",
				new_definition: ColumnDefinition {
					name: "website",
					type_definition: FieldType::VarChar(255u32),
					not_null: false,
					unique: false,
					primary_key: false,
					auto_increment: false,
					default: None,
				},
			},
			Operation::AlterColumn {
				table: "profile_profile",
				column: "bio",
				new_definition: ColumnDefinition {
					name: "bio",
					type_definition: FieldType::VarChar(500u32),
					not_null: false,
					unique: false,
					primary_key: false,
					auto_increment: false,
					default: None,
				},
			},
			Operation::AlterColumn {
				table: "profile_profile",
				column: "location",
				new_definition: ColumnDefinition {
					name: "location",
					type_definition: FieldType::VarChar(255u32),
					not_null: false,
					unique: false,
					primary_key: false,
					auto_increment: false,
					default: None,
				},
			},
			Operation::AlterColumn {
				table: "profile_profile",
				column: "id",
				new_definition: ColumnDefinition {
					name: "id",
					type_definition: FieldType::Uuid,
					not_null: false,
					unique: false,
					primary_key: false,
					auto_increment: false,
					default: None,
				},
			},
			Operation::AlterColumn {
				table: "profile_profile",
				column: "avatar_url",
				new_definition: ColumnDefinition {
					name: "avatar_url",
					type_definition: FieldType::VarChar(255u32),
					not_null: false,
					unique: false,
					primary_key: false,
					auto_increment: false,
					default: None,
				},
			},
			Operation::AlterColumn {
				table: "profile_profile",
				column: "user_id",
				new_definition: ColumnDefinition {
					name: "user_id",
					type_definition: FieldType::Uuid,
					not_null: false,
					unique: false,
					primary_key: false,
					auto_increment: false,
					default: None,
				},
			},
		],
		dependencies: vec![],
		atomic: true,
		replaces: vec![],
	}
}
