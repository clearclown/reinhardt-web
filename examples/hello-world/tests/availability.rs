use example_common::availability;

/// 最初に実行: crates.io から reinhardt を取得できるか確認
#[test]
fn test_reinhardt_available() {
    match availability::ensure_reinhardt_available() {
        Ok(_) => {
            println!("✅ reinhardt is available from crates.io");
        }
        Err(e) => {
            eprintln!("❌ reinhardt is NOT available from crates.io: {}", e);
            eprintln!("   All subsequent tests will be skipped.");
            eprintln!("   This is expected if reinhardt is not yet published.");

            // 公開前なので panic にはしない（CI で警告として表示）
            eprintln!("⚠️  Skipping examples tests (reinhardt not published)");
        }
    }
}
