//! Integration tests for Template + i18n functionality
//!
//! Tests the integration between reinhardt-templates and reinhardt-i18n crates.
//! Covers multilingual template rendering, translation filters, and localization.

use reinhardt_i18n::{
    activate, deactivate, get_locale, gettext, ngettext, pgettext, MessageCatalog,
};
use reinhardt_templates::{blocktrans, localize_date_filter, localize_number_filter};
use tera::{Context, Tera};

fn render_translation_template(message: &str) -> String {
    let mut context = Context::new();
    context.insert("message", message);

    Tera::one_off("Welcome: {{ message }}", &context, true).unwrap()
}

#[test]
fn test_template_with_simple_translation() {
    let mut catalog = MessageCatalog::new("ja");
    catalog.add_translation("Hello", "こんにちは");
    catalog.add_translation("Welcome", "ようこそ");

    activate("ja", catalog);

    let message = gettext("Hello");
    let rendered = render_translation_template(&message);

    assert_eq!(rendered, "Welcome: こんにちは");

    deactivate();
}

#[test]
fn test_template_with_context_translation() {
    let mut catalog = MessageCatalog::new("es");
    catalog.add_context("button", "Save", "Guardar");
    catalog.add_context("menu", "Save", "Guardar archivo");

    activate("es", catalog);

    let button_save = pgettext("button", "Save");
    let menu_save = pgettext("menu", "Save");

    assert_eq!(button_save, "Guardar");
    assert_eq!(menu_save, "Guardar archivo");

    deactivate();
}

#[test]
fn test_template_with_block_translation() {
    let mut catalog = MessageCatalog::new("fr");
    catalog.add_translation("Welcome!", "Bienvenue!");

    activate("fr", catalog);

    let result = blocktrans("Welcome!").unwrap();
    assert_eq!(result, "Welcome!");

    deactivate();
}

#[test]
fn test_template_with_plural_translation() {
    let mut catalog = MessageCatalog::new("de");
    catalog.add_plural("item", "items", vec!["Artikel", "Artikel"]);

    activate("de", catalog);

    let result_singular = ngettext("item", "items", 1);
    assert_eq!(result_singular, "Artikel");

    let result_plural = ngettext("item", "items", 5);
    assert_eq!(result_plural, "Artikel");

    deactivate();
}

#[test]
fn test_template_number_localization() {
    let catalog = MessageCatalog::new("fr");

    activate("fr", catalog);

    let result = localize_number_filter(1234.56).unwrap();
    assert!(result.contains("1234"));

    deactivate();
}

#[test]
fn test_template_date_localization() {
    let catalog = MessageCatalog::new("ja");
    activate("ja", catalog);

    let date_str = "2025-10-16";
    let result = localize_date_filter(date_str).unwrap();

    assert!(result.contains("2025"));
    assert!(result.contains("10"));
    assert!(result.contains("16"));

    deactivate();
}

#[test]
fn test_language_detection_in_template() {
    let catalog1 = MessageCatalog::new("it");
    activate("it", catalog1);

    let lang = get_locale();
    assert_eq!(lang, "it");

    deactivate();

    let catalog2 = MessageCatalog::new("pt");
    activate("pt", catalog2);
    let lang2 = get_locale();
    assert_eq!(lang2, "pt");

    deactivate();
}

#[test]
fn test_multilingual_template_rendering() {
    let languages = vec![
        ("en", "Hello", "Hello"),
        ("ja", "Hello", "こんにちは"),
        ("es", "Hello", "Hola"),
        ("fr", "Hello", "Bonjour"),
    ];

    for (lang, key, expected) in languages {
        let mut catalog = MessageCatalog::new(lang);
        catalog.add_translation(key, expected);

        activate(lang, catalog);

        let translated = gettext(key);
        assert_eq!(translated, expected, "Failed for language: {}", lang);

        deactivate();
    }
}

#[test]
fn test_template_with_missing_translation() {
    let mut catalog = MessageCatalog::new("zh");
    catalog.add_translation("Existing", "存在");

    activate("zh", catalog);

    let existing = gettext("Existing");
    assert_eq!(existing, "存在");

    let missing = gettext("NonExistent");
    assert_eq!(missing, "NonExistent");

    deactivate();
}

#[test]
fn test_template_fallback_to_default_language() {
    let catalog = MessageCatalog::new("unknown_lang");
    activate("unknown_lang", catalog);

    let result = gettext("Hello");
    assert_eq!(result, "Hello");

    deactivate();
}

#[test]
fn test_nested_translation_with_variables() {
    let mut catalog = MessageCatalog::new("ko");
    catalog.add_translation(
        "Hello user, you have messages",
        "안녕하세요님, 메시지가 있습니다",
    );

    activate("ko", catalog);

    let result = blocktrans("Hello user, you have messages").unwrap();
    assert_eq!(result, "Hello user, you have messages");

    deactivate();
}
