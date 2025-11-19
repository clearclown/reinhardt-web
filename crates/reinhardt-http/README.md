# reinhardt-http

HTTP request and response handling for the Reinhardt framework

## Overview

Core HTTP abstractions for the Reinhardt framework. Provides comprehensive request and response types, header handling, cookie management, content negotiation, and streaming support with a Django/DRF-inspired API design.

## Features

### Implemented ✓

#### Request Type

- **Complete HTTP request representation** with all standard components
  - HTTP method, URI, version, headers, body
  - Path parameters (`path_params`) and query string parsing (`query_params`)
  - HTTPS detection (`is_secure`)
  - Remote address tracking (`remote_addr`)
  - Type-safe extensions system (`Extensions`)
- **Builder pattern** for fluent request construction
  - `Request::builder()` - Start building
  - `.method()` - Set HTTP method
  - `.uri()` - Set URI (with automatic query parsing)
  - `.version()` - Set HTTP version (defaults to HTTP/1.1)
  - `.headers()` - Set headers
  - `.body()` - Set request body
  - `.build()` - Finalize construction
- **Convenience methods** for common operations
  - `Request::get(uri)` - Create GET request
  - `Request::post(uri)` - Create POST request
  - `Request::put(uri)` - Create PUT request
  - `Request::delete(uri)` - Create DELETE request
  - `Request::patch(uri)` - Create PATCH request
- **Request parsing** (with `parsers` feature)
  - JSON body parsing
  - Form data parsing
  - Multipart form data
  - Lazy parsing (parse on first access)

#### Response Type

- **Flexible HTTP response creation** with status code helpers
  - `Response::ok()` - 200 OK
  - `Response::created()` - 201 Created
  - `Response::no_content()` - 204 No Content
  - `Response::bad_request()` - 400 Bad Request
  - `Response::unauthorized()` - 401 Unauthorized
  - `Response::forbidden()` - 403 Forbidden
  - `Response::not_found()` - 404 Not Found
  - `Response::gone()` - 410 Gone
  - `Response::internal_server_error()` - 500 Internal Server Error
- **Redirect responses**
  - `Response::permanent_redirect(url)` - 301 Moved Permanently
  - `Response::temporary_redirect(url)` - 302 Found
  - `Response::see_other(url)` - 303 See Other
- **Builder pattern methods**
  - `.with_body(data)` - Set response body (bytes or string)
  - `.with_header(name, value)` - Add single header
  - `.with_headers(map)` - Set multiple headers
  - `.with_json(data)` - Serialize data to JSON and set Content-Type
  - `.with_location(url)` - Set Location header (for redirects)
  - `.with_stop_chain(bool)` - Control middleware chain execution
- **JSON serialization support** with automatic Content-Type
- **Middleware chain control** via `stop_chain` flag

#### StreamingResponse

- **Streaming response support** for large data or real-time content
  - Custom media type configuration
  - Header support
  - Stream-based body (any type implementing `Stream`)

#### Extensions System

- **Type-safe request extensions** for storing arbitrary typed data
  - `request.extensions.insert::<T>(value)` - Store typed data
  - `request.extensions.get::<T>()` - Retrieve typed data
  - Thread-safe with `Arc<Mutex<TypeMap>>`
  - Common use cases: authentication context, request ID, user data

#### Error Integration

- Re-exports `reinhardt_exception::Error` and `Result` for consistent error handling

## Installation

```toml
[dependencies]
reinhardt-http = "0.1.0-alpha.1"

# With parsers support (JSON, form data)
reinhardt-http = { version = "0.1.0-alpha.1", features = ["parsers"] }
```

## Usage Examples

### Basic Request Construction

```rust
use reinhardt_http::Request;
use hyper::Method;

// Using builder pattern
let request = Request::builder()
    .method(Method::POST)
    .uri("/api/users?page=1")
    .body(bytes::Bytes::from(r#"{"name": "Alice"}"#))
    .build()
    .unwrap();

assert_eq!(request.method, Method::POST);
assert_eq!(request.path(), "/api/users");
assert_eq!(request.query_params.get("page"), Some(&"1".to_string()));
```

### Convenience Methods

```rust
use reinhardt_http::Request;

// Quick GET request
let request = Request::get("/api/users");
assert_eq!(request.method, hyper::Method::GET);

// Quick POST request
let request = Request::post("/api/users")
    .with_body(r#"{"name": "Bob"}"#);
assert_eq!(request.method, hyper::Method::POST);
```

### Path and Query Parameters

```rust
use reinhardt_http::Request;

let mut request = Request::get("/api/users/123?sort=name&order=asc");

// Access query parameters
assert_eq!(request.query_params.get("sort"), Some(&"sort".to_string()));
assert_eq!(request.query_params.get("order"), Some(&"asc".to_string()));

// Add path parameters (typically done by router)
request.path_params.insert("id".to_string(), "123".to_string());
assert_eq!(request.path_params.get("id"), Some(&"123".to_string()));
```

### Request Extensions

```rust
use reinhardt_http::Request;

#[derive(Clone)]
struct UserId(i64);

let mut request = Request::get("/api/profile");

// Store typed data in extensions
request.extensions.insert(UserId(42));

// Retrieve typed data
let user_id = request.extensions.get::<UserId>().unwrap();
assert_eq!(user_id.0, 42);
```

### Response Helpers

