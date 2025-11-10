//! Core serialization traits and implementations
//!
//! Provides the foundational `Serializer` and `Deserializer` traits along with
//! error types for serialization operations.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Core serializer trait for converting between input and output representations
///
/// # Type Parameters
///
/// - `Input`: The source type to serialize
/// - `Output`: The target serialized representation
///
/// # Examples
///
/// ```
/// use reinhardt_core_serializers::{Serializer, JsonSerializer};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// struct User { id: i64, name: String }
///
/// let user = User { id: 1, name: "Alice".to_string() };
/// let serializer = JsonSerializer::<User>::new();
/// let json = serializer.serialize(&user).unwrap();
/// assert!(json.contains("Alice"));
/// ```
pub trait Serializer {
	type Input;
	type Output;

	fn serialize(&self, input: &Self::Input) -> Result<Self::Output, SerializerError>;
	fn deserialize(&self, output: &Self::Output) -> Result<Self::Input, SerializerError>;
}

/// Errors that can occur during validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidatorError {
	/// Unique constraint violation
	UniqueViolation {
		field_name: String,
		value: String,
		message: String,
	},
	/// Unique together constraint violation
	UniqueTogetherViolation {
		field_names: Vec<String>,
		values: HashMap<String, String>,
		message: String,
	},
	/// Required field missing
	RequiredField { field_name: String, message: String },
	/// Field validation error
	FieldValidation {
		field_name: String,
		value: String,
		constraint: String,
		message: String,
	},
	/// Database error
	DatabaseError {
		message: String,
		source: Option<String>,
	},
	/// Custom validation error
	Custom { message: String },
}

impl std::fmt::Display for ValidatorError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ValidatorError::UniqueViolation {
				field_name,
				value,
				message,
			} => write!(
				f,
				"Unique violation on field '{}' with value '{}': {}",
				field_name, value, message
			),
			ValidatorError::UniqueTogetherViolation {
				field_names,
				values,
				message,
			} => write!(
				f,
				"Unique together violation on fields {:?} with values {:?}: {}",
				field_names, values, message
			),
			ValidatorError::RequiredField {
				field_name,
				message,
			} => write!(f, "Required field '{}': {}", field_name, message),
			ValidatorError::FieldValidation {
				field_name,
				value,
				constraint,
				message,
			} => write!(
				f,
				"Field validation error on '{}' with value '{}' (constraint '{}'): {}",
				field_name, value, constraint, message
			),
			ValidatorError::DatabaseError { message, source } => {
				if let Some(src) = source {
					write!(f, "Database error: {} (source: {})", message, src)
				} else {
					write!(f, "Database error: {}", message)
				}
			}
			ValidatorError::Custom { message } => write!(f, "Validation error: {}", message),
		}
	}
}

impl std::error::Error for ValidatorError {}

/// Errors that can occur during serialization
#[derive(Debug, Clone)]
pub enum SerializerError {
	/// Validation error
	Validation(ValidatorError),
	/// Serde serialization/deserialization error
	Serde { message: String },
	/// Other error
	Other { message: String },
}

impl std::fmt::Display for SerializerError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			SerializerError::Validation(e) => write!(f, "Validation error: {}", e),
			SerializerError::Serde { message } => write!(f, "Serde error: {}", message),
			SerializerError::Other { message } => write!(f, "Serialization error: {}", message),
		}
	}
}

impl std::error::Error for SerializerError {}

impl From<ValidatorError> for SerializerError {
	fn from(err: ValidatorError) -> Self {
		SerializerError::Validation(err)
	}
}

// Integration with reinhardt_exception is moved to REST layer
// Base layer remains exception-agnostic

/// JSON serializer implementation
///
/// Provides JSON serialization/deserialization using serde_json.
///
/// # Examples
///
/// ```
/// use reinhardt_core_serializers::{Serializer, JsonSerializer};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct User { id: i64, name: String }
///
/// let user = User { id: 1, name: "Alice".to_string() };
/// let serializer = JsonSerializer::<User>::new();
///
/// let json = serializer.serialize(&user).unwrap();
/// let deserialized = serializer.deserialize(&json).unwrap();
/// assert_eq!(user.id, deserialized.id);
/// ```
pub struct JsonSerializer<T> {
	_phantom: std::marker::PhantomData<T>,
}

