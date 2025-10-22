//! Permission Operators
//!
//! Provides logical operators (AND, OR, NOT) for composing permissions.

use crate::permissions::{Permission, PermissionContext};
use async_trait::async_trait;

/// AND permission operator
///
/// Combines two permissions with logical AND. Both permissions must be satisfied.
///
/// # Examples
///
/// ```
/// use reinhardt_auth::permission_operators::AndPermission;
/// use reinhardt_auth::permissions::{IsAuthenticated, IsAdminUser, Permission, PermissionContext};
/// use bytes::Bytes;
/// use hyper::{HeaderMap, Method, Uri, Version};
/// use reinhardt_types::Request;
///
/// #[tokio::main]
/// async fn main() {
///     let permission = AndPermission::new(IsAuthenticated, IsAdminUser);
///     let request = Request::new(
///         Method::GET,
///         Uri::from_static("/"),
///         Version::HTTP_11,
///         HeaderMap::new(),
///         Bytes::new(),
///     );
///
///     // Both authenticated AND admin required
///     let context = PermissionContext {
///         request: &request,
///         is_authenticated: true,
///         is_admin: true,
///         is_active: true,
///     };
///     assert!(permission.has_permission(&context).await);
///
///     // Not admin - fails
///     let context = PermissionContext {
///         request: &request,
///         is_authenticated: true,
///         is_admin: false,
///         is_active: true,
///     };
///     assert!(!permission.has_permission(&context).await);
/// }
/// ```
pub struct AndPermission<A, B> {
    left: A,
    right: B,
}

impl<A, B> AndPermission<A, B> {
    /// Create a new AND permission
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_auth::permission_operators::AndPermission;
    /// use reinhardt_auth::permissions::{IsAuthenticated, IsActiveUser};
    ///
    /// let permission = AndPermission::new(IsAuthenticated, IsActiveUser);
    /// ```
    pub fn new(left: A, right: B) -> Self {
        Self { left, right }
    }
}

#[async_trait]
impl<A, B> Permission for AndPermission<A, B>
where
    A: Permission + Send + Sync,
    B: Permission + Send + Sync,
{
    async fn has_permission(&self, context: &PermissionContext<'_>) -> bool {
        self.left.has_permission(context).await && self.right.has_permission(context).await
    }
}

/// OR permission operator
///
/// Combines two permissions with logical OR. Either permission can be satisfied.
///
/// # Examples
///
/// ```
/// use reinhardt_auth::permission_operators::OrPermission;
/// use reinhardt_auth::permissions::{IsAuthenticated, AllowAny, Permission, PermissionContext};
/// use bytes::Bytes;
/// use hyper::{HeaderMap, Method, Uri, Version};
/// use reinhardt_types::Request;
///
/// #[tokio::main]
/// async fn main() {
///     let permission = OrPermission::new(IsAuthenticated, AllowAny);
///     let request = Request::new(
///         Method::GET,
///         Uri::from_static("/"),
///         Version::HTTP_11,
///         HeaderMap::new(),
///         Bytes::new(),
///     );
///
///     // Either authenticated OR allow any
///     let context = PermissionContext {
///         request: &request,
///         is_authenticated: false,
///         is_admin: false,
///         is_active: false,
///     };
///     assert!(permission.has_permission(&context).await);
/// }
/// ```
pub struct OrPermission<A, B> {
    left: A,
    right: B,
}

impl<A, B> OrPermission<A, B> {
    /// Create a new OR permission
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_auth::permission_operators::OrPermission;
    /// use reinhardt_auth::permissions::{IsAdminUser, IsActiveUser};
    ///
    /// let permission = OrPermission::new(IsAdminUser, IsActiveUser);
    /// ```
    pub fn new(left: A, right: B) -> Self {
        Self { left, right }
    }
}

#[async_trait]
impl<A, B> Permission for OrPermission<A, B>
where
    A: Permission + Send + Sync,
    B: Permission + Send + Sync,
{
    async fn has_permission(&self, context: &PermissionContext<'_>) -> bool {
        self.left.has_permission(context).await || self.right.has_permission(context).await
    }
}

