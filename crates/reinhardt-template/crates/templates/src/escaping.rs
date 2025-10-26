//! HTML escaping for template security
//!
//! Provides automatic HTML escaping to prevent XSS (Cross-Site Scripting) attacks.
//! All user-provided content should be escaped before rendering in HTML templates.
//!
//! Escaped characters:
//! - `<` → `&lt;`
//! - `>` → `&gt;`
//! - `&` → `&amp;`
//! - `"` → `&quot;`
//! - `'` → `&#x27;`

use askama::Result as AskamaResult;

/// Escape HTML special characters
///
/// # Examples
///
/// ```
/// use reinhardt_templates::escape_html;
///
/// assert_eq!(escape_html("<script>alert('XSS')</script>"),
///            "&lt;script&gt;alert(&#x27;XSS&#x27;)&lt;/script&gt;");
/// assert_eq!(escape_html("Hello & goodbye"), "Hello &amp; goodbye");
/// assert_eq!(escape_html(r#"<a href="test">link</a>"#),
///            "&lt;a href=&quot;test&quot;&gt;link&lt;/a&gt;");
/// ```
pub fn escape_html(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

/// Askama filter for HTML escaping
///
/// This filter can be used in Askama templates to escape HTML content.
///
/// # Examples
///
/// ```askama
/// {{ user_input|escape }}
/// ```
pub fn escape(s: &str) -> AskamaResult<String> {
    Ok(escape_html(s))
}

/// Unescape HTML entities
///
/// Converts HTML entities back to their original characters.
/// This is the inverse of `escape_html`.
///
/// # Examples
///
/// ```
/// use reinhardt_templates::unescape_html;
///
/// assert_eq!(unescape_html("&lt;div&gt;"), "<div>");
/// assert_eq!(unescape_html("&quot;quoted&quot;"), r#""quoted""#);
/// assert_eq!(unescape_html("&#x27;single&#x27;"), "''");
/// ```
pub fn unescape_html(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&#x27;", "'")
        .replace("&#39;", "'")
}

/// Askama filter for HTML unescaping
pub fn unescape(s: &str) -> AskamaResult<String> {
    Ok(unescape_html(s))
}

/// Mark a string as safe (already escaped)
///
/// This is a marker type that indicates a string has already been escaped
/// and should not be escaped again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeString(String);

impl SafeString {
    /// Create a new safe string
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_templates::SafeString;
    ///
    /// let safe = SafeString::new("<b>Bold</b>");
    /// assert_eq!(safe.as_str(), "<b>Bold</b>");
    /// ```
    pub fn new(s: impl Into<String>) -> Self {
        SafeString(s.into())
    }

    /// Get the inner string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to String
    pub fn into_string(self) -> String {
        self.0
    }
}

impl From<String> for SafeString {
    fn from(s: String) -> Self {
        SafeString(s)
    }
}

impl From<&str> for SafeString {
    fn from(s: &str) -> Self {
        SafeString(s.to_string())
    }
}

impl AsRef<str> for SafeString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SafeString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Escape HTML attributes
///
/// Similar to `escape_html` but specifically for use in HTML attributes.
/// More strict about escaping quotes.
///
/// # Examples
///
/// ```
/// use reinhardt_templates::escape_html_attr;
///
/// assert_eq!(escape_html_attr(r#"value with "quotes""#),
///            "value with &quot;quotes&quot;");
/// ```
pub fn escape_html_attr(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '&' => "&amp;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

/// Escape for use in JavaScript strings
///
/// # Examples
///
/// ```
/// use reinhardt_templates::escape_js;
///
/// assert_eq!(escape_js(r#"alert("test")"#), r#"alert(\"test\")"#);
/// assert_eq!(escape_js("line1\nline2"), r"line1\nline2");
/// ```
pub fn escape_js(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '"' => r#"\""#.to_string(),
            '\'' => r"\'".to_string(),
            '\\' => r"\\".to_string(),
            '\n' => r"\n".to_string(),
            '\r' => r"\r".to_string(),
            '\t' => r"\t".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

/// Escape for use in CSS
///
/// # Examples
///
/// ```
/// use reinhardt_templates::escape_css;
///
/// assert_eq!(escape_css("url('test')"), r"url(\'test\')");
/// ```
pub fn escape_css(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '"' => r#"\""#.to_string(),
            '\'' => r"\'".to_string(),
            '\\' => r"\\".to_string(),
            '\n' => r"\A ".to_string(),
            _ => c.to_string(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html() {
        assert_eq!(
            escape_html("<script>alert('XSS')</script>"),
            "&lt;script&gt;alert(&#x27;XSS&#x27;)&lt;/script&gt;"
        );
        assert_eq!(escape_html("Hello & goodbye"), "Hello &amp; goodbye");
        assert_eq!(
            escape_html(r#"<a href="test">link</a>"#),
            "&lt;a href=&quot;test&quot;&gt;link&lt;/a&gt;"
        );
        assert_eq!(escape_html("normal text"), "normal text");
    }

    #[test]
    fn test_escape_filter() {
        assert_eq!(
            escape("<div>").unwrap(),
            "&lt;div&gt;"
        );
    }

    #[test]
    fn test_unescape_html() {
        assert_eq!(unescape_html("&lt;div&gt;"), "<div>");
        assert_eq!(unescape_html("&quot;quoted&quot;"), r#""quoted""#);
        assert_eq!(unescape_html("&#x27;single&#x27;"), "'single'");
        assert_eq!(unescape_html("&#39;single&#39;"), "'single'");
        assert_eq!(unescape_html("Hello &amp; goodbye"), "Hello & goodbye");
    }

    #[test]
    fn test_unescape_filter() {
        assert_eq!(
            unescape("&lt;div&gt;").unwrap(),
            "<div>"
        );
    }

    #[test]
    fn test_safe_string() {
        let safe = SafeString::new("<b>Bold</b>");
        assert_eq!(safe.as_str(), "<b>Bold</b>");
        assert_eq!(safe.into_string(), "<b>Bold</b>");
    }

    #[test]
    fn test_safe_string_from() {
        let safe1: SafeString = String::from("<b>Bold</b>").into();
        assert_eq!(safe1.as_str(), "<b>Bold</b>");

        let safe2: SafeString = "<i>Italic</i>".into();
        assert_eq!(safe2.as_str(), "<i>Italic</i>");
    }

    #[test]
    fn test_escape_html_attr() {
        assert_eq!(
            escape_html_attr(r#"value with "quotes""#),
            "value with &quot;quotes&quot;"
        );
        assert_eq!(
            escape_html_attr("simple"),
            "simple"
        );
    }

    #[test]
    fn test_escape_js() {
        assert_eq!(escape_js(r#"alert("test")"#), r#"alert(\"test\")"#);
        assert_eq!(escape_js("line1\nline2"), r"line1\nline2");
        assert_eq!(escape_js("tab\there"), "tab\\there");
    }

    #[test]
    fn test_escape_css() {
        assert_eq!(escape_css("url('test')"), r"url(\'test\')");
        assert_eq!(escape_css(r#"url("test")"#), r#"url(\"test\")"#);
    }

    #[test]
    fn test_escape_roundtrip() {
        let original = r#"<div class="test">Hello & "goodbye"</div>"#;
        let escaped = escape_html(original);
        let unescaped = unescape_html(&escaped);
        assert_eq!(unescaped, original);
    }
}
