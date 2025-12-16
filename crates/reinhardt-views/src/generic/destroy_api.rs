//! DestroyAPIView implementation for deleting objects

use async_trait::async_trait;
use hyper::Method;
use reinhardt_core::exception::{Error, Result};
use reinhardt_core::http::{Request, Response};
use reinhardt_db::orm::{Model, QuerySet};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::core::View;

/// DestroyAPIView for deleting objects
///
/// Similar to Django REST Framework's DestroyAPIView, this view provides
/// delete-only access to model instances.
///
/// # Type Parameters
///
/// * `M` - The model type (must implement `Model`, `Serialize`, `Deserialize`)
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_views::DestroyAPIView;
/// use reinhardt_db::orm::Model;
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
/// let view = DestroyAPIView::<Article>::new()
///     .with_lookup_field("id".to_string());
/// ```
pub struct DestroyAPIView<M>
where
	M: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone,
{
	queryset: Option<QuerySet<M>>,
	lookup_field: String,
	_model: PhantomData<M>,
}

impl<M> DestroyAPIView<M>
where
	M: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone + 'static,
{
	/// Creates a new `DestroyAPIView` with default settings
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use reinhardt_views::DestroyAPIView;
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
	/// let view = DestroyAPIView::<Article>::new();
	/// ```
	pub fn new() -> Self {
		Self {
			queryset: None,
			lookup_field: "pk".to_string(),
			_model: PhantomData,
		}
	}

	/// Sets the queryset for this view
	pub fn with_queryset(mut self, queryset: QuerySet<M>) -> Self {
		self.queryset = Some(queryset);
		self
	}

	/// Sets the lookup field for object retrieval
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// # use reinhardt_views::DestroyAPIView;
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
	/// let view = DestroyAPIView::<Article>::new()
	///     .with_lookup_field("slug".to_string());
	/// ```
	pub fn with_lookup_field(mut self, field: String) -> Self {
		self.lookup_field = field;
		self
	}

	/// Gets the queryset, creating a default one if not set
	// TODO: Will be used when ORM query implementation is complete
	#[allow(dead_code)]
	fn get_queryset(&self) -> QuerySet<M> {
		self.queryset.clone().unwrap_or_default()
	}

	/// Retrieves the object to delete
	// TODO: Will be used when lookup implementation is complete
	#[allow(dead_code)]
	async fn get_object(&self, _request: &Request) -> Result<M> {
		// TODO: Extract lookup value from URL parameters
		// For now, return a placeholder error
		Err(Error::Http("Object lookup not yet implemented".to_string()))
	}

	/// Performs the object deletion
	async fn perform_destroy(&self, _request: &Request) -> Result<()> {
		// TODO: Implement actual object deletion with ORM
		// - Get object via Manager
		// - Delete via Manager
		todo!("Full ORM integration for object deletion")
	}
}

impl<M> Default for DestroyAPIView<M>
where
	M: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone + 'static,
{
	fn default() -> Self {
		Self::new()
	}
}

#[async_trait]
impl<M> View for DestroyAPIView<M>
where
	M: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone + 'static,
{
	async fn dispatch(&self, request: Request) -> Result<Response> {
		match request.method {
			Method::DELETE => {
				self.perform_destroy(&request).await?;

				// Return 204 No Content for successful deletion
				Ok(Response::no_content())
			}
			_ => Err(Error::Http("Method not allowed".to_string())),
		}
	}

	fn allowed_methods(&self) -> Vec<&'static str> {
		vec!["DELETE", "OPTIONS"]
	}
}
