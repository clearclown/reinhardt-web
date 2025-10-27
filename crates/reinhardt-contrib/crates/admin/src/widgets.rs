//! Custom field widgets for admin forms
//!
//! This module provides customizable widgets for rendering form fields
//! with enhanced functionality and styling.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Widget configuration for form fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Widget {
    /// Widget type
    pub widget_type: WidgetType,
    /// HTML attributes
    pub attrs: HashMap<String, String>,
    /// Widget-specific options
    pub options: HashMap<String, serde_json::Value>,
}

impl Widget {
    /// Create a new widget
    pub fn new(widget_type: WidgetType) -> Self {
        Self {
            widget_type,
            attrs: HashMap::new(),
            options: HashMap::new(),
        }
    }

    /// Add HTML attribute
    pub fn with_attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attrs.insert(key.into(), value.into());
        self
    }

    /// Add widget option
    pub fn with_option(mut self, key: impl Into<String>, value: serde_json::Value) -> Self {
        self.options.insert(key.into(), value);
        self
    }

    /// Render widget to HTML
    pub fn render(&self, name: &str, value: Option<&serde_json::Value>) -> String {
        match &self.widget_type {
            WidgetType::TextInput => self.render_text_input(name, value),
            WidgetType::TextArea { rows, cols } => self.render_textarea(name, value, *rows, *cols),
            WidgetType::Select { choices } => self.render_select(name, value, choices),
            WidgetType::CheckboxInput => self.render_checkbox(name, value),
            WidgetType::RadioSelect { choices } => self.render_radio(name, value, choices),
            WidgetType::DateInput => self.render_date_input(name, value),
            WidgetType::TimeInput => self.render_time_input(name, value),
            WidgetType::DateTimeInput => self.render_datetime_input(name, value),
            WidgetType::FileInput => self.render_file_input(name),
            WidgetType::HiddenInput => self.render_hidden_input(name, value),
            WidgetType::EmailInput => self.render_email_input(name, value),
            WidgetType::NumberInput => self.render_number_input(name, value),
            WidgetType::ColorPicker => self.render_color_picker(name, value),
            WidgetType::RichTextEditor => self.render_rich_text_editor(name, value),
            WidgetType::MultiSelect { choices } => self.render_multi_select(name, value, choices),
        }
    }

    fn render_attrs(&self) -> String {
        self.attrs
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn render_text_input(&self, name: &str, value: Option<&serde_json::Value>) -> String {
        let value_str = value
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .replace('"', "&quot;");
        let attrs = self.render_attrs();
        format!(
            "<input type=\"text\" name=\"{}\" value=\"{}\" {} />",
            name, value_str, attrs
        )
    }

    fn render_textarea(
        &self,
        name: &str,
        value: Option<&serde_json::Value>,
        rows: usize,
        cols: usize,
    ) -> String {
        let value_str = value.and_then(|v| v.as_str()).unwrap_or("");
        let attrs = self.render_attrs();
        format!(
            "<textarea name=\"{}\" rows=\"{}\" cols=\"{}\" {}>{}</textarea>",
            name, rows, cols, attrs, value_str
        )
    }

    fn render_select(
        &self,
        name: &str,
        value: Option<&serde_json::Value>,
        choices: &[(String, String)],
    ) -> String {
        let value_str = value.and_then(|v| v.as_str()).unwrap_or("");
        let attrs = self.render_attrs();
        let options = choices
            .iter()
            .map(|(val, label)| {
                let selected = if val == value_str { " selected" } else { "" };
                format!("<option value=\"{}\"{}>{}</option>", val, selected, label)
            })
            .collect::<Vec<_>>()
            .join("");
        format!("<select name=\"{}\" {}>{}</select>", name, attrs, options)
    }

    fn render_checkbox(&self, name: &str, value: Option<&serde_json::Value>) -> String {
        let checked = value.and_then(|v| v.as_bool()).unwrap_or(false);
        let checked_attr = if checked { " checked" } else { "" };
        let attrs = self.render_attrs();
        format!(
            "<input type=\"checkbox\" name=\"{}\" value=\"true\" {}{} />",
            name, attrs, checked_attr
        )
    }

    fn render_radio(
        &self,
        name: &str,
        value: Option<&serde_json::Value>,
        choices: &[(String, String)],
    ) -> String {
        let value_str = value.and_then(|v| v.as_str()).unwrap_or("");
        let attrs = self.render_attrs();
        choices
            .iter()
            .map(|(val, label)| {
                let checked = if val == value_str { " checked" } else { "" };
                format!(
                    "<label><input type=\"radio\" name=\"{}\" value=\"{}\" {}{} /> {}</label>",
                    name, val, attrs, checked, label
                )
            })
            .collect::<Vec<_>>()
            .join("<br>")
    }

    fn render_date_input(&self, name: &str, value: Option<&serde_json::Value>) -> String {
        let value_str = value
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .replace('"', "&quot;");
        let attrs = self.render_attrs();
        format!(
            "<input type=\"date\" name=\"{}\" value=\"{}\" {} />",
            name, value_str, attrs
        )
    }

    fn render_time_input(&self, name: &str, value: Option<&serde_json::Value>) -> String {
        let value_str = value
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .replace('"', "&quot;");
        let attrs = self.render_attrs();
        format!(
            "<input type=\"time\" name=\"{}\" value=\"{}\" {} />",
            name, value_str, attrs
        )
    }

    fn render_datetime_input(&self, name: &str, value: Option<&serde_json::Value>) -> String {
        let value_str = value
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .replace('"', "&quot;");
        let attrs = self.render_attrs();
        format!(
            "<input type=\"datetime-local\" name=\"{}\" value=\"{}\" {} />",
            name, value_str, attrs
        )
    }

    fn render_file_input(&self, name: &str) -> String {
        let attrs = self.render_attrs();
        format!("<input type=\"file\" name=\"{}\" {} />", name, attrs)
    }

    fn render_hidden_input(&self, name: &str, value: Option<&serde_json::Value>) -> String {
        let value_str = value
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .replace('"', "&quot;");
        format!(
            "<input type=\"hidden\" name=\"{}\" value=\"{}\" />",
            name, value_str
        )
    }

    fn render_email_input(&self, name: &str, value: Option<&serde_json::Value>) -> String {
        let value_str = value
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .replace('"', "&quot;");
        let attrs = self.render_attrs();
        format!(
            "<input type=\"email\" name=\"{}\" value=\"{}\" {} />",
            name, value_str, attrs
        )
    }

    fn render_number_input(&self, name: &str, value: Option<&serde_json::Value>) -> String {
        let value_str = value.map(|v| v.to_string()).unwrap_or_default();
        let attrs = self.render_attrs();
        format!(
            "<input type=\"number\" name=\"{}\" value=\"{}\" {} />",
            name, value_str, attrs
        )
    }

    fn render_color_picker(&self, name: &str, value: Option<&serde_json::Value>) -> String {
        let value_str = value.and_then(|v| v.as_str()).unwrap_or("#000000");
        let attrs = self.render_attrs();
        format!(
            "<input type=\"color\" name=\"{}\" value=\"{}\" {} />",
            name, value_str, attrs
        )
    }

    fn render_rich_text_editor(&self, name: &str, value: Option<&serde_json::Value>) -> String {
        let value_str = value.and_then(|v| v.as_str()).unwrap_or("");
        let attrs = self.render_attrs();
        format!(
            "<textarea name=\"{}\" class=\"rich-text-editor\" {}>{}</textarea>",
            name, attrs, value_str
        )
    }

    fn render_multi_select(
        &self,
        name: &str,
        value: Option<&serde_json::Value>,
        choices: &[(String, String)],
    ) -> String {
        let selected_values: Vec<String> = value
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        let attrs = self.render_attrs();
        let options = choices
            .iter()
            .map(|(val, label)| {
                let selected = if selected_values.contains(val) {
                    " selected"
                } else {
                    ""
                };
                format!("<option value=\"{}\"{}>{}</option>", val, selected, label)
            })
            .collect::<Vec<_>>()
            .join("");

        format!(
            "<select name=\"{}\" multiple {}>{}</select>",
            name, attrs, options
        )
    }
}

