//! AWS Secrets Manager provider
//!
//! This module provides integration with AWS Secrets Manager for retrieving secrets.

use crate::secrets::{SecretError, SecretMetadata, SecretProvider, SecretResult, SecretString};
use async_trait::async_trait;
use chrono::Utc;
use serde_json::Value;

#[cfg(feature = "aws-secrets")]
use aws_config::BehaviorVersion;
#[cfg(feature = "aws-secrets")]
use aws_sdk_secretsmanager::Client;

/// AWS Secrets Manager provider
///
/// This provider retrieves secrets from AWS Secrets Manager.
///
/// # Example
///
/// ```no_run
/// use reinhardt_settings::secrets::providers::aws::AwsSecretsProvider;
/// use reinhardt_settings::prelude::SecretProvider;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let provider = AwsSecretsProvider::new(None).await?;
/// let secret = provider.get_secret("database/password").await?;
/// # Ok(())
/// # }
/// ```
pub struct AwsSecretsProvider {
	#[cfg(feature = "aws-secrets")]
	client: Client,
	#[cfg(not(feature = "aws-secrets"))]
	_phantom: std::marker::PhantomData<()>,
	prefix: Option<String>,
}

impl AwsSecretsProvider {
	/// Create a new AWS Secrets Manager provider
	///
	/// # Arguments
	///
	/// * `prefix` - Optional prefix to prepend to all secret names
	///
	/// # Example
	///
	/// ```no_run
	/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
	/// use reinhardt_settings::secrets::providers::aws::AwsSecretsProvider;
	///
	// Without prefix
	/// let provider = AwsSecretsProvider::new(None).await?;
	///
	// With prefix
	/// let provider = AwsSecretsProvider::new(Some("myapp/".to_string())).await?;
	/// # Ok(())
	/// # }
	/// ```
	#[cfg(feature = "aws-secrets")]
	/// Documentation for `new`
	pub async fn new(prefix: Option<String>) -> SecretResult<Self> {
		let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
		let client = Client::new(&config);

		Ok(Self { client, prefix })
	}

	#[cfg(not(feature = "aws-secrets"))]
	/// Documentation for `new`
	pub async fn new(_prefix: Option<String>) -> SecretResult<Self> {
		Err(SecretError::Provider(
			"AWS Secrets Manager support not enabled. Enable the 'aws-secrets' feature."
				.to_string(),
		))
	}

	/// Create a provider with custom AWS config
	#[cfg(feature = "aws-secrets")]
	/// Documentation for `with_config`
	pub async fn with_config(
		config: aws_config::SdkConfig,
		prefix: Option<String>,
	) -> SecretResult<Self> {
		let client = Client::new(&config);
		Ok(Self { client, prefix })
	}

	/// Create a provider with custom endpoint (for testing)
	#[cfg(feature = "aws-secrets")]
	pub async fn with_endpoint(endpoint_url: String, prefix: Option<String>) -> SecretResult<Self> {
		use aws_sdk_secretsmanager::config::{Credentials, Region};
		use aws_smithy_runtime::client::http::hyper_014::HyperClientBuilder;

		// Create static credentials for testing
		let credentials = Credentials::new(
			"test-access-key",
			"test-secret-key",
			None,
			None,
			"static-credentials",
		);

		// Create HTTP client that supports both HTTP and HTTPS
		let http_client = HyperClientBuilder::new().build_https();

		let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
		let client = Client::from_conf(
			aws_sdk_secretsmanager::config::Builder::from(&config)
				.endpoint_url(endpoint_url)
				.region(Region::new("us-east-1"))
				.credentials_provider(credentials)
				.http_client(http_client)
				.build(),
		);
		Ok(Self { client, prefix })
	}

	/// Get the full secret name with prefix
	fn get_full_name(&self, key: &str) -> String {
		match &self.prefix {
			Some(prefix) => format!("{}{}", prefix, key),
			None => key.to_string(),
		}
	}

	/// Parse secret value from AWS response
	#[cfg(feature = "aws-secrets")]
	fn parse_secret_value(&self, secret_string: &str) -> SecretResult<String> {
		// Try to parse as JSON first
		if let Ok(json_value) = serde_json::from_str::<Value>(secret_string) {
			// If it's a JSON object with a single key, return that value
			if let Some(obj) = json_value.as_object()
				&& obj.len() == 1
				&& let Some(value) = obj.values().next()
				&& let Some(string_value) = value.as_str()
			{
				return Ok(string_value.to_string());
			}
		}

		// Otherwise, return the raw string
		Ok(secret_string.to_string())
	}
}

