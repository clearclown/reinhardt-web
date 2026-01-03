use std::env;
use std::path::PathBuf;

/// プロジェクトルートを検出するための複数の戦略を提供する
pub struct PathResolver;

impl PathResolver {
	/// マルチレイヤーパス解決
	///
	/// 以下の優先順位で解決を試みる:
	/// 1. CARGO_MANIFEST_DIR 環境変数を使用
	/// 2. 現在のディレクトリから上に Cargo.toml を検索
	/// 3. 現在のディレクトリをフォールバック
	///
	/// # Arguments
	///
	/// * `relative_path` - 解決する相対パス
	///
	/// # Returns
	///
	/// 解決されたパス。絶対パスの場合はそのまま返される。
	pub fn resolve_static_dir(relative_path: &str) -> PathBuf {
		let path = PathBuf::from(relative_path);

		// 絶対パスの場合はそのまま返す
		if path.is_absolute() {
			return path;
		}

		// レイヤー1: CARGO_MANIFEST_DIR を使用
		if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
			let candidate = PathBuf::from(manifest_dir).join(&path);
			if candidate.exists() {
				return candidate;
			}
		}

		// レイヤー2: Cargo.toml を検索してプロジェクトルートを見つける
		if let Some(project_root) = Self::find_project_root() {
			let candidate = project_root.join(&path);
			if candidate.exists() {
				return candidate;
			}
		}

		// レイヤー3: 現在のディレクトリから解決（既存の動作）
		env::current_dir()
			.ok()
			.and_then(|cwd| {
				let candidate = cwd.join(&path);
				if candidate.exists() {
					Some(candidate)
				} else {
					None
				}
			})
			.unwrap_or(path)
	}

	/// 現在のディレクトリから上に向かって Cargo.toml を検索
	///
	/// Cargo.toml が見つかった場合、そのディレクトリがプロジェクトルートとして返される。
	///
	/// # Returns
	///
	/// プロジェクトルートのパス（見つからない場合は None）
	fn find_project_root() -> Option<PathBuf> {
		let mut current = env::current_dir().ok()?;

		loop {
			let cargo_toml = current.join("Cargo.toml");
			if cargo_toml.exists() {
				// Cargo.toml が [[bin]] セクションまたは [package] セクションを持つか確認
				// （これがプロジェクトルートであることを示す）
				if let Ok(content) = std::fs::read_to_string(&cargo_toml)
					&& (content.contains("[[bin]]") || content.contains("[package]"))
				{
					return Some(current);
				}
			}

			// 親ディレクトリに移動
			if !current.pop() {
				break;
			}
		}

		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_absolute_path_unchanged() {
		let absolute = "/tmp/static";
		let resolved = PathResolver::resolve_static_dir(absolute);
		assert_eq!(resolved, PathBuf::from(absolute));
	}

	#[test]
	fn test_relative_path_resolution() {
		// テスト環境によって結果は異なるが、パニックしないことを確認
		let resolved = PathResolver::resolve_static_dir("dist");
		assert!(resolved.to_string_lossy().contains("dist"));
	}
}
