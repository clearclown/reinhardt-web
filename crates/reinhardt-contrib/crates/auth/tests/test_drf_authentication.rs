use reinhardt_auth::drf_authentication::{
    AuthError, AuthRequest, Authentication, BasicAuthentication, SessionData, TokenAuthentication,
};
use std::collections::HashMap;

// === Token Authentication Tests ===

#[tokio::test]
async fn test_token_authentication_missing_header() {
    let token_auth = TokenAuthentication::new();
    let request = AuthRequest::new();

    let result = token_auth.authenticate(&request).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthError::MissingCredentials));
}

#[tokio::test]
async fn test_token_authentication_invalid_format() {
    let token_auth = TokenAuthentication::new();

    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        "Token invalid_token".to_string(),
    );

    let request = AuthRequest::new().with_headers(headers);

    let result = token_auth.authenticate(&request).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthError::InvalidToken));
}

#[tokio::test]
async fn test_token_authentication_valid_format() {
    let token_auth = TokenAuthentication::new();

    // Valid 40-character hex token
    let valid_token = "a".repeat(40);
    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        format!("Token {}", valid_token),
    );

    let request = AuthRequest::new().with_headers(headers);

    let result = token_auth.authenticate(&request).await;
    // Without database feature, should return InvalidToken error
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthError::InvalidToken));
}

#[tokio::test]
async fn test_token_authentication_missing_token_prefix() {
    let token_auth = TokenAuthentication::new();

    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "a".repeat(40));

    let request = AuthRequest::new().with_headers(headers);

    let result = token_auth.authenticate(&request).await;
    // Should return MissingCredentials without "Token " prefix
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthError::MissingCredentials));
}

#[tokio::test]
async fn test_token_authentication_bearer_prefix() {
    let token_auth = TokenAuthentication::new();

    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        format!("Bearer {}", "a".repeat(40)),
    );

    let request = AuthRequest::new().with_headers(headers);

    let result = token_auth.authenticate(&request).await;
    // Should return MissingCredentials (not matching Bearer prefix, expecting Token)
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthError::MissingCredentials));
}

// === Basic Authentication Tests ===

#[tokio::test]
async fn test_basic_authentication_valid_credentials() {
    let basic_auth = BasicAuthentication::new();

    // Base64 encode "testuser:testpass"
    let credentials = base64::encode("testuser:testpass");
    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        format!("Basic {}", credentials),
    );

    let request = AuthRequest::new().with_headers(headers);

    let result = basic_auth.authenticate(&request).await;
    // Without database, will return error
    assert!(result.is_err());
}

#[tokio::test]
async fn test_basic_authentication_missing_header() {
    let basic_auth = BasicAuthentication::new();
    let request = AuthRequest::new();

    let result = basic_auth.authenticate(&request).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthError::MissingCredentials));
}

#[tokio::test]
async fn test_basic_authentication_invalid_base64() {
    let basic_auth = BasicAuthentication::new();

    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        "Basic invalid!!!base64".to_string(),
    );

    let request = AuthRequest::new().with_headers(headers);

    let result = basic_auth.authenticate(&request).await;
    // Invalid base64 cannot be decoded, returns MissingCredentials
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthError::MissingCredentials));
}

#[tokio::test]
async fn test_basic_authentication_missing_password() {
    let basic_auth = BasicAuthentication::new();

    // Base64 encode "testuser" (no colon, no password)
    let credentials = base64::encode("testuser");
    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        format!("Basic {}", credentials),
    );

    let request = AuthRequest::new().with_headers(headers);

    let result = basic_auth.authenticate(&request).await;
    // Missing password (no colon) cannot be parsed, returns MissingCredentials
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthError::MissingCredentials));
}

#[tokio::test]
async fn test_basic_authentication_empty_password() {
    let basic_auth = BasicAuthentication::new();

    // Base64 encode "testuser:" (empty password)
    let credentials = base64::encode("testuser:");
    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        format!("Basic {}", credentials),
    );

    let request = AuthRequest::new().with_headers(headers);

    let result = basic_auth.authenticate(&request).await;
    // Empty password should still be parsed
    assert!(result.is_err());
}

#[tokio::test]
async fn test_basic_authentication_utf8_credentials() {
    let basic_auth = BasicAuthentication::new();

    // UTF-8 username and password with special characters
    let credentials = base64::encode("用户名:パスワード");
    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        format!("Basic {}", credentials),
    );

    let request = AuthRequest::new().with_headers(headers);

    let result = basic_auth.authenticate(&request).await;
    // Should parse UTF-8 credentials correctly
    assert!(result.is_err());
}

