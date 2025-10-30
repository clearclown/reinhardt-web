//! crates.io availability check utilities

use std::sync::OnceLock;

static REINHARDT_AVAILABLE: OnceLock<bool> = OnceLock::new();

/// Check if reinhardt is available from crates.io
pub fn is_reinhardt_available() -> bool {
    *REINHARDT_AVAILABLE.get_or_init(|| check_reinhardt_availability())
}

fn check_reinhardt_availability() -> bool {
    // 1. Check Cargo.lock existence
    let cargo_lock_exists = std::path::Path::new("../Cargo.lock").exists();
    if !cargo_lock_exists {
        eprintln!("⚠️  Cargo.lock not found - dependency resolution may have failed");
        return false;
    }

    // 2. Verify reinhardt existence with cargo tree
    use std::process::Command;

    let output = Command::new("cargo")
        .args(&["tree", "-p", "reinhardt", "--depth", "0"])
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let available = stdout.contains("reinhardt");

                if !available {
                    eprintln!("⚠️  reinhardt not found in dependency tree");
                    eprintln!(
                        "   This is expected if reinhardt is not yet published to crates.io"
                    );
                }

                available
            } else {
                eprintln!(
                    "⚠️  cargo tree failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                false
            }
        }
        Err(e) => {
            eprintln!("⚠️  Failed to run cargo tree: {}", e);
            false
        }
    }
}

/// Availability check before tests (executed only once)
pub fn ensure_reinhardt_available() -> Result<(), String> {
    if is_reinhardt_available() {
        Ok(())
    } else {
        Err("reinhardt is not available from crates.io".to_string())
    }
}
