//! List view tests inspired by Django's ListView tests
//!
//! Tests for object list display, pagination, ordering, and context handling

use crate::{Context, MultipleObjectMixin, View};
use hyper::StatusCode;
use reinhardt_apps::{Error, Request, Response, Result};
use reinhardt_orm::Model;
use reinhardt_serializers::{JsonSerializer, Serializer};
use reinhardt_views_core::{assert_response_status, create_request};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::marker::PhantomData;

/// Pagination info for testing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaginationInfo {
    pub current_page: usize,
    pub total_pages: usize,
    pub has_next: bool,
    pub has_previous: bool,
    pub total_count: usize,
}

/// List view with pagination support for testing
pub struct PaginatedListView<T, S>
where
    T: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone,
    S: Serializer<T> + Send + Sync,
{
    objects: Vec<T>,
    paginate_by: Option<usize>,
    allow_empty: bool,
    context_object_name: Option<String>,
    ordering: Option<Vec<String>>,
    _serializer: PhantomData<S>,
}

impl<T, S> PaginatedListView<T, S>
where
    T: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone,
    S: Serializer<T> + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            paginate_by: None,
            allow_empty: true,
            context_object_name: None,
            ordering: None,
            _serializer: PhantomData,
        }
    }

    pub fn with_objects(mut self, objects: Vec<T>) -> Self {
        self.objects = objects;
        self
    }

    pub fn with_paginate_by(mut self, paginate_by: usize) -> Self {
        self.paginate_by = Some(paginate_by);
        self
    }

    pub fn with_allow_empty(mut self, allow_empty: bool) -> Self {
        self.allow_empty = allow_empty;
        self
    }

    pub fn with_context_object_name(mut self, name: impl Into<String>) -> Self {
        self.context_object_name = Some(name.into());
        self
    }

    pub fn with_ordering(mut self, ordering: Vec<String>) -> Self {
        self.ordering = Some(ordering);
        self
    }

    fn get_paginated_objects(&self, page: usize) -> Result<(Vec<T>, PaginationInfo)> {
        let total_count = self.objects.len();

        if let Some(paginate_by) = self.paginate_by {
            let total_pages = if total_count == 0 {
                1
            } else {
                (total_count + paginate_by - 1) / paginate_by
            };

            if page < 1 || page > total_pages {
                return Err(Error::NotFound(format!("Invalid page: {}", page)));
            }

            let start = (page - 1) * paginate_by;
            let end = std::cmp::min(start + paginate_by, total_count);

            let paginated_objects = if start < total_count {
                self.objects[start..end].to_vec()
            } else {
                Vec::new()
            };

            let pagination_info = PaginationInfo {
                current_page: page,
                total_pages,
                has_next: page < total_pages,
                has_previous: page > 1,
                total_count,
            };

            Ok((paginated_objects, pagination_info))
        } else {
            let pagination_info = PaginationInfo {
                current_page: 1,
                total_pages: 1,
                has_next: false,
                has_previous: false,
                total_count,
            };

            Ok((self.objects.clone(), pagination_info))
        }
    }

    fn apply_ordering(&self, mut objects: Vec<T>) -> Vec<T> {
        if let Some(ref ordering) = self.ordering {
            // Simple ordering implementation for testing
            objects.sort_by(|a, b| {
                for field in ordering {
                    let field_name = field.trim_start_matches('-');
                    let reverse = field.starts_with('-');

                    // Simple field comparison for testing
                    let comparison = match field_name {
                        "name" => {
                            if let (Some(a_name), Some(b_name)) =
                                (get_field_value(a, "name"), get_field_value(b, "name"))
                            {
                                a_name.cmp(&b_name)
                            } else {
                                std::cmp::Ordering::Equal
                            }
                        }
                        "created_at" => {
                            if let (Some(a_date), Some(b_date)) = (
                                get_field_value(a, "created_at"),
                                get_field_value(b, "created_at"),
                            ) {
                                a_date.cmp(&b_date)
                            } else {
                                std::cmp::Ordering::Equal
                            }
                        }
                        _ => std::cmp::Ordering::Equal,
                    };

                    if reverse {
                        return comparison.reverse();
                    } else {
                        return comparison;
                    }
                }
                std::cmp::Ordering::Equal
            });
        }
        objects
    }
}

// Helper function to get field values for ordering
fn get_field_value<T>(obj: &T, field: &str) -> Option<String>
where
    T: Serialize,
{
    /// let value = serde_json::to_value(obj).ok()?;
    match field {
        "name" => value.get("name")?.as_str().map(|s| s.to_string()),
        "created_at" => value.get("created_at")?.as_str().map(|s| s.to_string()),
        _ => None,
    }
}

#[async_trait::async_trait]
impl<T, S> MultipleObjectMixin<T> for PaginatedListView<T, S>
where
    T: Model + Serialize + for<'de> Deserialize<'de> + Send + Sync + Clone,
    S: Serializer<T> + Send + Sync,
{
    async fn get_objects(&self) -> Result<Vec<T>> {
        let mut objects = self.objects.clone();
        objects = self.apply_ordering(objects);
        Ok(objects)
    }

    fn get_ordering(&self) -> Option<Vec<String>> {
        self.ordering.clone()
    }

    fn allow_empty(&self) -> bool {
        self.allow_empty
    }

    fn get_paginate_by(&self) -> Option<usize> {
        self.paginate_by
    }

    fn get_context_object_name(&self) -> Option<&str> {
        self.context_object_name.as_deref()
    }

    fn get_context_data(&self, object_list: Vec<T>) -> Result<Context> {
        let mut context = Context::new();
        context.insert(
            "object_list".to_string(),
            serde_json::to_value(&object_list).unwrap(),
        );

        if let Some(name) = self.get_context_object_name() {
            context.insert(
                name.to_string(),
                serde_json::to_value(&object_list).unwrap(),
            );
        }

        Ok(context)
    }
}

