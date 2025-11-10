//! Template Pagination Integration Tests
//!
//! Integration tests for template rendering with pagination functionality
//! inspired by Django REST Framework's pagination tests.
//!
//! These tests cover:
//! - Pagination HTML generation
//! - Template rendering with paginated data
//! - Integration with reinhardt-pagination
//! - Error handling in paginated templates

use reinhardt_templates::{FileSystemTemplateLoader, custom_filters::*};
use std::collections::HashMap;
use tempfile::TempDir;
use tera::{Context, Tera};

// ============================================================================
// Pagination Data Structures
// ============================================================================

#[derive(Debug, Clone)]
pub struct PaginatedData<T> {
	pub items: Vec<T>,
	pub current_page: u32,
	pub total_pages: u32,
	pub total_items: u64,
	pub items_per_page: u32,
	pub has_previous: bool,
	pub has_next: bool,
	pub previous_page: Option<u32>,
	pub next_page: Option<u32>,
}

impl<T> PaginatedData<T> {
	pub fn new(items: Vec<T>, current_page: u32, total_items: u64, items_per_page: u32) -> Self {
		let total_pages = ((total_items as f64) / (items_per_page as f64)).ceil() as u32;
		let has_previous = current_page > 1;
		let has_next = current_page < total_pages;
		let previous_page = if has_previous {
			Some(current_page - 1)
		} else {
			None
		};
		let next_page = if has_next {
			Some(current_page + 1)
		} else {
			None
		};

		Self {
			items,
			current_page,
			total_pages,
			total_items,
			items_per_page,
			has_previous,
			has_next,
			previous_page,
			next_page,
		}
	}
}

#[derive(Debug, Clone)]
pub struct PaginationInfo {
	pub current_page: u32,
	pub total_pages: u32,
	pub total_items: u64,
	pub items_per_page: u32,
	pub has_previous: bool,
	pub has_next: bool,
	pub previous_page: Option<u32>,
	pub next_page: Option<u32>,
	pub page_range: Vec<u32>,
}

impl PaginationInfo {
	pub fn from_paginated_data<T>(data: &PaginatedData<T>) -> Self {
		let page_range = Self::generate_page_range(data.current_page, data.total_pages);

		Self {
			current_page: data.current_page,
			total_pages: data.total_pages,
			total_items: data.total_items,
			items_per_page: data.items_per_page,
			has_previous: data.has_previous,
			has_next: data.has_next,
			previous_page: data.previous_page,
			next_page: data.next_page,
			page_range,
		}
	}

	fn generate_page_range(current_page: u32, total_pages: u32) -> Vec<u32> {
		let mut pages = Vec::new();
		let start = (current_page.saturating_sub(2)).max(1);
		let end = (current_page + 2).min(total_pages);

		for page in start..=end {
			pages.push(page);
		}
		pages
	}
}

// ============================================================================
// Helper Functions for Template Rendering
// ============================================================================

fn render_pagination_template(pagination: &PaginationInfo) -> String {
	let mut context = Context::new();
	context.insert("has_previous", &pagination.has_previous);
	context.insert("has_next", &pagination.has_next);
	context.insert("previous_page", &pagination.previous_page);
	context.insert("next_page", &pagination.next_page);
	context.insert("page_range", &pagination.page_range);

	let template = r#"<div class="pagination">
{% if has_previous %}
<a href="?page={{ previous_page }}" class="prev">Previous</a>
{% endif %}

{% for page in page_range %}
<a href="?page={{ page }}">{{ page }}</a>
{% endfor %}

{% if has_next %}
<a href="?page={{ next_page }}" class="next">Next</a>
{% endif %}
</div>"#;

	Tera::one_off(template, &context, true).unwrap()
}

fn render_paginated_list(items: Vec<String>, pagination_html: String) -> String {
	let mut context = Context::new();
	context.insert("items", &items);
	context.insert("pagination_html", &pagination_html);

	let template = r#"<div class="paginated-list">
<h2>Items</h2>
<ul>
{% for item in items %}
<li>{{ item }}</li>
{% endfor %}
</ul>

{{ pagination_html | safe }}
</div>"#;

	Tera::one_off(template, &context, true).unwrap()
}

