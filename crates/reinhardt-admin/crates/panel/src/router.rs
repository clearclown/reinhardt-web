//! Admin router integration
//!
//! This module provides router integration for admin panel,
//! generating UnifiedRouter from AdminSite configuration.

use crate::{
	AdminDatabase, AdminSite,
	handlers::{
		AdminHandlers, BulkDeleteRequest, ExportQueryParams, ListQueryParams, MutationRequest,
	},
};
use hyper::Method;
use reinhardt_db::orm::{DatabaseConnection, Model};
use reinhardt_http::{Error, Request};
use reinhardt_urls::routers::UnifiedRouter;
use std::sync::Arc;

/// Admin router builder
///
/// Builds a UnifiedRouter from an AdminSite with all CRUD endpoints.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_panel::{AdminSite, AdminRouter, AdminDatabase};
/// use reinhardt_db::orm::DatabaseConnection;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let site = AdminSite::new("My Admin");
/// // ... register models ...
///
/// let conn = DatabaseConnection::connect("postgres://localhost/mydb").await?;
/// let router = AdminRouter::new(site, conn)
///     .build();
/// # Ok(())
/// # }
/// ```
pub struct AdminRouter {
	site: Arc<AdminSite>,
	db: AdminDatabase,
	favicon_data: Option<Vec<u8>>,
}

impl AdminRouter {
	/// Create a new admin router builder
	pub fn new(site: AdminSite, db: DatabaseConnection) -> Self {
		Self {
			site: Arc::new(site),
			db: AdminDatabase::new(db),
			favicon_data: None,
		}
	}

	/// Create a new admin router builder from Arc-wrapped site
	pub fn from_arc(site: Arc<AdminSite>, db: AdminDatabase) -> Self {
		Self {
			site,
			db,
			favicon_data: None,
		}
	}

	/// Set favicon data
	pub fn with_favicon(mut self, data: Vec<u8>) -> Self {
		self.favicon_data = Some(data);
		self
	}

	/// Load favicon from file path
	pub fn with_favicon_file(mut self, path: &str) -> Self {
		if let Ok(data) = std::fs::read(path) {
			self.favicon_data = Some(data);
		}
		self
	}

	/// Build the UnifiedRouter with all admin endpoints
	///
	/// Generated endpoints:
	/// - `GET /` - Dashboard (list of registered models)
	/// - `GET /favicon.ico` - Favicon
	/// - `GET /{model}/` - List model instances
	/// - `GET /{model}/{id}/` - Get model instance detail
	/// - `POST /{model}/` - Create model instance
	/// - `PUT /{model}/{id}/` - Update model instance
	/// - `DELETE /{model}/{id}/` - Delete model instance
	/// - `POST /{model}/bulk-delete/` - Bulk delete model instances
	/// - `GET /{model}/export/` - Export model data
	/// - `POST /{model}/import/` - Import model data
	pub fn build(self) -> UnifiedRouter {
		let handlers = Arc::new(
			AdminHandlers::new(Arc::clone(&self.site), self.db.clone())
				.with_favicon(self.favicon_data.clone().unwrap_or_default()),
		);

		let mut router = UnifiedRouter::new().with_namespace("admin");

		// Dashboard endpoint
		let h = Arc::clone(&handlers);
		router = router.function("/", Method::GET, move |_req: Request| {
			let handlers = Arc::clone(&h);
			async move {
				handlers
					.dashboard()
					.await
					.map_err(|e| Error::Internal(e.to_string()))
			}
		});

		// Favicon endpoint
		let h = Arc::clone(&handlers);
		router = router.function("/favicon.ico", Method::GET, move |_req: Request| {
			let handlers = Arc::clone(&h);
			async move {
				handlers
					.favicon()
					.await
					.map_err(|e| Error::NotFound(e.to_string()))
			}
		});

		// Register model-specific endpoints for each registered model
		for model_name in self.site.registered_models() {
			router = self.register_model_routes(router, &model_name, Arc::clone(&handlers));
		}

		router
	}