#[async_trait]
impl SecretProvider for AwsSecretsProvider {
	#[cfg(feature = "aws-secrets")]
	async fn get_secret(&self, key: &str) -> SecretResult<SecretString> {
		let full_name = self.get_full_name(key);

		let result = self
			.client
			.get_secret_value()
			.secret_id(&full_name)
			.send()
			.await;

		match result {
			Ok(output) => {
				if let Some(secret_string) = output.secret_string() {
					let value = self.parse_secret_value(secret_string)?;
					Ok(SecretString::new(value))
				} else {
					Err(SecretError::NotFound(format!(
						"Secret '{}' has no value",
						key
					)))
				}
			}
			Err(err) => {
				// Check error type using AWS SDK's error handling
				use aws_sdk_secretsmanager::operation::get_secret_value::GetSecretValueError;

				if err
					.as_service_error()
					.is_some_and(|e| matches!(e, GetSecretValueError::ResourceNotFoundException(_)))
				{
					Err(SecretError::NotFound(format!(
						"Secret '{}' not found in AWS Secrets Manager",
						key
					)))
				} else {
					Err(SecretError::Provider(format!(
						"AWS Secrets Manager error: {}",
						err
					)))
				}
			}
		}
	}

	#[cfg(not(feature = "aws-secrets"))]
	async fn get_secret(&self, _key: &str) -> SecretResult<SecretString> {
		Err(SecretError::Provider(
			"AWS Secrets Manager support not enabled".to_string(),
		))
	}

	#[cfg(feature = "aws-secrets")]
	async fn get_secret_with_metadata(
		&self,
		key: &str,
	) -> SecretResult<(SecretString, SecretMetadata)> {
		let full_name = self.get_full_name(key);

		let result = self
			.client
			.get_secret_value()
			.secret_id(&full_name)
			.send()
			.await;

		match result {
			Ok(output) => {
				if let Some(secret_string) = output.secret_string() {
					let value = self.parse_secret_value(secret_string)?;

					let metadata = SecretMetadata {
						created_at: output.created_date().map(|dt| {
							chrono::DateTime::from_timestamp(dt.secs(), dt.subsec_nanos())
								.unwrap_or_else(Utc::now)
						}),
						updated_at: Some(Utc::now()),
					};

					Ok((SecretString::new(value), metadata))
				} else {
					Err(SecretError::NotFound(format!(
						"Secret '{}' has no value",
						key
					)))
				}
			}
			Err(err) => {
				// Check error type using AWS SDK's error handling
				use aws_sdk_secretsmanager::operation::get_secret_value::GetSecretValueError;

				if err
					.as_service_error()
					.is_some_and(|e| matches!(e, GetSecretValueError::ResourceNotFoundException(_)))
				{
					Err(SecretError::NotFound(format!(
						"Secret '{}' not found in AWS Secrets Manager",
						key
					)))
				} else {
					Err(SecretError::Provider(format!(
						"AWS Secrets Manager error: {}",
						err
					)))
				}
			}
		}
	}

	#[cfg(not(feature = "aws-secrets"))]
	async fn get_secret_with_metadata(
		&self,
		_key: &str,
	) -> SecretResult<(SecretString, SecretMetadata)> {
		Err(SecretError::Provider(
			"AWS Secrets Manager support not enabled".to_string(),
		))
	}

	#[cfg(feature = "aws-secrets")]
	async fn set_secret(&self, key: &str, value: SecretString) -> SecretResult<()> {
		let full_name = self.get_full_name(key);

		// Try to update existing secret first
		let update_result = self
			.client
			.update_secret()
			.secret_id(&full_name)
			.secret_string(value.expose_secret())
			.send()
			.await;

		match update_result {
			Ok(_) => Ok(()),
			Err(err) => {
				// Check error type using AWS SDK's error handling
				use aws_sdk_secretsmanager::operation::update_secret::UpdateSecretError;

				// If secret doesn't exist, create it
				if err
					.as_service_error()
					.is_some_and(|e| matches!(e, UpdateSecretError::ResourceNotFoundException(_)))
				{
					self.client
						.create_secret()
						.name(&full_name)
						.secret_string(value.expose_secret())
						.send()
						.await
						.map_err(|e| {
							SecretError::Provider(format!("Failed to create secret: {}", e))
						})?;
					Ok(())
				} else {
					Err(SecretError::Provider(format!(
						"Failed to update secret: {}",
						err
					)))
				}
			}
		}
	}

