//! ViewSet tests inspired by Django REST Framework's ViewSet tests
//!
//! Tests for ViewSet functionality, action mapping, and extra actions

use crate::View;
use hyper::{Method, StatusCode};
use reinhardt_apps::{Error, Request, Response, Result};
use reinhardt_views_core::{assert_not_found_error, assert_response_status, create_request};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Pagination information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationInfo {
    pub current_page: usize,
    pub total_pages: usize,
    pub has_next: bool,
    pub has_previous: bool,
    pub total_count: usize,
}

/// Test model for API operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiTestModel {
    pub id: i64,
    pub name: String,
    pub value: String,
}

/// Basic ViewSet that implements standard CRUD operations
pub struct BasicViewSet {
    objects: Vec<ApiTestModel>,
    next_id: i64,
}

impl BasicViewSet {
    pub fn new() -> Self {
        Self {
            objects: create_api_test_objects(),
            next_id: 4,
        }
    }

    pub fn with_objects(mut self, objects: Vec<ApiTestModel>) -> Self {
        self.objects = objects;
        self.next_id = (self.objects.len() as i64) + 1;
        self
    }

    fn get_object_by_id(&self, id: i64) -> Result<&ApiTestModel> {
        self.objects
            .iter()
            .find(|obj| obj.id == Some(id))
            .ok_or_else(|| Error::NotFound(format!("Object with id={} not found", id)))
    }

    fn get_object_by_id_mut(&mut self, id: i64) -> Result<&mut ApiTestModel> {
        self.objects
            .iter_mut()
            .find(|obj| obj.id == Some(id))
            .ok_or_else(|| Error::NotFound(format!("Object with id={} not found", id)))
    }
}

#[async_trait::async_trait]
impl View for BasicViewSet {
    async fn dispatch(&self, request: Request) -> Result<Response> {
        // Extract action from URL path or query parameters
        let action = self.get_action(&request)?;

        match action.as_str() {
            "list" => self.list(request).await,
            "retrieve" => self.retrieve(request).await,
            "create" => self.create(request).await,
            "update" => self.update(request).await,
            "partial_update" => self.partial_update(request).await,
            "destroy" => self.destroy(request).await,
            _ => Err(Error::NotFound(format!("Action '{}' not found", action))),
        }
    }
}

impl BasicViewSet {
    fn get_action(&self, request: &Request) -> Result<String> {
        // Simple action detection based on method and path
        match request.method {
            Method::GET => {
                if request.uri.path().ends_with('/') {
                    Ok("list".to_string())
                } else {
                    Ok("retrieve".to_string())
                }
            }
            Method::POST => Ok("create".to_string()),
            Method::PUT => Ok("update".to_string()),
            Method::PATCH => Ok("partial_update".to_string()),
            Method::DELETE => Ok("destroy".to_string()),
            _ => Err(Error::Validation(format!(
                "Method {} not allowed",
                request.method
            ))),
        }
    }

    async fn list(&self, _request: Request) -> Result<Response> {
        let response_data = HashMap::from([
            (
                "results".to_string(),
                serde_json::to_value(&self.objects).unwrap(),
            ),
            (
                "count".to_string(),
                serde_json::Value::Number((self.objects.len() as i64).into()),
            ),
        ]);
        Response::ok().with_json(&response_data)
    }

    async fn retrieve(&self, request: Request) -> Result<Response> {
        let id = self.extract_id_from_path(&request)?;
        let object = self
            .objects
            .iter()
            .find(|obj| obj.id == Some(id))
            .ok_or_else(|| Error::NotFound(format!("Object with id={} not found", id)))?;
        Response::ok().with_json(object)
    }

    async fn create(&self, request: Request) -> Result<Response> {
        let body = request.read_body()?;
        let data: serde_json::Value = serde_json::from_slice(&body)
            .map_err(|_| Error::Validation("Invalid JSON".to_string()))?;

        let mut new_object = ApiTestModel {
            id: Some(self.next_id),
            title: data
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("Untitled")
                .to_string(),
            content: data
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };

        let response_data = HashMap::from([
            (
                "id".to_string(),
                serde_json::Value::Number(new_object.id.unwrap().into()),
            ),
            (
                "title".to_string(),
                serde_json::Value::String(new_object.title.clone()),
            ),
            (
                "content".to_string(),
                serde_json::Value::String(new_object.content.clone()),
            ),
        ]);

        Response::created().with_json(&response_data)
    }

