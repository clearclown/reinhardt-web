//! Model-based Permissions
//!
//! Provides permissions based on model-level operations.

use crate::permissions::{Permission, PermissionContext};
use async_trait::async_trait;
use std::marker::PhantomData;

/// Model permission
///
/// Checks permissions for CRUD operations on specific model types.
///
/// # Examples
///
/// ```
/// use reinhardt_auth::model_permissions::ModelPermission;
///
/// #[derive(Debug)]
/// struct Article;
///
/// let perm = ModelPermission::<Article>::new("create");
/// assert_eq!(perm.operation(), "create");
/// ```
pub struct ModelPermission<T> {
    /// Operation (create, read, update, delete)
    operation: String,
    _phantom: PhantomData<T>,
}

impl<T> ModelPermission<T> {
    /// Create a new model permission
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_auth::model_permissions::ModelPermission;
    ///
    /// #[derive(Debug)]
    /// struct Post;
    ///
    /// let perm = ModelPermission::<Post>::new("update");
    /// assert_eq!(perm.operation(), "update");
    /// ```
    pub fn new(operation: impl Into<String>) -> Self {
        Self {
            operation: operation.into(),
            _phantom: PhantomData,
        }
    }

    /// Get operation name
    pub fn operation(&self) -> &str {
        &self.operation
    }
}

#[async_trait]
impl<T: Send + Sync> Permission for ModelPermission<T> {
    async fn has_permission(&self, context: &PermissionContext<'_>) -> bool {
        context.is_authenticated
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use hyper::{HeaderMap, Method, Uri, Version};
    use reinhardt_types::Request;

    #[derive(Debug)]
    struct TestModel;

    #[test]
    fn test_model_permission_creation() {
        let perm = ModelPermission::<TestModel>::new("create");
        assert_eq!(perm.operation(), "create");
    }

    #[test]
    fn test_model_permission_operations() {
        let create = ModelPermission::<TestModel>::new("create");
        let read = ModelPermission::<TestModel>::new("read");
        let update = ModelPermission::<TestModel>::new("update");
        let delete = ModelPermission::<TestModel>::new("delete");

        assert_eq!(create.operation(), "create");
        assert_eq!(read.operation(), "read");
        assert_eq!(update.operation(), "update");
        assert_eq!(delete.operation(), "delete");
    }

    #[tokio::test]
    async fn test_model_permission_authenticated() {
        let perm = ModelPermission::<TestModel>::new("read");
        let request = Request::new(
            Method::GET,
            Uri::from_static("/"),
            Version::HTTP_11,
            HeaderMap::new(),
            Bytes::new(),
        );

        let context = PermissionContext {
            request: &request,
            is_authenticated: true,
            is_admin: false,
            is_active: true,
            user: None,
        };

        assert!(perm.has_permission(&context).await);
    }

    #[tokio::test]
    async fn test_model_permission_unauthenticated() {
        let perm = ModelPermission::<TestModel>::new("create");
        let request = Request::new(
            Method::POST,
            Uri::from_static("/"),
            Version::HTTP_11,
            HeaderMap::new(),
            Bytes::new(),
        );

        let context = PermissionContext {
            request: &request,
            is_authenticated: false,
            is_admin: false,
            is_active: false,
            user: None,
        };

        assert!(!perm.has_permission(&context).await);
    }

    #[derive(Debug)]
    struct Article;

    #[derive(Debug)]
    struct Comment;

    #[tokio::test]
    async fn test_different_model_types() {
        let article_perm = ModelPermission::<Article>::new("update");
        let comment_perm = ModelPermission::<Comment>::new("delete");

        let request = Request::new(
            Method::PUT,
            Uri::from_static("/"),
            Version::HTTP_11,
            HeaderMap::new(),
            Bytes::new(),
        );

        let context = PermissionContext {
            request: &request,
            is_authenticated: true,
            is_admin: false,
            is_active: true,
            user: None,
        };

        assert!(article_perm.has_permission(&context).await);
        assert!(comment_perm.has_permission(&context).await);
    }
}