	#[cfg(not(feature = "aws-secrets"))]
	async fn set_secret(&self, _key: &str, _value: SecretString) -> SecretResult<()> {
		Err(SecretError::Provider(
			"AWS Secrets Manager support not enabled".to_string(),
		))
	}

	#[cfg(feature = "aws-secrets")]
	async fn delete_secret(&self, key: &str) -> SecretResult<()> {
		let full_name = self.get_full_name(key);

		self.client
			.delete_secret()
			.secret_id(&full_name)
			.force_delete_without_recovery(true)
			.send()
			.await
			.map_err(|e| SecretError::Provider(format!("Failed to delete secret: {}", e)))?;

		Ok(())
	}

	#[cfg(not(feature = "aws-secrets"))]
	async fn delete_secret(&self, _key: &str) -> SecretResult<()> {
		Err(SecretError::Provider(
			"AWS Secrets Manager support not enabled".to_string(),
		))
	}

	#[cfg(feature = "aws-secrets")]
	async fn list_secrets(&self) -> SecretResult<Vec<String>> {
		let mut secrets = Vec::new();
		let mut next_token: Option<String> = None;

		loop {
			let mut request = self.client.list_secrets();

			if let Some(token) = next_token {
				request = request.next_token(token);
			}

			let response = request
				.send()
				.await
				.map_err(|e| SecretError::Provider(format!("Failed to list secrets: {}", e)))?;

			for secret in response.secret_list() {
				if let Some(name) = secret.name() {
					// Remove prefix if present
					let key = if let Some(prefix) = &self.prefix {
						if let Some(stripped) = name.strip_prefix(prefix) {
							stripped.to_string()
						} else {
							continue; // Skip secrets that don't match our prefix
						}
					} else {
						name.to_string()
					};

					secrets.push(key);
				}
			}

			next_token = response.next_token().map(|s| s.to_string());

			if next_token.is_none() {
				break;
			}
		}

		Ok(secrets)
	}

	#[cfg(not(feature = "aws-secrets"))]
	async fn list_secrets(&self) -> SecretResult<Vec<String>> {
		Err(SecretError::Provider(
			"AWS Secrets Manager support not enabled".to_string(),
		))
	}

	fn exists(&self, _key: &str) -> bool {
		// Cannot make async calls in sync method
		// Consumers should use get_secret() and check for NotFound error
		false
	}

	fn name(&self) -> &str {
		"aws-secrets-manager"
	}
}

#[cfg(all(test, feature = "aws-secrets"))]
mod tests {
	use super::*;
	use wiremock::{
		Mock, MockServer, ResponseTemplate,
		matchers::{method, path},
	};

	/// Create a mock AWS Secrets Manager endpoint
	async fn create_mock_server() -> MockServer {
		MockServer::start().await
	}

	/// Mock response for GetSecretValue API (success)
	fn mock_get_secret_response(secret_value: &str) -> String {
		serde_json::json!({
			"ARN": "arn:aws:secretsmanager:us-east-1:123456789012:secret:test-secret-AbCdEf",
			"Name": "test-secret",
			"SecretString": secret_value,
			"VersionId": "EXAMPLE1-90ab-cdef-fedc-ba987EXAMPLE",
			"CreatedDate": 1523477145.713
		})
		.to_string()
	}

	/// Test: AWS provider creation with mock server
	///
	/// This test verifies that AwsSecretsProvider can be created successfully
	/// with a custom endpoint (mock HTTP server).
	#[tokio::test]
	async fn test_aws_provider_creation() {
		let mock_server = create_mock_server().await;
		let endpoint = mock_server.uri();

		let result = AwsSecretsProvider::with_endpoint(endpoint, None).await;
		assert!(result.is_ok());
	}

	/// Test: Getting a secret successfully
	///
	/// This test verifies that the provider can successfully retrieve a secret
	/// from the mock AWS Secrets Manager API.
	#[tokio::test]
	async fn test_aws_get_secret_success() {
		let mock_server = create_mock_server().await;

		// Mock the GetSecretValue API endpoint
		Mock::given(method("POST"))
			.and(path("/"))
			.respond_with(
				ResponseTemplate::new(200)
					.set_body_string(mock_get_secret_response("my-secret-value"))
					.insert_header("content-type", "application/x-amz-json-1.1"),
			)
			.mount(&mock_server)
			.await;

		let provider = AwsSecretsProvider::with_endpoint(mock_server.uri(), None)
			.await
			.unwrap();

		let result = provider.get_secret("test-secret").await;
		assert!(result.is_ok(), "Expected Ok, got: {:?}", result);
		assert_eq!(result.unwrap().expose_secret(), "my-secret-value");
	}

