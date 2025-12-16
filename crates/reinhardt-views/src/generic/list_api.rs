//! ListAPIView implementation for displaying lists of objects

use async_trait::async_trait;
use hyper::Method;
use reinhardt_core::exception::{Error, Result};
use reinhardt_core::http::{Request, Response};
use reinhardt_db::orm::{Model, QuerySet};
use reinhardt_serializers::Serializer;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::core::View;

// TODO: These types will be properly defined when fully integrating with pagination/filtering systems
type PaginationConfig = ();
type FilterConfig = ();

/// ListAPIView for displaying paginated lists of objects
///
/// Similar to Django REST Framework's ListAPIView, this view provides
/// read-only access to a list of model instances with support for
/// pagination, filtering, and ordering.
///
/// # Type Parameters
///
/// * `M` - The model type (must implement `Model`, `Serialize`, `Deserialize`)
/// * `S` - The serializer type (must implement `Serializer`)
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_views::ListAPIView;
/// use reinhardt_db::orm::{Model, QuerySet};
/// use reinhardt_serializers::JsonSerializer;
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Debug, Clone, Serialize, Deserialize)]
/// struct Article {
///     id: Option<i64>,
///     title: String,
///     content: String,
/// }
///
/// impl Model for Article {
///     type PrimaryKey = i64;
///     fn table_name() -> &'static str { "articles" }
///     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
///     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
/// }
///
/// let view = ListAPIView::<Article, JsonSerializer<Article>>::new()
///     .with_paginate_by(10)
///     .with_ordering(vec!["-created_at".into()]);
/// ```
pub struct ListAPIView<M, S>
where
	M: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone,
	S: Serializer<Input = M, Output = String> + Send + Sync,
{
	queryset: Option<QuerySet<M>>,
	pagination_config: Option<PaginationConfig>,
	filter_config: Option<FilterConfig>,
	ordering: Option<Vec<String>>,
	_serializer: PhantomData<S>,
}

