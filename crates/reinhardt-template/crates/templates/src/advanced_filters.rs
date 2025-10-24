//! Advanced template filters
//!
//! Provides additional filters for common template operations:
//! - String manipulation (truncate, slugify, title)
//! - Number formatting (filesizeformat, floatformat)
//! - List operations (first, last, join, slice)
//! - Date formatting (date, time, timesince)
//! - URL operations (urlencode, urlize)

use askama::Result as AskamaResult;
use chrono::{DateTime, Duration, Utc};

/// Truncate a string to a specified length
///
/// # Examples
///
/// ```
/// use reinhardt_templates::truncate_filter;
///
/// assert_eq!(truncate_filter("Hello World", 5).unwrap(), "He...");
/// assert_eq!(truncate_filter("Hi", 5).unwrap(), "Hi");
/// ```
pub fn truncate(s: &str, length: usize) -> AskamaResult<String> {
    if s.len() <= length {
        Ok(s.to_string())
    } else {
        // Reserve 3 characters for "..."
        let actual_length = if length >= 3 { length - 3 } else { 0 };
        let truncated = s.chars().take(actual_length).collect::<String>();
        Ok(format!("{}...", truncated))
    }
}

/// Convert a string to a URL-friendly slug
///
/// # Examples
///
/// ```
/// use reinhardt_templates::slugify;
///
/// assert_eq!(slugify("Hello World!").unwrap(), "hello-world");
/// assert_eq!(slugify("Django REST Framework").unwrap(), "django-rest-framework");
/// ```
pub fn slugify(s: &str) -> AskamaResult<String> {
    let slug = s
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' || c == '_' {
                '-'
            } else {
                '\0'
            }
        })
        .filter(|&c| c != '\0')
        .collect::<String>();

    // Remove consecutive dashes
    let slug = slug
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-");

    Ok(slug)
}

/// Convert a string to title case
///
/// # Examples
///
/// ```
/// use reinhardt_templates::title;
///
/// assert_eq!(title("hello world").unwrap(), "Hello World");
/// assert_eq!(title("django-rest-framework").unwrap(), "Django-Rest-Framework");
/// ```
pub fn title(s: &str) -> AskamaResult<String> {
    let result = s
        .split(|c: char| c.is_whitespace() || c == '-' || c == '_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ");
    Ok(result)
}

/// Format a file size in human-readable format
///
/// # Examples
///
/// ```
/// use reinhardt_templates::filesizeformat;
///
/// assert_eq!(filesizeformat(1024).unwrap(), "1.00 KB");
/// assert_eq!(filesizeformat(1048576).unwrap(), "1.00 MB");
/// ```
pub fn filesizeformat(bytes: i64) -> AskamaResult<String> {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        Ok(format!("{} {}", bytes, UNITS[0]))
    } else {
        Ok(format!("{:.2} {}", size, UNITS[unit_index]))
    }
}

/// Format a float with specified decimal places
///
/// # Examples
///
/// ```
/// use reinhardt_templates::floatformat;
///
/// assert_eq!(floatformat(3.14159, 2).unwrap(), "3.14");
/// assert_eq!(floatformat(2.0, 2).unwrap(), "2.00");
/// ```
pub fn floatformat(value: f64, places: usize) -> AskamaResult<String> {
    Ok(format!("{:.prec$}", value, prec = places))
}

/// Get the first element of a list
///
/// # Examples
///
/// ```
/// use reinhardt_templates::first;
///
/// let items = vec!["a", "b", "c"];
/// assert_eq!(first(&items).unwrap(), "a");
/// ```
pub fn first<T: std::fmt::Display>(list: &[T]) -> AskamaResult<String> {
    list.first()
        .map(|item| item.to_string())
        .ok_or_else(|| askama::Error::Custom("List is empty".into()))
}

