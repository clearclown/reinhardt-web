//! Extended validator implementations
//!
//! This module provides advanced validator types for serializers,
//! including async validators, composite validators, and conditional validators.

use reinhardt_serializers::{ValidationError, ValidationResult};
use serde_json::Value;
use std::future::Future;
use std::pin::Pin;

/// Type alias for boxed futures returning validation results
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// An async validator that supports asynchronous validation
///
/// This validator allows you to perform validation asynchronously, useful for
/// operations that require I/O such as database lookups or API calls.
///
/// # Examples
///
/// ```
/// use serializers_ext::validator::AsyncValidator;
/// use serde_json::{json, Value};
/// use reinhardt_serializers::{ValidationResult, ValidationError};
///
/// #[tokio::main]
/// async fn main() {
///     let validator = AsyncValidator::new(|value: &Value| {
///         let s = value.as_str().unwrap_or("").to_string();
///         Box::pin(async move {
///             // Simulate async database check
///             tokio::time::sleep(std::time::Duration::from_millis(10)).await;
///             if s.len() > 3 {
///                 Ok(())
///             } else {
///                 Err(ValidationError::field_error("field", "Too short"))
///             }
///         })
///     });
///
///     let value = json!("valid");
///     assert!(validator.validate(&value).await.is_ok());
/// }
/// ```
pub struct AsyncValidator<F>
where
    F: Fn(&Value) -> BoxFuture<'static, ValidationResult>,
{
    validator_fn: F,
}

impl<F> AsyncValidator<F>
where
    F: Fn(&Value) -> BoxFuture<'static, ValidationResult>,
{
    /// Create a new async validator
    ///
    /// # Arguments
    ///
    /// * `validator_fn` - An async function that validates the value
    ///
    /// # Examples
    ///
    /// ```
    /// use serializers_ext::validator::AsyncValidator;
    /// use serde_json::Value;
    /// use reinhardt_serializers::{ValidationResult, ValidationError};
    ///
    /// let validator = AsyncValidator::new(|value: &Value| {
    ///     Box::pin(async move {
    ///         // Async validation logic
    ///         Ok(())
    ///     })
    /// });
    /// ```
    pub fn new(validator_fn: F) -> Self {
        Self { validator_fn }
    }

    /// Validate a value asynchronously
    ///
    /// # Arguments
    ///
    /// * `value` - The value to validate
    ///
    /// # Returns
    ///
    /// A future that resolves to the validation result
    pub async fn validate(&self, value: &Value) -> ValidationResult {
        (self.validator_fn)(value).await
    }
}

/// A composite validator that combines multiple validators
///
/// This validator runs multiple validators on the same value and collects
/// all validation errors. All validators are executed, even if earlier ones fail.
///
/// # Examples
///
/// ```
/// use serializers_ext::validator::CompositeValidator;
/// use serde_json::{json, Value};
/// use reinhardt_serializers::{ValidationResult, ValidationError, FieldValidator};
///
/// struct MinLengthValidator(usize);
///
/// impl FieldValidator for MinLengthValidator {
///     fn validate(&self, value: &Value) -> ValidationResult {
///         if let Some(s) = value.as_str() {
///             if s.len() >= self.0 {
///                 Ok(())
///             } else {
///                 Err(ValidationError::field_error("field", "Too short"))
///             }
///         } else {
///             Ok(())
///         }
///     }
/// }
///
/// struct MaxLengthValidator(usize);
///
/// impl FieldValidator for MaxLengthValidator {
///     fn validate(&self, value: &Value) -> ValidationResult {
///         if let Some(s) = value.as_str() {
///             if s.len() <= self.0 {
///                 Ok(())
///             } else {
///                 Err(ValidationError::field_error("field", "Too long"))
///             }
///         } else {
///             Ok(())
///         }
///     }
/// }
///
/// let mut validator = CompositeValidator::new();
/// validator.add(Box::new(MinLengthValidator(3)));
/// validator.add(Box::new(MaxLengthValidator(10)));
///
/// let value = json!("hello");
/// assert!(validator.validate(&value).is_ok());
///
/// let too_short = json!("hi");
/// assert!(validator.validate(&too_short).is_err());
/// ```
pub struct CompositeValidator {
    validators: Vec<Box<dyn reinhardt_serializers::FieldValidator>>,
}

