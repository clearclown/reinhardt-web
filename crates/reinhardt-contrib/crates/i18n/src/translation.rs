//! Core translation functions
//!
//! Provides Django-style translation functions:
//! - `gettext()` - Simple translation
//! - `ngettext()` - Plural translation
//! - `pgettext()` - Contextual translation
//! - `npgettext()` - Contextual plural translation

use crate::{LazyString, TRANSLATION_STATE};

/// Translate a message
///
/// # Example
/// ```
/// use reinhardt_i18n::{activate, gettext, MessageCatalog};
///
/// // Create and activate a Japanese catalog
/// let mut catalog = MessageCatalog::new("ja");
/// catalog.add_translation("Hello, world!", "こんにちは、世界！");
/// activate("ja", catalog);
///
/// let msg = gettext("Hello, world!");
/// assert_eq!(msg, "こんにちは、世界！");
/// ```
pub fn gettext(message: &str) -> String {
    let state = TRANSLATION_STATE.read().unwrap();
    let locale = state.get_locale();

    if let Some(catalog) = state.get_catalog(locale) {
        if let Some(translation) = catalog.get(message) {
            return translation.clone();
        }
    }

    // Try fallback locale
    let fallback = state.get_fallback_locale();
    if locale != fallback {
        if let Some(catalog) = state.get_catalog(fallback) {
            if let Some(translation) = catalog.get(message) {
                return translation.clone();
            }
        }
    }

    // Return original message if no translation found
    message.to_string()
}

/// Translate a message with plural support
///
/// # Example
/// ```
/// use reinhardt_i18n::{activate, ngettext, MessageCatalog};
///
/// // Set up German plural translations
/// let mut catalog = MessageCatalog::new("de");
/// catalog.add_plural("item", "items", vec!["Artikel", "Artikel"]);
/// activate("de", catalog);
///
/// // Singular form (1 item)
/// let msg_singular = ngettext("item", "items", 1);
/// assert_eq!(msg_singular, "Artikel");
///
/// // Plural form (5 items)
/// let msg_plural = ngettext("item", "items", 5);
/// assert_eq!(msg_plural, "Artikel");
/// ```
pub fn ngettext(singular: &str, plural: &str, count: usize) -> String {
    let state = TRANSLATION_STATE.read().unwrap();
    let locale = state.get_locale();

    if let Some(catalog) = state.get_catalog(locale) {
        if let Some(translation) = catalog.get_plural(singular, count) {
            return translation.clone();
        }
    }

    // Try fallback locale
    let fallback = state.get_fallback_locale();
    if locale != fallback {
        if let Some(catalog) = state.get_catalog(fallback) {
            if let Some(translation) = catalog.get_plural(singular, count) {
                return translation.clone();
            }
        }
    }

    // Use default English plural rules
    if count == 1 {
        singular.to_string()
    } else {
        plural.to_string()
    }
}

/// Translate a message with context
///
/// Context helps disambiguate translations. For example:
/// - pgettext("menu", "File") -> "ファイル"
/// - pgettext("verb", "File") -> "提出する"
///
/// # Example
/// ```
/// use reinhardt_i18n::{activate, pgettext, MessageCatalog};
///
/// // Set up contextual translations for Japanese
/// let mut catalog = MessageCatalog::new("ja");
/// catalog.add_context("menu", "File", "ファイル");
/// catalog.add_context("verb", "File", "提出する");
/// activate("ja", catalog);
///
/// // Same word, different meanings based on context
/// let menu_file = pgettext("menu", "File");
/// assert_eq!(menu_file, "ファイル");
///
/// let verb_file = pgettext("verb", "File");
/// assert_eq!(verb_file, "提出する");
/// ```
pub fn pgettext(context: &str, message: &str) -> String {
    let state = TRANSLATION_STATE.read().unwrap();
    let locale = state.get_locale();

    if let Some(catalog) = state.get_catalog(locale) {
        if let Some(translation) = catalog.get_context(context, message) {
            return translation.clone();
        }
    }

    // Try fallback locale
    let fallback = state.get_fallback_locale();
    if locale != fallback {
        if let Some(catalog) = state.get_catalog(fallback) {
            if let Some(translation) = catalog.get_context(context, message) {
                return translation.clone();
            }
        }
    }

    // Return original message if no translation found
    message.to_string()
}