/// NOT permission operator
///
/// Negates a permission. Returns true if the inner permission is false.
///
/// # Examples
///
/// ```
/// use reinhardt_auth::permission_operators::NotPermission;
/// use reinhardt_auth::permissions::{IsAuthenticated, Permission, PermissionContext};
/// use bytes::Bytes;
/// use hyper::{HeaderMap, Method, Uri, Version};
/// use reinhardt_types::Request;
///
/// #[tokio::main]
/// async fn main() {
///     let permission = NotPermission::new(IsAuthenticated);
///     let request = Request::new(
///         Method::GET,
///         Uri::from_static("/"),
///         Version::HTTP_11,
///         HeaderMap::new(),
///         Bytes::new(),
///     );
///
///     // NOT authenticated - only allows unauthenticated users
///     let context = PermissionContext {
///         request: &request,
///         is_authenticated: false,
///         is_admin: false,
///         is_active: false,
///     };
///     assert!(permission.has_permission(&context).await);
///
///     // Authenticated - denies
///     let context = PermissionContext {
///         request: &request,
///         is_authenticated: true,
///         is_admin: false,
///         is_active: true,
///     };
///     assert!(!permission.has_permission(&context).await);
/// }
/// ```
pub struct NotPermission<P> {
    inner: P,
}

impl<P> NotPermission<P> {
    /// Create a new NOT permission
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_auth::permission_operators::NotPermission;
    /// use reinhardt_auth::permissions::IsAdminUser;
    ///
    /// let permission = NotPermission::new(IsAdminUser);
    /// ```
    pub fn new(inner: P) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl<P> Permission for NotPermission<P>
where
    P: Permission + Send + Sync,
{
    async fn has_permission(&self, context: &PermissionContext<'_>) -> bool {
        !self.inner.has_permission(context).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::permissions::{AllowAny, IsAdminUser, IsAuthenticated};
    use bytes::Bytes;
    use hyper::{HeaderMap, Method, Uri, Version};
    use reinhardt_types::Request;

    #[tokio::test]
    async fn test_and_permission_both_true() {
        let permission = AndPermission::new(IsAuthenticated, IsAdminUser);
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
            is_admin: true,
            is_active: true,
        };

        assert!(permission.has_permission(&context).await);
    }

    #[tokio::test]
    async fn test_and_permission_left_false() {
        let permission = AndPermission::new(IsAuthenticated, IsAdminUser);
        let request = Request::new(
            Method::GET,
            Uri::from_static("/"),
            Version::HTTP_11,
            HeaderMap::new(),
            Bytes::new(),
        );

        let context = PermissionContext {
            request: &request,
            is_authenticated: false,
            is_admin: true,
            is_active: false,
        };

        assert!(!permission.has_permission(&context).await);
    }

    #[tokio::test]
    async fn test_and_permission_right_false() {
        let permission = AndPermission::new(IsAuthenticated, IsAdminUser);
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
        };

        assert!(!permission.has_permission(&context).await);
    }

    #[tokio::test]
    async fn test_or_permission_both_true() {
        let permission = OrPermission::new(IsAuthenticated, AllowAny);
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
        };

        assert!(permission.has_permission(&context).await);
    }

    #[tokio::test]
    async fn test_or_permission_left_true() {
        let permission = OrPermission::new(IsAuthenticated, IsAdminUser);
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
        };

        assert!(permission.has_permission(&context).await);
    }

    #[tokio::test]
    async fn test_or_permission_right_true() {
        let permission = OrPermission::new(IsAuthenticated, AllowAny);
        let request = Request::new(
            Method::GET,
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
        };

        assert!(permission.has_permission(&context).await);
    }

    #[tokio::test]
    async fn test_or_permission_both_false() {
        let permission = OrPermission::new(IsAuthenticated, IsAdminUser);
        let request = Request::new(
            Method::GET,
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
        };

        assert!(!permission.has_permission(&context).await);
    }

    #[tokio::test]
    async fn test_not_permission_true() {
        let permission = NotPermission::new(IsAuthenticated);
        let request = Request::new(
            Method::GET,
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
        };

        assert!(permission.has_permission(&context).await);
    }

    #[tokio::test]
    async fn test_not_permission_false() {
        let permission = NotPermission::new(IsAuthenticated);
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
        };

        assert!(!permission.has_permission(&context).await);
    }

    #[tokio::test]
    async fn test_complex_permission_combination() {
        let permission =
            OrPermission::new(AndPermission::new(IsAuthenticated, IsAdminUser), AllowAny);

        let request = Request::new(
            Method::GET,
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
        };

        assert!(permission.has_permission(&context).await);
    }
}
