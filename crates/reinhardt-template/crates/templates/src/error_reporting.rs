//! Enhanced error reporting for templates
//!
//! Provides detailed error messages with context, line numbers,
//! and helpful suggestions for fixing template errors.

use std::fmt;

/// Template error context
#[derive(Debug, Clone)]
pub struct TemplateErrorContext {
    /// Template name or file path
    pub template_name: String,
    /// Line number where the error occurred
    pub line: Option<usize>,
    /// Column number where the error occurred
    pub column: Option<usize>,
    /// The line of code that caused the error
    pub source_line: Option<String>,
    /// Additional context lines before the error
    pub context_before: Vec<String>,
    /// Additional context lines after the error
    pub context_after: Vec<String>,
    /// Error message
    pub message: String,
    /// Suggestion for fixing the error
    pub suggestion: Option<String>,
}

impl TemplateErrorContext {
    /// Create a new error context
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_templates::TemplateErrorContext;
    ///
    /// let context = TemplateErrorContext::new(
    ///     "user_list.html",
    ///     10,
    ///     Some(5),
    ///     "Undefined variable 'username'",
    /// );
    /// assert_eq!(context.line, Some(10));
    /// ```
    pub fn new(
        template_name: impl Into<String>,
        line: usize,
        column: Option<usize>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            template_name: template_name.into(),
            line: Some(line),
            column,
            source_line: None,
            context_before: Vec::new(),
            context_after: Vec::new(),
            message: message.into(),
            suggestion: None,
        }
    }

    /// Add source line
    pub fn with_source_line(mut self, line: impl Into<String>) -> Self {
        self.source_line = Some(line.into());
        self
    }

    /// Add context lines before the error
    pub fn with_context_before(mut self, lines: Vec<String>) -> Self {
        self.context_before = lines;
        self
    }

    /// Add context lines after the error
    pub fn with_context_after(mut self, lines: Vec<String>) -> Self {
        self.context_after = lines;
        self
    }

    /// Add a suggestion for fixing the error
    pub fn with_suggestion(mut self, suggestion: impl Into<String>) -> Self {
        self.suggestion = Some(suggestion.into());
        self
    }

    /// Format the error for display
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_templates::TemplateErrorContext;
    ///
    /// let context = TemplateErrorContext::new(
    ///     "template.html",
    ///     10,
    ///     Some(5),
    ///     "Undefined variable",
    /// )
    /// .with_source_line("{{ invalid_var }}")
    /// .with_suggestion("Did you mean 'valid_var'?");
    ///
    /// let formatted = context.format();
    /// assert!(formatted.contains("template.html"));
    /// assert!(formatted.contains("line 10"));
    /// ```
    pub fn format(&self) -> String {
        let mut output = String::new();

        // Template name and location
        output.push_str(&format!("Error in template: {}\n", self.template_name));
        if let Some(line) = self.line {
            output.push_str(&format!("  at line {}", line));
            if let Some(col) = self.column {
                output.push_str(&format!(", column {}", col));
            }
            output.push('\n');
        }
        output.push('\n');

        // Context before
        if !self.context_before.is_empty() {
            for (i, line) in self.context_before.iter().enumerate() {
                let line_num = self
                    .line
                    .unwrap_or(0)
                    .saturating_sub(self.context_before.len() - i);
                output.push_str(&format!("  {:4} | {}\n", line_num, line));
            }
        }

        // Error line
        if let Some(ref source_line) = self.source_line {
            let line_num = self.line.unwrap_or(0);
            output.push_str(&format!("  {:4} | {}\n", line_num, source_line));

            // Add marker for column
            if let Some(col) = self.column {
                output.push_str(&format!("       | {}^\n", " ".repeat(col)));
            }
        }

        // Context after
        if !self.context_after.is_empty() {
            for (i, line) in self.context_after.iter().enumerate() {
                let line_num = self.line.unwrap_or(0) + i + 1;
                output.push_str(&format!("  {:4} | {}\n", line_num, line));
            }
        }

        // Error message
        output.push('\n');
        output.push_str(&format!("Error: {}\n", self.message));

        // Suggestion
        if let Some(ref suggestion) = self.suggestion {
            output.push('\n');
            output.push_str(&format!("Suggestion: {}\n", suggestion));
        }

        output
    }
}

impl fmt::Display for TemplateErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

/// Template error type
#[derive(Debug, Clone)]
pub enum TemplateError {
    /// Undefined variable error
    UndefinedVariable {
        template: String,
        variable: String,
        line: usize,
        suggestion: Option<String>,
    },
    /// Syntax error
    SyntaxError {
        template: String,
        message: String,
        line: usize,
        column: Option<usize>,
    },
    /// Filter not found
    FilterNotFound {
        template: String,
        filter: String,
        line: usize,
    },
    /// Include/extends error
    IncludeError {
        template: String,
        included_template: String,
        message: String,
    },
    /// Generic error
    Generic { message: String },
}