/// Types of widgets available
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WidgetType {
    /// Standard text input
    TextInput,
    /// Multi-line text area
    TextArea { rows: usize, cols: usize },
    /// Dropdown select
    Select { choices: Vec<(String, String)> },
    /// Checkbox
    CheckboxInput,
    /// Radio buttons
    RadioSelect { choices: Vec<(String, String)> },
    /// Date picker
    DateInput,
    /// Time picker
    TimeInput,
    /// DateTime picker
    DateTimeInput,
    /// File upload
    FileInput,
    /// Hidden input
    HiddenInput,
    /// Email input with validation
    EmailInput,
    /// Number input
    NumberInput,
    /// Color picker
    ColorPicker,
    /// Rich text editor (WYSIWYG)
    RichTextEditor,
    /// Multiple select dropdown
    MultiSelect { choices: Vec<(String, String)> },
}

/// Widget factory for creating common widgets
pub struct WidgetFactory;

impl WidgetFactory {
    /// Create a text input widget
    pub fn text_input() -> Widget {
        Widget::new(WidgetType::TextInput)
            .with_attr("class", "form-control")
    }

    /// Create a textarea widget
    pub fn textarea(rows: usize, cols: usize) -> Widget {
        Widget::new(WidgetType::TextArea { rows, cols })
            .with_attr("class", "form-control")
    }