	/// Register routes for a specific model
	fn register_model_routes(
		&self,
		mut router: UnifiedRouter,
		model_name: &str,
		handlers: Arc<AdminHandlers>,
	) -> UnifiedRouter {
		let model_lower = model_name.to_lowercase();

		// List endpoint: GET /{model}/
		let h = Arc::clone(&handlers);
		let name = model_name.to_string();
		let list_path = format!("/{}/", model_lower);
		router = router.function(&list_path, Method::GET, move |req: Request| {
			let handlers = Arc::clone(&h);
			let model_name = name.clone();
			async move {
				let params = parse_list_query_params(&req);
				handlers
					.list::<DummyModel>(&model_name, params)
					.await
					.map_err(|e| Error::Internal(e.to_string()))
			}
		});

		// Detail endpoint: GET /{model}/{id}/
		let h = Arc::clone(&handlers);
		let name = model_name.to_string();
		let detail_path = format!("/{}/{{id}}/", model_lower);
		router = router.function(&detail_path, Method::GET, move |req: Request| {
			let handlers = Arc::clone(&h);
			let model_name = name.clone();
			async move {
				let id = req.path_params.get("id").cloned().unwrap_or_default();
				handlers
					.detail::<DummyModel>(&model_name, &id)
					.await
					.map_err(|e| Error::NotFound(e.to_string()))
			}
		});

		// Create endpoint: POST /{model}/
		let h = Arc::clone(&handlers);
		let name = model_name.to_string();
		let create_path = format!("/{}/", model_lower);
		router = router.function(&create_path, Method::POST, move |req: Request| {
			let handlers = Arc::clone(&h);
			let model_name = name.clone();
			async move {
				let body = req.body().clone();
				let request: MutationRequest = serde_json::from_slice(&body)
					.map_err(|e| Error::Serialization(e.to_string()))?;
				handlers
					.create::<DummyModel>(&model_name, request)
					.await
					.map_err(|e| Error::Internal(e.to_string()))
			}
		});

		// Update endpoint: PUT /{model}/{id}/
		let h = Arc::clone(&handlers);
		let name = model_name.to_string();
		let update_path = format!("/{}/{{id}}/", model_lower);
		router = router.function(&update_path, Method::PUT, move |req: Request| {
			let handlers = Arc::clone(&h);
			let model_name = name.clone();
			async move {
				let id = req.path_params.get("id").cloned().unwrap_or_default();
				let body = req.body().clone();
				let request: MutationRequest = serde_json::from_slice(&body)
					.map_err(|e| Error::Serialization(e.to_string()))?;
				handlers
					.update::<DummyModel>(&model_name, &id, request)
					.await
					.map_err(|e| Error::Internal(e.to_string()))
			}
		});

		// Delete endpoint: DELETE /{model}/{id}/
		let h = Arc::clone(&handlers);
		let name = model_name.to_string();
		let delete_path = format!("/{}/{{id}}/", model_lower);
		router = router.function(&delete_path, Method::DELETE, move |req: Request| {
			let handlers = Arc::clone(&h);
			let model_name = name.clone();
			async move {
				let id = req.path_params.get("id").cloned().unwrap_or_default();
				handlers
					.delete::<DummyModel>(&model_name, &id)
					.await
					.map_err(|e| Error::Internal(e.to_string()))
			}
		});

		// Bulk delete endpoint: POST /{model}/bulk-delete/
		let h = Arc::clone(&handlers);
		let name = model_name.to_string();
		let bulk_delete_path = format!("/{}/bulk-delete/", model_lower);
		router = router.function(&bulk_delete_path, Method::POST, move |req: Request| {
			let handlers = Arc::clone(&h);
			let model_name = name.clone();
			async move {
				let body = req.body().clone();
				let request: BulkDeleteRequest = serde_json::from_slice(&body)
					.map_err(|e| Error::Serialization(e.to_string()))?;
				handlers
					.bulk_delete::<DummyModel>(&model_name, request)
					.await
					.map_err(|e| Error::Internal(e.to_string()))
			}
		});

		// Export endpoint: GET /{model}/export/
		let h = Arc::clone(&handlers);
		let name = model_name.to_string();
		let export_path = format!("/{}/export/", model_lower);
		router = router.function(&export_path, Method::GET, move |req: Request| {
			let handlers = Arc::clone(&h);
			let model_name = name.clone();
			async move {
				let params = parse_export_query_params(&req);
				handlers
					.export::<DummyModel>(&model_name, params)
					.await
					.map_err(|e| Error::Internal(e.to_string()))
			}
		});

		// Import endpoint: POST /{model}/import/
		let h = Arc::clone(&handlers);
		let name = model_name.to_string();
		let import_path = format!("/{}/import/", model_lower);
		router = router.function(&import_path, Method::POST, move |req: Request| {
			let handlers = Arc::clone(&h);
			let model_name = name.clone();
			async move {
				let content_type = req
					.headers
					.get("content-type")
					.and_then(|h| h.to_str().ok())
					.unwrap_or("application/json");
				let body = req.body().clone();
				handlers
					.import::<DummyModel>(&model_name, content_type, body.to_vec())
					.await
					.map_err(|e| Error::Internal(e.to_string()))
			}
		});

		router
	}
}

/// Parse list query parameters from request
fn parse_list_query_params(req: &Request) -> ListQueryParams {
	let mut params = ListQueryParams::default();

	if let Some(page) = req.query_params.get("page") {
		params.page = page.parse().ok();
	}
	if let Some(page_size) = req.query_params.get("page_size") {
		params.page_size = page_size.parse().ok();
	}
	if let Some(search) = req.query_params.get("search") {
		params.search = Some(search.clone());
	}

	// Copy remaining query params as filters
	for (key, value) in &req.query_params {
		if !["page", "page_size", "search"].contains(&key.as_str()) {
			params.filters.insert(key.clone(), value.clone());
		}
	}

	params
}

/// Parse export query parameters from request
fn parse_export_query_params(req: &Request) -> ExportQueryParams {
	let mut params = ExportQueryParams::default();

	if let Some(format) = req.query_params.get("format") {
		params.format = match format.as_str() {
			"csv" => crate::handlers::ExportFormat::Csv,
			"tsv" => crate::handlers::ExportFormat::Tsv,
			_ => crate::handlers::ExportFormat::Json,
		};
	}

	params
}

/// Dummy model for generic handler calls
///
/// Since AdminDatabase methods require a Model type parameter,
/// we use this dummy model. The actual table operations use
/// the table_name from ModelAdmin config.
#[derive(Clone, serde::Serialize, serde::Deserialize)]
struct DummyModel {
	id: i64,
}

impl Model for DummyModel {
	type PrimaryKey = i64;

	fn table_name() -> &'static str {
		"dummy"
	}

	fn primary_key(&self) -> Option<&Self::PrimaryKey> {
		Some(&self.id)
	}

	fn set_primary_key(&mut self, pk: Self::PrimaryKey) {
		self.id = pk;
	}
}

mod tests {
	#[test]
	fn test_admin_router_new() {
		// This test would require a mock database connection
		// Just verify the struct can be created
	}
}