impl CompositeValidator {
    /// Create a new composite validator
    ///
    /// # Examples
    ///
    /// ```
    /// use serializers_ext::validator::CompositeValidator;
    ///
    /// let validator = CompositeValidator::new();
    /// ```
    pub fn new() -> Self {
        Self {
            validators: Vec::new(),
        }
    }

    /// Add a validator to the composite
    ///
    /// # Examples
    ///
    /// ```
    /// use serializers_ext::validator::CompositeValidator;
    /// use serde_json::Value;
    /// use reinhardt_serializers::{ValidationResult, ValidationError, FieldValidator};
    ///
    /// struct CustomValidator;
    ///
    /// impl FieldValidator for CustomValidator {
    ///     fn validate(&self, value: &Value) -> ValidationResult {
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let mut validator = CompositeValidator::new();
    /// validator.add(Box::new(CustomValidator));
    /// ```
    pub fn add(&mut self, validator: Box<dyn reinhardt_serializers::FieldValidator>) {
        self.validators.push(validator);
    }

    /// Validate a value using all validators
    ///
    /// # Arguments
    ///
    /// * `value` - The value to validate
    ///
    /// # Returns
    ///
    /// `Ok(())` if all validators pass, otherwise returns a `MultipleErrors`
    /// containing all validation errors
    pub fn validate(&self, value: &Value) -> ValidationResult {
        let mut errors = Vec::new();

        for validator in &self.validators {
            if let Err(e) = validator.validate(value) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else if errors.len() == 1 {
            Err(errors.into_iter().next().unwrap())
        } else {
            Err(ValidationError::multiple(errors))
        }
    }

    /// Get the number of validators in the composite
    ///
    /// # Examples
    ///
    /// ```
    /// use serializers_ext::validator::CompositeValidator;
    /// use serde_json::Value;
    /// use reinhardt_serializers::{ValidationResult, ValidationError, FieldValidator};
    ///
    /// struct CustomValidator;
    ///
    /// impl FieldValidator for CustomValidator {
    ///     fn validate(&self, value: &Value) -> ValidationResult {
    ///         Ok(())
    ///     }
    /// }
    ///
    /// let mut validator = CompositeValidator::new();
    /// assert_eq!(validator.len(), 0);
    ///
    /// validator.add(Box::new(CustomValidator));
    /// assert_eq!(validator.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.validators.len()
    }

    /// Check if the composite has no validators
    pub fn is_empty(&self) -> bool {
        self.validators.is_empty()
    }
}

impl Default for CompositeValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// A conditional validator that validates based on a condition
///
/// This validator only performs validation when a condition is met.
/// When the condition is not met, validation automatically passes.
///
/// # Examples
///
/// ```
/// use serializers_ext::validator::ConditionalValidator;
/// use serde_json::{json, Value};
/// use reinhardt_serializers::{ValidationResult, ValidationError};
///
/// let validator = ConditionalValidator::new(
///     |value: &Value| value.as_str().is_some(),
///     |value: &Value| {
///         if value.as_str().unwrap().len() > 3 {
///             Ok(())
///         } else {
///             Err(ValidationError::field_error("field", "Too short"))
///         }
///     }
/// );
///
/// // String value - condition met, validation runs
/// let string_value = json!("hello");
/// assert!(validator.validate(&string_value).is_ok());
///
/// // Number value - condition not met, validation skipped
/// let number_value = json!(42);
/// assert!(validator.validate(&number_value).is_ok());
/// ```
pub struct ConditionalValidator<C, V>
where
    C: Fn(&Value) -> bool,
    V: Fn(&Value) -> ValidationResult,
{
    condition: C,
    validator: V,
}