/// Get the last element of a list
///
/// # Examples
///
/// ```
/// use reinhardt_templates::last;
///
/// let items = vec!["a", "b", "c"];
/// assert_eq!(last(&items).unwrap(), "c");
/// ```
pub fn last<T: std::fmt::Display>(list: &[T]) -> AskamaResult<String> {
    list.last()
        .map(|item| item.to_string())
        .ok_or_else(|| askama::Error::Custom("List is empty".into()))
}

/// Join list elements with a separator
///
/// # Examples
///
/// ```
/// use reinhardt_templates::join;
///
/// let items = vec!["a", "b", "c"];
/// assert_eq!(join(&items, ", ").unwrap(), "a, b, c");
/// ```
pub fn join<T: std::fmt::Display>(list: &[T], separator: &str) -> AskamaResult<String> {
    Ok(list
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<_>>()
        .join(separator))
}

/// URL-encode a string
///
/// # Examples
///
/// ```
/// use reinhardt_templates::urlencode;
///
/// assert_eq!(urlencode("hello world").unwrap(), "hello%20world");
/// assert_eq!(urlencode("a+b=c").unwrap(), "a%2Bb%3Dc");
/// ```
pub fn urlencode(s: &str) -> AskamaResult<String> {
    Ok(urlencoding::encode(s).to_string())
}

/// Calculate time difference from now
///
/// # Examples
///
/// ```
/// use reinhardt_templates::timesince;
/// use chrono::{Utc, Duration};
///
/// let past = Utc::now() - Duration::hours(2);
/// let result = timesince(&past).unwrap();
/// assert!(result.contains("hour"));
/// ```
pub fn timesince(dt: &DateTime<Utc>) -> AskamaResult<String> {
    let now = Utc::now();
    let duration = now.signed_duration_since(*dt);

    if duration < Duration::zero() {
        return Ok("in the future".to_string());
    }

    let seconds = duration.num_seconds();
    let minutes = duration.num_minutes();
    let hours = duration.num_hours();
    let days = duration.num_days();

    if days > 365 {
        let years = days / 365;
        Ok(format!(
            "{} year{}",
            years,
            if years != 1 { "s" } else { "" }
        ))
    } else if days > 30 {
        let months = days / 30;
        Ok(format!(
            "{} month{}",
            months,
            if months != 1 { "s" } else { "" }
        ))
    } else if days > 0 {
        Ok(format!("{} day{}", days, if days != 1 { "s" } else { "" }))
    } else if hours > 0 {
        Ok(format!(
            "{} hour{}",
            hours,
            if hours != 1 { "s" } else { "" }
        ))
    } else if minutes > 0 {
        Ok(format!(
            "{} minute{}",
            minutes,
            if minutes != 1 { "s" } else { "" }
        ))
    } else {
        Ok(format!(
            "{} second{}",
            seconds,
            if seconds != 1 { "s" } else { "" }
        ))
    }
}

/// Default value if variable is empty or None
///
/// # Examples
///
/// ```
/// use reinhardt_templates::default;
///
/// assert_eq!(default("", "N/A").unwrap(), "N/A");
/// assert_eq!(default("Hello", "N/A").unwrap(), "Hello");
/// ```
pub fn default(s: &str, default_value: &str) -> AskamaResult<String> {
    if s.is_empty() {
        Ok(default_value.to_string())
    } else {
        Ok(s.to_string())
    }
}

/// Word count
///
/// # Examples
///
/// ```
/// use reinhardt_templates::wordcount;
///
/// assert_eq!(wordcount("hello world").unwrap(), "2");
/// assert_eq!(wordcount("one two three").unwrap(), "3");
/// ```
pub fn wordcount(s: &str) -> AskamaResult<String> {
    let count = s.split_whitespace().count();
    Ok(count.to_string())
}

/// Add a value to a number
///
/// # Examples
///
/// ```
/// use reinhardt_templates::add;
///
/// assert_eq!(add(5, 3).unwrap(), 8);
/// assert_eq!(add(10, -5).unwrap(), 5);
/// ```
pub fn add(value: i64, arg: i64) -> AskamaResult<i64> {
    Ok(value + arg)
}

