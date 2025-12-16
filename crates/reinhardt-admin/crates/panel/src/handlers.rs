//! HTTP handlers for admin panel
//!
//! This module provides HTTP handlers for admin panel CRUD operations.
//! All handlers return JSON responses.

use crate::templates::{
	AdminContext, AdminTemplateRenderer, AppContext, DashboardContext, ModelContext,
};
use crate::{
	AdminDatabase, AdminError, AdminResult, AdminSite, ImportBuilder, ImportError, ImportFormat,
	ImportResult,
};
use hyper::StatusCode;
use reinhardt_db::orm::{Filter, FilterCondition, FilterOperator, FilterValue, Model};
use reinhardt_http::Response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

/// Query parameters for list endpoint
#[derive(Debug, Deserialize, Default)]
pub struct ListQueryParams {
	/// Page number (1-indexed)
	pub page: Option<u64>,
	/// Items per page
	pub page_size: Option<u64>,
	/// Search query
	pub search: Option<String>,
	/// Filter field=value pairs
	#[serde(flatten)]
	pub filters: HashMap<String, String>,
}

/// Response for dashboard endpoint
#[derive(Debug, Serialize)]
pub struct DashboardResponse {
	/// Site name
	pub site_name: String,
	/// URL prefix
	pub url_prefix: String,
	/// Registered models with their metadata
	pub models: Vec<ModelInfo>,
}

/// Model information for dashboard
#[derive(Debug, Serialize)]
pub struct ModelInfo {
	/// Model name
	pub name: String,
	/// List URL
	pub list_url: String,
}

/// Response for list endpoint
#[derive(Debug, Serialize)]
pub struct ListResponse {
	/// Model name
	pub model_name: String,
	/// Total count of items
	pub count: u64,
	/// Current page
	pub page: u64,
	/// Items per page
	pub page_size: u64,
	/// Total pages
	pub total_pages: u64,
	/// Items on this page
	pub results: Vec<HashMap<String, serde_json::Value>>,
}

/// Response for detail endpoint
#[derive(Debug, Serialize)]
pub struct DetailResponse {
	/// Model name
	pub model_name: String,
	/// Item data
	pub data: HashMap<String, serde_json::Value>,
}

/// Request body for create/update
#[derive(Debug, Deserialize)]
pub struct MutationRequest {
	/// Data to create/update
	#[serde(flatten)]
	pub data: HashMap<String, serde_json::Value>,
}

/// Response for create/update/delete
#[derive(Debug, Serialize)]
pub struct MutationResponse {
	/// Success status
	pub success: bool,
	/// Message
	pub message: String,
	/// Affected rows (for update/delete)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub affected: Option<u64>,
	/// Created/Updated data (for create/update)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data: Option<HashMap<String, serde_json::Value>>,
}

/// Request body for bulk delete
#[derive(Debug, Deserialize)]
pub struct BulkDeleteRequest {
	/// IDs to delete
	pub ids: Vec<String>,
}

/// Response for bulk delete
#[derive(Debug, Serialize)]
pub struct BulkDeleteResponse {
	/// Success status
	pub success: bool,
	/// Number of deleted items
	pub deleted: u64,
	/// Message
	pub message: String,
}

/// Response for import endpoint
#[derive(Debug, Serialize)]
pub struct ImportResponse {
	/// Success status
	pub success: bool,
	/// Number of imported records
	pub imported: u64,
	/// Number of updated records
	pub updated: u64,
	/// Number of skipped records
	pub skipped: u64,
	/// Number of failed records
	pub failed: u64,
	/// Summary message
	pub message: String,
	/// Error messages (if any)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub errors: Option<Vec<String>>,
}

/// Export format
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum ExportFormat {
	#[default]
	Json,
	Csv,
	Tsv,
}

/// Query parameters for export endpoint
#[derive(Debug, Deserialize, Default)]
pub struct ExportQueryParams {
	/// Export format
	#[serde(default)]
	pub format: ExportFormat,
}

/// Admin handlers context
///
/// Contains shared state for admin handlers.
pub struct AdminHandlers {
	site: Arc<AdminSite>,
	db: AdminDatabase,
	favicon_data: Option<Vec<u8>>,
	template_renderer: AdminTemplateRenderer,
}

impl AdminHandlers {
	/// Create new admin handlers
	pub fn new(site: Arc<AdminSite>, db: AdminDatabase) -> Self {
		// Get template directory path (resolved at compile time)
		let manifest_dir = env!("CARGO_MANIFEST_DIR");
		let template_dir = std::path::PathBuf::from(manifest_dir).join("templates");

		let template_renderer =
			AdminTemplateRenderer::new(template_dir.to_str().expect("Invalid template path"));

		Self {
			site,
			db,
			favicon_data: None,
			template_renderer,
		}
	}

