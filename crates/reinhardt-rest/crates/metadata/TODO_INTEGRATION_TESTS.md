# Metadata Integration Tests TODO

This document lists integration tests from Django REST Framework that require multiple crates to implement properly.

## Tests Requiring Integration

### 1. test_none_metadata

**Dependencies**: reinhardt-views + reinhardt-metadata
**Description**: Test that OPTIONS requests with `metadata_class = None` return HTTP 405 Method Not Allowed
**Implementation Location**: tests/integration/metadata/
**Status**: TODO - Requires view infrastructure

### 2. test_global_permissions

**Dependencies**: reinhardt-auth + reinhardt-views + reinhardt-metadata
**Description**: Test that metadata excludes actions without global permissions
**Implementation Location**: tests/integration/metadata/
**Status**: TODO - Requires permission system

### 3. test_object_permissions

**Dependencies**: reinhardt-auth + reinhardt-views + reinhardt-metadata
**Description**: Test that metadata excludes actions without object-level permissions
**Implementation Location**: tests/integration/metadata/
**Status**: TODO - Requires permission system

### 4. test_bug_2455_clone_request

**Dependencies**: reinhardt-versioning + reinhardt-views + reinhardt-metadata
**Description**: Test that cloned request has 'version' attribute when using BrowsableAPIRenderer
**Implementation Location**: tests/integration/metadata/
**Status**: TODO - Requires versioning support

### 5. test_bug_2477_clone_request

**Dependencies**: reinhardt-versioning + reinhardt-views + reinhardt-metadata
**Description**: Test that cloned request has 'versioning_scheme' attribute
**Implementation Location**: tests/integration/metadata/
**Status**: TODO - Requires versioning support

### 6. test_read_only_primary_key_related_field

**Dependencies**: reinhardt-serializers + reinhardt-orm + reinhardt-metadata
**Description**: Test metadata generation with read-only PrimaryKeyRelatedField in ModelSerializer
**Implementation Location**: tests/integration/metadata/
**Status**: TODO - Requires ORM and ModelSerializer support

## Implementation Notes

These tests should be implemented as integration tests in the project root's `tests/` directory once the required dependencies are available. The tests verify the interaction between multiple crates and ensure proper metadata generation in complex scenarios.

## Reference

Original tests can be found in:

- django-rest-framework/tests/test_metadata.py
