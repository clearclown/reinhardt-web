#![cfg(feature = "templates")]

use reinhardt_shortcuts::redirect::{redirect, redirect_permanent};
use reinhardt_shortcuts::url::Url;

/// Test: Basic temporary redirect (302)
#[test]
fn test_redirect_temporary() {
	let response = redirect("/users/profile/");

	assert_eq!(
		response.status,
		hyper::StatusCode::FOUND,
		"Expected 302 Found status"
	);

	let location = response
		.headers
		.get("location")
		.expect("Location header should be present");
	assert_eq!(location.to_str().unwrap(), "/users/profile/");
}

/// Test: Permanent redirect (301)
#[test]
fn test_redirect_permanent() {
	let response = redirect_permanent("/new-location/");

	assert_eq!(
		response.status,
		hyper::StatusCode::MOVED_PERMANENTLY,
		"Expected 301 Moved Permanently status"
	);

	let location = response
		.headers
		.get("location")
		.expect("Location header should be present");
	assert_eq!(location.to_str().unwrap(), "/new-location/");
}

/// Test: Redirect with absolute URL
#[test]
fn test_redirect_absolute_url() {
	let response = redirect("https://example.com/page");

	assert_eq!(response.status, hyper::StatusCode::FOUND);

	let location = response
		.headers
		.get("location")
		.expect("Location header should be present");
	assert_eq!(location.to_str().unwrap(), "https://example.com/page");
}

/// Test: Redirect with query parameters
#[test]
fn test_redirect_with_query_params() {
	let response = redirect("/search?q=rust&sort=date");

	assert_eq!(response.status, hyper::StatusCode::FOUND);

	let location = response
		.headers
		.get("location")
		.expect("Location header should be present");
	assert_eq!(location.to_str().unwrap(), "/search?q=rust&sort=date");
}

/// Test: Url struct creation and validation
#[test]
fn test_url_struct_creation() {
	let url = Url::from("/users/123/");

	assert_eq!(url.as_ref(), "/users/123/");
	assert_eq!(url.to_string(), "/users/123/");
}

/// Test: Url struct with absolute URL
#[test]
fn test_url_absolute() {
	let url = Url::from("https://api.example.com/v1/users");

	assert_eq!(url.as_ref(), "https://api.example.com/v1/users");
	assert_eq!(url.to_string(), "https://api.example.com/v1/users");
}

/// Test: Redirect with Url struct
#[test]
fn test_redirect_with_url_struct() {
	let url = Url::from("/dashboard/");
	let response = redirect(url.as_ref());

	assert_eq!(response.status, hyper::StatusCode::FOUND);

	let location = response
		.headers
		.get("location")
		.expect("Location header should be present");
	assert_eq!(location.to_str().unwrap(), "/dashboard/");
}

/// Test: Multiple redirects with different URLs
#[test]
fn test_multiple_redirects() {
	let urls = vec![
		"/home/",
		"/about/",
		"/contact/",
		"https://external.com/page",
	];

	for url in urls {
		let response = redirect(url);
		assert_eq!(response.status, hyper::StatusCode::FOUND);

		let location = response
			.headers
			.get("location")
			.expect("Location header should be present");
		assert_eq!(location.to_str().unwrap(), url);
	}
}

/// Test: Redirect with URL encoding
#[test]
fn test_redirect_url_encoding() {
	let response = redirect("/search?q=hello%20world&category=rust%2Bprogramming");

	assert_eq!(response.status, hyper::StatusCode::FOUND);

	let location = response
		.headers
		.get("location")
		.expect("Location header should be present");
	assert_eq!(
		location.to_str().unwrap(),
		"/search?q=hello%20world&category=rust%2Bprogramming"
	);
}

/// Test: Redirect to fragment identifier
#[test]
fn test_redirect_fragment() {
	let response = redirect("/page#section-2");

	assert_eq!(response.status, hyper::StatusCode::FOUND);

	let location = response
		.headers
		.get("location")
		.expect("Location header should be present");
	assert_eq!(location.to_str().unwrap(), "/page#section-2");
}

/// Test: Redirect with empty path
#[test]
fn test_redirect_root() {
	let response = redirect("/");

	assert_eq!(response.status, hyper::StatusCode::FOUND);

	let location = response
		.headers
		.get("location")
		.expect("Location header should be present");
	assert_eq!(location.to_str().unwrap(), "/");
}

/// Test: Permanent redirect preserves URL structure
#[test]
fn test_permanent_redirect_structure() {
	let original_url = "/old-api/v1/users/123?format=json";
	let response = redirect_permanent(original_url);

	assert_eq!(response.status, hyper::StatusCode::MOVED_PERMANENTLY);

	let location = response
		.headers
		.get("location")
		.expect("Location header should be present");
	assert_eq!(location.to_str().unwrap(), original_url);
}

/// Test: Url as_ref trait implementation
#[test]
fn test_url_as_ref_trait() {
	let url = Url::from("/api/endpoint");

	// Test AsRef<str> trait
	fn takes_str_ref(s: &str) -> String {
		s.to_uppercase()
	}

	let result = takes_str_ref(url.as_ref());
	assert_eq!(result, "/API/ENDPOINT");
}

/// Test: Url Display trait implementation
#[test]
fn test_url_display_trait() {
	let url = Url::from("/users/profile/");

	let formatted = format!("Redirecting to: {}", url);
	assert_eq!(formatted, "Redirecting to: /users/profile/");
}

/// Test: Redirect response headers
#[tokio::test]
async fn test_redirect_response_headers() {
	let response = redirect("/target/");

	// Check required headers
	assert!(response.headers.contains_key("location"));

	// Verify no body content for redirect
	assert!(
		response.body.is_empty(),
		"Redirect response should have no body"
	);
}

/// Test: Integration of Url creation and redirect
#[test]
fn test_url_redirect_integration() {
	// Create URL dynamically
	let user_id = 42;
	let url_string = format!("/users/{}/profile/", user_id);
	let url = Url::from(url_string.as_str());

	// Use URL in redirect
	let response = redirect(url.as_ref());

	assert_eq!(response.status, hyper::StatusCode::FOUND);

	let location = response
		.headers
		.get("location")
		.expect("Location header should be present");
	assert_eq!(location.to_str().unwrap(), "/users/42/profile/");
}

/// Test: Redirect with relative paths
#[test]
fn test_redirect_relative_paths() {
	let relative_paths = vec!["./sibling", "../parent", "../../grandparent"];

	for path in relative_paths {
		let response = redirect(path);
		assert_eq!(response.status, hyper::StatusCode::FOUND);

		let location = response
			.headers
			.get("location")
			.expect("Location header should be present");
		assert_eq!(location.to_str().unwrap(), path);
	}
}

/// Test: Conditional redirect based on URL pattern
#[test]
fn test_conditional_redirect() {
	let urls = vec![
		("/admin/", true), // Should redirect permanently
		("/temp/", false), // Should redirect temporarily
		("/api/", false),  // Should redirect temporarily
	];

	for (url, is_permanent) in urls {
		let response = if is_permanent {
			redirect_permanent(url)
		} else {
			redirect(url)
		};

		let expected_status = if is_permanent {
			hyper::StatusCode::MOVED_PERMANENTLY
		} else {
			hyper::StatusCode::FOUND
		};

		assert_eq!(response.status, expected_status);
	}
}
