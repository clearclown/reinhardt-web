# reinhardt-serializers

Type-safe data serialization and validation for Rust, inspired by Django REST Framework.

## Overview

Provides serializers for converting between Rust types and various formats (JSON, XML, etc.), with built-in validation support. Includes automatic model serialization, validators for database constraints, and seamless integration with the ORM for type-safe data transformation.

## Features

### Implemented ✓

#### Core Serialization
- **`Serializer` trait**: Generic trait for data serialization and deserialization
  - `serialize()`: Convert Rust types to output format
  - `deserialize()`: Parse output format back to Rust types
  - `SerializerError`: Type-safe error handling for serialization failures

- **`JsonSerializer<T>`**: JSON serialization implementation
  - Built on `serde_json` for efficient JSON handling
  - Supports any type implementing `Serialize` and `Deserialize`
  - Automatic conversion between Rust types and JSON strings

- **`Deserializer` trait**: Dedicated deserialization interface
  - Separate trait for read-only deserialization operations
  - Enables more flexible data parsing pipelines

#### Model Serialization
- **`ModelSerializer<M>`**: Automatic serialization for ORM models
  - Django-style automatic field mapping from model definitions
  - Built-in validation support with `validate()` method
  - Seamless integration with `reinhardt-orm::Model` trait
  - JSON serialization/deserialization for database models
  - Extensible validation system for custom business logic

#### Database Validators
- **`UniqueValidator<M>`**: Enforce field uniqueness in database
  - Async validation against PostgreSQL database
  - Supports update operations (excludes current instance from uniqueness check)
  - Customizable field names
  - Database-level uniqueness verification

- **`UniqueTogetherValidator<M>`**: Ensure unique field combinations
  - Multi-field uniqueness constraints
  - Async PostgreSQL validation
  - Support for update operations
  - Flexible field combinations

#### Content Negotiation (Re-exported)
- **`ContentNegotiator`**: Select appropriate response format based on client request
- **`MediaType`**: Parse and compare media type strings

#### Renderers (Re-exported from `reinhardt-renderers`)
- **`JSONRenderer`**: Render data as JSON
- **`XMLRenderer`**: Render data as XML
- **`BrowsableAPIRenderer`**: Interactive HTML interface for API exploration

#### Parsers (Re-exported from `reinhardt-parsers`)
- **`JSONParser`**: Parse JSON request bodies
- **`FormParser`**: Parse form-encoded data
- **`MultiPartParser`**: Handle multipart/form-data (file uploads)
- **`FileUploadParser`**: Direct file upload handling
- **`ParseError`**: Error type for parsing failures

#### Field Types
- **`FieldError`**: Comprehensive error types for field validation failures
  - 14 error variants covering all validation scenarios
  - Display implementation for user-friendly error messages
- **`CharField`**: String field with length validation
  - Builder pattern with `min_length()`, `max_length()`, `required()`, `allow_blank()`
  - Default value support
  - Comprehensive doctests (7 tests) and unit tests (3 tests)
- **`IntegerField`**: Integer field with range validation
  - Builder pattern with `min_value()`, `max_value()`, `required()`, `allow_null()`
  - i64 value support
  - Comprehensive doctests (6 tests) and unit tests (3 tests)
- **`FloatField`**: Floating-point field with range validation
  - Builder pattern with `min_value()`, `max_value()`, `required()`, `allow_null()`
  - f64 value support
  - Comprehensive doctests (6 tests) and unit tests (1 test)
