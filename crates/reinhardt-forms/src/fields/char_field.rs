//! Character field for text input

use super::{Field, FormError, FormResult};

/// Character field with length validation
#[derive(Debug, Clone)]
pub struct CharField {
    pub required: bool,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub label: Option<String>,
    pub help_text: Option<String>,
    pub initial: Option<String>,
    pub strip: bool,
    pub empty_value: Option<String>,
}

impl CharField {
    /// Create a new CharField with default settings
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::CharField;
    ///
    /// let field = CharField::new();
    /// assert!(!field.required);
    /// assert_eq!(field.max_length, None);
    /// ```
    pub fn new() -> Self {
        Self {
            required: false,
            max_length: None,
            min_length: None,
            label: None,
            help_text: None,
            initial: None,
            strip: true,
            empty_value: None,
        }
    }
    /// Set the field as required
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::CharField;
    ///
    /// let field = CharField::new().required();
    /// assert!(field.required);
    /// ```
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    /// Set the maximum length for the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::CharField;
    ///
    /// let field = CharField::new().with_max_length(100);
    /// assert_eq!(field.max_length, Some(100));
    /// ```
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }
    /// Set the minimum length for the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::CharField;
    ///
    /// let field = CharField::new().with_min_length(5);
    /// assert_eq!(field.min_length, Some(5));
    /// ```
    pub fn with_min_length(mut self, min_length: usize) -> Self {
        self.min_length = Some(min_length);
        self
    }
    /// Set the label for the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::CharField;
    ///
    /// let field = CharField::new().with_label("Username");
    /// assert_eq!(field.label, Some("Username".to_string()));
    /// ```
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    /// Set the help text for the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::CharField;
    ///
    /// let field = CharField::new().with_help_text("Enter your username");
    /// assert_eq!(field.help_text, Some("Enter your username".to_string()));
    /// ```
    pub fn with_help_text(mut self, help_text: impl Into<String>) -> Self {
        self.help_text = Some(help_text.into());
        self
    }
    /// Set the initial value for the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::CharField;
    ///
    /// let field = CharField::new().with_initial("default value");
    /// assert_eq!(field.initial, Some("default value".to_string()));
    /// ```
    pub fn with_initial(mut self, initial: impl Into<String>) -> Self {
        self.initial = Some(initial.into());
        self
    }
    /// Disable whitespace stripping for the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::CharField;
    ///
    /// let field = CharField::new().no_strip();
    /// assert!(!field.strip);
    /// ```
    pub fn no_strip(mut self) -> Self {
        self.strip = false;
        self
    }
}

impl Default for CharField {
    fn default() -> Self {
        Self::new()
    }
}

impl FormField for CharField {
    fn clean(&self, value: Option<&str>) -> FormResult<Option<String>> {
        let value = match value {
            Some(v) => {
                let v = if self.strip { v.trim() } else { v };
                if v.is_empty() {
                    if self.required {
                        return Err(FormError::Validation("This field is required".to_string()));
                    }
                    return Ok(self.empty_value.clone());
                }
                v.to_string()
            }
            None => {
                if self.required {
                    return Err(FormError::Validation("This field is required".to_string()));
                }
                return Ok(self.empty_value.clone());
            }
        };

        // Validate length
        if let Some(max_length) = self.max_length {
            if value.len() > max_length {
                return Err(FormError::Validation(format!(
                    "Ensure this value has at most {} characters (it has {})",
                    max_length,
                    value.len()
                )));
            }
        }

        if let Some(min_length) = self.min_length {
            if value.len() < min_length {
                return Err(FormError::Validation(format!(
                    "Ensure this value has at least {} characters (it has {})",
                    min_length,
                    value.len()
                )));
            }
        }

        Ok(Some(value))
    }

    fn widget_type(&self) -> &str {
        "text"
    }

    fn is_required(&self) -> bool {
        self.required
    }

    fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn help_text(&self) -> Option<&str> {
        self.help_text.as_deref()
    }

    fn initial(&self) -> Option<&str> {
        self.initial.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_field_required() {
        let field = CharField::new().required();
        assert!(field.clean(None).is_err());
        assert!(field.clean(Some("")).is_err());
        assert!(field.clean(Some("  ")).is_err());
    }

    #[test]
    fn test_char_field_max_length() {
        let field = CharField::new().with_max_length(5);
        assert!(field.clean(Some("12345")).is_ok());
        assert!(field.clean(Some("123456")).is_err());
    }

    #[test]
    fn test_char_field_min_length() {
        let field = CharField::new().with_min_length(3);
        assert!(field.clean(Some("123")).is_ok());
        assert!(field.clean(Some("12")).is_err());
    }
}
