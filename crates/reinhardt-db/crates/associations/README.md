# reinhardt-associations

Model associations and relationships

## Overview

Provides SQLAlchemy-style association proxies for simplifying access to related objects through associations. This crate enables elegant and type-safe access to attributes of related objects without manual traversal.

## Installation

Add `reinhardt` to your `Cargo.toml`:

```toml
[dependencies]
reinhardt = { version = "0.1.0-alpha.1", features = ["db-associations"] }

# Or use a preset:
# reinhardt = { version = "0.1.0-alpha.1", features = ["standard"] }  # Recommended
# reinhardt = { version = "0.1.0-alpha.1", features = ["full"] }      # All features
```

Then import association features:

```rust
use reinhardt::db::associations::{AssociationProxy, AssociationCollection};
use reinhardt::db::associations::{ForeignKeyField, ManyToManyField, OneToOneField};
```

**Note:** Association features are included in the `standard` and `full` feature presets.

## Features

### Implemented ✓

#### Association Proxy (`AssociationProxy<S, A, T>`)

- **Single object attribute access**: Access attributes of related objects through foreign key and one-to-one relationships
- **Type-safe proxies**: Compile-time type checking for association chains
- **Generic implementation**: Works with any source type, associated type, and target attribute type
- **Key methods**:
  - `new()`: Create a new association proxy with custom getter functions
  - `get()`: Retrieve the target attribute through the association

#### Association Collection (`AssociationCollection<S, C, T>`)

- **Collection attribute access**: Access attributes of items in collections through one-to-many and many-to-many relationships
- **Batch operations**: Retrieve all target attributes from a collection at once
- **Collection utilities**: Count and check emptiness of collections
- **Key methods**:
  - `new()`: Create a new association collection proxy with custom getter functions
  - `get_all()`: Get all target attributes from the collection
  - `count()`: Count the number of items in the collection
  - `is_empty()`: Check if the collection is empty

#### Prelude Module

- Re-exports commonly used types for convenient importing

#### Relationship Types

- **ForeignKey** - Many-to-one relationships with cascade actions
  - Define foreign key relationships between models
  - Support for cascade operations (CASCADE, SET_NULL, SET_DEFAULT, RESTRICT, NO_ACTION)
  - Automatic reverse accessor generation

- **OneToOne** - Unique one-to-one relationships
  - Bidirectional one-to-one relationships
  - Unique constraint enforcement
  - Optional reverse relationship naming

- **OneToMany** - One-to-many relationships (reverse side of ForeignKey)
  - Collection-based access to related objects
  - Lazy loading by default
  - Custom related name support

- **ManyToMany** - Many-to-many relationships through junction tables
  - Automatic junction table management
  - Bidirectional access
  - Custom junction table configuration

- **PolymorphicAssociation** - Polymorphic one-to-many relationships
  - Generic foreign keys to multiple model types
  - Content type tracking
  - Type-safe polymorphic queries

- **PolymorphicManyToMany** - Polymorphic many-to-many relationships
  - Many-to-many with polymorphic targets
  - Generic relationship support

#### Cascade Actions

Define behavior when parent objects are deleted:

- **CASCADE** - Delete related objects when parent is deleted
- **SET_NULL** - Set foreign key to NULL when parent is deleted
- **SET_DEFAULT** - Set foreign key to default value when parent is deleted
- **RESTRICT** - Prevent deletion if related objects exist
- **NO_ACTION** - No automatic action (database constraint only)

#### Loading Strategies

Optimize how related objects are loaded:

- **LazyLoader** - Load related objects only when accessed (default)
  - Minimizes initial query overhead
  - Best for seldom-accessed relationships

- **EagerLoader** - Load related objects immediately with parent
  - Single query with JOIN
  - Best for always-accessed relationships

- **SelectInLoader** - Use SELECT IN strategy for collections
  - Efficient for loading multiple related collections
  - Avoids N+1 query problem

- **JoinedLoader** - Use SQL JOIN for single query loading
  - Fetch everything in one query
  - Best for small result sets

- **SubqueryLoader** - Use subquery for complex filtering
  - Advanced query optimization
  - Best for complex filtering requirements

#### Reverse Relationships

- **Automatic reverse accessor generation** - Related models get automatic reverse accessors
- **Custom naming** - Override default reverse accessor names with `related_name`
- **Singular forms** - Generate singular accessor names for one-to-one relationships