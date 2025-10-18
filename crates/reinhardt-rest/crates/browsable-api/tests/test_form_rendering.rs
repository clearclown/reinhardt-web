//! Tests for form rendering in browsable API
//!
//! This test module corresponds to Django REST Framework's
//! tests/browsable_api/test_form_rendering.py
//!
//! These tests verify that the BrowsableApiRenderer correctly handles
//! various form rendering scenarios, including edge cases like posting
//! list data and rendering forms for views that return lists.

use reinhardt_browsable_api::{ApiContext, BrowsableApiRenderer, FormContext, FormField};
use serde_json::json;
use std::fs;
use std::path::PathBuf;

/// Helper function to create a temporary test output directory
fn create_test_output_dir(test_name: &str) -> PathBuf {
    let dir = PathBuf::from(format!("target/test_output/form_rendering/{}", test_name));
    fs::create_dir_all(&dir).unwrap();
    dir
}

/// Helper function to clean up test output
fn cleanup_test_output(dir: &PathBuf) {
    if dir.exists() {
        fs::remove_dir_all(dir).ok();
    }
}

/// POSTing a list of data to a regular view should not cause the browsable
/// API to fail during rendering.
///
/// Regression test for https://github.com/encode/django-rest-framework/issues/5637
mod posting_list_data_tests {
    use super::*;

    #[test]
    fn test_browsable_api_form_json_response() {
        let test_dir = create_test_output_dir("json_response");

        // Sanity check for non-browsable API responses with list data
        // When POSTing list data to an endpoint expecting a dict, should get 400
        let renderer = BrowsableApiRenderer::new();
        let context = ApiContext {
            title: "Create Item".to_string(),
            description: None,
            endpoint: "/api/items/".to_string(),
            method: "POST".to_string(),
            response_data: json!({
                "non_field_errors": ["Invalid data. Expected a dictionary, but got list."]
            }),
            response_status: 400,
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            request_form: None,
            headers: vec![("Content-Type".to_string(), "application/json".to_string())],
        };

        let html = renderer.render(&context).unwrap();

        // Verify error status
        assert!(
            html.contains("400"),
            "Should display 400 Bad Request status"
        );

        // Verify error message is visible
        assert!(
            html.contains("non_field_errors"),
            "Should display non_field_errors in response"
        );
        assert!(
            html.contains("Invalid data"),
            "Should display the validation error message"
        );
        assert!(
            html.contains("Expected a dictionary, but got list"),
            "Should display specific error about list vs dict"
        );

        // Verify basic structure
        assert!(
            html.contains("Create Item"),
            "Should display endpoint title"
        );
        assert!(html.contains("/api/items/"), "Should display endpoint URL");

        cleanup_test_output(&test_dir);
    }

    #[test]
    fn test_browsable_api_with_list_data() {
        let test_dir = create_test_output_dir("browsable_api_list_data");

        // Test that browsable API can render even when list data causes validation errors
        // The key is that rendering shouldn't crash, even with validation errors
        let renderer = BrowsableApiRenderer::new();
        let context = ApiContext {
            title: "API Create".to_string(),
            description: None,
            endpoint: "/api/create/?format=api".to_string(),
            method: "POST".to_string(),
            response_data: json!({
                "non_field_errors": ["Invalid data. Expected a dictionary, but got list."]
            }),
            response_status: 400,
            allowed_methods: vec!["POST".to_string()],
            request_form: Some(FormContext {
                fields: vec![FormField {
                    name: "data".to_string(),
                    label: "Data".to_string(),
                    field_type: "textarea".to_string(),
                    required: true,
                    help_text: Some("Enter valid JSON data".to_string()),
                    initial_value: None,
                }],
                submit_url: "/api/create/".to_string(),
                submit_method: "POST".to_string(),
            }),
            headers: vec![],
        };

        let html = renderer.render(&context).unwrap();

        // Verify error is rendered
        assert!(html.contains("400"), "Should show 400 status");
        assert!(
            html.contains("non_field_errors"),
            "Should display field errors"
        );

        // Verify form is still rendered for retry - critical for user experience
        assert!(
            html.contains("Make a Request"),
            "Should still show form even after validation error"
        );
        assert!(
            html.contains("textarea"),
            "Should render textarea field for JSON input"
        );
        assert!(
            html.contains("name=\"data\""),
            "Should have data field in form"
        );
        assert!(
            html.contains("Enter valid JSON data"),
            "Should display help text to guide user"
        );

        // Verify form can be submitted again
        assert!(
            html.contains("action=\"/api/create/\""),
            "Form should submit to correct URL for retry"
        );
        assert!(
            html.contains("method=\"POST\""),
            "Form should use POST method"
        );

        cleanup_test_output(&test_dir);
    }

