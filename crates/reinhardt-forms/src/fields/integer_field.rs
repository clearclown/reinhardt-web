//! Integer field
use super::{Field, FormError, FormResult};

#[derive(Debug, Clone)]
pub struct IntegerField {
    pub required: bool,
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,
}

impl IntegerField {
    /// Create a new IntegerField with default settings
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::IntegerField;
    ///
    /// let field = IntegerField::new();
    /// assert!(!field.required);
    /// assert_eq!(field.min_value, None);
    /// ```
    pub fn new() -> Self {
        Self {
            required: false,
            min_value: None,
            max_value: None,
        }
    }
    /// Set the field as required
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::IntegerField;
    ///
    /// let field = IntegerField::new().required();
    /// assert!(field.required);
    /// ```
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    /// Set the minimum value for the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::IntegerField;
    ///
    /// let field = IntegerField::new().with_min_value(0);
    /// assert_eq!(field.min_value, Some(0));
    /// ```
    pub fn with_min_value(mut self, min_value: i64) -> Self {
        self.min_value = Some(min_value);
        self
    }
    /// Set the maximum value for the field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::IntegerField;
    ///
    /// let field = IntegerField::new().with_max_value(100);
    /// assert_eq!(field.max_value, Some(100));
    /// ```
    pub fn with_max_value(mut self, max_value: i64) -> Self {
        self.max_value = Some(max_value);
        self
    }
}

impl Default for IntegerField {
    fn default() -> Self {
        Self::new()
    }
}

impl FormField for IntegerField {
    fn clean(&self, value: Option<&str>) -> FormResult<Option<String>> {
        let value = match value {
            Some(v) if !v.trim().is_empty() => v.trim(),
            _ => {
                if self.required {
                    return Err(FormError::Validation("This field is required".to_string()));
                }
                return Ok(None);
            }
        };

        let num: i64 = value
            .parse()
            .map_err(|_| FormError::Validation("Enter a valid integer".to_string()))?;

        if let Some(min) = self.min_value {
            if num < min {
                return Err(FormError::Validation(format!(
                    "Ensure this value is greater than or equal to {}",
                    min
                )));
            }
        }

        if let Some(max) = self.max_value {
            if num > max {
                return Err(FormError::Validation(format!(
                    "Ensure this value is less than or equal to {}",
                    max
                )));
            }
        }

        Ok(Some(num.to_string()))
    }

    fn widget_type(&self) -> &str {
        "number"
    }

    fn is_required(&self) -> bool {
        self.required
    }
}