```rust
use reinhardt_http::Response;

// Success responses
let response = Response::ok()
    .with_body("Success");
assert_eq!(response.status, hyper::StatusCode::OK);

let response = Response::created()
    .with_json(&serde_json::json!({
        "id": 123,
        "name": "Alice"
    }))
    .unwrap();
assert_eq!(response.status, hyper::StatusCode::CREATED);
assert_eq!(
    response.headers.get("content-type").unwrap(),
    "application/json"
);

// Error responses
let response = Response::bad_request()
    .with_body("Invalid input");
assert_eq!(response.status, hyper::StatusCode::BAD_REQUEST);

let response = Response::not_found()
    .with_body("Resource not found");
assert_eq!(response.status, hyper::StatusCode::NOT_FOUND);
```

### Redirect Responses

```rust
use reinhardt_http::Response;

// Permanent redirect (301)
let response = Response::permanent_redirect("/new-location");
assert_eq!(response.status, hyper::StatusCode::MOVED_PERMANENTLY);
assert_eq!(
    response.headers.get("location").unwrap().to_str().unwrap(),
    "/new-location"
);

// Temporary redirect (302)
let response = Response::temporary_redirect("/login");
assert_eq!(response.status, hyper::StatusCode::FOUND);

// See Other (303) - after POST
let response = Response::see_other("/users/123");
assert_eq!(response.status, hyper::StatusCode::SEE_OTHER);
```

### JSON Response

```rust
use reinhardt_http::Response;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    id: i64,
    name: String,
}

let user = User {
    id: 1,
    name: "Alice".to_string(),
};

let response = Response::ok()
    .with_json(&user)
    .unwrap();

// Automatically sets Content-Type: application/json
assert_eq!(
    response.headers.get("content-type").unwrap(),
    "application/json"
);
```

### Middleware Chain Control

```rust
use reinhardt_http::Response;

// Stop middleware chain (useful for authentication, rate limiting)
let response = Response::unauthorized()
    .with_body("Authentication required")
    .with_stop_chain(true);

// This response will stop further middleware execution
assert!(response.should_stop_chain());
```

### Streaming Response

```rust
use reinhardt_http::StreamingResponse;
use futures::stream::{self, StreamExt};
use bytes::Bytes;

let data = vec![
    Bytes::from("chunk1"),
    Bytes::from("chunk2"),
    Bytes::from("chunk3"),
];

let stream = stream::iter(data.into_iter().map(Ok));

let response = StreamingResponse::new(
    hyper::StatusCode::OK,
    Box::pin(stream),
)
.with_content_type("text/plain");

// Use for large files, server-sent events, etc.
```

## API Reference

### Request

**Fields:**
- `method: Method` - HTTP method (GET, POST, etc.)
- `uri: Uri` - Request URI
- `version: Version` - HTTP version
- `headers: HeaderMap` - HTTP headers
- `path_params: HashMap<String, String>` - Path parameters from URL routing
- `query_params: HashMap<String, String>` - Query string parameters
- `is_secure: bool` - Whether request is over HTTPS
- `remote_addr: Option<SocketAddr>` - Client's remote address
- `extensions: Extensions` - Type-safe extension storage

**Methods:**
- `Request::builder()` - Create builder
- `Request::get(uri)` - Quick GET request
- `Request::post(uri)` - Quick POST request
- `Request::put(uri)` - Quick PUT request
- `Request::delete(uri)` - Quick DELETE request
- `Request::patch(uri)` - Quick PATCH request
- `.path()` - Get URI path without query
- `.body()` - Get request body as bytes
- `.body_string()` - Get body as UTF-8 string
- `.parse_json::<T>()` - Parse body as JSON (requires `parsers` feature)
- `.parse_form()` - Parse body as form data (requires `parsers` feature)

### Response

**Fields:**
- `status: StatusCode` - HTTP status code
- `headers: HeaderMap` - HTTP headers
- `body: Bytes` - Response body

**Constructor Methods:**
- `Response::new(status)` - Create with status code
- `Response::ok()` - 200 OK
- `Response::created()` - 201 Created
- `Response::no_content()` - 204 No Content
- `Response::bad_request()` - 400 Bad Request
- `Response::unauthorized()` - 401 Unauthorized
- `Response::forbidden()` - 403 Forbidden
- `Response::not_found()` - 404 Not Found
- `Response::gone()` - 410 Gone
- `Response::internal_server_error()` - 500 Internal Server Error
- `Response::permanent_redirect(url)` - 301 Moved Permanently
- `Response::temporary_redirect(url)` - 302 Found
- `Response::see_other(url)` - 303 See Other

**Builder Methods:**
- `.with_body(data)` - Set body (bytes or string)
- `.with_header(name, value)` - Add header
- `.with_headers(map)` - Set headers
- `.with_json(data)` - Serialize to JSON
- `.with_location(url)` - Set Location header
- `.with_stop_chain(bool)` - Control middleware chain
- `.should_stop_chain()` - Check if chain should stop

### Extensions

**Methods:**
- `.insert::<T>(value)` - Store typed value
- `.get::<T>()` - Retrieve typed value (returns `Option<T>`)
- `.remove::<T>()` - Remove typed value

## Feature Flags

- `parsers` - Enable request body parsing (JSON, form data, multipart)
  - Adds `parse_json()`, `parse_form()` methods to Request
  - Requires `reinhardt-parsers` crate

## Dependencies

- `hyper` - HTTP types (Method, Uri, StatusCode, HeaderMap, Version)
- `bytes` - Efficient byte buffer handling
- `futures` - Stream support for streaming responses
- `serde` - Serialization support (with `serde_json` for JSON)
- `reinhardt-exception` - Error handling
- `reinhardt-parsers` - Request body parsing (optional, with `parsers` feature)

## Testing

The crate includes comprehensive unit tests and doctests covering:
- Request construction and builder pattern
- Response helpers and status codes
- Redirect responses
- JSON serialization
- Extensions system
- Query parameter parsing
- Middleware chain control

Run tests with:
```bash
cargo test
cargo test --features parsers  # With parsers support
```

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.