	/// Test: Getting a JSON secret with single key
	///
	/// This test verifies that the provider correctly parses JSON secrets
	/// with a single key-value pair (common AWS Secrets Manager pattern).
	#[tokio::test]
	async fn test_aws_get_json_secret() {
		let mock_server = create_mock_server().await;

		// Mock JSON secret response
		let json_secret = r#"{"password":"super-secret-password"}"#;
		Mock::given(method("POST"))
			.and(path("/"))
			.respond_with(
				ResponseTemplate::new(200)
					.set_body_string(mock_get_secret_response(json_secret))
					.insert_header("content-type", "application/x-amz-json-1.1"),
			)
			.mount(&mock_server)
			.await;

		let provider = AwsSecretsProvider::with_endpoint(mock_server.uri(), None)
			.await
			.unwrap();

		let result = provider.get_secret("test-json-secret").await;
		assert!(result.is_ok());
		// Should extract the value from the JSON object
		assert_eq!(result.unwrap().expose_secret(), "super-secret-password");
	}

	/// Test: Getting a non-existent secret returns NotFound error
	///
	/// This test verifies that attempting to retrieve a non-existent secret
	/// returns the appropriate SecretError::NotFound error.
	#[tokio::test]
	async fn test_aws_get_nonexistent_secret() {
		let mock_server = create_mock_server().await;

		// Mock ResourceNotFoundException response with proper AWS Smithy error format
		// AWS SDK uses smithy error format, not just HTTP status codes
		let error_response = serde_json::json!({
			"__type": "com.amazonaws.secretsmanager#ResourceNotFoundException",
			"message": "Secrets Manager can't find the specified secret."
		})
		.to_string();

		Mock::given(method("POST"))
			.and(path("/"))
			.respond_with(
				ResponseTemplate::new(400)
					.set_body_string(error_response)
					.insert_header("x-amzn-errortype", "ResourceNotFoundException")
					.insert_header("content-type", "application/x-amz-json-1.1"),
			)
			.mount(&mock_server)
			.await;

		let provider =
			AwsSecretsProvider::with_endpoint(mock_server.uri(), Some("test/".to_string()))
				.await
				.unwrap();

		let result = provider.get_secret("nonexistent-secret-12345").await;

		assert!(result.is_err());
		match result {
			Err(SecretError::NotFound(_)) => {
				// Expected error type
			}
			_ => panic!("Expected NotFound error, got: {:?}", result),
		}
	}

	/// Test: Setting a secret (update existing)
	///
	/// This test verifies that the provider can update an existing secret.
	#[tokio::test]
	async fn test_aws_set_secret_update() {
		let mock_server = create_mock_server().await;

		// Mock successful UpdateSecret response
		Mock::given(method("POST"))
			.and(path("/"))
			.respond_with(
				ResponseTemplate::new(200)
					.set_body_string(
						serde_json::json!({
							"ARN": "arn:aws:secretsmanager:us-east-1:123456789012:secret:test-secret-AbCdEf",
							"Name": "test-secret",
							"VersionId": "EXAMPLE2-90ab-cdef-fedc-ba987EXAMPLE"
						})
						.to_string(),
					)
					.insert_header("content-type", "application/x-amz-json-1.1"),
			)
			.mount(&mock_server)
			.await;

		let provider = AwsSecretsProvider::with_endpoint(mock_server.uri(), None)
			.await
			.unwrap();

		let result = provider
			.set_secret("test-secret", SecretString::new("new-value".to_string()))
			.await;

		assert!(result.is_ok());
	}

	/// Test: Setting a secret (create new)
	///
	/// This test verifies that the provider can create a new secret
	/// when updating a non-existent secret.
	#[tokio::test]
	async fn test_aws_set_secret_create() {
		let mock_server = create_mock_server().await;

		// First call: UpdateSecret returns ResourceNotFoundException
		let error_response = serde_json::json!({
			"__type": "com.amazonaws.secretsmanager#ResourceNotFoundException",
			"message": "Secrets Manager can't find the specified secret."
		})
		.to_string();

		Mock::given(method("POST"))
			.and(path("/"))
			.respond_with(
				ResponseTemplate::new(400)
					.set_body_string(error_response)
					.insert_header("x-amzn-errortype", "ResourceNotFoundException")
					.insert_header("content-type", "application/x-amz-json-1.1"),
			)
			.up_to_n_times(1)
			.mount(&mock_server)
			.await;

		// Second call: CreateSecret succeeds
		Mock::given(method("POST"))
			.and(path("/"))
			.respond_with(
				ResponseTemplate::new(200)
					.set_body_string(
						serde_json::json!({
							"ARN": "arn:aws:secretsmanager:us-east-1:123456789012:secret:new-secret-AbCdEf",
							"Name": "new-secret",
							"VersionId": "EXAMPLE1-90ab-cdef-fedc-ba987EXAMPLE"
						})
						.to_string(),
					)
					.insert_header("content-type", "application/x-amz-json-1.1"),
			)
			.mount(&mock_server)
			.await;

		let provider = AwsSecretsProvider::with_endpoint(mock_server.uri(), None)
			.await
			.unwrap();

		let result = provider
			.set_secret("new-secret", SecretString::new("new-value".to_string()))
			.await;

		assert!(result.is_ok());
	}