impl<T> JsonSerializer<T> {
	/// Create a new JSON serializer
	pub fn new() -> Self {
		Self {
			_phantom: std::marker::PhantomData,
		}
	}
}

impl<T> Default for JsonSerializer<T> {
	fn default() -> Self {
		Self::new()
	}
}

impl<T> Serializer for JsonSerializer<T>
where
	T: Serialize + for<'de> Deserialize<'de>,
{
	type Input = T;
	type Output = String;

	fn serialize(&self, input: &Self::Input) -> Result<Self::Output, SerializerError> {
		serde_json::to_string(input).map_err(|e| SerializerError::Serde {
			message: format!("Serialization error: {}", e),
		})
	}

	fn deserialize(&self, output: &Self::Output) -> Result<Self::Input, SerializerError> {
		serde_json::from_str(output).map_err(|e| SerializerError::Serde {
			message: format!("Deserialization error: {}", e),
		})
	}
}

/// Deserializer trait for one-way deserialization
///
/// # Examples
///
/// ```
/// use reinhardt_core_serializers::Deserializer;
/// use serde::{Deserialize, Serialize};
///
/// struct JsonDeserializer;
///
/// impl Deserializer for JsonDeserializer {
///     type Input = String;
///     type Output = serde_json::Value;
///
///     fn deserialize(&self, input: &Self::Input) -> Result<Self::Output, reinhardt_core_serializers::SerializerError> {
///         serde_json::from_str(input).map_err(|e| reinhardt_core_serializers::SerializerError::Serde {
///             message: format!("Deserialization error: {}", e),
///         })
///     }
/// }
/// ```
pub trait Deserializer {
	type Input;
	type Output;

	fn deserialize(&self, input: &Self::Input) -> Result<Self::Output, SerializerError>;
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Serialize, Deserialize, PartialEq, Debug)]
	struct TestUser {
		id: i64,
		name: String,
	}

	#[test]
	fn test_json_serializer_roundtrip() {
		let user = TestUser {
			id: 1,
			name: "Alice".to_string(),
		};
		let serializer = JsonSerializer::<TestUser>::new();

		let json = serializer.serialize(&user).unwrap();
		let deserialized = serializer.deserialize(&json).unwrap();

		assert_eq!(user.id, deserialized.id);
		assert_eq!(user.name, deserialized.name);
	}

	#[test]
	fn test_json_serializer_serialize() {
		let user = TestUser {
			id: 1,
			name: "Alice".to_string(),
		};
		let serializer = JsonSerializer::<TestUser>::new();

		let json = serializer.serialize(&user).unwrap();
		assert!(json.contains("Alice"));
		assert!(json.contains("\"id\":1"));
	}

	#[test]
	fn test_json_serializer_deserialize() {
		let json = r#"{"id":1,"name":"Alice"}"#;
		let serializer = JsonSerializer::<TestUser>::new();

		let user = serializer.deserialize(json).unwrap();
		assert_eq!(user.id, 1);
		assert_eq!(user.name, "Alice");
	}

	#[test]
	fn test_json_serializer_deserialize_error() {
		let invalid_json = r#"{"invalid"}"#;
		let serializer = JsonSerializer::<TestUser>::new();

		let result = serializer.deserialize(invalid_json);
		assert!(result.is_err());
	}

	#[test]
	fn test_validator_error_display() {
		let err = ValidatorError::UniqueViolation {
			field_name: "email".to_string(),
			value: "test@example.com".to_string(),
			message: "Email already exists".to_string(),
		};
		assert!(err.to_string().contains("email"));
		assert!(err.to_string().contains("test@example.com"));
	}

	#[test]
	fn test_serializer_error_from_validator_error() {
		let validator_err = ValidatorError::Custom {
			message: "test error".to_string(),
		};
		let serializer_err: SerializerError = validator_err.into();

		match serializer_err {
			SerializerError::Validation(_) => {}
			_ => panic!("Expected Validation error"),
		}
	}
}