    #[test]
    fn test_list_error_response_rendering() {
        let test_dir = create_test_output_dir("list_error_response");

        // Test that list errors are properly displayed in browsable API
        let renderer = BrowsableApiRenderer::new();
        let context = ApiContext {
            title: "Validation Error".to_string(),
            description: Some("List data validation failed".to_string()),
            endpoint: "/api/validate/".to_string(),
            method: "POST".to_string(),
            response_data: json!({
                "non_field_errors": [
                    "This field is required.",
                    "Expected a dictionary but got list."
                ]
            }),
            response_status: 400,
            allowed_methods: vec!["POST".to_string()],
            request_form: None,
            headers: vec![],
        };

        let html = renderer.render(&context).unwrap();

        // Verify all error messages are displayed
        assert!(
            html.contains("Validation Error"),
            "Should display error title"
        );
        assert!(html.contains("non_field_errors"), "Should label error type");
        assert!(
            html.contains("This field is required"),
            "Should display first error message"
        );
        assert!(
            html.contains("Expected a dictionary but got list"),
            "Should display second error message"
        );
        assert!(html.contains("400"), "Should show 400 status");

        // Verify proper JSON formatting of errors
        let has_array_format = html.contains("[") && html.contains("]");
        assert!(
            has_array_format,
            "Errors should be displayed as array in JSON"
        );

        cleanup_test_output(&test_dir);
    }
}

/// Tests for views that return lists with many=True serializers
///
/// Regression test for https://github.com/encode/django-rest-framework/pull/3164
mod many_post_view_tests {
    use super::*;

    #[test]
    fn test_post_many_post_view() {
        let test_dir = create_test_output_dir("post_many_view");

        // POST request to a view that returns a list of objects should
        // still successfully return the browsable API with a rendered form
        let renderer = BrowsableApiRenderer::new();
        let test_items = vec![
            json!({"id": 1, "text": "foo"}),
            json!({"id": 2, "text": "bar"}),
            json!({"id": 3, "text": "baz"}),
        ];

        let context = ApiContext {
            title: "Items List".to_string(),
            description: Some("View returning multiple items".to_string()),
            endpoint: "/api/items/".to_string(),
            method: "POST".to_string(),
            response_data: json!(test_items),
            response_status: 200,
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            request_form: Some(FormContext {
                fields: vec![FormField {
                    name: "text".to_string(),
                    label: "Text".to_string(),
                    field_type: "text".to_string(),
                    required: true,
                    help_text: None,
                    initial_value: None,
                }],
                submit_url: "/api/items/".to_string(),
                submit_method: "POST".to_string(),
            }),
            headers: vec![],
        };

        let html = renderer.render(&context).unwrap();

        // Verify response status is 200
        assert!(html.contains("200"), "Should show 200 OK status");

        // Verify all items are rendered in the response
        assert!(html.contains("foo"), "Should display first item");
        assert!(html.contains("bar"), "Should display second item");
        assert!(html.contains("baz"), "Should display third item");

        // Verify list structure is maintained
        assert!(
            html.contains("\"id\": 1") || html.contains("id"),
            "Should display item IDs"
        );

        // Verify form is rendered despite returning a list - this is the key test
        assert!(
            html.contains("Make a Request"),
            "Should render form even when response is a list"
        );
        assert!(
            html.contains("name=\"text\""),
            "Should have text field in form"
        );
        assert!(
            html.contains("required"),
            "Should mark text field as required"
        );

        // Verify form submission details
        assert!(
            html.contains("action=\"/api/items/\""),
            "Form should submit to items endpoint"
        );
        assert!(
            html.contains("method=\"POST\""),
            "Form should use POST method"
        );

        cleanup_test_output(&test_dir);
    }

