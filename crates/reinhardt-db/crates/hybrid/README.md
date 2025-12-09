# reinhardt-hybrid

Hybrid property and validation support

## Overview

Hybrid properties that work both as instance attributes and class-level query expressions. Allows defining computed properties that can be used in database queries, similar to SQLAlchemy's hybrid properties.

## Features

### Implemented ✓

#### HybridProperty

- **Instance-level getters**: Define getters that work on struct instances
  - `HybridProperty::new()` - Create a property with instance-level behavior
  - `get()` - Get the value for an instance
- **SQL expression support**: Generate SQL expressions for database queries
  - `with_expression()` - Add SQL expression generation capability
  - `expression()` - Get the SQL expression string
- **Type-safe**: Full type safety with generics `HybridProperty<T, R>`

#### HybridMethod

- **Instance-level methods**: Define methods that accept parameters
  - `HybridMethod::new()` - Create a method with instance-level behavior
  - `call()` - Call the method for an instance with arguments
- **SQL expression methods**: Generate parameterized SQL expressions
  - `with_expression()` - Add SQL expression generation capability
  - `expression()` - Get the SQL expression string with arguments
- **Type-safe**: Full type safety with generics `HybridMethod<T, A, R>`

#### SQL Expression Builders

- **SqlExpression struct**: Serializable SQL expression container
  - `new()` - Create a SQL expression from a string
  - `concat()` - Generate CONCAT expressions
  - `lower()` - Generate LOWER expressions for case-insensitive operations
  - `upper()` - Generate UPPER expressions for case-insensitive operations
  - `coalesce()` - Generate COALESCE expressions for NULL handling
- **Expression trait**: Convert types to SQL strings
  - Implemented for `SqlExpression`, `String`, and `&str`
  - `to_sql()` - Convert to SQL string representation

#### Comparator System

- **Comparator trait**: Customize SQL comparison operations
  - `new()` - Create a comparator with an expression
  - `eq()`, `ne()` - Equality and inequality comparisons
  - `lt()`, `le()`, `gt()`, `ge()` - Ordering comparisons
- **UpperCaseComparator**: Built-in case-insensitive comparator
  - Automatically applies UPPER() to both sides of comparisons

#### Property Override Support

- **HybridPropertyOverride trait**: Define overridable property behavior
  - `get_instance()` - Get instance-level value
  - `get_expression()` - Get SQL expression (optional)
  - `set_instance()` - Set instance-level value (optional)
- **OverridableProperty wrapper**: Composition-based property override
  - `new()` - Create an overridable property with custom implementation
  - `get()`, `set()` - Instance-level getters and setters
  - `expression()` - SQL expression support
  - Enables polymorphic behavior without traditional inheritance

#### Macro Support

- **hybrid_property! macro**: Convenience macro for defining hybrid properties

## Usage Examples

### Basic HybridProperty

```rust
use reinhardt_hybrid::HybridProperty;

struct Article {
    id: i32,
    title: String,
    content: String,
    word_count: usize,
}

// Create a hybrid property for title
let title_prop = HybridProperty::new(|article: &Article| article.title.clone())
    .with_expression(|| "articles.title".to_string());

// Use as instance property
let article = Article {
    id: 1,
    title: "Hello".to_string(),
    content: "World".to_string(),
    word_count: 100,
};
assert_eq!(title_prop.get(&article), "Hello");

// Use as SQL expression
assert_eq!(title_prop.expression(), Some("articles.title".to_string()));
```

### HybridProperty with Transformation

```rust
use reinhardt_hybrid::HybridProperty;

// Property that transforms value (uppercase)
let upper_title = HybridProperty::new(|article: &Article| {
    article.title.to_uppercase()
}).with_expression(|| "UPPER(articles.title)".to_string());

let article = Article {
    id: 1,
    title: "hello".to_string(),
    content: "world".to_string(),
    word_count: 100,
};

// Instance-level returns uppercase
assert_eq!(upper_title.get(&article), "HELLO");

// SQL expression uses UPPER() function
assert_eq!(upper_title.expression(), Some("UPPER(articles.title)".to_string()));
```

### HybridMethod with Parameters

```rust
use reinhardt_hybrid::HybridMethod;

// Method that checks if word count exceeds a threshold
let exceeds_count = HybridMethod::new(|article: &Article, threshold: usize| {
    article.word_count > threshold
}).with_expression(|threshold: usize| {
    format!("articles.word_count > {}", threshold)
});

let article = Article {
    id: 1,
    title: "Test".to_string(),
    content: "Content".to_string(),
    word_count: 150,
};

// Instance-level call
assert_eq!(exceeds_count.call(&article, 100), true);

// SQL expression with parameter
assert_eq!(
    exceeds_count.expression(100),
    Some("articles.word_count > 100".to_string())
);
```

### Case-Insensitive Comparisons

```rust
use reinhardt_hybrid::{Comparator, UpperCaseComparator};

// Create case-insensitive comparator for title
let comparator = UpperCaseComparator::new("articles.title");

// Generate SQL for case-insensitive equality
let sql = comparator.eq("hello");
// Produces: UPPER(articles.title) = UPPER('hello')
```