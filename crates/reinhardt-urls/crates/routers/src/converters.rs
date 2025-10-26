//! Path parameter converters for type-specific validation and conversion.
//!
//! This module provides converters for common path parameter types:
//! - `IntegerConverter`: Validates and converts integer path parameters
//! - `UuidConverter`: Validates and converts UUID path parameters
//! - `SlugConverter`: Validates slug format (lowercase alphanumeric + hyphens)
//!
//! # Examples
//!
//! ```
//! use reinhardt_routers::converters::{Converter, IntegerConverter, UuidConverter, SlugConverter};
//!
//! // Integer converter
//! let int_conv = IntegerConverter::new();
//! assert!(int_conv.validate("123"));
//! assert!(!int_conv.validate("abc"));
//!
//! // UUID converter
//! let uuid_conv = UuidConverter;
//! assert!(uuid_conv.validate("550e8400-e29b-41d4-a716-446655440000"));
//! assert!(!uuid_conv.validate("not-a-uuid"));
//!
//! // Slug converter
//! let slug_conv = SlugConverter;
//! assert!(slug_conv.validate("my-blog-post"));
//! assert!(!slug_conv.validate("Invalid Slug!"));
//! ```

use regex::Regex;
use std::sync::OnceLock;
use thiserror::Error;

/// Error type for converter validation failures
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ConverterError {
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    #[error("Value out of range: {0}")]
    OutOfRange(String),
}

/// Result type for converter operations
pub type ConverterResult<T> = Result<T, ConverterError>;

/// Trait for path parameter converters
///
/// Converters validate and optionally transform path parameters
/// before they are used in route handlers.
pub trait Converter: Send + Sync {
    /// The target type this converter produces
    type Output;

    /// Validate a path parameter value
    ///
    /// Returns `true` if the value is valid for this converter.
    fn validate(&self, value: &str) -> bool;

    /// Convert a validated path parameter to the target type
    ///
    /// # Errors
    ///
    /// Returns `ConverterError` if the value cannot be converted.
    fn convert(&self, value: &str) -> ConverterResult<Self::Output>;

    /// Get the regex pattern for this converter
    ///
    /// Used for route pattern matching.
    fn pattern(&self) -> &str;
}

/// Integer converter with optional range validation
///
/// Validates that path parameters are valid integers, optionally
/// within a specified range.
///
/// # Examples
///
/// ```
/// use reinhardt_routers::converters::{Converter, IntegerConverter};
///
/// // Without range limits
/// let conv = IntegerConverter::new();
/// assert!(conv.validate("123"));
/// assert!(conv.validate("-456"));
/// assert!(!conv.validate("abc"));
///
/// // With range limits
/// let conv = IntegerConverter::with_range(1, 100);
/// assert!(conv.validate("50"));
/// assert!(!conv.validate("150")); // Out of range
/// ```
#[derive(Debug, Clone)]
pub struct IntegerConverter {
    min: Option<i64>,
    max: Option<i64>,
}

impl IntegerConverter {
    /// Create a new integer converter without range limits
    pub fn new() -> Self {
        Self {
            min: None,
            max: None,
        }
    }

    /// Create an integer converter with range limits
    ///
    /// # Arguments
    ///
    /// * `min` - Minimum allowed value (inclusive)
    /// * `max` - Maximum allowed value (inclusive)
    pub fn with_range(min: i64, max: i64) -> Self {
        Self {
            min: Some(min),
            max: Some(max),
        }
    }
}

impl Default for IntegerConverter {
    fn default() -> Self {
        Self::new()
    }
}

impl Converter for IntegerConverter {
    type Output = i64;

    fn validate(&self, value: &str) -> bool {
        if let Ok(num) = value.parse::<i64>() {
            if let Some(min) = self.min {
                if num < min {
                    return false;
                }
            }
            if let Some(max) = self.max {
                if num > max {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    fn convert(&self, value: &str) -> ConverterResult<Self::Output> {
        let num = value.parse::<i64>().map_err(|_| {
            ConverterError::InvalidFormat(format!("'{}' is not a valid integer", value))
        })?;

        if let Some(min) = self.min {
            if num < min {
                return Err(ConverterError::OutOfRange(format!(
                    "{} is less than minimum {}",
                    num, min
                )));
            }
        }

        if let Some(max) = self.max {
            if num > max {
                return Err(ConverterError::OutOfRange(format!(
                    "{} is greater than maximum {}",
                    num, max
                )));
            }
        }

        Ok(num)
    }

    fn pattern(&self) -> &str {
        r"-?\d+"
    }
}

/// UUID converter (version 4)
///
/// Validates that path parameters are valid UUIDs.
///
/// # Examples
///
/// ```
/// use reinhardt_routers::converters::{Converter, UuidConverter};
///
/// let conv = UuidConverter;
/// assert!(conv.validate("550e8400-e29b-41d4-a716-446655440000"));
/// assert!(conv.validate("6ba7b810-9dad-11d1-80b4-00c04fd430c8"));
/// assert!(!conv.validate("not-a-uuid"));
/// assert!(!conv.validate("550e8400-e29b-41d4-a716")); // Invalid format
/// ```
#[derive(Debug, Clone, Copy)]
pub struct UuidConverter;

impl UuidConverter {
    fn regex() -> &'static Regex {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        REGEX.get_or_init(|| {
            Regex::new(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$")
                .expect("Invalid UUID regex pattern")
        })
    }
}

impl Converter for UuidConverter {
    type Output = String;

    fn validate(&self, value: &str) -> bool {
        Self::regex().is_match(value)
    }

    fn convert(&self, value: &str) -> ConverterResult<Self::Output> {
        if self.validate(value) {
            Ok(value.to_string())
        } else {
            Err(ConverterError::InvalidFormat(format!(
                "'{}' is not a valid UUID",
                value
            )))
        }
    }

    fn pattern(&self) -> &str {
        r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}"
    }
}

/// Slug converter
///
/// Validates that path parameters are valid slugs (lowercase alphanumeric
/// characters and hyphens).
///
/// # Examples
///
/// ```
/// use reinhardt_routers::converters::{Converter, SlugConverter};
///
/// let conv = SlugConverter;
/// assert!(conv.validate("my-blog-post"));
/// assert!(conv.validate("article-123"));
/// assert!(conv.validate("hello-world"));
/// assert!(!conv.validate("Invalid Slug!"));
/// assert!(!conv.validate("no_underscores"));
/// assert!(!conv.validate("NO-UPPERCASE"));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct SlugConverter;

impl SlugConverter {
    fn regex() -> &'static Regex {
        static REGEX: OnceLock<Regex> = OnceLock::new();
        REGEX.get_or_init(|| {
            Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").expect("Invalid slug regex pattern")
        })
    }
}

