//! Integration tests for Server implementation

use hyper::Method;
use reinhardt_apps::Response;
use reinhardt_routers::UnifiedRouter as Router;
use reinhardt_server::HttpServer;
use std::sync::Arc;

#[tokio::test]
async fn test_server_basic_request() {
	let router = Arc::new(Router::new().function(
		"/test",
		Method::GET,
		|_req| async { Ok(Response::ok().with_body("Server works!")) },
	));

	// Spawn server in background
	let addr = "127.0.0.1:0".parse::<std::net::SocketAddr>().unwrap();
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	let server_addr = listener.local_addr().unwrap();

	let router_clone = router.clone();
	tokio::spawn(async move {
		let server = HttpServer::new(router_clone);
		let _ = server.listen(server_addr).await;
	});

	// Give server time to start
	tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

	// Make HTTP request
	let client = reqwest::Client::new();
	let response = client
		.get(format!("http://{}/test", server_addr))
		.send()
		.await
		.unwrap();

	assert_eq!(response.status(), reqwest::StatusCode::OK);
	assert_eq!(response.text().await.unwrap(), "Server works!");
}

#[tokio::test]
async fn test_server_multiple_requests() {
	let router = Arc::new(
		Router::new()
			.function(
				"/hello",
				Method::GET,
				|_req| async { Ok(Response::ok().with_body("Hello!")) },
			)
			.function(
				"/goodbye",
				Method::GET,
				|_req| async { Ok(Response::ok().with_body("Goodbye!")) },
			),
	);

	let addr = "127.0.0.1:0".parse::<std::net::SocketAddr>().unwrap();
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	let server_addr = listener.local_addr().unwrap();

	let router_clone = router.clone();
	tokio::spawn(async move {
		let server = HttpServer::new(router_clone);
		let _ = server.listen(server_addr).await;
	});

	tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

	let client = reqwest::Client::new();

	let response = client
		.get(format!("http://{}/hello", server_addr))
		.send()
		.await
		.unwrap();
	assert_eq!(response.text().await.unwrap(), "Hello!");

	let response = client
		.get(format!("http://{}/goodbye", server_addr))
		.send()
		.await
		.unwrap();
	assert_eq!(response.text().await.unwrap(), "Goodbye!");
}

#[tokio::test]
async fn test_server_post_request() {
	let router = Arc::new(Router::new().function(
		"/submit",
		Method::POST,
		|request| async move {
			let body_str = String::from_utf8(request.body().to_vec()).unwrap_or_default();
			Ok(Response::ok().with_body(format!("Received: {}", body_str)))
		},
	));

	let addr = "127.0.0.1:0".parse::<std::net::SocketAddr>().unwrap();
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	let server_addr = listener.local_addr().unwrap();

	let router_clone = router.clone();
	tokio::spawn(async move {
		let server = HttpServer::new(router_clone);
		let _ = server.listen(server_addr).await;
	});

	tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

	let client = reqwest::Client::new();
	let response = client
		.post(format!("http://{}/submit", server_addr))
		.body("test data")
		.send()
		.await
		.unwrap();

	assert_eq!(response.status(), reqwest::StatusCode::OK);
	assert_eq!(response.text().await.unwrap(), "Received: test data");
}

#[tokio::test]
async fn test_server_json_request_response() {
	let router = Arc::new(Router::new().function(
		"/echo",
		Method::POST,
		|request| async move {
			let json: serde_json::Value = request.json()?;
			Response::ok().with_json(&json)
		},
	));

	let addr = "127.0.0.1:0".parse::<std::net::SocketAddr>().unwrap();
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	let server_addr = listener.local_addr().unwrap();

	let router_clone = router.clone();
	tokio::spawn(async move {
		let server = HttpServer::new(router_clone);
		let _ = server.listen(server_addr).await;
	});

	tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

	let client = reqwest::Client::new();
	let test_data = serde_json::json!({
		"name": "Alice",
		"age": 30
	});

	let response = client
		.post(format!("http://{}/echo", server_addr))
		.json(&test_data)
		.send()
		.await
		.unwrap();

	assert_eq!(response.status(), reqwest::StatusCode::OK);
	let response_json: serde_json::Value = response.json().await.unwrap();
	assert_eq!(response_json["name"], "Alice");
	assert_eq!(response_json["age"], 30);
}

