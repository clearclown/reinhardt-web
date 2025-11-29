use crate::{ColumnDefinition, Operation};
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};

impl ToTokens for Operation {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		match self {
			Operation::CreateTable {
				name,
				columns,
				constraints,
			} => {
				let columns_tokens = columns.iter();
				let constraints_tokens = constraints.iter();
				tokens.extend(quote! {
					Operation::CreateTable {
						name: #name.to_string(),
						columns: vec![#(#columns_tokens),*],
						constraints: vec![#(#constraints_tokens.to_string()),*],
					}
				});
			}
			Operation::DropTable { name } => {
				tokens.extend(quote! {
					Operation::DropTable {
						name: #name.to_string(),
					}
				});
			}
			Operation::AddColumn { table, column } => {
				tokens.extend(quote! {
					Operation::AddColumn {
						table: #table.to_string(),
						column: #column,
					}
				});
			}
			Operation::DropColumn { table, column } => {
				tokens.extend(quote! {
					Operation::DropColumn {
						table: #table.to_string(),
						column: #column.to_string(),
					}
				});
			}
			Operation::AlterColumn {
				table,
				column,
				new_definition,
			} => {
				tokens.extend(quote! {
					Operation::AlterColumn {
						table: #table.to_string(),
						column: #column.to_string(),
						new_definition: #new_definition,
					}
				});
			}
			Operation::RenameTable { old_name, new_name } => {
				tokens.extend(quote! {
					Operation::RenameTable {
						old_name: #old_name.to_string(),
						new_name: #new_name.to_string(),
					}
				});
			}
			Operation::RenameColumn {
				table,
				old_name,
				new_name,
			} => {
				tokens.extend(quote! {
					Operation::RenameColumn {
						table: #table.to_string(),
						old_name: #old_name.to_string(),
						new_name: #new_name.to_string(),
					}
				});
			}
			Operation::AddConstraint {
				table,
				constraint_sql,
			} => {
				tokens.extend(quote! {
					Operation::AddConstraint {
						table: #table.to_string(),
						constraint_sql: #constraint_sql.to_string(),
					}
				});
			}
			Operation::DropConstraint {
				table,
				constraint_name,
			} => {
				tokens.extend(quote! {
					Operation::DropConstraint {
						table: #table.to_string(),
						constraint_name: #constraint_name.to_string(),
					}
				});
			}
			Operation::CreateIndex {
				table,
				columns,
				unique,
			} => {
				let columns_iter = columns.iter();
				tokens.extend(quote! {
					Operation::CreateIndex {
						table: #table.to_string(),
						columns: vec![#(#columns_iter.to_string()),*],
						unique: #unique,
					}
				});
			}
			Operation::DropIndex { table, columns } => {
				let columns_iter = columns.iter();
				tokens.extend(quote! {
					Operation::DropIndex {
						table: #table.to_string(),
						columns: vec![#(#columns_iter.to_string()),*],
					}
				});
			}
			Operation::RunSQL { sql, reverse_sql } => {
				let reverse_sql_token = match reverse_sql {
					Some(s) => quote! { Some(#s.to_string()) },
					None => quote! { None },
				};
				tokens.extend(quote! {
					Operation::RunSQL {
						sql: #sql.to_string(),
						reverse_sql: #reverse_sql_token,
					}
				});
			}
			Operation::RunRust { code, reverse_code } => {
				let reverse_code_token = match reverse_code {
					Some(s) => quote! { Some(#s.to_string()) },
					None => quote! { None },
				};
				tokens.extend(quote! {
					Operation::RunRust {
						code: #code.to_string(),
						reverse_code: #reverse_code_token,
					}
				});
			}
			Operation::AlterTableComment { table, comment } => {
				let comment_token = match comment {
					Some(s) => quote! { Some(#s.to_string()) },
					None => quote! { None },
				};
				tokens.extend(quote! {
					Operation::AlterTableComment {
						table: #table.to_string(),
						comment: #comment_token,
					}
				});
			}
			Operation::AlterUniqueTogether {
				table,
				unique_together,
			} => {
				let unique_together_tokens = unique_together.iter().map(|fields| {
					let fields_iter = fields.iter();
					quote! { vec![#(#fields_iter.to_string()),*] }
				});
				tokens.extend(quote! {
					Operation::AlterUniqueTogether {
						table: #table.to_string(),
						unique_together: vec![#(#unique_together_tokens),*],
					}
				});
			}
			Operation::AlterModelOptions { table, options } => {
				let keys = options.keys();
				let values = options.values();
				tokens.extend(quote! {
					Operation::AlterModelOptions {
						table: #table.to_string(),
						options: {
							let mut map = std::collections::HashMap::new();
							#(map.insert(#keys.to_string(), #values.to_string());)*
							map
						},
					}
				});
			}
			Operation::CreateInheritedTable {
				name,
				columns,
				base_table,
				join_column,
			} => {
				let columns_tokens = columns.iter();
				tokens.extend(quote! {
					Operation::CreateInheritedTable {
						name: #name.to_string(),
						columns: vec![#(#columns_tokens),*],
						base_table: #base_table.to_string(),
						join_column: #join_column.to_string(),
					}
				});
			}
			Operation::AddDiscriminatorColumn {
				table,
				column_name,
				default_value,
			} => {
				tokens.extend(quote! {
					Operation::AddDiscriminatorColumn {
						table: #table.to_string(),
						column_name: #column_name.to_string(),
						default_value: #default_value.to_string(),
					}
				});
			}
		}
	}
}

impl ToTokens for ColumnDefinition {
	fn to_tokens(&self, tokens: &mut TokenStream) {
		let name = &self.name;
		let type_definition = &self.type_definition;
		let not_null = self.not_null;
		let unique = self.unique;
		let primary_key = self.primary_key;
		let auto_increment = self.auto_increment;

		let default_token = match &self.default {
			Some(s) => quote! { Some(#s.to_string()) },
			None => quote! { None },
		};

		let max_length_token = match self.max_length {
			Some(l) => quote! { Some(#l) },
			None => quote! { None },
		};

		tokens.extend(quote! {
			ColumnDefinition {
				name: #name.to_string(),
				type_definition: #type_definition.to_string(),
				not_null: #not_null,
				unique: #unique,
				primary_key: #primary_key,
				auto_increment: #auto_increment,
				default: #default_token,
				max_length: #max_length_token,
			}
		});
	}
}
