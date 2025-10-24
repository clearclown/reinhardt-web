# Tutorial 6: ViewSets and Routers

Use ViewSets and Routers to reduce the amount of code needed to build your API.

## Using ViewSets

ViewSets allow you to implement common RESTful API patterns concisely.

### ModelViewSet

Provides full CRUD operations:

```rust
use reinhardt::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Snippet {
    id: i64,
    code: String,
    language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SnippetSerializer {
    id: i64,
    code: String,
    language: String,
}

// Create ViewSet
let snippet_viewset = ModelViewSet::<Snippet, SnippetSerializer>::new("snippet");
```

### ReadOnlyModelViewSet

Provides read-only operations:

```rust
use reinhardt::prelude::*;

let snippet_viewset = ReadOnlyModelViewSet::<Snippet, SnippetSerializer>::new("snippet");
```

## Using Routers

Register ViewSets with routers to automatically generate URLs:

```rust
use reinhardt::prelude::*;

#[tokio::main]
async fn main() {
    let mut router = DefaultRouter::new();

    // Register ViewSets
    let snippet_viewset = ModelViewSet::<Snippet, SnippetSerializer>::new("snippet");
    let user_viewset = ReadOnlyModelViewSet::<User, UserSerializer>::new("user");

    router.register_viewset("snippets", snippet_viewset);
    router.register_viewset("users", user_viewset);

    // URLs are automatically generated:
    // GET/POST    /snippets/           - List/create
    // GET/PUT/PATCH/DELETE /snippets/{id}/ - Detail/update/delete
    // GET         /users/              - List
    // GET         /users/{id}/         - Detail
}
```

## Automatic URL Generation

Routers automatically generate URL patterns from ViewSets:

| HTTP Method | URL Pattern     | ViewSet Action | Description              |
| ----------- | --------------- | -------------- | ------------------------ |
| GET         | /{prefix}/      | list           | List of objects          |
| POST        | /{prefix}/      | create         | Create new object        |
| GET         | /{prefix}/{id}/ | retrieve       | Retrieve specific object |
| PUT         | /{prefix}/{id}/ | update         | Update object            |
| PATCH       | /{prefix}/{id}/ | partial_update | Partial update           |
| DELETE      | /{prefix}/{id}/ | destroy        | Delete object            |

## ViewSet Benefits

1. **Less Code**: CRUD operations are automatically implemented
2. **Consistency**: Follows standard REST patterns
3. **Maintainability**: Focus on business logic
4. **Automatic URL Generation**: No routing configuration needed

## Views vs ViewSets

### Use Views When:

- Building simple endpoints
- Lots of custom logic required
- Not following standard CRUD patterns

### Use ViewSets When:

- Building standard RESTful APIs
- Multiple endpoints needed (list, detail, etc.)
- Code conciseness is important

## Complete Example

```rust
use reinhardt::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Snippet {
    id: i64,
    title: String,
    code: String,
    language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SnippetSerializer {
    id: i64,
    title: String,
    code: String,
    language: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: i64,
    username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserSerializer {
    id: i64,
    username: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut router = DefaultRouter::new();

    // Register ViewSets
    let snippet_viewset = ModelViewSet::<Snippet, SnippetSerializer>::new("snippet");
    let user_viewset = ReadOnlyModelViewSet::<User, UserSerializer>::new("user");

    router.register_viewset("snippets", snippet_viewset);
    router.register_viewset("users", user_viewset);

    println!("API endpoints:");
    println!("  GET/POST    /snippets/");
    println!("  GET/PUT/PATCH/DELETE /snippets/{{id}}/");
    println!("  GET         /users/");
    println!("  GET         /users/{{id}}/");

    Ok(())
}
```

## Summary

Throughout this tutorial series, you learned:

1. **Serialization** - Data serialization and validation
2. **Requests and Responses** - HTTP handling basics
3. **Class-Based Views** - Using generic views
4. **Authentication and Permissions** - API protection
5. **Hyperlinked APIs** - URL reverse routing and relationships
6. **ViewSets and Routers** - Efficient API building

You can now build production-ready RESTful APIs with this knowledge!

## Next Steps

- For more advanced topics, see the [API Reference](../../../api/README.md)
- Learn about [Dependency Injection](../../07-dependency-injection.md)
- Check out the [Feature Flags Guide](../../../FEATURE_FLAGS.md) for customization
- Join the community to ask questions