#[async_trait::async_trait]
impl<T, S> View for PaginatedListView<T, S>
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

        // Get page number from query parameters
        let page = request
            .query_params
            .get("page")
            .and_then(|p| p.parse::<usize>().ok())
            .unwrap_or(1);

        // Get objects
        let object_list = self.get_objects().await?;

        // Check if empty is allowed
        if !self.allow_empty() && object_list.is_empty() {
            return Err(Error::NotFound(
                "Empty list and allow_empty is false".to_string(),
            ));
        }

        // Handle pagination
        let (paginated_objects, pagination_info) = if let Some(paginate_by) = self.get_paginate_by()
        {
            self.get_paginated_objects(page)?
        } else {
            (
                object_list,
                PaginationInfo {
                    current_page: 1,
                    total_pages: 1,
                    has_next: false,
                    has_previous: false,
                    total_count: self.objects.len(),
                },
            )
        };

        // Build context
        let mut context = self.get_context_data(paginated_objects.clone())?;
        context.insert(
            "paginator".to_string(),
            serde_json::to_value(&pagination_info).unwrap(),
        );
        context.insert(
            "page_obj".to_string(),
            serde_json::to_value(&pagination_info).unwrap(),
        );
        context.insert(
            "is_paginated".to_string(),
            serde_json::to_value(pagination_info.total_pages > 1).unwrap(),
        );

        // Serialize objects
        let serializer = S::default();
        let serialized_objects: Result<Vec<_>> = paginated_objects
            .iter()
            .map(|obj| serializer.serialize(obj))
            .collect();

        let serialized_objects = serialized_objects?;

        // Build response
        Response::ok().with_json(&serialized_objects)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::Method;

    #[tokio::test]
    async fn test_list_view_basic() {
        let objects = create_test_objects();
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects.clone());

        let request = create_request(Method::GET, "/list/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_post_not_allowed() {
        let objects = create_test_objects();
        let view =
            PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new().with_objects(objects);

        let request = create_request(Method::POST, "/list/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_list_view_pagination() {
        let objects = create_large_test_objects(100);
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_paginate_by(10);

        let request = create_request(Method::GET, "/list/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_pagination_page_2() {
        let objects = create_large_test_objects(100);
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_paginate_by(10);

        let mut query_params = HashMap::new();
        query_params.insert("page".to_string(), "2".to_string());
        let request = create_request(Method::GET, "/list/", Some(query_params), None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_pagination_invalid_page() {
        let objects = create_large_test_objects(10);
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_paginate_by(10);

        let mut query_params = HashMap::new();
        query_params.insert("page".to_string(), "42".to_string());
        let request = create_request(Method::GET, "/list/", Some(query_params), None, None);
        let result = view.dispatch(request).await;

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn test_list_view_ordering() {
        let objects = create_test_objects();
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_ordering(vec!["name".to_string()]);

        let request = create_request(Method::GET, "/list/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_ordering_reverse() {
        let objects = create_test_objects();
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_ordering(vec!["-name".to_string()]);

        let request = create_request(Method::GET, "/list/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_allow_empty_true() {
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(Vec::new())
            .with_allow_empty(true);

        let request = create_request(Method::GET, "/list/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_allow_empty_false() {
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(Vec::new())
            .with_allow_empty(false);

        let request = create_request(Method::GET, "/list/", None, None, None);
        let result = view.dispatch(request).await;

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn test_list_view_context_object_name() {
        let objects = create_test_objects();
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_context_object_name("items");

        let request = create_request(Method::GET, "/list/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_verbose_name() {
        let objects = create_test_objects();
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_context_object_name("testmodel_list");

        let request = create_request(Method::GET, "/list/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_pagination_info() {
        let objects = create_large_test_objects(25);
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_paginate_by(10);

        let request = create_request(Method::GET, "/list/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_pagination_last_page() {
        let objects = create_large_test_objects(25);
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_paginate_by(10);

        let mut query_params = HashMap::new();
        query_params.insert("page".to_string(), "3".to_string());
        let request = create_request(Method::GET, "/list/", Some(query_params), None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_pagination_orphaned() {
        let objects = create_large_test_objects(92);
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_paginate_by(30);

        let request = create_request(Method::GET, "/list/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_multiple_ordering() {
        let objects = create_test_objects();
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_ordering(vec!["-created_at".to_string(), "name".to_string()]);

        let request = create_request(Method::GET, "/list/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_no_pagination() {
        let objects = create_test_objects();
        let view =
            PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new().with_objects(objects);

        let request = create_request(Method::GET, "/list/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_empty_with_pagination() {
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(Vec::new())
            .with_paginate_by(10)
            .with_allow_empty(true);

        let request = create_request(Method::GET, "/list/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_view_page_zero() {
        let objects = create_large_test_objects(10);
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_paginate_by(10);

        let mut query_params = HashMap::new();
        query_params.insert("page".to_string(), "0".to_string());
        let request = create_request(Method::GET, "/list/", Some(query_params), None, None);
        let result = view.dispatch(request).await;

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn test_list_view_invalid_page_string() {
        let objects = create_large_test_objects(10);
        let view = PaginatedListView::<TestModel, JsonSerializer<TestModel>>::new()
            .with_objects(objects)
            .with_paginate_by(10);

        let mut query_params = HashMap::new();
        query_params.insert("page".to_string(), "invalid".to_string());
        let request = create_request(Method::GET, "/list/", Some(query_params), None, None);
        let response = view.dispatch(request).await.unwrap();

        // Should default to page 1
        assert_response_status(&response, StatusCode::OK);
    }
}
