use reinhardt_static::hashed_storage::{HashedFileStorage, HashedStorageConfig};
use reinhardt_static::storage::Storage;
use std::collections::HashMap;
use tempfile::TempDir;

#[test]
fn test_css_url_replacement() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    // Use save_with_dependencies to handle dependency order
    let mut files = HashMap::new();

    // Add the referenced image
    files.insert("img/logo.png".to_string(), b"fake image data".to_vec());

    // Add CSS that references it
    files.insert(
        "styles.css".to_string(),
        b"body { background: url(img/logo.png); }".to_vec(),
    );

    // Process with dependency resolution
    storage.save_with_dependencies(files).unwrap();

    // Read back the saved CSS
    let saved_css = storage.open("styles.css").unwrap();
    let saved_str = String::from_utf8(saved_css).unwrap();

    // Should contain hashed version
    assert!(
        saved_str.contains("img/logo.") && saved_str.contains(".png"),
        "CSS should contain hashed image reference, got: {}",
        saved_str
    );
    // Should not contain original reference
    assert!(!saved_str.contains("url(img/logo.png)"));
}

#[test]
fn test_css_url_with_quotes() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    let img_content = b"fake image";
    storage.save("bg.jpg", img_content).unwrap();

    let css_content = b"body { background: url('bg.jpg'); }";
    storage.save("app.css", css_content).unwrap();

    let saved_css = storage.open("app.css").unwrap();
    let saved_str = String::from_utf8(saved_css).unwrap();

    // Should contain hashed version
    assert!(saved_str.contains("bg."));
    assert!(saved_str.contains(".jpg"));
}

#[test]
fn test_absolute_url_not_replaced() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    let css_content = b"body { background: url(https://example.com/image.png); }";
    storage.save("external.css", css_content).unwrap();

    let saved_css = storage.open("external.css").unwrap();
    let saved_str = String::from_utf8(saved_css).unwrap();

    // Absolute URL should remain unchanged
    assert!(saved_str.contains("https://example.com/image.png"));
}

#[test]
fn test_data_uri_not_replaced() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    let css_content = b"body { background: url(data:image/png;base64,iVBORw0KGgoAAAA); }";
    storage.save("inline.css", css_content).unwrap();

    let saved_css = storage.open("inline.css").unwrap();
    let saved_str = String::from_utf8(saved_css).unwrap();

    // Data URI should remain unchanged
    assert!(saved_str.contains("data:image/png;base64"));
}

#[test]
fn test_protocol_relative_url_not_replaced() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    let css_content = b"body { background: url(//cdn.example.com/image.png); }";
    storage.save("cdn.css", css_content).unwrap();

    let saved_css = storage.open("cdn.css").unwrap();
    let saved_str = String::from_utf8(saved_css).unwrap();

    // Protocol-relative URL should remain unchanged
    assert!(saved_str.contains("//cdn.example.com/image.png"));
}

#[test]
fn test_hash_anchor_not_replaced() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    let css_content = b"a { color: url(#gradient); }";
    storage.save("anchor.css", css_content).unwrap();

    let saved_css = storage.open("anchor.css").unwrap();
    let saved_str = String::from_utf8(saved_css).unwrap();

    // Hash anchor should remain unchanged
    assert!(saved_str.contains("#gradient"));
}

#[test]
fn test_css_import_replacement() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    // Save imported file first
    let imported_content = b"body { color: red; }";
    storage.save("base.css", imported_content).unwrap();

    // Save file that imports it
    let css_content = b"@import url(base.css);";
    storage.save("main.css", css_content).unwrap();

    let saved_css = storage.open("main.css").unwrap();
    let saved_str = String::from_utf8(saved_css).unwrap();

    // Should contain hashed version
    assert!(saved_str.contains("base."));
    assert!(saved_str.contains(".css"));
    // Should not contain original
    assert!(!saved_str.contains("@import url(base.css)"));
}

