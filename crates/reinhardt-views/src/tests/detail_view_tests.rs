//! Detail view tests inspired by Django's DetailView tests
//!
//! Tests for single object display, URL parameter handling, and context management

use crate::{Context, SingleObjectMixin, View};
use hyper::StatusCode;
use reinhardt_apps::{Error, Request, Response, Result};
use reinhardt_orm::Model;
use reinhardt_serializers::{JsonSerializer, Serializer};
use reinhardt_views_core::{assert_response_status, create_request};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;

/// Detail view with custom object lookup for testing
pub struct CustomDetailView<T, S>
where
    T: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone,
    S: Serializer<T> + Send + Sync,
{
    object: Option<T>,
    slug_field: String,
    pk_url_kwarg_name: String,
    slug_url_kwarg_name: String,
    context_object_name: Option<String>,
    _serializer: PhantomData<S>,
}

impl<T, S> CustomDetailView<T, S>
where
    T: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone,
    S: Serializer<T> + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            object: None,
            slug_field: "slug".to_string(),
            pk_url_kwarg_name: "pk".to_string(),
            slug_url_kwarg_name: "slug".to_string(),
            context_object_name: None,
            _serializer: PhantomData,
        }
    }

    pub fn with_object(mut self, object: T) -> Self {
        self.object = Some(object);
        self
    }

    pub fn with_slug_field(mut self, slug_field: impl Into<String>) -> Self {
        self.slug_field = slug_field.into();
        self
    }

    pub fn with_pk_url_kwarg(mut self, kwarg: impl Into<String>) -> Self {
        self.pk_url_kwarg_name = kwarg.into();
        self
    }

    pub fn with_slug_url_kwarg(mut self, kwarg: impl Into<String>) -> Self {
        self.slug_url_kwarg_name = kwarg.into();
        self
    }

    pub fn with_context_object_name(mut self, name: impl Into<String>) -> Self {
        self.context_object_name = Some(name.into());
        self
    }

    fn get_object_by_pk(&self, pk: &str) -> Result<T> {
        // In a real implementation, this would query the database
        // For testing, we'll simulate object lookup
        if let Some(ref object) = self.object {
            if let Some(object_pk) = object.primary_key() {
                if object_pk.to_string() == pk {
                    return Ok(object.clone());
                }
            }
        }
        Err(Error::NotFound(format!(
            "Object with pk='{}' not found",
            pk
        )))
    }

    fn get_object_by_slug(&self, slug: &str) -> Result<T> {
        // In a real implementation, this would query the database by slug
        // For testing, we'll simulate object lookup
        if let Some(ref object) = self.object {
            // Simple slug matching for testing
            if let Some(slug_value) = get_field_value(object, &self.slug_field) {
                if slug_value == slug {
                    return Ok(object.clone());
                }
            }
        }
        Err(Error::NotFound(format!(
            "Object with slug='{}' not found",
            slug
        )))
    }
}

// Helper function to get field values
fn get_field_value<T>(obj: &T, field: &str) -> Option<String>
where
    T: Serialize,
{
    /// let value = serde_json::to_value(obj).ok()?;
    match field {
        "slug" => value.get("slug")?.as_str().map(|s| s.to_string()),
        _ => None,
    }
}

#[async_trait::async_trait]
impl<T, S> SingleObjectMixin<T> for CustomDetailView<T, S>
where
    T: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone,
    S: Serializer<T> + Send + Sync,
{
    async fn get_object(&self, request: &Request) -> Result<T> {
        // Try to extract pk or slug from URL parameters
        let pk_kwarg = self.pk_url_kwarg();
        let slug_kwarg = self.slug_url_kwarg();

        // Try to get pk from URL parameters
        if let Some(pk_value) = request.path_params.get(pk_kwarg) {
            return self.get_object_by_pk(pk_value);
        }

        // Try to get slug from URL parameters
        if let Some(slug_value) = request.path_params.get(slug_kwarg) {
            return self.get_object_by_slug(slug_value);
        }

        // If no pk or slug in URL, return the stored object if available
        if let Some(ref object) = self.object {
            return Ok(object.clone());
        }

        Err(Error::NotFound(
            "Object not found - no pk or slug provided in URL".to_string(),
        ))
    }

    fn get_slug_field(&self) -> &str {
        &self.slug_field
    }

    fn pk_url_kwarg(&self) -> &str {
        &self.pk_url_kwarg_name
    }

    fn slug_url_kwarg(&self) -> &str {
        &self.slug_url_kwarg_name
    }

    fn get_context_object_name(&self) -> Option<&str> {
        self.context_object_name.as_deref()
    }

    fn get_context_data(&self, object: T) -> Result<Context> {
        let mut context = Context::new();
        context.insert("object".to_string(), serde_json::to_value(&object).unwrap());

        if let Some(name) = self.get_context_object_name() {
            context.insert(name.to_string(), serde_json::to_value(&object).unwrap());
        }

        Ok(context)
    }
}