impl Converter for SlugConverter {
    type Output = String;

    fn validate(&self, value: &str) -> bool {
        Self::regex().is_match(value)
    }

    fn convert(&self, value: &str) -> ConverterResult<Self::Output> {
        if self.validate(value) {
            Ok(value.to_string())
        } else {
            Err(ConverterError::InvalidFormat(format!(
                "'{}' is not a valid slug (must be lowercase alphanumeric with hyphens)",
                value
            )))
        }
    }

    fn pattern(&self) -> &str {
        r"[a-z0-9]+(-[a-z0-9]+)*"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integer_converter_basic() {
        let conv = IntegerConverter::new();

        // Valid integers
        assert!(conv.validate("123"));
        assert!(conv.validate("-456"));
        assert!(conv.validate("0"));

        // Invalid values
        assert!(!conv.validate("abc"));
        assert!(!conv.validate("12.5"));
        assert!(!conv.validate(""));
    }

    #[test]
    fn test_integer_converter_with_range() {
        let conv = IntegerConverter::with_range(1, 100);

        // Within range
        assert!(conv.validate("50"));
        assert!(conv.validate("1"));
        assert!(conv.validate("100"));

        // Out of range
        assert!(!conv.validate("0"));
        assert!(!conv.validate("101"));
        assert!(!conv.validate("-10"));
    }

    #[test]
    fn test_integer_converter_convert() {
        let conv = IntegerConverter::new();

        assert_eq!(conv.convert("123").unwrap(), 123);
        assert_eq!(conv.convert("-456").unwrap(), -456);
        assert!(conv.convert("abc").is_err());
    }

    #[test]
    fn test_integer_converter_convert_with_range() {
        let conv = IntegerConverter::with_range(1, 100);

        assert_eq!(conv.convert("50").unwrap(), 50);
        assert!(conv.convert("0").is_err());
        assert!(conv.convert("101").is_err());
    }

    #[test]
    fn test_uuid_converter() {
        let conv = UuidConverter;

        // Valid UUIDs
        assert!(conv.validate("550e8400-e29b-41d4-a716-446655440000"));
        assert!(conv.validate("6ba7b810-9dad-11d1-80b4-00c04fd430c8"));

        // Invalid UUIDs
        assert!(!conv.validate("not-a-uuid"));
        assert!(!conv.validate("550e8400-e29b-41d4-a716")); // Too short
        assert!(!conv.validate("550e8400-e29b-41d4-a716-446655440000-extra")); // Too long
        assert!(!conv.validate("550E8400-E29B-41D4-A716-446655440000")); // Uppercase
    }

    #[test]
    fn test_uuid_converter_convert() {
        let conv = UuidConverter;

        let uuid = "550e8400-e29b-41d4-a716-446655440000";
        assert_eq!(conv.convert(uuid).unwrap(), uuid);
        assert!(conv.convert("not-a-uuid").is_err());
    }

    #[test]
    fn test_slug_converter() {
        let conv = SlugConverter;

        // Valid slugs
        assert!(conv.validate("my-blog-post"));
        assert!(conv.validate("article-123"));
        assert!(conv.validate("hello-world"));
        assert!(conv.validate("simple"));

        // Invalid slugs
        assert!(!conv.validate("Invalid Slug!"));
        assert!(!conv.validate("no_underscores"));
        assert!(!conv.validate("NO-UPPERCASE"));
        assert!(!conv.validate("-starts-with-hyphen"));
        assert!(!conv.validate("ends-with-hyphen-"));
        assert!(!conv.validate("double--hyphens"));
    }

    #[test]
    fn test_slug_converter_convert() {
        let conv = SlugConverter;

        assert_eq!(conv.convert("my-blog-post").unwrap(), "my-blog-post");
        assert!(conv.convert("Invalid Slug!").is_err());
    }

    #[test]
    fn test_converter_patterns() {
        let int_conv = IntegerConverter::new();
        let uuid_conv = UuidConverter;
        let slug_conv = SlugConverter;

        assert_eq!(int_conv.pattern(), r"-?\d+");
        assert_eq!(
            uuid_conv.pattern(),
            r"[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}"
        );
        assert_eq!(slug_conv.pattern(), r"[a-z0-9]+(-[a-z0-9]+)*");
    }
}