    async fn update(&self, request: Request) -> Result<Response> {
        let id = self.extract_id_from_path(&request)?;
        let _object = self.get_object_by_id(id)?;

        let body = request.read_body()?;
        let data: serde_json::Value = serde_json::from_slice(&body)
            .map_err(|_| Error::Validation("Invalid JSON".to_string()))?;

        let updated_object = ApiTestModel {
            id: Some(id),
            title: data
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("Untitled")
                .to_string(),
            content: data
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };

        Response::ok().with_json(&updated_object)
    }

    async fn partial_update(&self, request: Request) -> Result<Response> {
        let id = self.extract_id_from_path(&request)?;
        let existing_object = self.get_object_by_id(id)?;

        let body = request.read_body()?;
        let data: serde_json::Value = serde_json::from_slice(&body)
            .map_err(|_| Error::Validation("Invalid JSON".to_string()))?;

        let updated_object = ApiTestModel {
            id: Some(id),
            title: data
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or(&existing_object.title)
                .to_string(),
            content: data
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or(&existing_object.content)
                .to_string(),
        };

        Response::ok().with_json(&updated_object)
    }

    async fn destroy(&self, request: Request) -> Result<Response> {
        let _id = self.extract_id_from_path(&request)?;
        let _object = self.get_object_by_id(_id)?;

        Ok(Response::no_content())
    }

    fn extract_id_from_path(&self, request: &Request) -> Result<i64> {
        let path = request.uri.path();
        let id_str = path
            .trim_end_matches('/')
            .split('/')
            .last()
            .ok_or_else(|| Error::Validation("No ID in path".to_string()))?;

        id_str
            .parse::<i64>()
            .map_err(|_| Error::Validation("Invalid ID format".to_string()))
    }
}

/// ViewSet with custom actions
pub struct ActionViewSet {
    objects: Vec<ApiTestModel>,
}

impl ActionViewSet {
    pub fn new() -> Self {
        Self {
            objects: create_api_test_objects(),
        }
    }

    fn get_action(&self, request: &Request) -> Result<String> {
        // Check for custom actions in the path
        let path = request.uri.path();

        if path.ends_with("/custom_action/") {
            Ok("custom_action".to_string())
        } else if path.ends_with("/stats/") {
            Ok("stats".to_string())
        } else if path.ends_with("/invalid_action/") {
            Err(Error::NotFound(
                "Action 'invalid_action' not found".to_string(),
            ))
        } else {
            // Fall back to basic ViewSet actions
            match request.method {
                Method::GET => {
                    if path.ends_with('/') {
                        Ok("list".to_string())
                    } else {
                        Ok("retrieve".to_string())
                    }
                }
                Method::POST => Ok("create".to_string()),
                _ => Err(Error::Validation(format!(
                    "Method {} not allowed",
                    request.method
                ))),
            }
        }
    }
}

#[async_trait::async_trait]
impl View for ActionViewSet {
    async fn dispatch(&self, request: Request) -> Result<Response> {
        let action = self.get_action(&request)?;

        match action.as_str() {
            "list" => self.list(request).await,
            "retrieve" => self.retrieve(request).await,
            "create" => self.create(request).await,
            "custom_action" => self.custom_action(request).await,
            "stats" => self.stats(request).await,
            _ => Err(Error::NotFound(format!("Action '{}' not found", action))),
        }
    }
}

impl ActionViewSet {
    async fn list(&self, _request: Request) -> Result<Response> {
        let response_data = HashMap::from([
            (
                "results".to_string(),
                serde_json::to_value(&self.objects).unwrap(),
            ),
            (
                "count".to_string(),
                serde_json::Value::Number((self.objects.len() as i64).into()),
            ),
        ]);
        Response::ok().with_json(&response_data)
    }

    async fn retrieve(&self, request: Request) -> Result<Response> {
        let id = self.extract_id_from_path(&request)?;
        let object = self
            .objects
            .iter()
            .find(|obj| obj.id == Some(id))
            .ok_or_else(|| Error::NotFound(format!("Object with id={} not found", id)))?;
        Response::ok().with_json(object)
    }

    async fn create(&self, request: Request) -> Result<Response> {
        let body = request.read_body()?;
        let data: serde_json::Value = serde_json::from_slice(&body)
            .map_err(|_| Error::Validation("Invalid JSON".to_string()))?;

        let new_object = ApiTestModel {
            id: Some((self.objects.len() as i64) + 1),
            title: data
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("Untitled")
                .to_string(),
            content: data
                .get("content")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
        };

        Response::created().with_json(&new_object)
    }

    async fn custom_action(&self, _request: Request) -> Result<Response> {
        let response_data = HashMap::from([
            (
                "message".to_string(),
                serde_json::Value::String("Custom action executed".to_string()),
            ),
            (
                "timestamp".to_string(),
                serde_json::Value::String("2023-01-01T00:00:00Z".to_string()),
            ),
        ]);
        Response::ok().with_json(&response_data)
    }

