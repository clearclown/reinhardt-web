use crate::field::{FieldError, FieldResult, FormField, Widget};
use chrono::NaiveDateTime;

/// DateTimeField for date and time input
pub struct DateTimeField {
    pub name: String,
    pub label: Option<String>,
    pub required: bool,
    pub help_text: Option<String>,
    pub widget: Widget,
    pub initial: Option<serde_json::Value>,
    pub input_formats: Vec<String>,
}

impl DateTimeField {
    /// Create a new DateTimeField
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::DateTimeField;
    ///
    /// let field = DateTimeField::new("event_time".to_string());
    /// assert_eq!(field.name, "event_time");
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            label: None,
            required: true,
            help_text: None,
            widget: Widget::TextInput,
            initial: None,
            input_formats: vec![
                "%Y-%m-%d %H:%M:%S".to_string(),
                "%Y-%m-%d %H:%M".to_string(),
                "%Y-%m-%dT%H:%M:%S".to_string(),
                "%Y-%m-%dT%H:%M".to_string(),
                "%m/%d/%Y %H:%M:%S".to_string(),
                "%m/%d/%Y %H:%M".to_string(),
                "%m/%d/%y %H:%M:%S".to_string(),
                "%m/%d/%y %H:%M".to_string(),
            ],
        }
    }

    fn parse_datetime(&self, s: &str) -> Result<NaiveDateTime, String> {
        for fmt in &self.input_formats {
            if let Ok(dt) = NaiveDateTime::parse_from_str(s, fmt) {
                return Ok(dt);
            }
        }
        Err("Enter a valid date/time".to_string())
    }
}

impl FormField for DateTimeField {
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

                let dt = self
                    .parse_datetime(s)
                    .map_err(|e| FieldError::Validation(e))?;

                Ok(serde_json::Value::String(
                    dt.format("%Y-%m-%d %H:%M:%S").to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetimefield_valid() {
        let field = DateTimeField::new("created_at".to_string());

        assert_eq!(
            field
                .clean(Some(&serde_json::json!("2025-01-15 14:30:00")))
                .unwrap(),
            serde_json::json!("2025-01-15 14:30:00")
        );
        assert_eq!(
            field
                .clean(Some(&serde_json::json!("2025-01-15T14:30:00")))
                .unwrap(),
            serde_json::json!("2025-01-15 14:30:00")
        );
        assert_eq!(
            field
                .clean(Some(&serde_json::json!("01/15/2025 14:30:00")))
                .unwrap(),
            serde_json::json!("2025-01-15 14:30:00")
        );
    }

    #[test]
    fn test_datetimefield_invalid() {
        let field = DateTimeField::new("created_at".to_string());

        assert!(matches!(
            field.clean(Some(&serde_json::json!("not a datetime"))),
            Err(FieldError::Validation(_))
        ));
        assert!(matches!(
            field.clean(Some(&serde_json::json!("2025-13-01 14:30:00"))),
            Err(FieldError::Validation(_))
        ));
    }

    #[test]
    fn test_datetimefield_optional() {
        let mut field = DateTimeField::new("created_at".to_string());
        field.required = false;

        assert_eq!(field.clean(None).unwrap(), serde_json::Value::Null);
        assert_eq!(
            field.clean(Some(&serde_json::json!(""))).unwrap(),
            serde_json::Value::Null
        );
    }
}