fn render_pagination_info(pagination: &PaginationInfo) -> String {
	let mut context = Context::new();
	context.insert("current_page", &pagination.current_page);
	context.insert("total_pages", &pagination.total_pages);
	context.insert("total_items", &pagination.total_items);

	let template = r#"<div class="pagination-info">
Showing {{ current_page }} to {{ total_pages }} of {{ total_items }} results
</div>"#;

	Tera::one_off(template, &context, true).unwrap()
}

fn render_pagination_controls(pagination: &PaginationInfo) -> String {
	let mut context = Context::new();
	context.insert("has_previous", &pagination.has_previous);
	context.insert("has_next", &pagination.has_next);
	context.insert("previous_page", &pagination.previous_page);
	context.insert("next_page", &pagination.next_page);
	context.insert("current_page", &pagination.current_page);
	context.insert("total_pages", &pagination.total_pages);

	let template = r#"<div class="pagination-controls">
{% if has_previous %}
<a href="?page=1" class="first">First</a>
<a href="?page={{ previous_page }}" class="prev">Previous</a>
{% endif %}

<span class="page-info">Page {{ current_page }} of {{ total_pages }}</span>

{% if has_next %}
<a href="?page={{ next_page }}" class="next">Next</a>
<a href="?page={{ total_pages }}" class="last">Last</a>
{% endif %}
</div>"#;

	Tera::one_off(template, &context, true).unwrap()
}

// ============================================================================
// Pagination HTML Generation Tests
// ============================================================================

#[test]
fn test_pagination_html_generation() {
	let pagination = PaginationInfo {
		current_page: 3,
		total_pages: 10,
		total_items: 100,
		items_per_page: 10,
		has_previous: true,
		has_next: true,
		previous_page: Some(2),
		next_page: Some(4),
		page_range: vec![1, 2, 3, 4, 5],
	};

	let result = render_pagination_template(&pagination);

	assert!(
		result.contains("href=\"?page=2\""),
		"Expected href=\"?page=2\" in pagination HTML, got: {}",
		result
	);
	assert!(
		result.contains("href=\"?page=4\""),
		"Expected href=\"?page=4\" in pagination HTML, got: {}",
		result
	);
	assert!(
		result.contains("href=\"?page=3\""),
		"Expected href=\"?page=3\" in pagination HTML, got: {}",
		result
	);
	assert!(
		result.contains("class=\"prev\">Previous</a>"),
		"Expected 'Previous' link with class='prev', got: {}",
		result
	);
	assert!(
		result.contains("class=\"next\">Next</a>"),
		"Expected 'Next' link with class='next', got: {}",
		result
	);
}

#[test]
fn test_pagination_html_first_page() {
	let pagination = PaginationInfo {
		current_page: 1,
		total_pages: 5,
		total_items: 50,
		items_per_page: 10,
		has_previous: false,
		has_next: true,
		previous_page: None,
		next_page: Some(2),
		page_range: vec![1, 2, 3],
	};

	let result = render_pagination_template(&pagination);

	assert!(
		!result.contains("Previous"),
		"Expected no 'Previous' link on first page, got: {}",
		result
	);
	assert!(
		result.contains("class=\"next\">Next</a>"),
		"Expected 'Next' link with class='next', got: {}",
		result
	);
	assert!(
		result.contains("href=\"?page=1\""),
		"Expected href=\"?page=1\" in pagination HTML, got: {}",
		result
	);
}