	/// Set favicon data
	pub fn with_favicon(mut self, data: Vec<u8>) -> Self {
		self.favicon_data = Some(data);
		self
	}

	/// Get the admin site
	pub fn site(&self) -> &AdminSite {
		&self.site
	}

	/// Get the database
	pub fn db(&self) -> &AdminDatabase {
		&self.db
	}

	/// Dashboard handler - returns list of registered models
	pub async fn dashboard(&self) -> AdminResult<Response> {
		// Collect model information
		let models: Vec<ModelInfo> = self
			.site
			.registered_models()
			.into_iter()
			.map(|name| {
				let list_url = format!("{}/{}/", self.site.url_prefix(), name.to_lowercase());
				ModelInfo { name, list_url }
			})
			.collect();

		// Build AppContext (used in template)
		let mut available_apps = Vec::new();
		if !models.is_empty() {
			let app_models: Vec<ModelContext> = models
				.iter()
				.map(|m| ModelContext {
					name: m.name.clone(),
					label: m.name.clone(),
					url: m.list_url.clone(),
					add_url: Some(format!("{}add/", m.list_url)),
				})
				.collect();

			available_apps.push(AppContext {
				name: "Models".to_string(),
				label: "models".to_string(),
				models: app_models,
			});
		}

		// Build DashboardContext
		let admin_context = AdminContext::new(self.site.name()).with_available_apps(available_apps);

		let context = DashboardContext {
			admin: admin_context,
			widgets: vec![],        // Can add widgets in the future
			recent_actions: vec![], // Can add recent actions in the future
		};

		// Render template
		let html = self.template_renderer.render_dashboard(&context)?;

		// Return HTML response
		Ok(Response::new(StatusCode::OK)
			.with_header("Content-Type", "text/html; charset=utf-8")
			.with_body(html))
	}

	/// Favicon handler - returns favicon image
	pub async fn favicon(&self) -> AdminResult<Response> {
		match &self.favicon_data {
			Some(data) => {
				let response = Response::new(StatusCode::OK)
					.with_header("Content-Type", "image/png")
					.with_body(data.clone());
				Ok(response)
			}
			None => Err(AdminError::ValidationError("Favicon not configured".into())),
		}
	}

	/// List handler - returns paginated list of model instances
	pub async fn list<M: Model>(
		&self,
		model_name: &str,
		params: ListQueryParams,
	) -> AdminResult<Response> {
		let model_admin = self.site.get_model_admin(model_name)?;

		let page = params.page.unwrap_or(1).max(1);
		let page_size = params
			.page_size
			.unwrap_or_else(|| model_admin.list_per_page().unwrap_or(100) as u64);
		let offset = (page - 1) * page_size;

		// Build search filter condition (OR across multiple fields)
		let search_condition = if let Some(search) = &params.search {
			let search_fields = model_admin.search_fields();
			if !search_fields.is_empty() {
				// Build OR condition across all search fields
				let search_filters: Vec<Filter> = search_fields
					.iter()
					.map(|field| {
						Filter::new(
							field.to_string(),
							FilterOperator::Contains,
							FilterValue::String(search.clone()),
						)
					})
					.collect();

				// Create OR condition for search (matches any field)
				Some(FilterCondition::or_filters(search_filters))
			} else {
				None
			}
		} else {
			None
		};

		// Build additional filters from query params (AND logic)
		let mut additional_filters = Vec::new();
		let filter_fields = model_admin.list_filter();
		for field in filter_fields {
			if let Some(value) = params.filters.get(field) {
				additional_filters.push(Filter::new(
					field.to_string(),
					FilterOperator::Eq,
					FilterValue::String(value.clone()),
				));
			}
		}

		let table_name = model_admin.table_name();

		// Get total count
		let count = self
			.db
			.count_with_condition::<M>(
				table_name,
				search_condition.as_ref(),
				additional_filters.clone(),
			)
			.await?;

		// Get items
		let results = self
			.db
			.list_with_condition::<M>(
				table_name,
				search_condition.as_ref(),
				additional_filters,
				offset,
				page_size,
			)
			.await?;

		let total_pages = count.div_ceil(page_size);

		let response = ListResponse {
			model_name: model_name.to_string(),
			count,
			page,
			page_size,
			total_pages,
			results,
		};

		json_response(StatusCode::OK, &response)
	}