#[test]
fn test_multiple_url_replacements() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    // Save multiple images
    storage.save("img1.png", b"image 1").unwrap();
    storage.save("img2.jpg", b"image 2").unwrap();

    let css_content = b"body { background: url(img1.png); } .header { background: url(img2.jpg); }";
    storage.save("multi.css", css_content).unwrap();

    let saved_css = storage.open("multi.css").unwrap();
    let saved_str = String::from_utf8(saved_css).unwrap();

    // Both should be replaced
    assert!(saved_str.contains("img1."));
    assert!(saved_str.contains("img2."));
    assert!(saved_str.contains(".png"));
    assert!(saved_str.contains(".jpg"));
}

#[test]
fn test_js_file_not_post_processed() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    // JS files are marked for post-processing but don't have URL replacement yet
    let js_content = b"console.log('test');";
    let url = storage.save("app.js", js_content).unwrap();

    assert!(url.contains("app."));
    assert!(url.ends_with(".js"));

    // Should be able to open it
    let saved_js = storage.open("app.js").unwrap();
    assert_eq!(saved_js, js_content);
}

#[test]
fn test_image_file_hashed_but_not_processed() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    let img_content = b"fake PNG data";
    let url = storage.save("photo.png", img_content).unwrap();

    // Should have hash in URL
    assert!(url.contains("photo."));
    assert!(url.ends_with(".png"));

    // Content should be unchanged
    let saved_img = storage.open("photo.png").unwrap();
    assert_eq!(saved_img, img_content);
}

#[test]
fn test_exists_uses_hashed_name() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    let content = b"test";
    storage.save("test.txt", content).unwrap();

    // exists() should work with original name
    assert!(storage.exists("test.txt"));
}

#[test]
fn test_url_uses_hashed_name() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    let content = b"test content";
    storage.save("file.txt", content).unwrap();

    // url() should return hashed URL
    let url = storage.url("file.txt");
    assert!(url.contains("file."));
    assert!(url.contains(".txt"));
    assert!(url.starts_with("/static/"));
}

#[test]
fn test_delete_removes_hashed_file() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    let content = b"test";
    storage.save("remove.txt", content).unwrap();

    assert!(storage.exists("remove.txt"));

    storage.delete("remove.txt").unwrap();

    assert!(!storage.exists("remove.txt"));
}

#[test]
fn test_keep_original_option() {
    let temp_dir = TempDir::new().unwrap();
    let mut config = HashedStorageConfig::default();
    config.keep_original = true;

    let storage = HashedFileStorage::with_config(temp_dir.path(), "/static/", config);

    let content = b"keep me";
    storage.save("original.txt", content).unwrap();

    // Both original and hashed versions should exist
    let original_path = temp_dir.path().join("original.txt");
    assert!(original_path.exists());

    // Hashed version should also exist
    let hashed_name = storage.get_hashed_name("original.txt").unwrap();
    let hashed_path = temp_dir.path().join(&hashed_name);
    assert!(hashed_path.exists());
}

#[test]
fn test_nested_path_handling() {
    let temp_dir = TempDir::new().unwrap();
    let storage = HashedFileStorage::new(temp_dir.path(), "/static/");

    let content = b"nested content";
    let url = storage.save("css/nested/style.css", content).unwrap();

    assert!(url.contains("/static/"));
    assert!(url.contains("style."));
    assert!(url.ends_with(".css"));
}

#[test]
fn test_hash_consistency_across_instances() {
    let temp_dir = TempDir::new().unwrap();

    let storage1 = HashedFileStorage::new(temp_dir.path(), "/static/");
    let storage2 = HashedFileStorage::new(temp_dir.path(), "/static/");

    let content = b"consistent content";

    let hash1 = storage1.hashed_name("test.txt", content);
    let hash2 = storage2.hashed_name("test.txt", content);

    // Same content should produce same hash across different instances
    assert_eq!(hash1, hash2);
}