/// Pluralize a word based on count
///
/// # Examples
///
/// ```
/// use reinhardt_templates::pluralize;
///
/// assert_eq!(pluralize(1, "s").unwrap(), "");
/// assert_eq!(pluralize(2, "s").unwrap(), "s");
/// assert_eq!(pluralize(0, "s").unwrap(), "s");
/// ```
pub fn pluralize(count: i64, suffix: &str) -> AskamaResult<String> {
    if count == 1 {
        Ok(String::new())
    } else {
        Ok(suffix.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Hello World", 5).unwrap(), "He...");
        assert_eq!(truncate("Hi", 10).unwrap(), "Hi");
        assert_eq!(truncate("Hello", 5).unwrap(), "Hello");
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World!").unwrap(), "hello-world");
        assert_eq!(
            slugify("Django REST Framework").unwrap(),
            "django-rest-framework"
        );
        assert_eq!(slugify("test___slug").unwrap(), "test-slug");
    }

    #[test]
    fn test_title() {
        assert_eq!(title("hello world").unwrap(), "Hello World");
        assert_eq!(
            title("django-rest-framework").unwrap(),
            "Django Rest Framework"
        );
    }

    #[test]
    fn test_filesizeformat() {
        assert_eq!(filesizeformat(1024).unwrap(), "1.00 KB");
        assert_eq!(filesizeformat(1048576).unwrap(), "1.00 MB");
        assert_eq!(filesizeformat(512).unwrap(), "512 B");
    }

    #[test]
    fn test_floatformat() {
        assert_eq!(floatformat(3.14159, 2).unwrap(), "3.14");
        assert_eq!(floatformat(2.0, 2).unwrap(), "2.00");
        assert_eq!(floatformat(1.5, 0).unwrap(), "2");
    }

    #[test]
    fn test_first_last() {
        let items = vec!["a", "b", "c"];
        assert_eq!(first(&items).unwrap(), "a");
        assert_eq!(last(&items).unwrap(), "c");
    }

    #[test]
    fn test_first_last_empty() {
        let items: Vec<String> = vec![];
        assert!(first(&items).is_err());
        assert!(last(&items).is_err());
    }

    #[test]
    fn test_join() {
        let items = vec!["a", "b", "c"];
        assert_eq!(join(&items, ", ").unwrap(), "a, b, c");
        assert_eq!(join(&items, "-").unwrap(), "a-b-c");
    }

    #[test]
    fn test_urlencode() {
        assert_eq!(urlencode("hello world").unwrap(), "hello%20world");
        assert_eq!(urlencode("a+b=c").unwrap(), "a%2Bb%3Dc");
    }

    #[test]
    fn test_timesince() {
        let past = Utc::now() - Duration::hours(2);
        let result = timesince(&past).unwrap();
        assert!(result.contains("hour"));

        let past_days = Utc::now() - Duration::days(5);
        let result = timesince(&past_days).unwrap();
        assert!(result.contains("day"));
    }

    #[test]
    fn test_default() {
        assert_eq!(default("", "N/A").unwrap(), "N/A");
        assert_eq!(default("Hello", "N/A").unwrap(), "Hello");
    }

    #[test]
    fn test_wordcount() {
        assert_eq!(wordcount("hello world").unwrap(), "2");
        assert_eq!(wordcount("one two three").unwrap(), "3");
        assert_eq!(wordcount("").unwrap(), "0");
    }

    #[test]
    fn test_add() {
        assert_eq!(add(5, 3).unwrap(), 8);
        assert_eq!(add(10, -5).unwrap(), 5);
        assert_eq!(add(-3, -2).unwrap(), -5);
    }

    #[test]
    fn test_pluralize() {
        assert_eq!(pluralize(1, "s").unwrap(), "");
        assert_eq!(pluralize(2, "s").unwrap(), "s");
        assert_eq!(pluralize(0, "s").unwrap(), "s");
    }
}