impl<C, V> ConditionalValidator<C, V>
where
    C: Fn(&Value) -> bool,
    V: Fn(&Value) -> ValidationResult,
{
    /// Create a new conditional validator
    ///
    /// # Arguments
    ///
    /// * `condition` - A function that determines whether to validate
    /// * `validator` - A function that performs the validation when condition is true
    ///
    /// # Examples
    ///
    /// ```
    /// use serializers_ext::validator::ConditionalValidator;
    /// use serde_json::Value;
    /// use reinhardt_serializers::{ValidationResult, ValidationError};
    ///
    /// let validator = ConditionalValidator::new(
    ///     |value: &Value| value.as_str().is_some(),
    ///     |value: &Value| {
    ///         if value.as_str().unwrap().len() > 3 {
    ///             Ok(())
    ///         } else {
    ///             Err(ValidationError::field_error("field", "Too short"))
    ///         }
    ///     }
    /// );
    /// ```
    pub fn new(condition: C, validator: V) -> Self {
        Self {
            condition,
            validator,
        }
    }

    /// Validate a value if condition is met
    ///
    /// # Arguments
    ///
    /// * `value` - The value to validate
    ///
    /// # Returns
    ///
    /// The validation result if condition is true, otherwise `Ok(())`
    pub fn validate(&self, value: &Value) -> ValidationResult {
        if (self.condition)(value) {
            (self.validator)(value)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reinhardt_serializers::FieldValidator;
    use serde_json::json;

    struct MinLengthValidator(usize);

    impl FieldValidator for MinLengthValidator {
        fn validate(&self, value: &Value) -> ValidationResult {
            if let Some(s) = value.as_str() {
                if s.len() >= self.0 {
                    Ok(())
                } else {
                    Err(ValidationError::field_error("field", "Too short"))
                }
            } else {
                Ok(())
            }
        }
    }

    struct MaxLengthValidator(usize);

    impl FieldValidator for MaxLengthValidator {
        fn validate(&self, value: &Value) -> ValidationResult {
            if let Some(s) = value.as_str() {
                if s.len() <= self.0 {
                    Ok(())
                } else {
                    Err(ValidationError::field_error("field", "Too long"))
                }
            } else {
                Ok(())
            }
        }
    }

    #[tokio::test]
    async fn test_async_validator_valid() {
        let validator = AsyncValidator::new(|value: &Value| {
            let s = value.as_str().unwrap_or("").to_string();
            Box::pin(async move {
                if s.len() > 3 {
                    Ok(())
                } else {
                    Err(ValidationError::field_error("field", "Too short"))
                }
            })
        });

        let value = json!("valid");
        assert!(validator.validate(&value).await.is_ok());
    }

    #[tokio::test]
    async fn test_async_validator_invalid() {
        let validator = AsyncValidator::new(|value: &Value| {
            let s = value.as_str().unwrap_or("").to_string();
            Box::pin(async move {
                if s.len() > 3 {
                    Ok(())
                } else {
                    Err(ValidationError::field_error("field", "Too short"))
                }
            })
        });

        let value = json!("hi");
        assert!(validator.validate(&value).await.is_err());
    }

    #[tokio::test]
    async fn test_async_validator_with_delay() {
        let validator = AsyncValidator::new(|value: &Value| {
            let num = value.as_i64().unwrap_or(0);
            Box::pin(async move {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                if num > 10 {
                    Ok(())
                } else {
                    Err(ValidationError::field_error("field", "Too small"))
                }
            })
        });

        let value = json!(42);
        assert!(validator.validate(&value).await.is_ok());

        let small_value = json!(5);
        assert!(validator.validate(&small_value).await.is_err());
    }

    #[test]
    fn test_composite_validator_all_valid() {
        let mut validator = CompositeValidator::new();
        validator.add(Box::new(MinLengthValidator(3)));
        validator.add(Box::new(MaxLengthValidator(10)));

        let value = json!("hello");
        assert!(validator.validate(&value).is_ok());
    }

    #[test]
    fn test_composite_validator_one_invalid() {
        let mut validator = CompositeValidator::new();
        validator.add(Box::new(MinLengthValidator(3)));
        validator.add(Box::new(MaxLengthValidator(10)));

        let too_short = json!("hi");
        let result = validator.validate(&too_short);
        assert!(result.is_err());
    }

    #[test]
    fn test_composite_validator_multiple_invalid() {
        let mut validator = CompositeValidator::new();
        validator.add(Box::new(MinLengthValidator(5)));
        validator.add(Box::new(MaxLengthValidator(3)));

        let value = json!("test");
        let result = validator.validate(&value);
        assert!(result.is_err());

        if let Err(ValidationError::MultipleErrors(errors)) = result {
            assert_eq!(errors.len(), 2);
        } else {
            panic!("Expected MultipleErrors");
        }
    }

    #[test]
    fn test_composite_validator_empty() {
        let validator = CompositeValidator::new();
        assert!(validator.is_empty());
        assert_eq!(validator.len(), 0);

        let value = json!("test");
        assert!(validator.validate(&value).is_ok());
    }

    #[test]
    fn test_composite_validator_len() {
        let mut validator = CompositeValidator::new();
        assert_eq!(validator.len(), 0);

        validator.add(Box::new(MinLengthValidator(3)));
        assert_eq!(validator.len(), 1);

        validator.add(Box::new(MaxLengthValidator(10)));
        assert_eq!(validator.len(), 2);
    }

    #[test]
    fn test_conditional_validator_condition_true_valid() {
        let validator = ConditionalValidator::new(
            |value: &Value| value.as_str().is_some(),
            |value: &Value| {
                if value.as_str().unwrap().len() > 3 {
                    Ok(())
                } else {
                    Err(ValidationError::field_error("field", "Too short"))
                }
            },
        );

        let value = json!("hello");
        assert!(validator.validate(&value).is_ok());
    }

    #[test]
    fn test_conditional_validator_condition_true_invalid() {
        let validator = ConditionalValidator::new(
            |value: &Value| value.as_str().is_some(),
            |value: &Value| {
                if value.as_str().unwrap().len() > 3 {
                    Ok(())
                } else {
                    Err(ValidationError::field_error("field", "Too short"))
                }
            },
        );

        let value = json!("hi");
        assert!(validator.validate(&value).is_err());
    }

    #[test]
    fn test_conditional_validator_condition_false() {
        let validator = ConditionalValidator::new(
            |value: &Value| value.as_str().is_some(),
            |value: &Value| {
                if value.as_str().unwrap().len() > 3 {
                    Ok(())
                } else {
                    Err(ValidationError::field_error("field", "Too short"))
                }
            },
        );

        // Number value - condition false, validation skipped
        let value = json!(42);
        assert!(validator.validate(&value).is_ok());
    }

    #[test]
    fn test_conditional_validator_with_complex_condition() {
        let validator = ConditionalValidator::new(
            |value: &Value| {
                value
                    .get("validate_email")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
            },
            |value: &Value| {
                let email = value.get("email").and_then(|v| v.as_str()).unwrap_or("");
                if email.contains('@') {
                    Ok(())
                } else {
                    Err(ValidationError::field_error("email", "Invalid email"))
                }
            },
        );

        // Validation enabled with valid email
        let valid_obj = json!({"validate_email": true, "email": "test@example.com"});
        assert!(validator.validate(&valid_obj).is_ok());

        // Validation enabled with invalid email
        let invalid_obj = json!({"validate_email": true, "email": "invalid"});
        assert!(validator.validate(&invalid_obj).is_err());

        // Validation disabled with invalid email
        let disabled_obj = json!({"validate_email": false, "email": "invalid"});
        assert!(validator.validate(&disabled_obj).is_ok());
    }

    #[test]
    fn test_conditional_validator_missing_condition_field() {
        let validator = ConditionalValidator::new(
            |value: &Value| {
                value
                    .get("should_validate")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
            },
            |_value: &Value| Err(ValidationError::field_error("field", "Always fails")),
        );

        // Missing condition field defaults to false
        let value = json!({"data": "test"});
        assert!(validator.validate(&value).is_ok());
    }
}
