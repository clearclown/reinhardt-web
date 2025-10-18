use crate::field::{FieldError, FieldResult, FormField, Widget};
use chrono::NaiveDate;

/// DateField for date input
pub struct DateField {
    pub name: String,
    pub label: Option<String>,
    pub required: bool,
    pub help_text: Option<String>,
    pub widget: Widget,
    pub initial: Option<serde_json::Value>,
    pub input_formats: Vec<String>,
    pub localize: bool,
    pub locale: Option<String>,
}

impl DateField {
    /// Create a new DateField with the given name
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::DateField;
    ///
    /// let field = DateField::new("birth_date".to_string());
    /// assert_eq!(field.name, "birth_date");
    /// assert!(field.required);
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            label: None,
            required: true,
            help_text: None,
            widget: Widget::DateInput,
            initial: None,
            input_formats: vec![
                "%Y-%m-%d".to_string(),  // 2025-01-15
                "%m/%d/%Y".to_string(),  // 01/15/2025
                "%m/%d/%y".to_string(),  // 01/15/25
                "%b %d %Y".to_string(),  // Jan 15 2025
                "%b %d, %Y".to_string(), // Jan 15, 2025
                "%d %b %Y".to_string(),  // 15 Jan 2025
                "%d %b, %Y".to_string(), // 15 Jan, 2025
                "%B %d %Y".to_string(),  // January 15 2025
                "%B %d, %Y".to_string(), // January 15, 2025
                "%d %B %Y".to_string(),  // 15 January 2025
                "%d %B, %Y".to_string(), // 15 January, 2025
            ],
            localize: false,
            locale: None,
        }
    }
    /// Enable localization for this field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::DateField;
    ///
    /// let field = DateField::new("date".to_string()).with_localize(true);
    /// assert!(field.localize);
    /// ```
    pub fn with_localize(mut self, localize: bool) -> Self {
        self.localize = localize;
        self
    }
    /// Set the locale for this field
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::DateField;
    ///
    /// let field = DateField::new("date".to_string()).with_locale("en_US".to_string());
    /// assert_eq!(field.locale, Some("en_US".to_string()));
    /// ```
    pub fn with_locale(mut self, locale: String) -> Self {
        self.locale = Some(locale);
        self
    }

    fn parse_date(&self, s: &str) -> Result<NaiveDate, String> {
        for format in &self.input_formats {
            if let Ok(date) = NaiveDate::parse_from_str(s, format) {
                return Ok(date);
            }
        }
        Err(format!("Enter a valid date"))
    }
}

impl FormField for DateField {
    fn name(&self) -> &str {
        &self.name
    }

    fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    fn required(&self) -> bool {
        self.required
    }

    fn help_text(&self) -> Option<&str> {
        self.help_text.as_deref()
    }

    fn widget(&self) -> &Widget {
        &self.widget
    }

    fn initial(&self) -> Option<&serde_json::Value> {
        self.initial.as_ref()
    }

    fn clean(&self, value: Option<&serde_json::Value>) -> FieldResult<serde_json::Value> {
        match value {
            None if self.required => Err(FieldError::required(None)),
            None => Ok(serde_json::Value::Null),
            Some(v) => {
                let s = v
                    .as_str()
                    .ok_or_else(|| FieldError::Invalid("Expected string".to_string()))?;

                let s = s.trim();

                if s.is_empty() {
                    if self.required {
                        return Err(FieldError::required(None));
                    }
                    return Ok(serde_json::Value::Null);
                }

                let date = self.parse_date(s).map_err(|e| FieldError::Validation(e))?;

                // Return in ISO 8601 format
                Ok(serde_json::json!(date.format("%Y-%m-%d").to_string()))
            }
        }
    }
}