/// Translate a message with context and plural support
///
/// # Example
/// ```
/// use reinhardt_i18n::{activate, npgettext, MessageCatalog};
///
/// // Set up contextual plural translations for Spanish
/// let mut catalog = MessageCatalog::new("es");
/// catalog.add_context_plural("email", "message", "messages", vec!["mensaje", "mensajes"]);
/// catalog.add_context_plural("notification", "message", "messages", vec!["notificación", "notificaciones"]);
/// activate("es", catalog);
///
/// // Email context (1 message)
/// let email_singular = npgettext("email", "message", "messages", 1);
/// assert_eq!(email_singular, "mensaje");
///
/// // Email context (5 messages)
/// let email_plural = npgettext("email", "message", "messages", 5);
/// assert_eq!(email_plural, "mensajes");
///
/// // Notification context (3 messages)
/// let notification_plural = npgettext("notification", "message", "messages", 3);
/// assert_eq!(notification_plural, "notificaciones");
/// ```
pub fn npgettext(context: &str, singular: &str, plural: &str, count: usize) -> String {
    let state = TRANSLATION_STATE.read().unwrap();
    let locale = state.get_locale();

    if let Some(catalog) = state.get_catalog(locale) {
        if let Some(translation) = catalog.get_context_plural(context, singular, count) {
            return translation.clone();
        }
    }

    // Try fallback locale
    let fallback = state.get_fallback_locale();
    if locale != fallback {
        if let Some(catalog) = state.get_catalog(fallback) {
            if let Some(translation) = catalog.get_context_plural(context, singular, count) {
                return translation.clone();
            }
        }
    }

    // Use default English plural rules
    if count == 1 {
        singular.to_string()
    } else {
        plural.to_string()
    }
}

/// Create a lazy translation that is evaluated when converted to string
///
/// # Example
/// ```
/// use reinhardt_i18n::{activate, gettext_lazy, MessageCatalog};
///
/// // Create lazy translation before setting up catalog
/// let lazy_msg = gettext_lazy("Good morning");
///
/// // Set up catalog later
/// let mut catalog = MessageCatalog::new("ko");
/// catalog.add_translation("Good morning", "좋은 아침");
/// activate("ko", catalog);
///
/// // Translation happens when we use it
/// assert_eq!(lazy_msg.to_string(), "좋은 아침");
/// ```
pub fn gettext_lazy(message: &str) -> LazyString {
    LazyString::new(message.to_string(), None, false)
}

/// Create a lazy plural translation
///
/// # Example
/// ```
/// use reinhardt_i18n::{activate, ngettext_lazy, MessageCatalog};
///
/// // Create lazy plural translation
/// let lazy_msg = ngettext_lazy("apple", "apples", 7);
///
/// // Set up catalog with plural forms
/// let mut catalog = MessageCatalog::new("pl");
/// catalog.add_plural("apple", "apples", vec!["jabłko", "jabłka"]);
/// activate("pl", catalog);
///
/// // Translation happens when evaluated
/// assert_eq!(lazy_msg.to_string(), "jabłka");
/// ```
pub fn ngettext_lazy(singular: &str, plural: &str, count: usize) -> LazyString {
    LazyString::new_plural(singular.to_string(), plural.to_string(), count, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gettext_no_translation() {
        let result = gettext("Untranslated message");
        assert_eq!(result, "Untranslated message");
    }

    #[test]
    fn test_ngettext_default_rules_unit() {
        let result_singular = ngettext("There is {} item", "There are {} items", 1);
        assert_eq!(result_singular, "There is {} item");

        let result_plural = ngettext("There is {} item", "There are {} items", 5);
        assert_eq!(result_plural, "There are {} items");
    }

    #[test]
    fn test_pgettext_no_translation() {
        let result = pgettext("menu", "File");
        assert_eq!(result, "File");
    }
}