    async fn stats(&self, _request: Request) -> Result<Response> {
        let response_data = HashMap::from([
            (
                "total_objects".to_string(),
                serde_json::Value::Number((self.objects.len() as i64).into()),
            ),
            ("average_title_length".to_string(), {
                let avg_length = if self.objects.is_empty() {
                    0.0
                } else {
                    self.objects
                        .iter()
                        .map(|obj| obj.title.len())
                        .sum::<usize>() as f64
                        / self.objects.len() as f64
                };
                serde_json::Value::Number(serde_json::Number::from_f64(avg_length).unwrap())
            }),
        ]);
        Response::ok().with_json(&response_data)
    }

    fn extract_id_from_path(&self, request: &Request) -> Result<i64> {
        let path = request.uri.path();
        let id_str = path
            .trim_end_matches('/')
            .split('/')
            .last()
            .ok_or_else(|| Error::Validation("No ID in path".to_string()))?;

        id_str
            .parse::<i64>()
            .map_err(|_| Error::Validation("Invalid ID format".to_string()))
    }
}

/// ViewSet with pagination support
pub struct PaginatedViewSet {
    objects: Vec<ApiTestModel>,
    page_size: usize,
}

impl PaginatedViewSet {
    pub fn new() -> Self {
        Self {
            objects: create_large_test_objects(100)
                .into_iter()
                .map(|obj| ApiTestModel {
                    id: obj.id,
                    title: obj.name.clone(),
                    content: format!("Content for {}", obj.name),
                })
                .collect(),
            page_size: 10,
        }
    }

    fn get_page_from_request(&self, request: &Request) -> usize {
        request
            .query_params
            .get("page")
            .and_then(|p| p.parse::<usize>().ok())
            .unwrap_or(1)
    }

