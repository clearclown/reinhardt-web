use crate::field::{FieldError, FieldResult, FormField, Widget};
use chrono::NaiveTime;

/// TimeField for time input
pub struct TimeField {
    pub name: String,
    pub label: Option<String>,
    pub required: bool,
    pub help_text: Option<String>,
    pub widget: Widget,
    pub initial: Option<serde_json::Value>,
    pub input_formats: Vec<String>,
}

impl TimeField {
    /// Create a new TimeField
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::TimeField;
    ///
    /// let field = TimeField::new("start_time".to_string());
    /// assert_eq!(field.name, "start_time");
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
                "%H:%M:%S".to_string(),
                "%H:%M".to_string(),
                "%I:%M:%S %p".to_string(),
                "%I:%M %p".to_string(),
            ],
        }
    }

    fn parse_time(&self, s: &str) -> Result<NaiveTime, String> {
        for fmt in &self.input_formats {
            if let Ok(time) = NaiveTime::parse_from_str(s, fmt) {
                return Ok(time);
            }
        }
        Err("Enter a valid time".to_string())
    }
}

impl FormField for TimeField {
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

                let time = self.parse_time(s).map_err(|e| FieldError::Validation(e))?;

                Ok(serde_json::Value::String(
                    time.format("%H:%M:%S").to_string(),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timefield_valid() {
        let field = TimeField::new("start_time".to_string());

        assert_eq!(
            field.clean(Some(&serde_json::json!("14:30:00"))).unwrap(),
            serde_json::json!("14:30:00")
        );
        assert_eq!(
            field.clean(Some(&serde_json::json!("14:30"))).unwrap(),
            serde_json::json!("14:30:00")
        );
        assert_eq!(
            field
                .clean(Some(&serde_json::json!("02:30:00 PM")))
                .unwrap(),
            serde_json::json!("14:30:00")
        );
    }

    #[test]
    fn test_timefield_invalid() {
        let field = TimeField::new("start_time".to_string());

        assert!(matches!(
            field.clean(Some(&serde_json::json!("not a time"))),
            Err(FieldError::Validation(_))
        ));
        assert!(matches!(
            field.clean(Some(&serde_json::json!("25:00:00"))),
            Err(FieldError::Validation(_))
        ));
    }

    #[test]
    fn test_timefield_optional() {
        let mut field = TimeField::new("start_time".to_string());
        field.required = false;

        assert_eq!(field.clean(None).unwrap(), serde_json::Value::Null);
        assert_eq!(
            field.clean(Some(&serde_json::json!(""))).unwrap(),
            serde_json::Value::Null
        );
    }
}