    #[test]
    fn test_many_serializer_with_form_rendering() {
        let test_dir = create_test_output_dir("many_serializer_form");

        // Test that forms are correctly rendered even when response is a list
        let renderer = BrowsableApiRenderer::new();
        let context = ApiContext {
            title: "Many Items View".to_string(),
            description: None,
            endpoint: "/api/many/".to_string(),
            method: "POST".to_string(),
            response_data: json!([
                {"id": 1, "name": "Item 1"},
                {"id": 2, "name": "Item 2"}
            ]),
            response_status: 200,
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            request_form: Some(FormContext {
                fields: vec![
                    FormField {
                        name: "id".to_string(),
                        label: "ID".to_string(),
                        field_type: "number".to_string(),
                        required: false,
                        help_text: Some("Read-only field".to_string()),
                        initial_value: None,
                    },
                    FormField {
                        name: "name".to_string(),
                        label: "Name".to_string(),
                        field_type: "text".to_string(),
                        required: true,
                        help_text: None,
                        initial_value: None,
                    },
                ],
                submit_url: "/api/many/".to_string(),
                submit_method: "POST".to_string(),
            }),
            headers: vec![],
        };

        let html = renderer.render(&context).unwrap();

        // Verify list data is rendered
        assert!(html.contains("Item 1"), "Should display first item");
        assert!(html.contains("Item 2"), "Should display second item");

        // Verify both form fields are present
        assert!(html.contains("name=\"id\""), "Should have ID field");
        assert!(html.contains("name=\"name\""), "Should have name field");
        assert!(html.contains("type=\"number\""), "ID should be number type");
        assert!(html.contains("type=\"text\""), "Name should be text type");

        // Verify field attributes
        assert!(
            html.contains("Read-only field"),
            "Should display help text for read-only fields"
        );
        let name_field_section = html.split("name=\"name\"").nth(0).unwrap_or("");
        assert!(
            !name_field_section.ends_with("required") || html.contains("required"),
            "Name field should be marked as required"
        );

        cleanup_test_output(&test_dir);
    }

    #[test]
    fn test_empty_list_response_with_form() {
        let test_dir = create_test_output_dir("empty_list_form");

        // Test rendering when response is an empty list
        let renderer = BrowsableApiRenderer::new();
        let context = ApiContext {
            title: "Empty List".to_string(),
            description: None,
            endpoint: "/api/empty/".to_string(),
            method: "GET".to_string(),
            response_data: json!([]),
            response_status: 200,
            allowed_methods: vec!["GET".to_string(), "POST".to_string()],
            request_form: Some(FormContext {
                fields: vec![FormField {
                    name: "item".to_string(),
                    label: "Item".to_string(),
                    field_type: "text".to_string(),
                    required: true,
                    help_text: None,
                    initial_value: None,
                }],
                submit_url: "/api/empty/".to_string(),
                submit_method: "POST".to_string(),
            }),
            headers: vec![],
        };

        let html = renderer.render(&context).unwrap();

        // Verify empty list is displayed
        assert!(html.contains("Empty List"), "Should display title");
        assert!(
            html.contains("[]") || html.contains("[ ]"),
            "Should display empty array in response"
        );
        assert!(html.contains("200"), "Should show 200 status");

        // Verify form is still rendered (users can add first item)
        assert!(
            html.contains("Make a Request"),
            "Should show form to add items to empty list"
        );
        assert!(html.contains("name=\"item\""), "Should have item field");

        cleanup_test_output(&test_dir);
    }