#[test]
fn test_pagination_html_last_page() {
	let pagination = PaginationInfo {
		current_page: 5,
		total_pages: 5,
		total_items: 50,
		items_per_page: 10,
		has_previous: true,
		has_next: false,
		previous_page: Some(4),
		next_page: None,
		page_range: vec![3, 4, 5],
	};

	let result = render_pagination_template(&pagination);

	assert!(
		result.contains("class=\"prev\">Previous</a>"),
		"Expected 'Previous' link with class='prev', got: {}",
		result
	);
	assert!(
		!result.contains("Next"),
		"Expected no 'Next' link on last page, got: {}",
		result
	);
	assert!(
		result.contains("href=\"?page=5\""),
		"Expected href=\"?page=5\" in pagination HTML, got: {}",
		result
	);
}

#[test]
fn test_pagination_html_single_page() {
	let pagination = PaginationInfo {
		current_page: 1,
		total_pages: 1,
		total_items: 5,
		items_per_page: 10,
		has_previous: false,
		has_next: false,
		previous_page: None,
		next_page: None,
		page_range: vec![1],
	};

	let result = render_pagination_template(&pagination);

	assert!(
		!result.contains("Previous"),
		"Expected no 'Previous' link on single page, got: {}",
		result
	);
	assert!(
		!result.contains("Next"),
		"Expected no 'Next' link on single page, got: {}",
		result
	);
	assert!(
		result.contains("href=\"?page=1\""),
		"Expected href=\"?page=1\" in pagination HTML, got: {}",
		result
	);
}

// ============================================================================
// Paginated Data Rendering Tests
// ============================================================================

#[test]
fn test_paginated_list_rendering() {
	let items = vec![
		"Item 1".to_string(),
		"Item 2".to_string(),
		"Item 3".to_string(),
	];

	let pagination = PaginationInfo {
		current_page: 1,
		total_pages: 3,
		total_items: 8,
		items_per_page: 3,
		has_previous: false,
		has_next: true,
		previous_page: None,
		next_page: Some(2),
		page_range: vec![1, 2, 3],
	};

	let pagination_html = render_pagination_template(&pagination);
	let result = render_paginated_list(items, pagination_html);

	assert!(
		result.contains("<h2>Items</h2>"),
		"Expected '<h2>Items</h2>' heading, got: {}",
		result
	);
	assert!(
		result.contains("<li>Item 1</li>"),
		"Expected '<li>Item 1</li>' in rendered list, got: {}",
		result
	);
	assert!(
		result.contains("<li>Item 2</li>"),
		"Expected '<li>Item 2</li>' in rendered list, got: {}",
		result
	);
	assert!(
		result.contains("<li>Item 3</li>"),
		"Expected '<li>Item 3</li>' in rendered list, got: {}",
		result
	);
	assert!(
		result.contains("class=\"next\">Next</a>"),
		"Expected 'Next' link with class='next', got: {}",
		result
	);
}

#[test]
fn test_paginated_list_empty() {
	let items = vec![];

	let pagination = PaginationInfo {
		current_page: 1,
		total_pages: 0,
		total_items: 0,
		items_per_page: 10,
		has_previous: false,
		has_next: false,
		previous_page: None,
		next_page: None,
		page_range: vec![],
	};

	let pagination_html = render_pagination_template(&pagination);
	let result = render_paginated_list(items, pagination_html);

	assert!(
		result.contains("<h2>Items</h2>"),
		"Expected '<h2>Items</h2>' heading, got: {}",
		result
	);
	assert!(
		!result.contains("<li>"),
		"Expected no list items in empty pagination, got: {}",
		result
	);
	assert!(
		!result.contains("Previous"),
		"Expected no 'Previous' link in empty pagination, got: {}",
		result
	);
	assert!(
		!result.contains("Next"),
		"Expected no 'Next' link in empty pagination, got: {}",
		result
	);
}

// ============================================================================
// Pagination Info Tests
// ============================================================================

#[test]
fn test_pagination_info_rendering() {
	let pagination = PaginationInfo {
		current_page: 2,
		total_pages: 5,
		total_items: 47,
		items_per_page: 10,
		has_previous: true,
		has_next: true,
		previous_page: Some(1),
		next_page: Some(3),
		page_range: vec![1, 2, 3, 4, 5],
	};

	let result = render_pagination_info(&pagination);

	assert!(
		result.contains("Showing 2 to 5 of 47 results"),
		"Expected 'Showing 2 to 5 of 47 results' in pagination info, got: {}",
		result
	);
}

