//! Message catalog tests
//!
//! Tests based on Django's i18n catalog functionality

use reinhardt_i18n::{CatalogLoader, MessageCatalog};
use serial_test::serial;
use std::fs;
use tempfile::TempDir;
use unic_langid::LanguageIdentifier;

#[test]
#[serial(i18n)]
fn test_catalog_simple_message_integration() {
    let locale: LanguageIdentifier = "en-US".parse().unwrap();
    let mut catalog = MessageCatalog::new(locale);

    catalog.add("hello".to_string(), "Hello!".to_string());
    assert_eq!(catalog.get("hello"), Some(&"Hello!".to_string()));
    assert_eq!(catalog.get("nonexistent"), None);
}

#[test]
#[serial(i18n)]
fn test_catalog_plural_english() {
    let locale: LanguageIdentifier = "en-US".parse().unwrap();
    let mut catalog = MessageCatalog::new(locale);

    catalog.add_plural(
        "item".to_string(),
        vec!["One item".to_string(), "Many items".to_string()],
    );

    assert_eq!(catalog.get_plural("item", 1), Some(&"One item".to_string()));
    assert_eq!(
        catalog.get_plural("item", 0),
        Some(&"Many items".to_string())
    );
    assert_eq!(
        catalog.get_plural("item", 5),
        Some(&"Many items".to_string())
    );
    assert_eq!(
        catalog.get_plural("item", 2),
        Some(&"Many items".to_string())
    );
}

#[test]
#[serial(i18n)]
fn test_catalog_plural_french() {
    let locale: LanguageIdentifier = "fr-FR".parse().unwrap();
    let mut catalog = MessageCatalog::new(locale);

    // French: 0 and 1 are singular (index 0), > 1 is plural (index 1)
    catalog.add_plural(
        "jour".to_string(),
        vec!["jour".to_string(), "jours".to_string()],
    );

    assert_eq!(catalog.get_plural("jour", 0), Some(&"jour".to_string()));
    assert_eq!(catalog.get_plural("jour", 1), Some(&"jour".to_string()));
    assert_eq!(catalog.get_plural("jour", 2), Some(&"jours".to_string()));
    assert_eq!(catalog.get_plural("jour", 5), Some(&"jours".to_string()));
}

#[test]
#[serial(i18n)]
fn test_catalog_plural_japanese() {
    let locale: LanguageIdentifier = "ja-JP".parse().unwrap();
    let mut catalog = MessageCatalog::new(locale);

    // Japanese has no plural forms - always uses index 0
    catalog.add_plural("item".to_string(), vec!["アイテム".to_string()]);

    assert_eq!(catalog.get_plural("item", 0), Some(&"アイテム".to_string()));
    assert_eq!(catalog.get_plural("item", 1), Some(&"アイテム".to_string()));
    assert_eq!(
        catalog.get_plural("item", 100),
        Some(&"アイテム".to_string())
    );
}

#[test]
#[serial(i18n)]
fn test_catalog_context_integration() {
    let locale: LanguageIdentifier = "ja-JP".parse().unwrap();
    let mut catalog = MessageCatalog::new(locale);

    catalog.add_context(
        "menu".to_string(),
        "File".to_string(),
        "ファイル".to_string(),
    );
    catalog.add_context(
        "verb".to_string(),
        "File".to_string(),
        "提出する".to_string(),
    );

    assert_eq!(
        catalog.get_context("menu", "File"),
        Some(&"ファイル".to_string())
    );
    assert_eq!(
        catalog.get_context("verb", "File"),
        Some(&"提出する".to_string())
    );
    assert_eq!(catalog.get_context("other", "File"), None);
}

#[test]
#[serial(i18n)]
fn test_catalog_context_plural() {
    let locale: LanguageIdentifier = "de-DE".parse().unwrap();
    let mut catalog = MessageCatalog::new(locale);

    // Context plural uses "context:msgid" as key
    catalog.add_plural(
        "email:message".to_string(),
        vec!["Nachricht".to_string(), "Nachrichten".to_string()],
    );

    assert_eq!(
        catalog.get_context_plural("email", "message", 1),
        Some(&"Nachricht".to_string())
    );
    assert_eq!(
        catalog.get_context_plural("email", "message", 5),
        Some(&"Nachrichten".to_string())
    );
}

#[test]
#[serial(i18n)]
fn test_catalog_multiple_contexts() {
    let locale: LanguageIdentifier = "en-US".parse().unwrap();
    let mut catalog = MessageCatalog::new(locale);

    catalog.add_context(
        "food".to_string(),
        "Apple".to_string(),
        "Food apple".to_string(),
    );
    catalog.add_context(
        "company".to_string(),
        "Apple".to_string(),
        "Apple Inc.".to_string(),
    );
    catalog.add_context(
        "color".to_string(),
        "Apple".to_string(),
        "Apple color".to_string(),
    );

    assert_eq!(
        catalog.get_context("food", "Apple"),
        Some(&"Food apple".to_string())
    );
    assert_eq!(
        catalog.get_context("company", "Apple"),
        Some(&"Apple Inc.".to_string())
    );
    assert_eq!(
        catalog.get_context("color", "Apple"),
        Some(&"Apple color".to_string())
    );
}

#[test]
#[serial(i18n)]
fn test_catalog_empty_message() {
    let locale: LanguageIdentifier = "en-US".parse().unwrap();
    let mut catalog = MessageCatalog::new(locale);

    catalog.add("".to_string(), "".to_string());
    assert_eq!(catalog.get(""), Some(&"".to_string()));
}