#[async_trait::async_trait]
impl<T, S> View for CustomDetailView<T, S>
where
    T: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone + 'static,
    S: Serializer<T> + Send + Sync + Default + 'static,
{
    async fn dispatch(&self, request: Request) -> Result<Response> {
        if request.method != hyper::Method::GET {
            return Err(Error::Validation(format!(
                "Method {} not allowed",
                request.method
            )));
        }

        // Get object - extracts pk/slug from URL parameters
        let object = self.get_object(&request).await?;

        // Serialize object
        let serializer = S::default();
        let serialized = serializer.serialize(&object)?;

        // Build response
        Response::ok().with_json(&serialized)
    }
}

/// Detail view that raises an exception for testing error handling
pub struct ErrorDetailView;

#[async_trait::async_trait]
impl View for ErrorDetailView {
    async fn dispatch(&self, _request: Request) -> Result<Response> {
        Err(Error::Internal("Test error".to_string()))
    }
}

/// Detail view with custom context handling
pub struct ContextDetailView<T, S>
where
    T: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone,
    S: Serializer<T> + Send + Sync,
{
    object: T,
    context_object_name: Option<String>,
    _serializer: PhantomData<S>,
}

impl<T, S> ContextDetailView<T, S>
where
    T: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone,
    S: Serializer<T> + Send + Sync,
{
    pub fn new(object: T) -> Self {
        Self {
            object,
            context_object_name: None,
            _serializer: PhantomData,
        }
    }

    pub fn with_context_object_name(mut self, name: impl Into<String>) -> Self {
        self.context_object_name = Some(name.into());
        self
    }

    fn get_context_data(&self, object: T) -> Result<Context> {
        let mut context = Context::new();
        context.insert("object".to_string(), serde_json::to_value(&object).unwrap());

        if let Some(name) = &self.context_object_name {
            context.insert(name.clone(), serde_json::to_value(&object).unwrap());
        }

        Ok(context)
    }
}