	/// Test: Deleting a secret
	///
	/// This test verifies that the provider can delete a secret.
	#[tokio::test]
	async fn test_aws_delete_secret() {
		let mock_server = create_mock_server().await;

		// Mock successful DeleteSecret response
		Mock::given(method("POST"))
			.and(path("/"))
			.respond_with(
				ResponseTemplate::new(200).set_body_string(
					serde_json::json!({
						"ARN": "arn:aws:secretsmanager:us-east-1:123456789012:secret:test-secret-AbCdEf",
						"Name": "test-secret",
						"DeletionDate": 1524085349.095
					})
					.to_string(),
				),
			)
			.mount(&mock_server)
			.await;

		let provider = AwsSecretsProvider::with_endpoint(mock_server.uri(), None)
			.await
			.unwrap();

		let result = provider.delete_secret("test-secret").await;
		assert!(result.is_ok());
	}

	/// Test: Listing secrets with prefix
	///
	/// This test verifies that the provider can list secrets and correctly
	/// filter by prefix.
	#[tokio::test]
	async fn test_aws_list_secrets_with_prefix() {
		let mock_server = create_mock_server().await;

		// Mock ListSecrets response
		Mock::given(method("POST"))
			.and(path("/"))
			.respond_with(
				ResponseTemplate::new(200).set_body_string(
					serde_json::json!({
						"SecretList": [
							{
								"ARN": "arn:aws:secretsmanager:us-east-1:123456789012:secret:myapp/db-password-AbCdEf",
								"Name": "myapp/db-password"
							},
							{
								"ARN": "arn:aws:secretsmanager:us-east-1:123456789012:secret:myapp/api-key-GhIjKl",
								"Name": "myapp/api-key"
							},
							{
								"ARN": "arn:aws:secretsmanager:us-east-1:123456789012:secret:other-secret-MnOpQr",
								"Name": "other-secret"
							}
						]
					})
					.to_string(),
				),
			)
			.mount(&mock_server)
			.await;

		let provider =
			AwsSecretsProvider::with_endpoint(mock_server.uri(), Some("myapp/".to_string()))
				.await
				.unwrap();

		let result = provider.list_secrets().await;
		assert!(result.is_ok());

		let secrets = result.unwrap();
		assert_eq!(secrets.len(), 2);
		assert!(secrets.contains(&"db-password".to_string()));
		assert!(secrets.contains(&"api-key".to_string()));
		// "other-secret" should be filtered out
		assert!(!secrets.contains(&"other-secret".to_string()));
	}

	/// Test: Getting secret with metadata
	///
	/// This test verifies that the provider can retrieve a secret along
	/// with its metadata (creation date, etc.).
	#[tokio::test]
	async fn test_aws_get_secret_with_metadata() {
		let mock_server = create_mock_server().await;

		Mock::given(method("POST"))
			.and(path("/"))
			.respond_with(
				ResponseTemplate::new(200)
					.set_body_string(mock_get_secret_response("secret-with-metadata")),
			)
			.mount(&mock_server)
			.await;

		let provider = AwsSecretsProvider::with_endpoint(mock_server.uri(), None)
			.await
			.unwrap();

		let result = provider.get_secret_with_metadata("test-secret").await;
		assert!(result.is_ok());

		let (secret, metadata) = result.unwrap();
		assert_eq!(secret.expose_secret(), "secret-with-metadata");
		assert!(metadata.created_at.is_some());
		assert!(metadata.updated_at.is_some());
	}
}

#[cfg(all(test, not(feature = "aws-secrets")))]
mod tests_no_feature {
	use super::*;

	#[tokio::test]
	async fn test_aws_provider_disabled() {
		let result = AwsSecretsProvider::new(None).await;
		assert!(result.is_err());
	}
}