#[tokio::test]
async fn test_basic_authentication_credentials_with_colon() {
    let basic_auth = BasicAuthentication::new();

    // Password contains colon - should only split on first colon
    let credentials = base64::encode("testuser:pass:word:123");
    let mut headers = HashMap::new();
    headers.insert(
        "Authorization".to_string(),
        format!("Basic {}", credentials),
    );

    let request = AuthRequest::new().with_headers(headers);

    let result = basic_auth.authenticate(&request).await;
    // Should parse correctly (username=testuser, password=pass:word:123)
    assert!(result.is_err());
}

// === Session Data Tests ===

#[test]
fn test_session_data_creation() {
    let session = SessionData::new("session_key_123");

    assert_eq!(session.session_key, "session_key_123");
    assert!(session.user_id.is_none());
    assert!(session.data.is_empty());
}

#[test]
fn test_session_data_with_user_id() {
    let session = SessionData::new("session_key_123").with_user_id(42);

    assert_eq!(session.user_id, Some(42));
}

#[test]
fn test_session_data_get_set() {
    let mut session = SessionData::new("session_key_123");

    session.set("key1", "value1");
    session.set("key2", "value2");

    assert_eq!(session.get("key1"), Some(&"value1".to_string()));
    assert_eq!(session.get("key2"), Some(&"value2".to_string()));
    assert_eq!(session.get("nonexistent"), None);
}

#[test]
fn test_session_data_overwrite() {
    let mut session = SessionData::new("session_key_123");

    session.set("key", "value1");
    assert_eq!(session.get("key"), Some(&"value1".to_string()));

    session.set("key", "value2");
    assert_eq!(session.get("key"), Some(&"value2".to_string()));
}

// === AuthRequest Tests ===

#[test]
fn test_auth_request_creation() {
    let request = AuthRequest::new();

    assert!(request.headers.is_empty());
    assert!(request.cookies.is_empty());
    assert!(request.session_data.is_none());
}

#[test]
fn test_auth_request_with_headers() {
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Token abc123".to_string());

    let request = AuthRequest::new().with_headers(headers.clone());

    assert_eq!(
        request.get_header("Authorization"),
        Some(&"Token abc123".to_string())
    );
    assert_eq!(request.get_header("Nonexistent"), None);
}

#[test]
fn test_auth_request_with_cookies() {
    let mut cookies = HashMap::new();
    cookies.insert("sessionid".to_string(), "session123".to_string());

    let request = AuthRequest::new().with_cookies(cookies.clone());

    assert_eq!(
        request.get_cookie("sessionid"),
        Some(&"session123".to_string())
    );
    assert_eq!(request.get_cookie("nonexistent"), None);
}

#[test]
fn test_auth_request_with_session() {
    let session = SessionData::new("session_key").with_user_id(42);
    let request = AuthRequest::new().with_session(session.clone());

    assert!(request.session_data.is_some());
    let req_session = request.session_data.unwrap();
    assert_eq!(req_session.session_key, "session_key");
    assert_eq!(req_session.user_id, Some(42));
}

#[test]
fn test_auth_request_builder_pattern() {
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), "Token abc".to_string());

    let mut cookies = HashMap::new();
    cookies.insert("sessionid".to_string(), "sess".to_string());

    let session = SessionData::new("key");

    let request = AuthRequest::new()
        .with_headers(headers)
        .with_cookies(cookies)
        .with_session(session);

    assert!(request.get_header("Authorization").is_some());
    assert!(request.get_cookie("sessionid").is_some());
    assert!(request.session_data.is_some());
}

// === AuthError Tests ===

#[test]
fn test_auth_error_debug_format() {
    // Test that all AuthError variants can be formatted for debugging
    let err1 = AuthError::InvalidCredentials;
    let err2 = AuthError::TokenExpired;
    let err3 = AuthError::MissingCredentials;
    let err4 = AuthError::InvalidToken;
    let err5 = AuthError::DatabaseError("connection failed".to_string());
    let err6 = AuthError::SessionError("session not found".to_string());
    let err7 = AuthError::MfaRequired;

    // Verify debug output contains expected strings
    assert!(format!("{:?}", err1).contains("InvalidCredentials"));
    assert!(format!("{:?}", err2).contains("TokenExpired"));
    assert!(format!("{:?}", err3).contains("MissingCredentials"));
    assert!(format!("{:?}", err4).contains("InvalidToken"));
    assert!(format!("{:?}", err5).contains("DatabaseError"));
    assert!(format!("{:?}", err5).contains("connection failed"));
    assert!(format!("{:?}", err6).contains("SessionError"));
    assert!(format!("{:?}", err6).contains("session not found"));
    assert!(format!("{:?}", err7).contains("MfaRequired"));
}
