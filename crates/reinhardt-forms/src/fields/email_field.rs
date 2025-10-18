//! Email field with validation

use super::{Field, FormError, FormResult};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct EmailField {
    pub required: bool,
    pub label: Option<String>,
}

impl EmailField {
    /// Create a new EmailField with default settings
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::EmailField;
    ///
    /// let field = EmailField::new();
    /// assert!(!field.required);
    /// assert_eq!(field.label, None);
    /// ```
    pub fn new() -> Self {
        Self {
            required: false,
            label: None,
        }
    }
    /// Set the field as required
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::EmailField;
    ///
    /// let field = EmailField::new().required();
    /// assert!(field.required);
    /// ```
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

impl Default for EmailField {
    fn default() -> Self {
        Self::new()
    }
}

impl FormField for EmailField {
    fn clean(&self, value: Option<&str>) -> FormResult<Option<String>> {
        let value = match value {
            Some(v) if !v.trim().is_empty() => v.trim().to_string(),
            _ => {
                if self.required {
                    return Err(FormError::Validation("This field is required".to_string()));
                }
                return Ok(None);
            }
        };

        // Simple email regex validation
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(&value) {
            return Err(FormError::Validation(
                "Enter a valid email address".to_string(),
            ));
        }

        Ok(Some(value))
    }

    fn widget_type(&self) -> &str {
        "email"
    }

    fn is_required(&self) -> bool {
        self.required
    }
}
