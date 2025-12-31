// Auto-generated module file for introspect integration tests
// Each test file in introspect/ subdirectory is explicitly included with #[path] attribute

// Specialized fixtures for introspect tests
#[path = "introspect/fixtures.rs"]
mod fixtures;

// Happy path tests
#[path = "introspect/happy_path_integration.rs"]
mod happy_path_integration;

// Sanity tests
#[path = "introspect/sanity_integration.rs"]
mod sanity_integration;

// Edge cases tests
#[path = "introspect/edge_cases_integration.rs"]
mod edge_cases_integration;