impl TemplateError {
    /// Convert to error context
    pub fn to_context(&self) -> TemplateErrorContext {
        match self {
            TemplateError::UndefinedVariable {
                template,
                variable,
                line,
                suggestion,
            } => {
                let mut ctx = TemplateErrorContext::new(
                    template,
                    *line,
                    None,
                    format!("Undefined variable '{}'", variable),
                );
                if let Some(sug) = suggestion {
                    ctx = ctx.with_suggestion(sug);
                }
                ctx
            }
            TemplateError::SyntaxError {
                template,
                message,
                line,
                column,
            } => TemplateErrorContext::new(template, *line, *column, message),
            TemplateError::FilterNotFound {
                template,
                filter,
                line,
            } => TemplateErrorContext::new(
                template,
                *line,
                None,
                format!("Filter '{}' not found", filter),
            )
            .with_suggestion("Check if the filter is registered or imported"),
            TemplateError::IncludeError {
                template,
                included_template,
                message,
            } => TemplateErrorContext::new(
                template,
                0,
                None,
                format!("Failed to include '{}': {}", included_template, message),
            ),
            TemplateError::Generic { message } => {
                TemplateErrorContext::new("unknown", 0, None, message)
            }
        }
    }

    /// Format the error with context
    pub fn format(&self) -> String {
        self.to_context().format()
    }
}

impl fmt::Display for TemplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl std::error::Error for TemplateError {}

/// Helper to suggest similar variable names
///
/// # Examples
///
/// ```
/// use reinhardt_templates::suggest_similar;
///
/// let available = vec!["username", "user_id", "email"];
/// let suggestion = suggest_similar("usrname", &available);
/// assert_eq!(suggestion, Some("username".to_string()));
/// ```
pub fn suggest_similar(input: &str, available: &[&str]) -> Option<String> {
    let input_lower = input.to_lowercase();
    let mut best_match: Option<(&str, usize)> = None;

    for &candidate in available {
        let distance = levenshtein_distance(&input_lower, &candidate.to_lowercase());
        if distance <= 3 {
            // Allow up to 3 character differences
            if let Some((_, best_dist)) = best_match {
                if distance < best_dist {
                    best_match = Some((candidate, distance));
                }
            } else {
                best_match = Some((candidate, distance));
            }
        }
    }

    best_match.map(|(name, _)| name.to_string())
}

/// Calculate Levenshtein distance between two strings
fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let cost = if c1 == c2 { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost,
            );
        }
    }

    matrix[len1][len2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_error_context_new() {
        let context = TemplateErrorContext::new("template.html", 10, Some(5), "Test error");
        assert_eq!(context.template_name, "template.html");
        assert_eq!(context.line, Some(10));
        assert_eq!(context.column, Some(5));
        assert_eq!(context.message, "Test error");
    }

    #[test]
    fn test_template_error_context_with_source() {
        let context = TemplateErrorContext::new("template.html", 10, None, "Error")
            .with_source_line("{{ variable }}");
        assert_eq!(context.source_line, Some("{{ variable }}".to_string()));
    }

    #[test]
    fn test_template_error_context_with_suggestion() {
        let context = TemplateErrorContext::new("template.html", 10, None, "Error")
            .with_suggestion("Try this instead");
        assert_eq!(context.suggestion, Some("Try this instead".to_string()));
    }

    #[test]
    fn test_template_error_context_format() {
        let context = TemplateErrorContext::new("template.html", 10, Some(5), "Test error")
            .with_source_line("{{ invalid }}")
            .with_suggestion("Check variable name");

        let formatted = context.format();
        assert!(formatted.contains("template.html"));
        assert!(formatted.contains("line 10"));
        assert!(formatted.contains("Test error"));
        assert!(formatted.contains("Check variable name"));
    }

    #[test]
    fn test_suggest_similar() {
        let available = vec!["username", "user_id", "email"];
        assert_eq!(
            suggest_similar("usrname", &available),
            Some("username".to_string())
        );
        assert_eq!(
            suggest_similar("userid", &available),
            Some("user_id".to_string())
        );
        assert_eq!(suggest_similar("totally_different", &available), None);
    }

    #[test]
    fn test_levenshtein_distance() {
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
        assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
        assert_eq!(levenshtein_distance("abc", "abc"), 0);
    }

    #[test]
    fn test_template_error_undefined_variable() {
        let error = TemplateError::UndefinedVariable {
            template: "test.html".to_string(),
            variable: "invalid_var".to_string(),
            line: 10,
            suggestion: Some("Did you mean 'valid_var'?".to_string()),
        };

        let formatted = error.format();
        assert!(formatted.contains("Undefined variable 'invalid_var'"));
        assert!(formatted.contains("Did you mean 'valid_var'?"));
    }

    #[test]
    fn test_template_error_syntax_error() {
        let error = TemplateError::SyntaxError {
            template: "test.html".to_string(),
            message: "Unexpected token".to_string(),
            line: 5,
            column: Some(10),
        };

        let formatted = error.format();
        assert!(formatted.contains("Unexpected token"));
        assert!(formatted.contains("line 5"));
    }

    #[test]
    fn test_template_error_filter_not_found() {
        let error = TemplateError::FilterNotFound {
            template: "test.html".to_string(),
            filter: "custom_filter".to_string(),
            line: 8,
        };

        let formatted = error.format();
        assert!(formatted.contains("Filter 'custom_filter' not found"));
    }

    #[test]
    fn test_template_error_context_with_context_lines() {
        let context = TemplateErrorContext::new("template.html", 10, None, "Error")
            .with_context_before(vec!["line 8".to_string(), "line 9".to_string()])
            .with_source_line("line 10 with error")
            .with_context_after(vec!["line 11".to_string()]);

        let formatted = context.format();
        assert!(formatted.contains("line 8"));
        assert!(formatted.contains("line 9"));
        assert!(formatted.contains("line 10 with error"));
        assert!(formatted.contains("line 11"));
    }
}
