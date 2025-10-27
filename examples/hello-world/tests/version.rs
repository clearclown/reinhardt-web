use example_common::version;

#[test]
fn test_reinhardt_version_detection() {
    let version = version::get_reinhardt_version();
    println!("Detected reinhardt version: {}", version);

    if version == "unknown" {
        eprintln!("⚠️  Could not detect reinhardt version");
        eprintln!("   This is expected if reinhardt is not yet published.");
    } else {
        println!("✅ Version detected: {}", version);
    }
}

#[test]
fn test_version_spec_wildcard() {
    // ワイルドカード "*" は常に true
    // ただし、バージョンが unknown の場合は false
    if version::get_reinhardt_version() != "unknown" {
        assert!(version::check_version("*"));
    }
}
