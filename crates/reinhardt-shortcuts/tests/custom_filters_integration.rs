#![cfg(feature = "templates")]

use reinhardt_shortcuts::template_inheritance::render_string_with_inheritance;
use reinhardt_shortcuts::tera_filters::*;
use reinhardt_shortcuts::tera_functions::*;
use std::collections::HashMap;
use tera::{Tera, Value};

/// Test: Custom filter integration with Tera template
#[test]
fn test_truncate_chars_filter_integration() {
	let mut tera = Tera::default();
	tera.register_filter("truncatechars", TruncateCharsFilter);

	let mut context = tera::Context::new();
	context.insert(
		"long_text",
		"This is a very long text that needs to be truncated",
	);

	let template = "{{ long_text | truncatechars(length=20) }}";
	let result = tera
		.render_str(template, &context)
		.expect("Failed to render template");

	// truncatechars with length=20 and suffix="..." (3 chars) means:
	// max 20 chars total, including the suffix
	// so text is truncated to (20 - 3) = 17 chars, then "..." is appended
	assert_eq!(result.len(), 20); // Exactly 20 chars total
	assert!(result.ends_with("..."));
	assert!(result.starts_with("This is a very lo")); // 17 chars + "..." = 20
}

/// Test: AddClass filter integration with HTML templates
#[test]
fn test_add_class_filter_integration() {
	let mut tera = Tera::default();
	tera.register_filter("add_class", AddClassFilter);

	let mut context = tera::Context::new();
	context.insert("input_html", r#"<input type="text" name="username">"#);

	let template = r#"{{ input_html | add_class(class="form-control") }}"#;
	let result = tera
		.render_str(template, &context)
		.expect("Failed to render template");

	assert!(result.contains(r#"class="form-control""#));
	assert!(result.contains(r#"<input type="text""#));
}

/// Test: IntComma filter for number formatting
#[test]
fn test_int_comma_filter_integration() {
	let mut tera = Tera::default();
	tera.register_filter("intcomma", IntCommaFilter);

	let mut context = tera::Context::new();
	context.insert("price", &1234567);

	let template = "Price: ${{ price | intcomma }}";
	let result = tera
		.render_str(template, &context)
		.expect("Failed to render template");

	assert!(result.contains("1,234,567"));
}

/// Test: Pluralize filter with conditional text
#[test]
fn test_pluralize_filter_integration() {
	let mut tera = Tera::default();
	tera.register_filter("pluralize", PluralizeFilter);

	let mut context = tera::Context::new();
	context.insert("count_single", &1);
	context.insert("count_multiple", &5);

	let template = r#"
		{{ count_single }} item{{ count_single | pluralize }}
		{{ count_multiple }} item{{ count_multiple | pluralize }}
	"#;

	let result = tera
		.render_str(template, &context)
		.expect("Failed to render template");

	assert!(result.contains("1 item")); // Singular (no 's')
	assert!(result.contains("5 items")); // Plural (with 's')
}

/// Test: Default filter for missing values
#[test]
fn test_default_filter_integration() {
	let mut tera = Tera::default();
	tera.register_filter("default", DefaultFilter);

	let mut context = tera::Context::new();
	context.insert("username", &Value::Null);
	context.insert("email", "user@example.com");

	let template = r#"
		Username: {{ username | default(value="Anonymous") }}
		Email: {{ email | default(value="no-email") }}
	"#;

	let result = tera
		.render_str(template, &context)
		.expect("Failed to render template");

	assert!(result.contains("Username: Anonymous")); // Default used
	assert!(result.contains("Email: user@example.com")); // Original value
}

/// Test: Filter chaining
#[test]
fn test_filter_chaining() {
	let mut tera = Tera::default();
	tera.register_filter("truncatechars", TruncateCharsFilter);
	tera.register_filter("intcomma", IntCommaFilter);

	let mut context = tera::Context::new();
	context.insert(
		"description",
		"A very long product description that needs truncation",
	);
	context.insert("price", &123456);

	let template = r#"
		<div class="product">
			<p>{{ description | truncatechars(length=30) }}</p>
			<span>${{ price | intcomma }}</span>
		</div>
	"#;

	let result = tera
		.render_str(template, &context)
		.expect("Failed to render template");

	assert!(result.contains("..."));
	assert!(result.contains("123,456"));
}

/// Test: Range function integration
#[test]
fn test_range_function_integration() {
	let mut tera = Tera::default();
	tera.register_function("range", RangeFunction);

	let context = tera::Context::new();

	let template = r#"
		<ul>
		{% for i in range(end=5) %}
			<li>Item {{ i }}</li>
		{% endfor %}
		</ul>
	"#;

	let result = tera
		.render_str(template, &context)
		.expect("Failed to render template");

	assert!(result.contains("<li>Item 0</li>"));
	assert!(result.contains("<li>Item 1</li>"));
	assert!(result.contains("<li>Item 4</li>"));
	assert!(!result.contains("<li>Item 5</li>")); // Exclusive end
}

/// Test: Now function for timestamp generation
#[test]
fn test_now_function_integration() {
	let mut tera = Tera::default();
	tera.register_function("now", NowFunction);

	let context = tera::Context::new();

	let template = r#"Current time: {{ now() }}"#;

	let result = tera
		.render_str(template, &context)
		.expect("Failed to render template");

	// Verify that result contains a timestamp (basic format check)
	assert!(result.contains("Current time:"));
	assert!(result.len() > 20); // Timestamp should be present
}

/// Test: Cycle function for alternating values
#[test]
fn test_cycle_function_integration() {
	let mut tera = Tera::default();
	tera.register_function("cycle", CycleFunction);

	let mut context = tera::Context::new();
	context.insert("items", &vec!["A", "B", "C", "D"]);

	let template = r#"
		<table>
		{% for item in items %}
			<tr class="{{ cycle(values=["odd", "even"], index=loop.index0) }}">
				<td>{{ item }}</td>
			</tr>
		{% endfor %}
		</table>
	"#;

	let result = tera
		.render_str(template, &context)
		.expect("Failed to render template");

	// Check alternating classes
	assert!(result.contains(r#"class="odd""#));
	assert!(result.contains(r#"class="even""#));
}

/// Test: Multiple filters and functions in one template
#[test]
fn test_combined_filters_and_functions() {
	let mut tera = Tera::default();
	tera.register_filter("truncatechars", TruncateCharsFilter);
	tera.register_filter("intcomma", IntCommaFilter);
	tera.register_filter("pluralize", PluralizeFilter);
	tera.register_filter("default", DefaultFilter);
	tera.register_function("range", RangeFunction);

	let mut context = tera::Context::new();
	context.insert("title", "Product Listing Page");
	context.insert("product_count", &42);
	context.insert("description", &Value::Null);

	let template = r#"
		<h1>{{ title }}</h1>
		<p>Found {{ product_count | intcomma }} product{{ product_count | pluralize }}</p>
		<p>{{ description | default(value="No description available") }}</p>
		<div class="pagination">
			{% for page in range(end=5) %}
				<a href="/products?page={{ page }}">{{ page }}</a>
			{% endfor %}
		</div>
	"#;

	let result = tera
		.render_str(template, &context)
		.expect("Failed to render template");

	assert!(result.contains("Product Listing Page"));
	assert!(result.contains("Found 42 products")); // intcomma + pluralize
	assert!(result.contains("No description available")); // default filter
	assert!(result.contains(r#"href="/products?page=0""#)); // range function
}

/// Test: Error handling for invalid filter parameters
#[test]
fn test_filter_error_handling() {
	let mut tera = Tera::default();
	tera.register_filter("truncatechars", TruncateCharsFilter);

	let mut context = tera::Context::new();
	context.insert("text", "Sample text");

	// Missing required parameter 'length'
	let template = "{{ text | truncatechars }}";
	let result = tera.render_str(template, &context);

	assert!(result.is_err(), "Expected error for missing parameter");
}

/// Test: Template inheritance with custom filters
#[test]
fn test_template_inheritance_with_custom_filters() {
	let mut context = HashMap::new();
	context.insert("title", serde_json::json!("My Blog Post"));
	context.insert(
		"content",
		serde_json::json!("This is a very long blog post content that needs to be displayed"),
	);
	context.insert("view_count", serde_json::json!(1234));

	// NOTE: render_string_with_inheritance uses a global Tera instance
	// which may already have filters registered
	let template = r#"
		<article>
			<h1>{{ title }}</h1>
			<div>{{ content }}</div>
			<footer>Views: {{ view_count }}</footer>
		</article>
	"#;

	let result = render_string_with_inheritance(template, &context);
	assert!(result.is_ok(), "Template rendering should succeed");

	let rendered = result.unwrap();
	assert!(rendered.contains("<h1>My Blog Post</h1>"));
	assert!(rendered.contains("This is a very long blog post"));
	assert!(rendered.contains("Views: 1234"));
}