    fn get_paginated_objects(&self, page: usize) -> Result<(Vec<&ApiTestModel>, PaginationInfo)> {
        let total_count = self.objects.len();
        let total_pages = if total_count == 0 {
            1
        } else {
            (total_count + self.page_size - 1) / self.page_size
        };

        if page < 1 || page > total_pages {
            return Err(Error::NotFound(format!("Invalid page: {}", page)));
        }

        let start = (page - 1) * self.page_size;
        let end = std::cmp::min(start + self.page_size, total_count);

        let paginated_objects = if start < total_count {
            self.objects[start..end].iter().collect()
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
    }
}

#[async_trait::async_trait]
impl View for PaginatedViewSet {
    async fn dispatch(&self, request: Request) -> Result<Response> {
        if request.method != Method::GET {
            return Err(Error::Validation(format!(
                "Method {} not allowed",
                request.method
            )));
        }

        let page = self.get_page_from_request(&request);
        let (objects, pagination_info) = self.get_paginated_objects(page)?;

        let response_data = HashMap::from([
            (
                "results".to_string(),
                serde_json::to_value(objects).unwrap(),
            ),
            (
                "count".to_string(),
                serde_json::Value::Number(pagination_info.total_count.into()),
            ),
            (
                "next".to_string(),
                if pagination_info.has_next {
                    serde_json::Value::String(format!("?page={}", page + 1))
                } else {
                    serde_json::Value::Null
                },
            ),
            (
                "previous".to_string(),
                if pagination_info.has_previous {
                    serde_json::Value::String(format!("?page={}", page - 1))
                } else {
                    serde_json::Value::Null
                },
            ),
        ]);

        Response::ok().with_json(&response_data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_basic_viewset_list() {
        let view = BasicViewSet::new();
        let request = create_request(Method::GET, "/api/objects/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(&response, "count", &serde_json::Value::Number(3.into()));
    }

    #[tokio::test]
    async fn test_basic_viewset_retrieve() {
        let view = BasicViewSet::new();
        let request = create_request(Method::GET, "/api/objects/1/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_basic_viewset_retrieve_not_found() {
        let view = BasicViewSet::new();
        let request = create_request(Method::GET, "/api/objects/999", None, None, None);
        let result = view.dispatch(request).await;

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn test_basic_viewset_create() {
        let view = BasicViewSet::new();
        let json_data = serde_json::json!({
            "title": "New Post",
            "content": "This is a new post"
        });
        let request = create_json_request(Method::POST, "/api/objects/", &json_data);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::CREATED);
        assert_json_response_contains(
            &response,
            "title",
            &serde_json::Value::String("New Post".to_string()),
        );
    }

    #[tokio::test]
    async fn test_basic_viewset_update() {
        let view = BasicViewSet::new();
        let json_data = serde_json::json!({
            "title": "Updated Post",
            "content": "This is an updated post"
        });
        let request = create_json_request(Method::PUT, "/api/objects/1/", &json_data);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "title",
            &serde_json::Value::String("Updated Post".to_string()),
        );
    }

    #[tokio::test]
    async fn test_basic_viewset_partial_update() {
        let view = BasicViewSet::new();
        let json_data = serde_json::json!({
            "title": "Partially Updated Post"
        });
        let request = create_json_request(Method::PATCH, "/api/objects/1/", &json_data);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "title",
            &serde_json::Value::String("Partially Updated Post".to_string()),
        );
    }

    #[tokio::test]
    async fn test_basic_viewset_destroy() {
        let view = BasicViewSet::new();
        let request = create_request(Method::DELETE, "/api/objects/1/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::NO_CONTENT);
    }

    #[tokio::test]
    async fn test_action_viewset_custom_action() {
        let view = ActionViewSet::new();
        let request = create_request(Method::GET, "/api/objects/custom_action/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "message",
            &serde_json::Value::String("Custom action executed".to_string()),
        );
    }

    #[tokio::test]
    async fn test_action_viewset_stats() {
        let view = ActionViewSet::new();
        let request = create_request(Method::GET, "/api/objects/stats/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(
            &response,
            "total_objects",
            &serde_json::Value::Number(3.into()),
        );
    }

    #[tokio::test]
    async fn test_paginated_viewset_first_page() {
        let view = PaginatedViewSet::new();
        let request = create_request(Method::GET, "/api/objects/", None, None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
        assert_json_response_contains(&response, "count", &serde_json::Value::Number(100.into()));
    }

    #[tokio::test]
    async fn test_paginated_viewset_second_page() {
        let view = PaginatedViewSet::new();
        let mut query_params = HashMap::new();
        query_params.insert("page".to_string(), "2".to_string());
        let request = create_request(Method::GET, "/api/objects/", Some(query_params), None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_paginated_viewset_invalid_page() {
        let view = PaginatedViewSet::new();
        let mut query_params = HashMap::new();
        query_params.insert("page".to_string(), "999".to_string());
        let request = create_request(Method::GET, "/api/objects/", Some(query_params), None, None);
        let result = view.dispatch(request).await;

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn test_basic_viewset_head() {
        let view = BasicViewSet::new();
        let request = create_request(Method::HEAD, "/api/objects/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_basic_viewset_options() {
        let view = BasicViewSet::new();
        let request = create_request(Method::OPTIONS, "/api/objects/", None, None, None);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_action_viewset_invalid_action() {
        let view = ActionViewSet::new();
        let request = create_request(
            Method::GET,
            "/api/objects/invalid_action/",
            None,
            None,
            None,
        );
        let result = view.dispatch(request).await;

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn test_basic_viewset_create_invalid_json() {
        let view = BasicViewSet::new();
        let body = bytes::Bytes::from("invalid json");
        let request = create_request(Method::POST, "/api/objects/", None, None, Some(body));
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_basic_viewset_update_invalid_id() {
        let view = BasicViewSet::new();
        let json_data = serde_json::json!({
            "title": "Updated Post",
            "content": "This is an updated post"
        });
        let request = create_json_request(Method::PUT, "/api/objects/invalid/", &json_data);
        let result = view.dispatch(request).await;

        assert_validation_error(result);
    }

    #[tokio::test]
    async fn test_paginated_viewset_last_page() {
        let view = PaginatedViewSet::new();
        let mut query_params = HashMap::new();
        query_params.insert("page".to_string(), "10".to_string());
        let request = create_request(Method::GET, "/api/objects/", Some(query_params), None, None);
        let response = view.dispatch(request).await.unwrap();

        assert_response_status(&response, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_paginated_viewset_page_zero() {
        let view = PaginatedViewSet::new();
        let mut query_params = HashMap::new();
        query_params.insert("page".to_string(), "0".to_string());
        let request = create_request(Method::GET, "/api/objects/", Some(query_params), None, None);
        let result = view.dispatch(request).await;

        assert_not_found_error(result);
    }

    #[tokio::test]
    async fn test_paginated_viewset_invalid_page_string() {
        let view = PaginatedViewSet::new();
        let mut query_params = HashMap::new();
        query_params.insert("page".to_string(), "invalid".to_string());
        let request = create_request(Method::GET, "/api/objects/", Some(query_params), None, None);
        let response = view.dispatch(request).await.unwrap();

        // Should default to page 1
        assert_response_status(&response, StatusCode::OK);
    }
}