impl<M, S> ListAPIView<M, S>
where
	M: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone + 'static,
	S: Serializer<Input = M, Output = String> + Send + Sync + 'static,
{
	/// Creates a new `ListAPIView` with default settings
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use reinhardt_views::ListAPIView;
	/// use reinhardt_serializers::JsonSerializer;
	/// # use reinhardt_db::orm::Model;
	/// # use serde::{Serialize, Deserialize};
	/// # #[derive(Debug, Clone, Serialize, Deserialize)]
	/// # struct Article { id: Option<i64>, title: String }
	/// # impl Model for Article {
	/// #     type PrimaryKey = i64;
	/// #     fn table_name() -> &'static str { "articles" }
	/// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
	/// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
	/// # }
	///
	/// let view = ListAPIView::<Article, JsonSerializer<Article>>::new();
	/// ```
	pub fn new() -> Self {
		Self {
			queryset: None,
			pagination_config: None,
			filter_config: None,
			ordering: None,
			_serializer: PhantomData,
		}
	}

	/// Sets the queryset for this view
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// # use reinhardt_views::ListAPIView;
	/// # use reinhardt_db::orm::{Model, QuerySet};
	/// # use reinhardt_serializers::JsonSerializer;
	/// # use serde::{Serialize, Deserialize};
	/// # #[derive(Debug, Clone, Serialize, Deserialize)]
	/// # struct Article { id: Option<i64>, title: String }
	/// # impl Model for Article {
	/// #     type PrimaryKey = i64;
	/// #     fn table_name() -> &'static str { "articles" }
	/// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
	/// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
	/// # }
	///
	/// let queryset = QuerySet::<Article>::new();
	/// let view = ListAPIView::<Article, JsonSerializer<Article>>::new()
	///     .with_queryset(queryset);
	/// ```
	pub fn with_queryset(mut self, queryset: QuerySet<M>) -> Self {
		self.queryset = Some(queryset);
		self
	}

	/// Sets the number of items per page for pagination
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// # use reinhardt_views::ListAPIView;
	/// # use reinhardt_serializers::JsonSerializer;
	/// # use reinhardt_db::orm::Model;
	/// # use serde::{Serialize, Deserialize};
	/// # #[derive(Debug, Clone, Serialize, Deserialize)]
	/// # struct Article { id: Option<i64>, title: String }
	/// # impl Model for Article {
	/// #     type PrimaryKey = i64;
	/// #     fn table_name() -> &'static str { "articles" }
	/// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
	/// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
	/// # }
	///
	/// let view = ListAPIView::<Article, JsonSerializer<Article>>::new()
	///     .with_paginate_by(20);
	/// ```
	pub fn with_paginate_by(mut self, _page_size: usize) -> Self {
		// TODO: Implement pagination configuration
		self.pagination_config = Some(());
		self
	}

	/// Sets the ordering for the queryset
	///
	/// Fields can be prefixed with `-` for descending order.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// # use reinhardt_views::ListAPIView;
	/// # use reinhardt_serializers::JsonSerializer;
	/// # use reinhardt_db::orm::Model;
	/// # use serde::{Serialize, Deserialize};
	/// # #[derive(Debug, Clone, Serialize, Deserialize)]
	/// # struct Article { id: Option<i64>, title: String }
	/// # impl Model for Article {
	/// #     type PrimaryKey = i64;
	/// #     fn table_name() -> &'static str { "articles" }
	/// #     fn primary_key(&self) -> Option<&Self::PrimaryKey> { self.id.as_ref() }
	/// #     fn set_primary_key(&mut self, value: Self::PrimaryKey) { self.id = Some(value); }
	/// # }
	///
	/// let view = ListAPIView::<Article, JsonSerializer<Article>>::new()
	///     .with_ordering(vec!["-created_at".into(), "title".into()]);
	/// ```
	pub fn with_ordering(mut self, ordering: Vec<String>) -> Self {
		self.ordering = Some(ordering);
		self
	}

	/// Sets the filter configuration
	pub fn with_filter_config(mut self, filter_config: FilterConfig) -> Self {
		self.filter_config = Some(filter_config);
		self
	}

	/// Gets the queryset, creating a default one if not set
	fn get_queryset(&self) -> QuerySet<M> {
		self.queryset.clone().unwrap_or_default()
	}

	/// Gets the objects to display
	async fn get_objects(&self, _request: &Request) -> Result<Vec<M>> {
		let mut queryset = self.get_queryset();

		// Apply ordering if configured
		if let Some(ref ordering) = self.ordering {
			let order_fields: Vec<&str> = ordering.iter().map(|s| s.as_str()).collect();
			queryset = queryset.order_by(&order_fields);
		}

		// TODO: Apply filtering based on request parameters

		// TODO: Apply pagination based on request parameters

		// For now, return all objects (pagination will be added later)
		queryset.all().await.map_err(|e| Error::Http(e.to_string()))
	}
}

impl<M, S> Default for ListAPIView<M, S>
where
	M: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone + 'static,
	S: Serializer<Input = M, Output = String> + Send + Sync + 'static,
{
	fn default() -> Self {
		Self::new()
	}
}

#[async_trait]
impl<M, S> View for ListAPIView<M, S>
where
	M: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone + 'static,
	S: Serializer<Input = M, Output = String> + Send + Sync + 'static + Default,
{
	async fn dispatch(&self, request: Request) -> Result<Response> {
		match request.method {
			Method::GET | Method::HEAD => {
				let objects = self.get_objects(&request).await?;

				// Serialize the objects
				let serializer = S::default();
				let serialized = objects
					.iter()
					.map(|obj| {
						serializer
							.serialize(obj)
							.map_err(|e| Error::Http(e.to_string()))
					})
					.collect::<Result<Vec<_>>>()?;

				// Build response with pagination metadata
				let response_body = if self.pagination_config.is_some() {
					// TODO: Implement proper pagination
					serde_json::json!({
						"count": serialized.len(),
						"page": 1,
						"page_size": 10,
						"total_pages": 1,
						"next": null,
						"previous": null,
						"results": serialized.iter().map(|s| {
							serde_json::from_str::<serde_json::Value>(s)
								.unwrap_or(serde_json::Value::Null)
						}).collect::<Vec<_>>()
					})
				} else {
					serde_json::json!(
						serialized
							.iter()
							.map(|s| {
								serde_json::from_str::<serde_json::Value>(s)
									.unwrap_or(serde_json::Value::Null)
							})
							.collect::<Vec<_>>()
					)
				};

				Response::ok().with_json(&response_body)
			}
			_ => Err(Error::Http("Method not allowed".to_string())),
		}
	}

	fn allowed_methods(&self) -> Vec<&'static str> {
		vec!["GET", "HEAD", "OPTIONS"]
	}
}
