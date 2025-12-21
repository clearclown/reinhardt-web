//! Field definitions Server Function
//!
//! Provides field information for dynamic form generation.

use reinhardt_admin_adapters::{AdminDatabase, AdminRecord, AdminSite, FieldInfo, FieldType};
use reinhardt_admin_types::FieldsResponse;
use reinhardt_pages::server_fn::{ServerFnError, server_fn};
use std::sync::Arc;

#[cfg(not(target_arch = "wasm32"))]
use super::error::MapServerFnError;

/// Get field definitions for dynamic form generation
///
/// Retrieves field metadata for creating or editing model instances.
/// When `id` is provided, also retrieves the existing field values for editing.
///
/// # Server Function
///
/// This function is automatically exposed as an HTTP endpoint by the `#[server_fn]` macro.
/// AdminSite and AdminDatabase dependencies are automatically injected via the DI system.
///
/// # Example
///
/// ```ignore
/// use reinhardt_admin_server::get_fields;
///
/// // Client-side usage for create form
/// let response = get_fields("User".to_string(), None).await?;
/// println!("Fields: {:?}", response.fields);
///
/// // Client-side usage for edit form
/// let response = get_fields("User".to_string(), Some("42".to_string())).await?;
/// println!("Existing values: {:?}", response.values);
/// ```
#[server_fn(use_inject = true)]
pub async fn get_fields(
	model_name: String,
	id: Option<String>,
	#[inject] site: Arc<AdminSite>,
	#[inject] db: Arc<AdminDatabase>,
) -> Result<FieldsResponse, ServerFnError> {
	let model_admin = site.get_model_admin(&model_name).map_server_fn_error()?;
	let field_names = model_admin
		.fields()
		.unwrap_or_else(|| model_admin.list_display());
	let readonly_fields = model_admin.readonly_fields();

	// Build field metadata
	let fields = field_names
		.iter()
		.map(|&name| {
			let is_readonly = readonly_fields.contains(&name);

			FieldInfo {
				name: name.to_string(),
				label: name.to_string(),     // TODO: Humanize field name
				field_type: FieldType::Text, // TODO: Infer field type from schema
				required: false,             // TODO: Infer required from schema
				readonly: is_readonly,
				help_text: None,
				placeholder: None,
			}
		})
		.collect();

	// Fetch existing values if editing
	let values = if let Some(id) = id {
		db.get::<AdminRecord>(model_admin.table_name(), model_admin.pk_field(), &id)
			.await
			.map_server_fn_error()?
	} else {
		None
	};

	Ok(FieldsResponse {
		model_name,
		fields,
		values,
	})
}