#[async_trait::async_trait]
impl<T, S> View for ContextDetailView<T, S>
where
    T: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone + 'static,
    S: Serializer<T> + Send + Sync + Default + 'static,
{
    async fn dispatch(&self, request: Request) -> Result<Response> {
        if request.method != hyper::Method::GET {
            return Err(Error::Validation(format!(
                "Method {} not allowed",
                request.method
            )));
        }

        // Get context data
        let context = self.get_context_data(self.object.clone())?;

        // Serialize object
        let serializer = S::default();
        let serialized = serializer.serialize(&self.object)?;

        // Build response with context
        let mut response_data = serde_json::Map::new();
        response_data.insert(
            "object".to_string(),
            serde_json::to_value(serialized).unwrap(),
        );
        response_data.insert(
            "context".to_string(),
            serde_json::to_value(context).unwrap(),
        );

        Response::ok().with_json(&response_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::Method;

    #[tokio::test]
    async fn test_detail_view_with_object() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view = CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_object(object.clone());

        let request = create_request(Method::GET, "/detail/1/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_detail_view_by_pk() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view =
            CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new().with_object(object);

        let mut path_params = HashMap::new();
        path_params.insert("pk".to_string(), "1".to_string());
        let request = create_request_with_path_params(
            Method::GET,
            "/detail/1/",
            path_params,
            None,
            None,
            None,
        );
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_detail_view_by_slug() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view =
            CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new().with_object(object);

        let mut path_params = HashMap::new();
        path_params.insert("slug".to_string(), "test-object".to_string());
        let request = create_request_with_path_params(
            Method::GET,
            "/detail/test-object/",
            path_params,
            None,
            None,
            None,
        );
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_detail_view_missing_object() {
        let view = CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new();

        let mut path_params = HashMap::new();
        path_params.insert("pk".to_string(), "500".to_string());
        let request = create_request_with_path_params(
            Method::GET,
            "/detail/500/",
            path_params,
            None,
            None,
            None,
        );
        let result = view.dispatch(request).await;

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn test_detail_view_post_not_allowed() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view =
            CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new().with_object(object);

        let request = create_request(Method::POST, "/detail/1/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_detail_view_custom_slug_field() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view = CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_object(object)
            .with_slug_field("slug");

        let mut path_params = HashMap::new();
        path_params.insert("slug".to_string(), "test-object".to_string());
        let request = create_request_with_path_params(
            Method::GET,
            "/detail/test-object/",
            path_params,
            None,
            None,
            None,
        );
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_detail_view_custom_pk_url_kwarg() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view = CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_object(object)
            .with_pk_url_kwarg("id");

        let mut path_params = HashMap::new();
        path_params.insert("id".to_string(), "1".to_string());
        let request = create_request_with_path_params(
            Method::GET,
            "/detail/1/",
            path_params,
            None,
            None,
            None,
        );
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_detail_view_custom_slug_url_kwarg() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view = CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_object(object)
            .with_slug_url_kwarg("custom_slug");

        let mut path_params = HashMap::new();
        path_params.insert("custom_slug".to_string(), "test-object".to_string());
        let request = create_request_with_path_params(
            Method::GET,
            "/detail/test-object/",
            path_params,
            None,
            None,
            None,
        );
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_detail_view_context_object_name() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view = CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_object(object)
            .with_context_object_name("item");

        let request = create_request(Method::GET, "/detail/1/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_detail_view_verbose_name() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view = CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_object(object)
            .with_context_object_name("testmodel");

        let request = create_request(Method::GET, "/detail/1/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_detail_view_no_pk_or_slug() {
        let view = CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new();

        let request = create_request(Method::GET, "/detail/", None, None, None);
        let result = view.dispatch(request).await;

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn test_detail_view_slug_not_found() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view =
            CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new().with_object(object);

        let mut path_params = HashMap::new();
        path_params.insert("slug".to_string(), "non-existent".to_string());
        let request = create_request_with_path_params(
            Method::GET,
            "/detail/non-existent/",
            path_params,
            None,
            None,
            None,
        );
        let result = view.dispatch(request).await;

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn test_context_detail_view() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view = ContextDetailView::<TestModel, JsonSerializer<TestModel>>::new(object)
            .with_context_object_name("item");

        let request = create_request(Method::GET, "/detail/1/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_context_detail_view_without_context_object_name() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view = ContextDetailView::<TestModel, JsonSerializer<TestModel>>::new(object);

        let request = create_request(Method::GET, "/detail/1/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_error_detail_view() {
        let view = ErrorDetailView;
        let request = create_request(Method::GET, "/detail/1/", None, None, None);
        let result = view.dispatch(request).await;

        assert_internal_error(result);
    }

    #[tokio::test]
    async fn test_detail_view_head() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view =
            CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new().with_object(object);

        let request = create_request(Method::HEAD, "/detail/1/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_detail_view_options() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view =
            CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new().with_object(object);

        let request = create_request(Method::OPTIONS, "/detail/1/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_detail_view_put() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view =
            CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new().with_object(object);

        let request = create_request(Method::PUT, "/detail/1/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_detail_view_patch() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view =
            CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new().with_object(object);

        let request = create_request(Method::PATCH, "/detail/1/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_detail_view_delete() {
        let object = TestModel {
            id: Some(1),
            name: "Test Object".to_string(),
            slug: "test-object".to_string(),
            created_at: "2023-01-01T00:00:00Z".to_string(),
        };
        let view =
            CustomDetailView::<TestModel, JsonSerializer<TestModel>>::new().with_object(object);

        let request = create_request(Method::DELETE, "/detail/1/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }
}