	/// Detail handler - returns single model instance
	pub async fn detail<M: Model>(&self, model_name: &str, id: &str) -> AdminResult<Response> {
		let model_admin = self.site.get_model_admin(model_name)?;
		let table_name = model_admin.table_name();
		let pk_field = model_admin.pk_field();

		let data = self
			.db
			.get::<M>(table_name, pk_field, id)
			.await?
			.ok_or_else(|| {
				AdminError::ValidationError(format!("{} with id {} not found", model_name, id))
			})?;

		let response = DetailResponse {
			model_name: model_name.to_string(),
			data,
		};

		json_response(StatusCode::OK, &response)
	}

	/// Create handler - creates new model instance
	pub async fn create<M: Model>(
		&self,
		model_name: &str,
		request: MutationRequest,
	) -> AdminResult<Response> {
		let model_admin = self.site.get_model_admin(model_name)?;
		let table_name = model_admin.table_name();

		let affected = self
			.db
			.create::<M>(table_name, request.data.clone())
			.await?;

		let response = MutationResponse {
			success: affected > 0,
			message: format!("{} created successfully", model_name),
			affected: Some(affected),
			data: Some(request.data),
		};

		json_response(StatusCode::CREATED, &response)
	}

	/// Update handler - updates existing model instance
	pub async fn update<M: Model>(
		&self,
		model_name: &str,
		id: &str,
		request: MutationRequest,
	) -> AdminResult<Response> {
		let model_admin = self.site.get_model_admin(model_name)?;
		let table_name = model_admin.table_name();
		let pk_field = model_admin.pk_field();

		let affected = self
			.db
			.update::<M>(table_name, pk_field, id, request.data)
			.await?;

		let response = MutationResponse {
			success: affected > 0,
			message: if affected > 0 {
				format!("{} updated successfully", model_name)
			} else {
				format!("{} with id {} not found", model_name, id)
			},
			affected: Some(affected),
			data: None,
		};

		let status = if affected > 0 {
			StatusCode::OK
		} else {
			StatusCode::NOT_FOUND
		};

		json_response(status, &response)
	}

	/// Delete handler - deletes model instance
	pub async fn delete<M: Model>(&self, model_name: &str, id: &str) -> AdminResult<Response> {
		let model_admin = self.site.get_model_admin(model_name)?;
		let table_name = model_admin.table_name();
		let pk_field = model_admin.pk_field();

		let affected = self.db.delete::<M>(table_name, pk_field, id).await?;

		let response = MutationResponse {
			success: affected > 0,
			message: if affected > 0 {
				format!("{} deleted successfully", model_name)
			} else {
				format!("{} with id {} not found", model_name, id)
			},
			affected: Some(affected),
			data: None,
		};

		let status = if affected > 0 {
			StatusCode::OK
		} else {
			StatusCode::NOT_FOUND
		};

		json_response(status, &response)
	}

	/// Bulk delete handler - deletes multiple model instances
	pub async fn bulk_delete<M: Model>(
		&self,
		model_name: &str,
		request: BulkDeleteRequest,
	) -> AdminResult<Response> {
		let model_admin = self.site.get_model_admin(model_name)?;
		let table_name = model_admin.table_name();
		let pk_field = model_admin.pk_field();

		let deleted = self
			.db
			.bulk_delete_by_table(table_name, pk_field, request.ids)
			.await?;

		let response = BulkDeleteResponse {
			success: true,
			deleted,
			message: format!("{} {} deleted successfully", deleted, model_name),
		};

		json_response(StatusCode::OK, &response)
	}