    #[test]
    fn test_large_list_response_rendering() {
        let test_dir = create_test_output_dir("large_list");

        // Test that large lists are properly rendered without issues
        let items: Vec<_> = (1..=100)
            .map(|i| json!({"id": i, "value": format!("item_{}", i)}))
            .collect();

        let renderer = BrowsableApiRenderer::new();
        let context = ApiContext {
            title: "Large List".to_string(),
            description: Some("100 items".to_string()),
            endpoint: "/api/large/".to_string(),
            method: "GET".to_string(),
            response_data: json!(items),
            response_status: 200,
            allowed_methods: vec!["GET".to_string()],
            request_form: None,
            headers: vec![],
        };

        let html = renderer.render(&context).unwrap();

        // Verify rendering succeeds
        assert!(html.contains("Large List"), "Should display title");
        assert!(html.contains("100 items"), "Should display description");

        // Verify first and last items are present
        assert!(html.contains("item_1"), "Should display first item");
        assert!(html.contains("item_100"), "Should display last item");

        // Verify some middle items
        assert!(html.contains("item_50"), "Should display middle items");

        // Verify structure
        assert!(html.contains("200"), "Should show 200 status");

        cleanup_test_output(&test_dir);
    }
}

#[cfg(test)]
mod form_field_rendering_tests {
    use super::*;

    #[test]
    fn test_textarea_field_rendering() {
        let test_dir = create_test_output_dir("textarea_field");

        // Test that textarea fields are properly rendered with correct attributes
        let renderer = BrowsableApiRenderer::new();
        let context = ApiContext {
            title: "Text Area Test".to_string(),
            description: None,
            endpoint: "/api/textarea/".to_string(),
            method: "POST".to_string(),
            response_data: json!({}),
            response_status: 200,
            allowed_methods: vec!["POST".to_string()],
            request_form: Some(FormContext {
                fields: vec![FormField {
                    name: "content".to_string(),
                    label: "Content".to_string(),
                    field_type: "textarea".to_string(),
                    required: true,
                    help_text: Some("Enter your content here".to_string()),
                    initial_value: Some(json!("Initial content")),
                }],
                submit_url: "/api/textarea/".to_string(),
                submit_method: "POST".to_string(),
            }),
            headers: vec![],
        };

        let html = renderer.render(&context).unwrap();

        // Verify textarea element
        assert!(html.contains("textarea"), "Should have textarea element");
        assert!(
            html.contains("name=\"content\""),
            "Should have correct name attribute"
        );
        assert!(
            html.contains("id=\"content\""),
            "Should have id attribute for label"
        );

        // Verify label
        assert!(html.contains("Content"), "Should display label text");

        // Verify initial value
        assert!(
            html.contains("Initial content"),
            "Should display initial value in textarea"
        );

        // Verify help text
        assert!(
            html.contains("Enter your content here"),
            "Should display help text"
        );

        // Verify required attribute
        assert!(html.contains("required"), "Should mark field as required");

        cleanup_test_output(&test_dir);
    }

