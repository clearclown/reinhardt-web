use crate::field::{FieldError, FieldResult, FormField, Widget};

/// ChoiceField for selecting from predefined choices
pub struct ChoiceField {
    pub name: String,
    pub label: Option<String>,
    pub required: bool,
    pub help_text: Option<String>,
    pub widget: Widget,
    pub initial: Option<serde_json::Value>,
    pub choices: Vec<(String, String)>, // (value, label)
}

impl ChoiceField {
    /// Create a new ChoiceField
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::fields::ChoiceField;
    ///
    /// let choices = vec![("1".to_string(), "Option 1".to_string())];
    /// let field = ChoiceField::new("choice".to_string(), choices);
    /// assert_eq!(field.name, "choice");
    /// ```
    pub fn new(name: String, choices: Vec<(String, String)>) -> Self {
        Self {
            name,
            label: None,
            required: true,
            help_text: None,
            widget: Widget::Select {
                choices: choices.clone(),
            },
            initial: None,
            choices,
        }
    }
}

impl FormField for ChoiceField {
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
            None => Ok(serde_json::Value::String(String::new())),
            Some(v) => {
                let s = v
                    .as_str()
                    .ok_or_else(|| FieldError::Invalid("Expected string".to_string()))?;

                let s = s.trim();

                if s.is_empty() {
                    if self.required {
                        return Err(FieldError::required(None));
                    }
                    return Ok(serde_json::Value::String(String::new()));
                }

                // Check if value is in choices
                let valid = self.choices.iter().any(|(value, _)| value == s);
                if !valid {
                    return Err(FieldError::Validation(format!(
                        "Select a valid choice. '{}' is not one of the available choices",
                        s
                    )));
                }

                Ok(serde_json::Value::String(s.to_string()))
            }
        }
    }
}

/// MultipleChoiceField for selecting multiple choices
pub struct MultipleChoiceField {
    pub name: String,
    pub label: Option<String>,
    pub required: bool,
    pub help_text: Option<String>,
    pub widget: Widget,
    pub initial: Option<serde_json::Value>,
    pub choices: Vec<(String, String)>,
}

impl MultipleChoiceField {
    pub fn new(name: String, choices: Vec<(String, String)>) -> Self {
        Self {
            name,
            label: None,
            required: true,
            help_text: None,
            widget: Widget::Select {
                choices: choices.clone(),
            },
            initial: None,
            choices,
        }
    }
}

impl FormField for MultipleChoiceField {
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
            None => Ok(serde_json::json!([])),
            Some(v) => {
                let values: Vec<String> = if let Some(arr) = v.as_array() {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                } else if let Some(s) = v.as_str() {
                    vec![s.to_string()]
                } else {
                    return Err(FieldError::Invalid("Expected array or string".to_string()));
                };

                if values.is_empty() && self.required {
                    return Err(FieldError::required(None));
                }

                // Validate all values are in choices
                for val in &values {
                    let valid = self.choices.iter().any(|(choice, _)| choice == val);
                    if !valid {
                        return Err(FieldError::Validation(format!(
                            "Select a valid choice. '{}' is not one of the available choices",
                            val
                        )));
                    }
                }

                Ok(serde_json::json!(values))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choicefield_valid() {
        let choices = vec![
            ("1".to_string(), "One".to_string()),
            ("2".to_string(), "Two".to_string()),
        ];
        let field = ChoiceField::new("number".to_string(), choices);

        assert_eq!(
            field.clean(Some(&serde_json::json!("1"))).unwrap(),
            serde_json::json!("1")
        );
    }

    #[test]
    fn test_choicefield_invalid() {
        let choices = vec![("1".to_string(), "One".to_string())];
        let field = ChoiceField::new("number".to_string(), choices);

        assert!(matches!(
            field.clean(Some(&serde_json::json!("3"))),
            Err(FieldError::Validation(_))
        ));
    }

    #[test]
    fn test_multiplechoicefield() {
        let choices = vec![
            ("a".to_string(), "A".to_string()),
            ("b".to_string(), "B".to_string()),
        ];
        let field = MultipleChoiceField::new("letters".to_string(), choices);

        assert_eq!(
            field.clean(Some(&serde_json::json!(["a", "b"]))).unwrap(),
            serde_json::json!(["a", "b"])
        );

        assert!(matches!(
            field.clean(Some(&serde_json::json!(["a", "c"]))),
            Err(FieldError::Validation(_))
        ));
    }
}