	/// Export handler - exports model data
	pub async fn export<M: Model>(
		&self,
		model_name: &str,
		params: ExportQueryParams,
	) -> AdminResult<Response> {
		let model_admin = self.site.get_model_admin(model_name)?;
		let table_name = model_admin.table_name();

		// Get all items (no pagination for export)
		let items = self.db.list::<M>(table_name, vec![], 0, u64::MAX).await?;

		match params.format {
			ExportFormat::Json => {
				let json = serde_json::to_string_pretty(&items)
					.map_err(|e| AdminError::ValidationError(e.to_string()))?;

				let content_disposition = format!(
					"attachment; filename=\"{}.json\"",
					model_name.to_lowercase()
				);
				let response = Response::new(StatusCode::OK)
					.with_header("Content-Type", "application/json")
					.with_header("Content-Disposition", &content_disposition)
					.with_body(json);

				Ok(response)
			}
			ExportFormat::Csv | ExportFormat::Tsv => {
				let delimiter = if params.format == ExportFormat::Csv {
					b','
				} else {
					b'\t'
				};
				let extension = if params.format == ExportFormat::Csv {
					"csv"
				} else {
					"tsv"
				};

				let mut wtr = csv::WriterBuilder::new()
					.delimiter(delimiter)
					.from_writer(vec![]);

				// Write headers from first item
				if let Some(first) = items.first() {
					let headers: Vec<&str> = first.keys().map(|s| s.as_str()).collect();
					wtr.write_record(&headers)
						.map_err(|e| AdminError::ValidationError(e.to_string()))?;
				}

				// Write data rows
				for item in &items {
					let values: Vec<String> = item
						.values()
						.map(|v| match v {
							serde_json::Value::String(s) => s.clone(),
							_ => v.to_string(),
						})
						.collect();
					wtr.write_record(&values)
						.map_err(|e| AdminError::ValidationError(e.to_string()))?;
				}

				let data = wtr
					.into_inner()
					.map_err(|e| AdminError::ValidationError(e.to_string()))?;

				let content_type = if params.format == ExportFormat::Csv {
					"text/csv"
				} else {
					"text/tab-separated-values"
				};

				let content_disposition = format!(
					"attachment; filename=\"{}.{}\"",
					model_name.to_lowercase(),
					extension
				);
				let response = Response::new(StatusCode::OK)
					.with_header("Content-Type", content_type)
					.with_header("Content-Disposition", &content_disposition)
					.with_body(data);

				Ok(response)
			}
		}
	}

	/// Import handler - imports model data
	///
	/// Parses the request body based on content type and inserts records.
	/// Supports JSON, CSV, and TSV formats.
	///
	/// # Arguments
	///
	/// * `model_name` - Name of the model to import data into
	/// * `content_type` - Content-Type header value
	/// * `body` - Raw request body bytes
	///
	/// # Returns
	///
	/// Returns an `ImportResponse` with counts of imported/skipped/failed records.
	pub async fn import<M: Model>(
		&self,
		model_name: &str,
		content_type: &str,
		body: Vec<u8>,
	) -> AdminResult<Response> {
		// 1. Parse content type to determine format
		let format = ImportFormat::from_content_type(content_type).ok_or_else(|| {
			AdminError::ValidationError(format!(
				"Unsupported content type: {}. Supported types: application/json, text/csv, text/tab-separated-values",
				content_type
			))
		})?;

		// 2. Get model admin for table information
		let model_admin = self.site.get_model_admin(model_name)?;
		let table_name = model_admin.table_name();

		// 3. Parse body based on format
		let builder = ImportBuilder::new(model_name, format).data(body);
		let records = builder.parse()?;

		// 4. Insert records
		let mut result = ImportResult::new();
		let mut row_number = 1usize;

		for record in records {
			// Convert HashMap<String, String> to HashMap<String, serde_json::Value>
			let data: HashMap<String, serde_json::Value> = record
				.into_iter()
				.map(|(k, v)| (k, serde_json::Value::String(v)))
				.collect();

			match self.db.create::<M>(table_name, data).await {
				Ok(_) => {
					result.add_imported();
				}
				Err(e) => {
					let error =
						ImportError::new(row_number, format!("Failed to insert record: {}", e));
					result.add_failed(error);
				}
			}
			row_number += 1;
		}

		// Build response
		let response = ImportResponse {
			success: result.is_successful(),
			imported: result.imported_count as u64,
			updated: result.updated_count as u64,
			skipped: result.skipped_count as u64,
			failed: result.failed_count as u64,
			message: format!(
				"Import completed: {} imported, {} updated, {} skipped, {} failed",
				result.imported_count,
				result.updated_count,
				result.skipped_count,
				result.failed_count
			),
			errors: if result.errors.is_empty() {
				None
			} else {
				Some(result.errors.iter().map(|e| e.message.clone()).collect())
			},
		};

		// Return OK for full success or partial success (imported_count > 0)
		// Return BAD_REQUEST only when no records were imported at all
		let status = if result.is_successful() || result.imported_count > 0 {
			StatusCode::OK
		} else {
			StatusCode::BAD_REQUEST
		};

		json_response(status, &response)
	}
}

