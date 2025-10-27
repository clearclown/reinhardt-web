//! バージョン検証ユーティリティ

use semver::{Version, VersionReq};
use std::sync::OnceLock;

static REINHARDT_VERSION: OnceLock<Option<String>> = OnceLock::new();

/// reinhardt のバージョンを取得
pub fn get_reinhardt_version() -> &'static str {
    REINHARDT_VERSION
        .get_or_init(|| {
            // 複数の方法でバージョンを取得
            get_version_from_cargo_metadata().or_else(|| get_version_from_cargo_lock())
        })
        .as_ref()
        .map(|s| s.as_str())
        .unwrap_or("unknown")
}

/// Cargo バージョン指定子でバージョンをチェック
///
/// Cargo.toml と同じ構文をサポート:
/// - "0.1.0" - 正確なバージョン
/// - "^0.1" - キャレット要件 (0.1.x)
/// - "~0.1.2" - チルダ要件 (0.1.2 <= version < 0.2.0)
/// - ">=0.1, <0.2" - 範囲指定
/// - "*" - ワイルドカード (最新)
pub fn check_version(version_spec: &str) -> bool {
    let actual_version = get_reinhardt_version();

    if actual_version == "unknown" {
        eprintln!("⚠️  Warning: Could not determine reinhardt version");
        return false;
    }

    // "*" は最新版（常に true）
    if version_spec == "*" {
        return true;
    }

    // セマンティックバージョニングでチェック
    match (Version::parse(actual_version), VersionReq::parse(version_spec)) {
        (Ok(version), Ok(req)) => {
            let matches = req.matches(&version);
            if !matches {
                eprintln!(
                    "   Version mismatch: required {}, found {}",
                    version_spec, actual_version
                );
            }
            matches
        }
        (Err(e), _) => {
            eprintln!(
                "❌ Failed to parse actual version '{}': {}",
                actual_version, e
            );
            false
        }
        (_, Err(e)) => {
            eprintln!(
                "❌ Failed to parse version requirement '{}': {}",
                version_spec, e
            );
            false
        }
    }
}

/// cargo metadata から reinhardt のバージョンを取得
fn get_version_from_cargo_metadata() -> Option<String> {
    use std::process::Command;

    let output = Command::new("cargo")
        .args(&["metadata", "--format-version", "1", "--no-deps"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    let packages = metadata.get("packages")?.as_array()?;

    for package in packages {
        if package.get("name")?.as_str()? == "reinhardt" {
            return Some(package.get("version")?.as_str()?.to_string());
        }
    }

    None
}

/// Cargo.lock から reinhardt のバージョンを取得
fn get_version_from_cargo_lock() -> Option<String> {
    use std::fs;

    let cargo_lock = fs::read_to_string("../Cargo.lock").ok()?;

    // TOML パースを避けて単純な文字列検索
    let mut found_reinhardt = false;
    for line in cargo_lock.lines() {
        if line.trim() == "name = \"reinhardt\"" {
            found_reinhardt = true;
            continue;
        }

        if found_reinhardt {
            if let Some(version_line) = line.trim().strip_prefix("version = \"") {
                if let Some(version) = version_line.strip_suffix("\"") {
                    return Some(version.to_string());
                }
            }
            // name の後に version がなければ次の name まで探索
            if line.trim().starts_with("name = ") {
                found_reinhardt = false;
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_spec_parsing() {
        // バージョン指定子のパースが正しく動作するか確認
        let specs = vec!["0.1.0", "^0.1", "~0.1.2", ">=0.1, <0.2", "*"];

        for spec in specs {
            if spec != "*" {
                assert!(
                    VersionReq::parse(spec).is_ok(),
                    "Failed to parse version spec: {}",
                    spec
                );
            }
        }
    }

    #[test]
    fn test_wildcard_always_passes() {
        // ワイルドカードは常に true を返す
        // ただし、バージョンが unknown の場合は false
        if get_reinhardt_version() != "unknown" {
            assert!(check_version("*"));
        }
    }
}
