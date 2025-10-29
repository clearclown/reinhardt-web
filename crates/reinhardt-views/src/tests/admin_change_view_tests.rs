//! Unit tests for AdminChangeView

use crate::{admin::views::AdminChangeView, View};
use reinhardt_apps::Request;
use reinhardt_orm::{Model, QuerySet};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct TestArticle {
    id: Option<i64>,
    title: String,
    content: String,
}

impl Model for TestArticle {
    type PrimaryKey = i64;

    fn table_name() -> &'static str {
        "articles"
    }

    fn primary_key(&self) -> Option<&Self::PrimaryKey> {
        self.id.as_ref()
    }

    fn set_primary_key(&mut self, value: Self::PrimaryKey) {
        self.id = Some(value);
    }
}

#[test]
fn test_admin_change_view_new() {
    let queryset = QuerySet::<TestArticle>::new();
    let _view = AdminChangeView::new(queryset);
}

#[test]
fn test_get_object_returns_not_found_for_empty_queryset() {
    let queryset = QuerySet::<TestArticle>::new();
    let view = AdminChangeView::new(queryset);

    let result = view.get_object("1");
    assert!(result.is_err());

    if let Err(Error::NotFound(msg)) = result {
        assert!(msg.contains("not found"));
    } else {
        panic!("Expected NotFound error");
    }
}

#[test]
fn test_update_object_returns_not_found_for_missing_record() {
    let queryset = QuerySet::<TestArticle>::new();
    let view = AdminChangeView::new(queryset);

    let article = TestArticle {
        id: Some(1),
        title: "Updated".to_string(),
        content: "Content".to_string(),
    };

    let result = view.update_object("1", article);
    assert!(result.is_err());

    if let Err(Error::NotFound(_)) = result {
        // Expected
    } else {
        panic!("Expected NotFound error");
    }
}

#[tokio::test]
async fn test_dispatch_missing_id_parameter() {
    let queryset = QuerySet::<TestArticle>::new();
    let view = AdminChangeView::new(queryset);

    // Create a GET request without path parameters
    let request = Request::new(
        hyper::Method::GET,
        "/admin/article/change/".parse::<hyper::Uri>().unwrap(),
        hyper::Version::HTTP_11,
        hyper::HeaderMap::new(),
        bytes::Bytes::new(),
    );

    let result = view.dispatch(request).await;
    assert!(result.is_err());

    if let Err(Error::Validation(msg)) = result {
        assert!(msg.contains("Missing 'id' or 'pk' parameter"));
    } else {
        panic!("Expected Validation error for missing ID parameter");
    }
}

#[tokio::test]
async fn test_dispatch_options_method() {
    let queryset = QuerySet::<TestArticle>::new();
    let view = AdminChangeView::new(queryset);

    let request = Request::new(
        hyper::Method::OPTIONS,
        "/admin/article/change/1/".parse::<hyper::Uri>().unwrap(),
        hyper::Version::HTTP_11,
        hyper::HeaderMap::new(),
        bytes::Bytes::new(),
    );

    let result = view.dispatch(request).await;
    assert!(result.is_ok());

    let response = result.unwrap();
    assert_eq!(response.status, hyper::StatusCode::OK);

    // Check for Allow header
    let allow_header = response.headers.get("Allow");
    assert!(allow_header.is_some());

    let allow_value = allow_header.unwrap().to_str().unwrap();
    assert!(allow_value.contains("GET"));
    assert!(allow_value.contains("PUT"));
    assert!(allow_value.contains("PATCH"));
}

#[tokio::test]
async fn test_dispatch_invalid_method() {
    let queryset = QuerySet::<TestArticle>::new();
    let view = AdminChangeView::new(queryset);

    let mut path_params = HashMap::new();
    path_params.insert("id".to_string(), "1".to_string());

    let mut request = Request::new(
        hyper::Method::DELETE,
        "/admin/article/change/1/".parse::<hyper::Uri>().unwrap(),
        hyper::Version::HTTP_11,
        hyper::HeaderMap::new(),
        bytes::Bytes::new(),
    );
    request.path_params = path_params;

    let result = view.dispatch(request).await;
    assert!(result.is_err());

    if let Err(Error::Validation(msg)) = result {
        assert!(msg.contains("not allowed"));
    } else {
        panic!("Expected Validation error for invalid method");
    }
}

#[tokio::test]
async fn test_dispatch_head_method() {
    let queryset = QuerySet::<TestArticle>::new();
    let view = AdminChangeView::new(queryset);

    let mut path_params = HashMap::new();
    path_params.insert("id".to_string(), "1".to_string());

    let mut request = Request::new(
        hyper::Method::HEAD,
        "/admin/article/change/1/".parse::<hyper::Uri>().unwrap(),
        hyper::Version::HTTP_11,
        hyper::HeaderMap::new(),
        bytes::Bytes::new(),
    );
    request.path_params = path_params;

    // HEAD request should fail because the queryset is empty
    let result = view.dispatch(request).await;
    assert!(result.is_err());
}

#[test]
fn test_allowed_methods() {
    use std::collections::HashSet;

    let queryset = QuerySet::<TestArticle>::new();
    let view = AdminChangeView::new(queryset);

    let methods = view.allowed_methods();
    assert_eq!(methods.len(), 5);

    let expected_methods = HashSet::from(["GET", "HEAD", "PUT", "PATCH", "OPTIONS"]);
    assert_eq!(
        methods.iter().collect::<HashSet<_>>(),
        expected_methods.iter().collect::<HashSet<_>>(),
        "Admin allowed methods mismatch. Expected methods: {:?}, Got: {:?}",
        expected_methods,
        methods
    );
}
