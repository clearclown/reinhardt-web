//! Boolean field
use super::{Field, FormResult};

#[derive(Debug, Clone)]
pub struct BooleanField {
    pub required: bool,
}

impl BooleanField {
    /// Create a new BooleanField with default settings
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::BooleanField;
    ///
    /// let field = BooleanField::new();
    /// assert!(!field.required);
    /// ```
    pub fn new() -> Self {
        Self { required: false }
    }
    /// Set the field as required
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::BooleanField;
    ///
    /// let field = BooleanField::new().required();
    /// assert!(field.required);
    /// ```
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
}

impl Default for BooleanField {
    fn default() -> Self {
        Self::new()
    }
}

impl FormField for BooleanField {
    fn clean(&self, value: Option<&str>) -> FormResult<Option<String>> {
        let result = match value {
            Some("true") | Some("1") | Some("on") | Some("yes") => "true",
            _ => "false",
        };
        Ok(Some(result.to_string()))
    }

    fn widget_type(&self) -> &str {
        "checkbox"
    }

    fn is_required(&self) -> bool {
        self.required
    }
}
