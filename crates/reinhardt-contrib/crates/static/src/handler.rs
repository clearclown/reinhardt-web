use mime_guess::from_path;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum StaticError {
    #[error("File not found: {0}")]
    NotFound(String),
    #[error("Directory traversal blocked: {0}")]
    DirectoryTraversal(String),
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

#[derive(Debug)]
pub struct StaticFile {
    pub content: Vec<u8>,
    pub path: PathBuf,
    pub mime_type: String,
}

impl StaticFile {
    /// Generate ETag for the file based on content hash
    pub fn etag(&self) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        self.content.hash(&mut hasher);
        format!("\"{}\"", hasher.finish())
    }
}

pub struct StaticFileHandler {
    root: PathBuf,
}

impl StaticFileHandler {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    pub async fn serve(&self, path: &str) -> Result<StaticFile, StaticError> {
        let resolved = self.resolve_path(path).await?;
        let content = fs::read(&resolved)?;
        let mime_type = from_path(&resolved).first_or_octet_stream().to_string();

        Ok(StaticFile {
            content,
            path: resolved,
            mime_type,
        })
    }

    pub async fn resolve_path(&self, path: &str) -> Result<PathBuf, StaticError> {
        let path = path.trim_start_matches('/');

        // Prevent directory traversal attacks
        if path.contains("..") {
            return Err(StaticError::DirectoryTraversal(path.to_string()));
        }

        let file_path = self.root.join(path);

        // Canonicalize paths to prevent traversal
        let canonical_file = file_path
            .canonicalize()
            .map_err(|_| StaticError::NotFound(path.to_string()))?;
        let canonical_root = self.root.canonicalize().map_err(|e| StaticError::Io(e))?;

        // Ensure file is within root directory
        if !canonical_file.starts_with(&canonical_root) {
            return Err(StaticError::DirectoryTraversal(path.to_string()));
        }

        Ok(canonical_file)
    }
}

pub type StaticResult<T> = Result<T, StaticError>;
