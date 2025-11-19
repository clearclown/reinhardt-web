//! Migration file writer
//!
//! Generates Rust migration files from Migration structs.
//!
//! ## AST-Based Code Generation
//!
//! This module uses Abstract Syntax Tree (AST) parsing via `syn` and `quote`
//! for robust migration file generation. Benefits include:
//!
//! - **Syntax Guarantee**: Generates syntactically correct Rust code
//! - **Consistent Formatting**: Uses `prettyplease` for standardized output
//! - **Maintainability**: Structural code generation via quote! macro
//! - **Extensibility**: Easy to add new operation types

use crate::{Migration, Operation, Result};
use std::fs;
use std::path::Path;

/// Writer for generating migration files
pub struct MigrationWriter {
	migration: Migration,
}

impl MigrationWriter {
	/// Create a new migration writer
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_migrations::{Migration, writer::MigrationWriter};
	///
	/// let migration = Migration::new("0001_initial", "myapp");
	/// let writer = MigrationWriter::new(migration);
	/// ```
	pub fn new(migration: Migration) -> Self {
		Self { migration }
	}
	/// Generate the migration file content
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_migrations::{Migration, Operation, ColumnDefinition, writer::MigrationWriter};
	///
	/// let migration = Migration::new("0001_initial", "myapp")
	///     .add_operation(Operation::CreateTable {
	///         name: "users".to_string(),
	///         columns: vec![ColumnDefinition::new("id", "INTEGER PRIMARY KEY")],
	///         constraints: vec![],
	///     });
	///
	/// let writer = MigrationWriter::new(migration);
	/// let content = writer.as_string();
	///
	/// assert!(content.contains("//! Name: 0001_initial"));
	/// assert!(content.contains("//! App: myapp"));
	/// assert!(content.contains("Migration::new"));
	/// ```
	/// Generate the migration file content
	///
	/// # Examples
	///
	/// ```
	/// use reinhardt_migrations::{Migration, Operation, ColumnDefinition, writer::MigrationWriter};
	///
	/// let migration = Migration::new("0001_initial", "myapp")
	///     .add_operation(Operation::CreateTable {
	///         name: "users".to_string(),
	///         columns: vec![ColumnDefinition::new("id", "INTEGER PRIMARY KEY")],
	///         constraints: vec![],
	///     });
	///
	/// let writer = MigrationWriter::new(migration);
	/// let content = writer.as_string();
	///
	/// assert!(content.contains("Name: 0001_initial"));
	/// assert!(content.contains("App: myapp"));
	/// assert!(content.contains("Migration::new"));
	/// ```
	pub fn as_string(&self) -> String {
		use quote::quote;

		let name = &self.migration.name;
		let app_label = &self.migration.app_label;
		let func_name = quote::format_ident!("migration_{}", name.replace('-', "_"));

		// Generate dependencies
		let dependencies = self
			.migration
			.dependencies
			.iter()
			.map(|(dep_app, dep_name)| {
				quote! {
					.add_dependency(#dep_app, #dep_name)
				}
			});

		// Generate operations
		let operations = self
			.migration
			.operations
			.iter()
			.map(|op| self.quote_operation(op));

		let name_doc = format!(" Name: {}", name);
		let app_doc = format!(" App: {}", app_label);

		// Generate the full code using quote!
		let code = quote! {
			#![doc = " Auto-generated migration"]
			#![doc = #name_doc]
			#![doc = #app_doc]

			use reinhardt_migrations::{
				Migration, Operation, CreateTable, AddColumn, AlterColumn,
				ColumnDefinition,
			};

			pub fn #func_name() -> Migration {
				Migration::new(#name, #app_label)
					#(#dependencies)*
					#(#operations)*
			}
		};

		// Parse and format
		let syntax_tree = syn::parse2(code).expect("Failed to parse generated code");
		prettyplease::unparse(&syntax_tree)
	}

	fn quote_operation(&self, operation: &Operation) -> proc_macro2::TokenStream {
		use quote::quote;

		match operation {
			Operation::CreateTable {
				name,
				columns,
				constraints,
			} => {
				let columns = columns.iter().map(|col| self.quote_column(col));
				let constraints = constraints.iter();

				quote! {
					.add_operation(Operation::CreateTable {
						name: #name.to_string(),
						columns: vec![#(#columns),*],
						constraints: vec![#(#constraints.to_string()),*],
					})
				}
			}
			Operation::DropTable { name } => {
				quote! {
					.add_operation(Operation::DropTable {
						name: #name.to_string(),
					})
				}
			}
			Operation::AddColumn { table, column } => {
				let column = self.quote_column(column);
				quote! {
					.add_operation(Operation::AddColumn {
						table: #table.to_string(),
						column: #column,
					})
				}
			}
			Operation::AlterColumn {
				table,
				column,
				new_definition,
			} => {
				let new_definition = self.quote_column(new_definition);
				quote! {
					.add_operation(Operation::AlterColumn {
						table: #table.to_string(),
						column: #column.to_string(),
						new_definition: #new_definition,
					})
				}
			}
			Operation::DropColumn { table, column } => {
				quote! {
					.add_operation(Operation::DropColumn {
						table: #table.to_string(),
						column: #column.to_string(),
					})
				}
			}
			_ => {
				quote! {
					.add_operation(Operation::RunSQL {
						sql: "-- Unsupported operation".to_string(),
						reverse_sql: None,
					})
				}
			}
		}
	}

	fn quote_column(&self, column: &crate::ColumnDefinition) -> proc_macro2::TokenStream {
		use quote::quote;
		let name = &column.name;
		let type_definition = &column.type_definition;

		quote! {
			ColumnDefinition {
				name: #name.to_string(),
				type_definition: #type_definition.to_string(),
			}
		}
	}

	/// Write migration to file
	///
	/// # Examples
	///
	/// ```no_run
	/// use reinhardt_migrations::{Migration, writer::MigrationWriter};
	/// use std::path::PathBuf;
	///
	/// let migration = Migration::new("0001_initial", "myapp");
	/// let writer = MigrationWriter::new(migration);
	///
	/// let temp_dir = PathBuf::from("/tmp/migrations");
	/// let filepath = writer.write_to_file(&temp_dir).unwrap();
	/// assert!(filepath.ends_with("0001_initial.rs"));
	/// ```
	pub fn write_to_file<P: AsRef<Path>>(&self, directory: P) -> Result<String> {
		let dir_path = directory.as_ref();
		fs::create_dir_all(dir_path)?;

		let filename = format!("{}.rs", self.migration.name);
		let filepath = dir_path.join(&filename);

		fs::write(&filepath, self.as_string())?;

		Ok(filepath.to_string_lossy().into_owned())
	}
}

// Tests are in tests/test_writer.rs
