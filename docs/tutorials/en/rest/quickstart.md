# Quickstart

Create a simple API for administrators to view and edit users and groups in the system.

## Project Setup

Create a new Reinhardt project called tutorial.

```bash
# Create project directory
mkdir tutorial
cd tutorial

# Setup new project
cargo init --name tutorial
```

Add Reinhardt dependencies to `Cargo.toml`:

```toml
[dependencies]
reinhardt = { version = "0.1.0", features = ["standard", "rest", "serializers", "viewsets", "routers", "auth"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1", features = ["full"] }
async-trait = "0.1"
```

> **Note:** We use the `"standard"` feature flag along with additional REST API features (`rest`, `serializers`, `viewsets`, `routers`, `auth`). All functionality is provided by the unified `reinhardt` crate—you don't need to add individual `reinhardt-*` dependencies. For more details on customizing features for your project, see the [Feature Flags Guide](../../../FEATURE_FLAGS.md).

Project layout:

```
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs
```

## Models

For this quickstart, we'll use Reinhardt's built-in `User` and `Group` models provided by the auth feature.

## Serializers

Define serializers for data representation. Add to `src/main.rs`:

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserSerializer {
    pub id: i64,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GroupSerializer {
    pub id: i64,
    pub name: String,
}
```

This example uses simple data structures. In real applications, you can implement the `Serializer` trait to add validation and data transformation logic.

## ViewSets

Use ViewSets to implement CRUD operations. Add to `src/main.rs`:

```rust
use reinhardt::prelude::*;

// UserViewSet - full CRUD operations
let user_viewset = ModelViewSet::<User, UserSerializer>::new("user");

// GroupViewSet - read-only
let group_viewset = ReadOnlyModelViewSet::<Group, GroupSerializer>::new("group");
```

`ModelViewSet` provides all standard CRUD operations (list, retrieve, create, update, delete). `ReadOnlyModelViewSet` provides only list and retrieve operations.

## Routing

Register ViewSets with the router to automatically generate URLs:

```rust
use reinhardt::prelude::*;
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<()> {
    let mut router = DefaultRouter::new();

    let user_viewset = ModelViewSet::<User, UserSerializer>::new("user");
    let group_viewset = ReadOnlyModelViewSet::<Group, GroupSerializer>::new("group");

    router.register_viewset("users", user_viewset);
    router.register_viewset("groups", group_viewset);

    // URLs automatically generated:
    // GET/POST    /users/      - User list/create
    // GET/PUT/DELETE /users/{id}/ - User detail/update/delete
    // GET         /groups/     - Group list
    // GET         /groups/{id}/ - Group detail

    Ok(())
}
```

## Testing the API

Test the API using curl or httpie:

```bash
# Get list of users
curl http://127.0.0.1:8000/users/

# Create new user
curl -X POST http://127.0.0.1:8000/users/ \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"alice@example.com"}'

# Get specific user
curl http://127.0.0.1:8000/users/1/

# Update user
curl -X PUT http://127.0.0.1:8000/users/1/ \
  -H "Content-Type: application/json" \
  -d '{"username":"alice","email":"newemail@example.com"}'

# Delete user
curl -X DELETE http://127.0.0.1:8000/users/1/
```

## Summary

In this quickstart, you learned:

1. Setting up a Reinhardt project
2. Defining serializers
3. Creating CRUD APIs with ViewSets
4. Automatic URL generation with routers

For more detailed information, see the [tutorials](1-serialization.md).