#[test]
fn test_pagination_controls_rendering() {
	let pagination = PaginationInfo {
		current_page: 3,
		total_pages: 10,
		total_items: 100,
		items_per_page: 10,
		has_previous: true,
		has_next: true,
		previous_page: Some(2),
		next_page: Some(4),
		page_range: vec![1, 2, 3, 4, 5],
	};

	let result = render_pagination_controls(&pagination);

	assert!(
		result.contains("href=\"?page=1\""),
		"Expected href=\"?page=1\" for 'First' link, got: {}",
		result
	);
	assert!(
		result.contains("href=\"?page=2\""),
		"Expected href=\"?page=2\" for 'Previous' link, got: {}",
		result
	);
	assert!(
		result.contains("href=\"?page=4\""),
		"Expected href=\"?page=4\" for 'Next' link, got: {}",
		result
	);
	assert!(
		result.contains("href=\"?page=10\""),
		"Expected href=\"?page=10\" for 'Last' link, got: {}",
		result
	);
	assert!(
		result.contains("Page 3 of 10"),
		"Expected 'Page 3 of 10' in page info, got: {}",
		result
	);
	assert!(
		result.contains("class=\"first\">First</a>"),
		"Expected 'First' link with class='first', got: {}",
		result
	);
	assert!(
		result.contains("class=\"prev\">Previous</a>"),
		"Expected 'Previous' link with class='prev', got: {}",
		result
	);
	assert!(
		result.contains("class=\"next\">Next</a>"),
		"Expected 'Next' link with class='next', got: {}",
		result
	);
	assert!(
		result.contains("class=\"last\">Last</a>"),
		"Expected 'Last' link with class='last', got: {}",
		result
	);
}

// ============================================================================
// Pagination Data Generation Tests
// ============================================================================

#[test]
fn test_paginated_data_generation() {
	let items = vec![
		"Item 1".to_string(),
		"Item 2".to_string(),
		"Item 3".to_string(),
		"Item 4".to_string(),
		"Item 5".to_string(),
	];

	let paginated = PaginatedData::new(items, 2, 5, 2);

	assert_eq!(paginated.current_page, 2);
	assert_eq!(paginated.total_pages, 3);
	assert_eq!(paginated.total_items, 5);
	assert_eq!(paginated.items_per_page, 2);
	assert!(paginated.has_previous);
	assert!(paginated.has_next);
	assert_eq!(paginated.previous_page, Some(1));
	assert_eq!(paginated.next_page, Some(3));
}

#[test]
fn test_paginated_data_single_page() {
	let items = vec!["Item 1".to_string(), "Item 2".to_string()];
	let paginated = PaginatedData::new(items, 1, 2, 10);

	assert_eq!(paginated.current_page, 1);
	assert_eq!(paginated.total_pages, 1);
	assert_eq!(paginated.total_items, 2);
	assert_eq!(paginated.items_per_page, 10);
	assert!(!paginated.has_previous);
	assert!(!paginated.has_next);
	assert_eq!(paginated.previous_page, None);
	assert_eq!(paginated.next_page, None);
}

#[test]
fn test_pagination_info_from_data() {
	let items = vec!["Item 1".to_string(), "Item 2".to_string()];
	let paginated = PaginatedData::new(items, 2, 5, 2);
	let info = PaginationInfo::from_paginated_data(&paginated);

	assert_eq!(info.current_page, 2);
	assert_eq!(info.total_pages, 3);
	assert_eq!(info.total_items, 5);
	assert_eq!(info.items_per_page, 2);
	assert!(info.has_previous);
	assert!(info.has_next);
	assert_eq!(info.previous_page, Some(1));
	assert_eq!(info.next_page, Some(3));
	assert_eq!(info.page_range, vec![1, 2, 3]);
}