    /// Create a select widget
    pub fn select(choices: Vec<(String, String)>) -> Widget {
        Widget::new(WidgetType::Select { choices })
            .with_attr("class", "form-select")
    }

    /// Create a checkbox widget
    pub fn checkbox() -> Widget {
        Widget::new(WidgetType::CheckboxInput)
            .with_attr("class", "form-check-input")
    }

    /// Create a radio select widget
    pub fn radio_select(choices: Vec<(String, String)>) -> Widget {
        Widget::new(WidgetType::RadioSelect { choices })
            .with_attr("class", "form-check-input")
    }

    /// Create a date input widget
    pub fn date_input() -> Widget {
        Widget::new(WidgetType::DateInput)
            .with_attr("class", "form-control")
    }

    /// Create an email input widget
    pub fn email_input() -> Widget {
        Widget::new(WidgetType::EmailInput)
            .with_attr("class", "form-control")
    }

    /// Create a number input widget
    pub fn number_input() -> Widget {
        Widget::new(WidgetType::NumberInput)
            .with_attr("class", "form-control")
    }

    /// Create a color picker widget
    pub fn color_picker() -> Widget {
        Widget::new(WidgetType::ColorPicker)
            .with_attr("class", "form-control form-control-color")
    }

    /// Create a rich text editor widget
    pub fn rich_text_editor() -> Widget {
        Widget::new(WidgetType::RichTextEditor)
            .with_attr("class", "form-control rich-text-editor")
    }