- **`BooleanField`**: Boolean field handling
  - Builder pattern with `required()`, `allow_null()`, `default()`
  - Always valid validation (booleans can't be invalid)
  - Comprehensive doctests (3 tests) and unit tests (1 test)
- **`EmailField`**: Email format validation
  - Builder pattern with `required()`, `allow_blank()`, `allow_null()`
  - Basic RFC-compliant email validation (@ sign, domain with dot)
  - Comprehensive doctests (4 tests) and unit tests (2 tests)
- **`URLField`**: URL format validation
  - Builder pattern with `required()`, `allow_blank()`, `allow_null()`
  - HTTP/HTTPS protocol validation
  - Comprehensive doctests (4 tests) and unit tests (2 tests)
- **`ChoiceField`**: Enumerated value validation
  - Builder pattern with `required()`, `allow_blank()`, `allow_null()`
  - Configurable list of valid choices
  - Comprehensive doctests (3 tests) and unit tests (2 tests)

#### Advanced Serialization
- **`SerializerMethodField`**: Compute custom read-only fields
  - Method-based computed fields for serializers
  - Custom method names with `.method_name()`
  - HashMap-based context for method values
  - Read-only field support (always `read_only: true`)
  - Example: `full_name` field computed from `first_name` + `last_name`
  - Comprehensive doctests (2 tests) and unit tests (7 tests)

- **`MethodFieldProvider` trait**: Support for serializers with method fields
  - `compute_method_fields()`: Generate all method field values
  - `compute_method()`: Generate single method field value
  - Integration with serializer context

- **`MethodFieldRegistry`**: Manage multiple method fields
  - Register method fields with `.register()`
  - Retrieve fields with `.get()` and `.contains()`
  - Access all fields with `.all()`

#### Validation System
- **`ValidationError`**: Structured validation error messages
  - `FieldError`: Single field validation errors with field name and message
  - `MultipleErrors`: Collection of multiple validation errors
  - `ObjectError`: Object-level validation errors
  - Helper methods: `field_error()`, `object_error()`, `multiple()`
  - thiserror integration for error handling

- **`FieldValidator` trait**: Field-level validation
  - `validate()`: Validate individual field values
  - Implemented by custom validators (EmailValidator, AgeValidator, etc.)
  - JSON Value-based validation

- **`ObjectValidator` trait**: Object-level validation
  - `validate()`: Validate entire objects with multiple fields
  - Cross-field validation support
  - Example: Password confirmation matching

- **`FieldLevelValidation` trait**: Serializer field-level validation
  - `validate_field()`: Validate specific field by name
  - `get_field_validators()`: Get all registered field validators
  - Django-style `validate_<field>()` pattern support

- **`ObjectLevelValidation` trait**: Serializer object-level validation
  - `validate()`: Validate entire serialized object
  - Called after all field validations pass

- **`validate_fields()` helper**: Validate all fields in a data object
  - Takes HashMap of field validators
  - Returns single error or MultipleErrors
  - Comprehensive doctests (3 tests) and unit tests (13 tests)

### Planned

#### Additional Field Types
- `DateField`, `DateTimeField`: Date and time handling with chrono integration

#### Advanced Serialization
- `HyperlinkedModelSerializer`: Generate hyperlinked relations instead of primary keys
- `NestedSerializer`: Handle nested object serialization
- `WritableNestedSerializer`: Support updates to nested objects
- `ListSerializer`: Serialize collections of objects

#### Relations
- `PrimaryKeyRelatedField`: Represent relations using primary keys
- `HyperlinkedRelatedField`: Represent relations using hyperlinks
- `SlugRelatedField`: Represent relations using slug fields
- `StringRelatedField`: Read-only string representation of related objects

#### Additional Renderers
- `YAMLRenderer`: Render data as YAML
- `CSVRenderer`: Render data as CSV (for list endpoints)
- `OpenAPIRenderer`: Generate OpenAPI/Swagger specifications

#### Meta Options
- Field inclusion/exclusion
- Read-only/write-only fields
- Custom field mappings
- Depth control for nested serialization

## Usage Examples

### Basic JSON Serialization

```rust
use reinhardt_serializers::{JsonSerializer, Serializer};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct User {
    name: String,
    age: i64,
}

let serializer = JsonSerializer::<User>::new();
let user = User { name: "Alice".to_string(), age: 30 };

// Serialize to JSON
let json = serializer.serialize(&user).unwrap();
assert_eq!(json, r#"{"name":"Alice","age":30}"#);

// Deserialize from JSON
let parsed = serializer.deserialize(&json).unwrap();
assert_eq!(parsed.name, "Alice");
```

### ModelSerializer with Validation

```rust
use reinhardt_serializers::{ModelSerializer, Serializer};
use reinhardt_orm::Model;

// Assuming you have a User model that implements Model
let serializer = ModelSerializer::<User>::new();

let user = User {
    id: Some(1),
    username: "alice".to_string(),
    email: "alice@example.com".to_string(),
};

// Validate before serialization
assert!(serializer.validate(&user).is_ok());

// Serialize to JSON
let json = serializer.serialize(&user).unwrap();
```

### Unique Field Validation

```rust
use reinhardt_serializers::UniqueValidator;
use sqlx::PgPool;

let pool: PgPool = /* your database connection */;
let validator = UniqueValidator::<User>::new("email");

// Validate that email is unique (for new user)
validator.validate(&pool, "alice@example.com", None).await?;

// Validate for update (exclude current user's ID)
validator.validate(&pool, "alice@example.com", Some(&user_id)).await?;
```

### Unique Together Validation

```rust
use reinhardt_serializers::UniqueTogetherValidator;
use std::collections::HashMap;

let validator = UniqueTogetherValidator::<User>::new(vec!["first_name", "last_name"]);

let mut values = HashMap::new();
values.insert("first_name".to_string(), "Alice".to_string());
values.insert("last_name".to_string(), "Smith".to_string());

validator.validate(&pool, &values, None).await?;
```

### SerializerMethodField for Computed Fields

```rust
use reinhardt_serializers::{SerializerMethodField, MethodFieldProvider, MethodFieldRegistry};
use serde_json::{json, Value};
use std::collections::HashMap;

struct UserSerializer {
    method_fields: MethodFieldRegistry,
}

impl UserSerializer {
    fn new() -> Self {
        let mut method_fields = MethodFieldRegistry::new();
        method_fields.register("full_name", SerializerMethodField::new("full_name"));
        Self { method_fields }
    }
}

impl MethodFieldProvider for UserSerializer {
    fn compute_method_fields(&self, instance: &Value) -> HashMap<String, Value> {
        let mut context = HashMap::new();

        if let Some(obj) = instance.as_object() {
            if let (Some(first), Some(last)) = (
                obj.get("first_name").and_then(|v| v.as_str()),
                obj.get("last_name").and_then(|v| v.as_str()),
            ) {
                let full_name = format!("{} {}", first, last);
                context.insert("full_name".to_string(), json!(full_name));
            }
        }

        context
    }

    fn compute_method(&self, method_name: &str, instance: &Value) -> Option<Value> {
        let context = self.compute_method_fields(instance);
        context.get(method_name).cloned()
    }
}

// Usage
let serializer = UserSerializer::new();
let user_data = json!({
    "first_name": "Alice",
    "last_name": "Johnson"
});

let context = serializer.compute_method_fields(&user_data);
assert_eq!(context.get("full_name").unwrap(), &json!("Alice Johnson"));
```

### Field-Level Validation

```rust
use reinhardt_serializers::{FieldValidator, ValidationResult, ValidationError, validate_fields};
use serde_json::{json, Value};
use std::collections::HashMap;

struct EmailValidator;

impl FieldValidator for EmailValidator {
    fn validate(&self, value: &Value) -> ValidationResult {
        if let Some(email) = value.as_str() {
            if email.contains('@') && email.contains('.') {
                Ok(())
            } else {
                Err(ValidationError::field_error("email", "Invalid email format"))
            }
        } else {
            Err(ValidationError::field_error("email", "Must be a string"))
        }
    }
}

// Register validators
let mut validators: HashMap<String, Box<dyn FieldValidator>> = HashMap::new();
validators.insert("email".to_string(), Box::new(EmailValidator));

// Validate data
let mut data = HashMap::new();
data.insert("email".to_string(), json!("user@example.com"));

let result = validate_fields(&data, &validators);
assert!(result.is_ok());
```

### Object-Level Validation

```rust
use reinhardt_serializers::{ObjectValidator, ValidationResult, ValidationError};
use serde_json::{json, Value};
use std::collections::HashMap;

struct PasswordMatchValidator;

impl ObjectValidator for PasswordMatchValidator {
    fn validate(&self, data: &HashMap<String, Value>) -> ValidationResult {
        let password = data.get("password").and_then(|v| v.as_str());
        let confirm = data.get("password_confirm").and_then(|v| v.as_str());

        if password == confirm {
            Ok(())
        } else {
            Err(ValidationError::object_error("Passwords do not match"))
        }
    }
}

// Validate
let validator = PasswordMatchValidator;
let mut data = HashMap::new();
data.insert("password".to_string(), json!("secret123"));
data.insert("password_confirm".to_string(), json!("secret123"));

assert!(validator.validate(&data).is_ok());
```

### Content Negotiation

```rust
use reinhardt_serializers::{ContentNegotiator, JSONRenderer, XMLRenderer};

let negotiator = ContentNegotiator::new();
negotiator.register(Box::new(JSONRenderer::new()));
negotiator.register(Box::new(XMLRenderer::new()));

// Select renderer based on Accept header
let renderer = negotiator.select("application/json")?;
```

## Dependencies

- `reinhardt-orm`: ORM integration for ModelSerializer
- `reinhardt-parsers`: Request body parsing
- `reinhardt-renderers`: Response rendering
- `reinhardt-negotiation`: Content type negotiation
- `serde`, `serde_json`: Serialization infrastructure
- `sqlx`: Database operations for validators
- `chrono`: Date and time handling
- `thiserror`: Error type definitions for validation and method fields
- `async-trait`: Async trait support

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE))
- MIT license ([LICENSE-MIT](../../LICENSE-MIT))

at your option.