// ============================================================================
// Filter Integration with Pagination Tests
// ============================================================================

#[test]
fn test_pagination_with_filters() {
	let pagination = PaginationInfo {
		current_page: 1,
		total_pages: 3,
		total_items: 8,
		items_per_page: 3,
		has_previous: false,
		has_next: true,
		previous_page: None,
		next_page: Some(2),
		page_range: vec![1, 2, 3],
	};

	let title_text = "page 1 of 3";
	let formatted_value = title(&tera::Value::String(title_text.to_string()), &HashMap::new()).unwrap();
	let formatted_title = formatted_value.as_str().unwrap();
	assert_eq!(formatted_title, "Page 1 Of 3");

	let page_info = format!(
		"Page {} of {}",
		pagination.current_page, pagination.total_pages
	);
	assert_eq!(page_info, "Page 1 of 3");
}

#[test]
fn test_pagination_url_generation() {
	let base_url = "https://example.com/api/items";
	let page = 2;

	let url = format!("{}?page={}", base_url, page);
	assert_eq!(
		url, "https://example.com/api/items?page=2",
		"Expected URL with page parameter"
	);

	let url_with_params = format!("{}?page={}&search=test&sort=name", base_url, page);
	assert_eq!(
		url_with_params, "https://example.com/api/items?page=2&search=test&sort=name",
		"Expected URL with multiple query parameters"
	);
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_pagination_error_handling() {
	let pagination = PaginationInfo {
		current_page: 0,
		total_pages: 5,
		total_items: 50,
		items_per_page: 10,
		has_previous: false,
		has_next: true,
		previous_page: None,
		next_page: Some(1),
		page_range: vec![1, 2, 3],
	};

	let result = render_pagination_template(&pagination);

	assert!(
		result.contains("href=\"?page=1\""),
		"Expected href=\"?page=1\" even with invalid current page, got: {}",
		result
	);
}

#[test]
fn test_pagination_overflow_handling() {
	let pagination = PaginationInfo {
		current_page: 10,
		total_pages: 5,
		total_items: 50,
		items_per_page: 10,
		has_previous: true,
		has_next: false,
		previous_page: Some(9),
		next_page: None,
		page_range: vec![3, 4, 5],
	};

	let result = render_pagination_template(&pagination);

	assert!(
		result.contains("href=\"?page=5\""),
		"Expected href=\"?page=5\" in overflow pagination, got: {}",
		result
	);
	assert!(
		!result.contains("Next"),
		"Expected no 'Next' link when current page exceeds total pages, got: {}",
		result
	);
}

// ============================================================================
// Performance Tests
// ============================================================================

#[test]
fn test_pagination_performance_large_dataset() {
	let large_items: Vec<String> = (0..10000).map(|i| format!("Item {}", i)).collect();

	let paginated = PaginatedData::new(large_items, 100, 10000, 100);
	let info = PaginationInfo::from_paginated_data(&paginated);

	let start = std::time::Instant::now();
	let result = render_pagination_template(&info);
	let duration = start.elapsed();

	assert!(
		duration.as_millis() < 100,
		"Expected rendering to complete in <100ms, took: {}ms",
		duration.as_millis()
	);
	assert!(
		result.contains("href=\"?page=100\""),
		"Expected href=\"?page=100\" in large dataset pagination, got: {}",
		result
	);
}

#[test]
fn test_pagination_performance_many_pages() {
	let pagination = PaginationInfo {
		current_page: 500,
		total_pages: 1000,
		total_items: 100000,
		items_per_page: 100,
		has_previous: true,
		has_next: true,
		previous_page: Some(499),
		next_page: Some(501),
		page_range: vec![498, 499, 500, 501, 502],
	};

	let start = std::time::Instant::now();
	let result = render_pagination_template(&pagination);
	let duration = start.elapsed();

	assert!(
		duration.as_millis() < 100,
		"Expected rendering to complete in <100ms, took: {}ms",
		duration.as_millis()
	);
	assert!(
		result.contains("href=\"?page=500\""),
		"Expected href=\"?page=500\" in many pages pagination, got: {}",
		result
	);
}

// ============================================================================
// Integration with File System Loader Tests
// ============================================================================

#[test]
fn test_pagination_with_file_system_loader() {
	let temp_dir = TempDir::new().unwrap();
	let template_path = temp_dir.path().join("pagination.html");

	let template_content = r#"<div class="pagination">
{% if pagination.has_previous %}
<a href="?page={{ pagination.previous_page }}">Previous</a>
{% endif %}
<span>Page {{ pagination.current_page }} of {{ pagination.total_pages }}</span>
{% if pagination.has_next %}
<a href="?page={{ pagination.next_page }}">Next</a>
{% endif %}
</div>"#;

	std::fs::write(&template_path, template_content).unwrap();

	let loader = FileSystemTemplateLoader::new(temp_dir.path());
	let content = loader.load("pagination.html").unwrap();

	assert!(
		content.contains("{% if pagination.has_previous %}"),
		"Expected '{{% if pagination.has_previous %}}' in template, got: {}",
		content
	);
	assert!(
		content.contains("{% if pagination.has_next %}"),
		"Expected '{{% if pagination.has_next %}}' in template, got: {}",
		content
	);
	assert!(
		content.contains("{{ pagination.current_page }}"),
		"Expected '{{ pagination.current_page }}' in template, got: {}",
		content
	);
}

// ============================================================================
// Mock Integration Tests
// ============================================================================

#[test]
fn test_integration_with_orm_mock() {
	let mock_items = vec![
		"Database Item 1".to_string(),
		"Database Item 2".to_string(),
		"Database Item 3".to_string(),
	];

	let paginated = PaginatedData::new(mock_items, 1, 3, 10);
	let info = PaginationInfo::from_paginated_data(&paginated);

	let result = render_pagination_template(&info);

	assert!(
		result.contains("href=\"?page=1\""),
		"Expected href=\"?page=1\" in ORM mock pagination, got: {}",
		result
	);
	assert!(
		!result.contains("Previous"),
		"Expected no 'Previous' link in single page ORM mock, got: {}",
		result
	);
	assert!(
		!result.contains("Next"),
		"Expected no 'Next' link in single page ORM mock, got: {}",
		result
	);
}

#[test]
fn test_integration_with_rest_api_mock() {
	let api_response = HashMap::from([
		("page".to_string(), "2".to_string()),
		("total_pages".to_string(), "5".to_string()),
		("total_items".to_string(), "50".to_string()),
	]);

	let current_page = api_response.get("page").unwrap().parse::<u32>().unwrap();
	let total_pages = api_response
		.get("total_pages")
		.unwrap()
		.parse::<u32>()
		.unwrap();
	let total_items = api_response
		.get("total_items")
		.unwrap()
		.parse::<u64>()
		.unwrap();

	let pagination = PaginationInfo {
		current_page,
		total_pages,
		total_items,
		items_per_page: 10,
		has_previous: current_page > 1,
		has_next: current_page < total_pages,
		previous_page: if current_page > 1 {
			Some(current_page - 1)
		} else {
			None
		},
		next_page: if current_page < total_pages {
			Some(current_page + 1)
		} else {
			None
		},
		page_range: vec![1, 2, 3, 4, 5],
	};

	let result = render_pagination_template(&pagination);

	assert!(
		result.contains("href=\"?page=2\""),
		"Expected href=\"?page=2\" in REST API mock pagination, got: {}",
		result
	);
	assert!(
		result.contains("class=\"prev\">Previous</a>"),
		"Expected 'Previous' link with class='prev' in REST API mock, got: {}",
		result
	);
	assert!(
		result.contains("class=\"next\">Next</a>"),
		"Expected 'Next' link with class='next' in REST API mock, got: {}",
		result
	);
}