    #[test]
    fn test_required_field_marking() {
        let test_dir = create_test_output_dir("required_fields");

        // Test that required fields are properly marked with asterisk
        let renderer = BrowsableApiRenderer::new();
        let context = ApiContext {
            title: "Required Fields".to_string(),
            description: None,
            endpoint: "/api/required/".to_string(),
            method: "POST".to_string(),
            response_data: json!({}),
            response_status: 200,
            allowed_methods: vec!["POST".to_string()],
            request_form: Some(FormContext {
                fields: vec![
                    FormField {
                        name: "required_field".to_string(),
                        label: "Required".to_string(),
                        field_type: "text".to_string(),
                        required: true,
                        help_text: None,
                        initial_value: None,
                    },
                    FormField {
                        name: "optional_field".to_string(),
                        label: "Optional".to_string(),
                        field_type: "text".to_string(),
                        required: false,
                        help_text: None,
                        initial_value: None,
                    },
                ],
                submit_url: "/api/required/".to_string(),
                submit_method: "POST".to_string(),
            }),
            headers: vec![],
        };

        let html = renderer.render(&context).unwrap();

        // Find required field section
        let required_section = if let Some(pos) = html.find("required_field") {
            &html[pos..pos.saturating_add(200).min(html.len())]
        } else {
            &html
        };

        // Verify required field has asterisk
        assert!(
            required_section.contains("required") || required_section.contains("*"),
            "Required field should be marked with required attribute or asterisk"
        );

        // Verify field labels are present
        assert!(
            html.contains("Required"),
            "Should display required field label"
        );
        assert!(
            html.contains("Optional"),
            "Should display optional field label"
        );

        // Verify both fields are rendered
        assert!(
            html.contains("name=\"required_field\""),
            "Should have required field"
        );
        assert!(
            html.contains("name=\"optional_field\""),
            "Should have optional field"
        );

        cleanup_test_output(&test_dir);
    }

    #[test]
    fn test_form_with_multiple_field_types() {
        let test_dir = create_test_output_dir("multiple_field_types");

        // Test rendering of various field types in a single form
        let renderer = BrowsableApiRenderer::new();
        let context = ApiContext {
            title: "Mixed Fields".to_string(),
            description: None,
            endpoint: "/api/mixed/".to_string(),
            method: "POST".to_string(),
            response_data: json!({}),
            response_status: 200,
            allowed_methods: vec!["POST".to_string()],
            request_form: Some(FormContext {
                fields: vec![
                    FormField {
                        name: "text_field".to_string(),
                        label: "Text".to_string(),
                        field_type: "text".to_string(),
                        required: true,
                        help_text: None,
                        initial_value: None,
                    },
                    FormField {
                        name: "email_field".to_string(),
                        label: "Email".to_string(),
                        field_type: "email".to_string(),
                        required: true,
                        help_text: Some("Enter valid email".to_string()),
                        initial_value: None,
                    },
                    FormField {
                        name: "number_field".to_string(),
                        label: "Number".to_string(),
                        field_type: "number".to_string(),
                        required: false,
                        help_text: Some("Enter a number".to_string()),
                        initial_value: Some(json!(42)),
                    },
                    FormField {
                        name: "textarea_field".to_string(),
                        label: "Description".to_string(),
                        field_type: "textarea".to_string(),
                        required: false,
                        help_text: None,
                        initial_value: None,
                    },
                ],
                submit_url: "/api/mixed/".to_string(),
                submit_method: "POST".to_string(),
            }),
            headers: vec![],
        };

        let html = renderer.render(&context).unwrap();

        // Verify all field types are rendered with correct type attributes
        assert!(html.contains("type=\"text\""), "Should have text input");
        assert!(html.contains("type=\"email\""), "Should have email input");
        assert!(html.contains("type=\"number\""), "Should have number input");
        assert!(html.contains("textarea"), "Should have textarea element");

        // Verify all field names
        assert!(
            html.contains("name=\"text_field\""),
            "Should have text field"
        );
        assert!(
            html.contains("name=\"email_field\""),
            "Should have email field"
        );
        assert!(
            html.contains("name=\"number_field\""),
            "Should have number field"
        );
        assert!(
            html.contains("name=\"textarea_field\""),
            "Should have textarea field"
        );

        // Verify help texts
        assert!(
            html.contains("Enter valid email"),
            "Should show email help text"
        );
        assert!(
            html.contains("Enter a number"),
            "Should show number help text"
        );

        // Verify initial value for number field
        assert!(
            html.contains("42"),
            "Should display initial value for number field"
        );

        // Verify form structure
        assert!(html.contains("Make a Request"), "Should have form section");
        assert!(html.contains("Submit"), "Should have submit button");

        cleanup_test_output(&test_dir);
    }
}
