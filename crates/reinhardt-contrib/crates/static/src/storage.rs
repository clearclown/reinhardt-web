use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Storage trait for static files
pub trait Storage {
    fn save(&self, name: &str, content: &[u8]) -> io::Result<String>;
    fn exists(&self, name: &str) -> bool;
    fn open(&self, name: &str) -> io::Result<Vec<u8>>;
    fn delete(&self, name: &str) -> io::Result<()>;
    fn url(&self, name: &str) -> String;
}

pub struct FileSystemStorage {
    pub location: PathBuf,
    pub base_url: String,
}

impl FileSystemStorage {
    pub fn new<P: Into<PathBuf>>(location: P, base_url: &str) -> Self {
        Self {
            location: location.into(),
            base_url: base_url.to_string(),
        }
    }

    fn normalize_path(&self, name: &str) -> PathBuf {
        let name = name.trim_start_matches('/');
        self.location.join(name)
    }

    fn normalize_url(&self, base: &str, name: &str) -> String {
        let base = base.trim_end_matches('/');
        let name = name.trim_start_matches('/');
        format!("{}/{}", base, name)
    }
}

impl Storage for FileSystemStorage {
    fn save(&self, name: &str, content: &[u8]) -> io::Result<String> {
        let file_path = self.normalize_path(name);

        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, content)?;
        Ok(self.url(name))
    }

    fn exists(&self, name: &str) -> bool {
        self.normalize_path(name).exists()
    }

    fn open(&self, name: &str) -> io::Result<Vec<u8>> {
        fs::read(self.normalize_path(name))
    }

    fn delete(&self, name: &str) -> io::Result<()> {
        let file_path = self.normalize_path(name);
        if file_path.exists() {
            fs::remove_file(file_path)?;
        }
        Ok(())
    }

    fn url(&self, name: &str) -> String {
        self.normalize_url(&self.base_url, name)
    }
}

pub struct MemoryStorage {
    base_url: String,
    files: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

impl MemoryStorage {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            files: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn normalize_url(&self, base: &str, name: &str) -> String {
        let base = base.trim_end_matches('/');
        let name = name.trim_start_matches('/');
        format!("{}/{}", base, name)
    }
}

impl Storage for MemoryStorage {
    fn save(&self, name: &str, content: &[u8]) -> io::Result<String> {
        let mut files = self.files.write().unwrap();
        files.insert(name.to_string(), content.to_vec());
        Ok(self.url(name))
    }

    fn exists(&self, name: &str) -> bool {
        self.files.read().unwrap().contains_key(name)
    }

    fn open(&self, name: &str) -> io::Result<Vec<u8>> {
        self.files
            .read()
            .unwrap()
            .get(name)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }

    fn delete(&self, name: &str) -> io::Result<()> {
        self.files.write().unwrap().remove(name);
        Ok(())
    }

    fn url(&self, name: &str) -> String {
        self.normalize_url(&self.base_url, name)
    }
}

impl Default for MemoryStorage {
    fn default() -> Self {
        Self::new("/static/")
    }
}

#[derive(Debug, Clone)]
pub struct StaticFilesConfig {
    pub static_root: PathBuf,
    pub static_url: String,
    pub staticfiles_dirs: Vec<PathBuf>,
    pub media_url: Option<String>,
}

impl Default for StaticFilesConfig {
    fn default() -> Self {
        Self {
            static_root: PathBuf::from("static"),
            static_url: "/static/".to_string(),
            staticfiles_dirs: Vec::new(),
            media_url: None,
        }
    }
}

pub struct StaticFilesFinder {
    pub directories: Vec<PathBuf>,
}

impl StaticFilesFinder {
    pub fn new(directories: Vec<PathBuf>) -> Self {
        Self { directories }
    }

    pub fn find(&self, path: &str) -> Result<PathBuf, io::Error> {
        for dir in &self.directories {
            let file_path = dir.join(path);
            if file_path.exists() {
                return Ok(file_path);
            }
        }
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File not found in any directory: {}", path),
        ))
    }
}

pub struct HashedFileStorage {
    pub location: PathBuf,
    pub base_url: String,
}

impl HashedFileStorage {
    pub fn new<P: Into<PathBuf>>(location: P, base_url: &str) -> Self {
        Self {
            location: location.into(),
            base_url: base_url.to_string(),
        }
    }
}

pub struct ManifestStaticFilesStorage {
    pub location: PathBuf,
    pub base_url: String,
    pub manifest_name: String,
    pub manifest_strict: bool,
}

impl ManifestStaticFilesStorage {
    pub fn new<P: Into<PathBuf>>(location: P, base_url: &str) -> Self {
        Self {
            location: location.into(),
            base_url: base_url.to_string(),
            manifest_name: "staticfiles.json".to_string(),
            manifest_strict: true,
        }
    }

    pub fn with_manifest_strict(mut self, strict: bool) -> Self {
        self.manifest_strict = strict;
        self
    }
}