#[tokio::test]
async fn test_server_404_response() {
	let router = Arc::new(Router::new().function(
		"/exists",
		Method::GET,
		|_req| async { Ok(Response::ok().with_body("I exist!")) },
	));

	let addr = "127.0.0.1:0".parse::<std::net::SocketAddr>().unwrap();
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	let server_addr = listener.local_addr().unwrap();

	let router_clone = router.clone();
	tokio::spawn(async move {
		let server = HttpServer::new(router_clone);
		let _ = server.listen(server_addr).await;
	});

	tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

	let client = reqwest::Client::new();
	let response = client
		.get(format!("http://{}/notfound", server_addr))
		.send()
		.await
		.unwrap();

	assert_eq!(response.status(), reqwest::StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_server_concurrent_requests() {
	let router = Arc::new(Router::new().function(
		"/test",
		Method::GET,
		|request| async move { Ok(Response::ok().with_body(format!("Method: {}", request.method))) },
	));

	let addr = "127.0.0.1:0".parse::<std::net::SocketAddr>().unwrap();
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	let server_addr = listener.local_addr().unwrap();

	let router_clone = router.clone();
	tokio::spawn(async move {
		let server = HttpServer::new(router_clone);
		let _ = server.listen(server_addr).await;
	});

	tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

	// Make multiple concurrent requests
	let mut handles = vec![];
	for _ in 0..10 {
		let addr = server_addr;
		let handle = tokio::spawn(async move {
			let client = reqwest::Client::new();
			client
				.get(format!("http://{}/test", addr))
				.send()
				.await
				.unwrap()
				.text()
				.await
				.unwrap()
		});
		handles.push(handle);
	}

	// Wait for all requests to complete
	for handle in handles {
		let result = handle.await.unwrap();
		assert_eq!(result, "Method: GET");
	}
}

#[tokio::test]
async fn test_server_custom_headers() {
	let router = Arc::new(Router::new().function(
		"/headers",
		Method::GET,
		|request| async move {
			let user_agent = request
				.headers
				.get(hyper::header::USER_AGENT)
				.and_then(|v| v.to_str().ok())
				.unwrap_or("Unknown");
			Ok(Response::ok().with_body(format!("User-Agent: {}", user_agent)))
		},
	));

	let addr = "127.0.0.1:0".parse::<std::net::SocketAddr>().unwrap();
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	let server_addr = listener.local_addr().unwrap();

	let router_clone = router.clone();
	tokio::spawn(async move {
		let server = HttpServer::new(router_clone);
		let _ = server.listen(server_addr).await;
	});

	tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

	let client = reqwest::Client::new();
	let response = client
		.get(format!("http://{}/headers", server_addr))
		.header("User-Agent", "TestAgent/1.0")
		.send()
		.await
		.unwrap();

	assert_eq!(response.status(), reqwest::StatusCode::OK);
	assert_eq!(response.text().await.unwrap(), "User-Agent: TestAgent/1.0");
}

#[tokio::test]
async fn test_server_path_parameters() {
	let router = Arc::new(Router::new().function(
		"/users/:id",
		Method::GET,
		|request| async move {
			let id = request.path_params.get("id").unwrap();
			Ok(Response::ok().with_body(format!("ID: {}", id)))
		},
	));

	let addr = "127.0.0.1:0".parse::<std::net::SocketAddr>().unwrap();
	let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
	let server_addr = listener.local_addr().unwrap();

	let router_clone = router.clone();
	tokio::spawn(async move {
		let server = HttpServer::new(router_clone);
		let _ = server.listen(server_addr).await;
	});

	tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

	let client = reqwest::Client::new();
	let response = client
		.get(format!("http://{}/users/123", server_addr))
		.send()
		.await
		.unwrap();

	assert_eq!(response.status(), reqwest::StatusCode::OK);
	assert_eq!(response.text().await.unwrap(), "ID: 123");
}