    /// Create a multi-select widget
    pub fn multi_select(choices: Vec<(String, String)>) -> Widget {
        Widget::new(WidgetType::MultiSelect { choices })
            .with_attr("class", "form-select")
            .with_attr("size", "5")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_new() {
        let widget = Widget::new(WidgetType::TextInput);
        assert!(matches!(widget.widget_type, WidgetType::TextInput));
        assert!(widget.attrs.is_empty());
    }

    #[test]
    fn test_widget_with_attr() {
        let widget = Widget::new(WidgetType::TextInput)
            .with_attr("class", "form-control")
            .with_attr("placeholder", "Enter text");

        assert_eq!(widget.attrs.len(), 2);
        assert_eq!(widget.attrs.get("class"), Some(&"form-control".to_string()));
    }

    #[test]
    fn test_render_text_input() {
        let widget = Widget::new(WidgetType::TextInput)
            .with_attr("class", "form-control");

        let html = widget.render("username", Some(&serde_json::Value::String("alice".to_string())));
        assert!(html.contains("type=\"text\""));
        assert!(html.contains("name=\"username\""));
        assert!(html.contains("value=\"alice\""));
        assert!(html.contains("class=\"form-control\""));
    }

    #[test]
    fn test_render_textarea() {
        let widget = Widget::new(WidgetType::TextArea { rows: 5, cols: 40 });
        let html = widget.render("bio", Some(&serde_json::Value::String("Hello".to_string())));

        assert!(html.contains("<textarea"));
        assert!(html.contains("rows=\"5\""));
        assert!(html.contains("cols=\"40\""));
        assert!(html.contains(">Hello</textarea>"));
    }

    #[test]
    fn test_render_select() {
        let choices = vec![
            ("active".to_string(), "Active".to_string()),
            ("inactive".to_string(), "Inactive".to_string()),
        ];
        let widget = Widget::new(WidgetType::Select { choices });
        let html = widget.render("status", Some(&serde_json::Value::String("active".to_string())));

        assert!(html.contains("<select"));
        assert!(html.contains("value=\"active\" selected"));
        assert!(html.contains("value=\"inactive\""));
    }

    #[test]
    fn test_render_checkbox() {
        let widget = Widget::new(WidgetType::CheckboxInput);
        let html = widget.render("is_active", Some(&serde_json::Value::Bool(true)));

        assert!(html.contains("type=\"checkbox\""));
        assert!(html.contains("checked"));
    }

    #[test]
    fn test_render_date_input() {
        let widget = Widget::new(WidgetType::DateInput);
        let html = widget.render("birth_date", Some(&serde_json::Value::String("2025-01-01".to_string())));

        assert!(html.contains("type=\"date\""));
        assert!(html.contains("value=\"2025-01-01\""));
    }

    #[test]
    fn test_render_email_input() {
        let widget = Widget::new(WidgetType::EmailInput);
        let html = widget.render("email", Some(&serde_json::Value::String("test@example.com".to_string())));

        assert!(html.contains("type=\"email\""));
        assert!(html.contains("value=\"test@example.com\""));
    }

    #[test]
    fn test_render_hidden_input() {
        let widget = Widget::new(WidgetType::HiddenInput);
        let html = widget.render("id", Some(&serde_json::Value::String("123".to_string())));

        assert!(html.contains("type=\"hidden\""));
        assert!(html.contains("value=\"123\""));
    }

    #[test]
    fn test_widget_factory_text_input() {
        let widget = WidgetFactory::text_input();
        assert!(matches!(widget.widget_type, WidgetType::TextInput));
        assert_eq!(widget.attrs.get("class"), Some(&"form-control".to_string()));
    }

    #[test]
    fn test_widget_factory_select() {
        let choices = vec![
            ("1".to_string(), "Option 1".to_string()),
            ("2".to_string(), "Option 2".to_string()),
        ];
        let widget = WidgetFactory::select(choices);

        if let WidgetType::Select { choices } = &widget.widget_type {
            assert_eq!(choices.len(), 2);
        } else {
            panic!("Expected Select widget type");
        }
    }

    #[test]
    fn test_render_multi_select() {
        let choices = vec![
            ("tag1".to_string(), "Tag 1".to_string()),
            ("tag2".to_string(), "Tag 2".to_string()),
            ("tag3".to_string(), "Tag 3".to_string()),
        ];
        let widget = Widget::new(WidgetType::MultiSelect { choices });

        let selected = serde_json::json!(["tag1", "tag3"]);
        let html = widget.render("tags", Some(&selected));

        assert!(html.contains("multiple"));
        assert!(html.contains("value=\"tag1\" selected"));
        assert!(html.contains("value=\"tag3\" selected"));
        assert!(!html.contains("value=\"tag2\" selected"));
    }

    #[test]
    fn test_render_color_picker() {
        let widget = Widget::new(WidgetType::ColorPicker);
        let html = widget.render("color", Some(&serde_json::Value::String("#ff0000".to_string())));

        assert!(html.contains("type=\"color\""));
        assert!(html.contains("value=\"#ff0000\""));
    }
}
