# Tutorial 4: Authentication and Permissions

Protect your API with authentication and permission controls.

## Authentication

Reinhardt provides authentication:

```rust
use reinhardt::prelude::*;

async fn login(username: &str, password: &str) -> Option<User> {
    // Authenticate user
    authenticate(username, password).await
}
```

## Permission System

Implement custom permissions using the `Permission` trait:

```rust
use reinhardt::prelude::*;
use async_trait::async_trait;

pub struct IsAuthenticated;

#[async_trait]
impl Permission for IsAuthenticated {
    async fn has_permission(&self, context: &PermissionContext<'_>) -> bool {
        context.is_authenticated
    }
}
```

## PermissionContext

The `PermissionContext` provides request information for permission checks:

```rust
pub struct PermissionContext<'a> {
    pub is_authenticated: bool,
    pub user: Option<&'a User>,
    pub method: &'a Method,
    pub path: &'a str,
}
```

## Built-in Permissions

Reinhardt provides common permission classes:

### IsAuthenticated

Only authenticated users can access:

```rust
use reinhardt::prelude::*;

let permission = IsAuthenticated;
```

### IsAdminUser

Only admin users can access:

```rust
use reinhardt::prelude::*;

pub struct IsAdminUser;

#[async_trait]
impl Permission for IsAdminUser {
    async fn has_permission(&self, context: &PermissionContext<'_>) -> bool {
        if let Some(user) = context.user {
            user.is_staff
        } else {
            false
        }
    }
}
```

### AllowAny

All users can access (default):

```rust
use reinhardt::prelude::*;

pub struct AllowAny;

#[async_trait]
impl Permission for AllowAny {
    async fn has_permission(&self, _context: &PermissionContext<'_>) -> bool {
        true
    }
}
```

## Custom Permissions

Create custom permissions for specific requirements:

```rust
use reinhardt::prelude::*;
use async_trait::async_trait;

pub struct IsOwnerOrReadOnly;

#[async_trait]
impl Permission for IsOwnerOrReadOnly {
    async fn has_permission(&self, context: &PermissionContext<'_>) -> bool {
        // Read permissions for any request
        if context.method == &Method::GET {
            return true;
        }

        // Write permissions only for authenticated users
        if !context.is_authenticated {
            return false;
        }

        // Additional ownership check would go here
        // For example, check if user owns the resource
        true
    }
}
```

## Using Permissions with ViewSets

Apply permissions to ViewSets:

```rust
use reinhardt::prelude::*;

let viewset = ModelViewSet::<Snippet, SnippetSerializer>::new("snippet")
    .with_permission(IsAuthenticated);
```

## Object-Level Permissions

Check permissions for specific objects:

```rust
use reinhardt::prelude::*;
use async_trait::async_trait;

pub struct IsOwner;

#[async_trait]
impl Permission for IsOwner {
    async fn has_permission(&self, context: &PermissionContext<'_>) -> bool {
        // Allow all GET requests
        if context.method == &Method::GET {
            return true;
        }

        // For write operations, check ownership
        if let Some(user) = context.user {
            // Extract object ID from path and check ownership
            // This is a simplified example
            true
        } else {
            false
        }
    }
}
```

## Permission Combinations

Combine multiple permissions:

```rust
use reinhardt::prelude::*;
use async_trait::async_trait;

pub struct IsAuthenticatedAndActive;

#[async_trait]
impl Permission for IsAuthenticatedAndActive {
    async fn has_permission(&self, context: &PermissionContext<'_>) -> bool {
        if !context.is_authenticated {
            return false;
        }

        if let Some(user) = context.user {
            user.is_active
        } else {
            false
        }
    }
}
```

## Complete Example

Full authentication and permission workflow:

```rust
use reinhardt::prelude::*;
use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use hyper::Method;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Snippet {
    id: i64,
    title: String,
    code: String,
    owner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SnippetSerializer {
    id: i64,
    title: String,
    code: String,
    owner: String,
}

// Custom permission: Owner or read-only
pub struct IsOwnerOrReadOnly;

#[async_trait]
impl Permission for IsOwnerOrReadOnly {
    async fn has_permission(&self, context: &PermissionContext<'_>) -> bool {
        // Read operations are allowed for everyone
        if matches!(context.method, Method::GET | Method::HEAD | Method::OPTIONS) {
            return true;
        }

        // Write operations require authentication
        if let Some(user) = context.user {
            // In a real app, check if user owns the snippet
            user.is_authenticated()
        } else {
            false
        }
    }
}

// Create ViewSet with permissions
let viewset = ModelViewSet::<Snippet, SnippetSerializer>::new("snippet")
    .with_permission(IsOwnerOrReadOnly);
```

## Summary

In this tutorial, you learned:

1. Basic authentication with the `reinhardt-auth` crate
2. Implementing custom permissions with the `Permission` trait
3. Using `PermissionContext` for permission checks
4. Built-in permission classes
5. Object-level permissions
6. Combining multiple permissions
7. Applying permissions to ViewSets

Next tutorial: [Tutorial 5: Relationships and Hyperlinked APIs](5-relationships-and-hyperlinked-apis.md)