/// Helper function to create JSON response
fn json_response<T: Serialize>(status: StatusCode, body: &T) -> AdminResult<Response> {
	let json =
		serde_json::to_string(body).map_err(|e| AdminError::ValidationError(e.to_string()))?;

	let response = Response::new(status)
		.with_header("Content-Type", "application/json")
		.with_body(json);

	Ok(response)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_list_query_params_default() {
		let params = ListQueryParams::default();
		assert!(params.page.is_none());
		assert!(params.page_size.is_none());
		assert!(params.search.is_none());
	}

	#[test]
	fn test_export_format_default() {
		let format = ExportFormat::default();
		assert_eq!(format, ExportFormat::Json);
	}

	#[test]
	fn test_dashboard_response_serialize() {
		let response = DashboardResponse {
			site_name: "Test Admin".to_string(),
			url_prefix: "/admin".to_string(),
			models: vec![ModelInfo {
				name: "User".to_string(),
				list_url: "/admin/user/".to_string(),
			}],
		};

		let json = serde_json::to_string(&response).unwrap();
		assert!(json.contains("Test Admin"));
		assert!(json.contains("User"));
	}

	#[test]
	fn test_mutation_response_serialize() {
		let response = MutationResponse {
			success: true,
			message: "Created".to_string(),
			affected: Some(1),
			data: None,
		};

		let json = serde_json::to_string(&response).unwrap();
		assert!(json.contains("success"));
		assert!(json.contains("true"));
		assert!(!json.contains("data")); // skip_serializing_if = "Option::is_none"
	}

	// ==================== ImportResponse tests ====================

	#[test]
	fn test_import_response_serialize() {
		let response = ImportResponse {
			success: true,
			imported: 10,
			updated: 5,
			skipped: 2,
			failed: 1,
			message: "Import completed".to_string(),
			errors: Some(vec!["Row 5: Invalid email format".to_string()]),
		};

		let json = serde_json::to_string(&response).unwrap();
		assert!(json.contains("\"success\":true"));
		assert!(json.contains("\"imported\":10"));
		assert!(json.contains("\"updated\":5"));
		assert!(json.contains("\"skipped\":2"));
		assert!(json.contains("\"failed\":1"));
		assert!(json.contains("\"message\":\"Import completed\""));
		assert!(json.contains("\"errors\":[\"Row 5: Invalid email format\"]"));
	}

	#[test]
	fn test_import_response_serialize_no_errors() {
		let response = ImportResponse {
			success: true,
			imported: 10,
			updated: 0,
			skipped: 0,
			failed: 0,
			message: "All records imported successfully".to_string(),
			errors: None,
		};

		let json = serde_json::to_string(&response).unwrap();
		assert!(json.contains("\"success\":true"));
		assert!(json.contains("\"imported\":10"));
		// errors field should be skipped when None (skip_serializing_if = "Option::is_none")
		assert!(!json.contains("\"errors\""));
	}

	#[test]
	fn test_import_response_serialize_empty_errors() {
		let response = ImportResponse {
			success: true,
			imported: 5,
			updated: 3,
			skipped: 1,
			failed: 0,
			message: "Import completed".to_string(),
			errors: Some(vec![]),
		};

		let json = serde_json::to_string(&response).unwrap();
		// Empty vec is still serialized (not skipped)
		assert!(json.contains("\"errors\":[]"));
	}

	#[test]
	fn test_import_response_serialize_multiple_errors() {
		let response = ImportResponse {
			success: false,
			imported: 5,
			updated: 0,
			skipped: 0,
			failed: 3,
			message: "Import completed with errors".to_string(),
			errors: Some(vec![
				"Row 1: Missing required field 'name'".to_string(),
				"Row 3: Invalid date format".to_string(),
				"Row 7: Duplicate entry".to_string(),
			]),
		};

		let json = serde_json::to_string(&response).unwrap();
		assert!(json.contains("\"success\":false"));
		assert!(json.contains("\"failed\":3"));
		assert!(json.contains("Missing required field 'name'"));
		assert!(json.contains("Invalid date format"));
		assert!(json.contains("Duplicate entry"));
	}

	#[test]
	fn test_import_response_deserialize_roundtrip() {
		let original = ImportResponse {
			success: true,
			imported: 10,
			updated: 5,
			skipped: 2,
			failed: 1,
			message: "Import completed".to_string(),
			errors: Some(vec!["Error 1".to_string(), "Error 2".to_string()]),
		};

		let json = serde_json::to_string(&original).unwrap();
		let deserialized: serde_json::Value = serde_json::from_str(&json).unwrap();

		// Verify structure
		assert_eq!(deserialized["success"], true);
		assert_eq!(deserialized["imported"], 10);
		assert_eq!(deserialized["updated"], 5);
		assert_eq!(deserialized["skipped"], 2);
		assert_eq!(deserialized["failed"], 1);
		assert_eq!(deserialized["message"], "Import completed");
		assert!(deserialized["errors"].is_array());
		assert_eq!(deserialized["errors"].as_array().unwrap().len(), 2);
	}
}