#[test]
#[serial(i18n)]
fn test_catalog_special_characters() {
    let locale: LanguageIdentifier = "en-US".parse().unwrap();
    let mut catalog = MessageCatalog::new(locale);

    catalog.add("Hello\nWorld".to_string(), "Bonjour\nMonde".to_string());
    catalog.add("Tab\tSeparated".to_string(), "Tab\tSéparé".to_string());
    catalog.add("Quote\"Test".to_string(), "Citation\"Test".to_string());

    assert_eq!(
        catalog.get("Hello\nWorld"),
        Some(&"Bonjour\nMonde".to_string())
    );
    assert_eq!(
        catalog.get("Tab\tSeparated"),
        Some(&"Tab\tSéparé".to_string())
    );
    assert_eq!(
        catalog.get("Quote\"Test"),
        Some(&"Citation\"Test".to_string())
    );
}

#[test]
#[serial(i18n)]
fn test_catalog_locale() {
    let locale: LanguageIdentifier = "fr-FR".parse().unwrap();
    let catalog = MessageCatalog::new(locale.clone());

    assert_eq!(catalog.locale().to_string(), "fr-FR");
}

#[test]
#[serial(i18n)]
fn test_catalog_loader_json() {
    let temp_dir = TempDir::new().unwrap();
    let locale_dir = temp_dir.path().join("locale");
    let fr_dir = locale_dir.join("fr-FR");

    fs::create_dir_all(&fr_dir).unwrap();

    let json_content = r#"{
        "Hello": "Bonjour",
        "Goodbye": "Au revoir",
        "Welcome": "Bienvenue"
    }"#;

    fs::write(fr_dir.join("messages.json"), json_content).unwrap();

    let mut loader = CatalogLoader::new();
    loader.add_locale_dir(locale_dir.to_string_lossy().to_string());

    let locale: LanguageIdentifier = "fr-FR".parse().unwrap();
    let catalog = loader.load_json(&locale, "messages").unwrap();

    assert_eq!(catalog.get("Hello"), Some(&"Bonjour".to_string()));
    assert_eq!(catalog.get("Goodbye"), Some(&"Au revoir".to_string()));
    assert_eq!(catalog.get("Welcome"), Some(&"Bienvenue".to_string()));
    assert_eq!(catalog.get("NonExistent"), None);

    // Cleanup is automatic with TempDir
}

#[test]
#[serial(i18n)]
fn test_catalog_loader_json_not_found() {
    let temp_dir = TempDir::new().unwrap();

    let mut loader = CatalogLoader::new();
    loader.add_locale_dir(temp_dir.path().to_string_lossy().to_string());

    let locale: LanguageIdentifier = "xx-XX".parse().unwrap();
    let result = loader.load_json(&locale, "messages");

    assert!(result.is_err());

    // Cleanup is automatic with TempDir
}

#[test]
#[serial(i18n)]
fn test_catalog_loader_multiple_dirs() {
    let temp_dir1 = TempDir::new().unwrap();
    let temp_dir2 = TempDir::new().unwrap();

    let locale_dir1 = temp_dir1.path().join("locale1");
    let locale_dir2 = temp_dir2.path().join("locale2");

    let fr_dir1 = locale_dir1.join("fr-FR");
    let fr_dir2 = locale_dir2.join("fr-FR");

    fs::create_dir_all(&fr_dir1).unwrap();
    fs::create_dir_all(&fr_dir2).unwrap();

    // First directory has priority
    fs::write(fr_dir1.join("test.json"), r#"{"Hello": "Bonjour1"}"#).unwrap();
    fs::write(fr_dir2.join("test.json"), r#"{"Hello": "Bonjour2"}"#).unwrap();

    let mut loader = CatalogLoader::new();
    loader.add_locale_dir(locale_dir1.to_string_lossy().to_string());
    loader.add_locale_dir(locale_dir2.to_string_lossy().to_string());

    let locale: LanguageIdentifier = "fr-FR".parse().unwrap();
    let catalog = loader.load_json(&locale, "test").unwrap();

    // Should load from first directory
    assert_eq!(catalog.get("Hello"), Some(&"Bonjour1".to_string()));

    // Cleanup is automatic with TempDir
}

#[test]
#[serial(i18n)]
fn test_catalog_plural_nonexistent() {
    let locale: LanguageIdentifier = "en-US".parse().unwrap();
    let catalog = MessageCatalog::new(locale);

    assert_eq!(catalog.get_plural("nonexistent", 1), None);
}

#[test]
#[serial(i18n)]
fn test_catalog_overwrite() {
    let locale: LanguageIdentifier = "en-US".parse().unwrap();
    let mut catalog = MessageCatalog::new(locale);

    catalog.add("test".to_string(), "first".to_string());
    assert_eq!(catalog.get("test"), Some(&"first".to_string()));

    // Overwrite
    catalog.add("test".to_string(), "second".to_string());
    assert_eq!(catalog.get("test"), Some(&"second".to_string()));
}

#[test]
#[serial(i18n)]
fn test_catalog_unicode_messages() {
    let locale: LanguageIdentifier = "ja-JP".parse().unwrap();
    let mut catalog = MessageCatalog::new(locale);

    catalog.add("こんにちは".to_string(), "Hello".to_string());
    catalog.add("Hello".to_string(), "こんにちは".to_string());
    catalog.add("emoji".to_string(), "😀🎉🚀".to_string());

    assert_eq!(catalog.get("こんにちは"), Some(&"Hello".to_string()));
    assert_eq!(catalog.get("Hello"), Some(&"こんにちは".to_string()));
    assert_eq!(catalog.get("emoji"), Some(&"😀🎉🚀".to_string()));
}
