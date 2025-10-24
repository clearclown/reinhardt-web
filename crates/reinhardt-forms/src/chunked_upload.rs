//! Chunked upload handling for large files
//!
//! This module provides functionality for handling large file uploads
//! by splitting them into manageable chunks, supporting resumable uploads,
//! and assembling chunks back into complete files.

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Errors that can occur during chunked upload
#[derive(Debug, thiserror::Error)]
pub enum ChunkedUploadError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
    #[error("Upload session not found: {0}")]
    SessionNotFound(String),
    #[error("Invalid chunk: expected {expected}, got {actual}")]
    InvalidChunk { expected: usize, actual: usize },
    #[error("Chunk out of order: expected {expected}, got {actual}")]
    ChunkOutOfOrder { expected: usize, actual: usize },
    #[error("Upload already completed")]
    AlreadyCompleted,
    #[error("Checksum mismatch")]
    ChecksumMismatch,
}

/// Metadata for a chunked upload session
#[derive(Debug, Clone)]
pub struct ChunkedUploadSession {
    /// Unique session ID
    pub session_id: String,
    /// Original filename
    pub filename: String,
    /// Total file size in bytes
    pub total_size: usize,
    /// Chunk size in bytes
    pub chunk_size: usize,
    /// Total number of chunks
    pub total_chunks: usize,
    /// Number of chunks received so far
    pub received_chunks: usize,
    /// Temporary directory for chunks
    pub temp_dir: PathBuf,
    /// Whether the upload is complete
    pub completed: bool,
}

impl ChunkedUploadSession {
    /// Create a new upload session
    pub fn new(
        session_id: String,
        filename: String,
        total_size: usize,
        chunk_size: usize,
        temp_dir: PathBuf,
    ) -> Self {
        let total_chunks = (total_size + chunk_size - 1) / chunk_size;
        Self {
            session_id,
            filename,
            total_size,
            chunk_size,
            total_chunks,
            received_chunks: 0,
            temp_dir,
            completed: false,
        }
    }

    /// Get progress percentage
    pub fn progress(&self) -> f64 {
        if self.total_chunks == 0 {
            0.0
        } else {
            (self.received_chunks as f64 / self.total_chunks as f64) * 100.0
        }
    }

    /// Check if upload is complete
    pub fn is_complete(&self) -> bool {
        self.completed || self.received_chunks >= self.total_chunks
    }

    /// Get the path for a specific chunk
    fn chunk_path(&self, chunk_number: usize) -> PathBuf {
        self.temp_dir
            .join(format!("{}_{}.chunk", self.session_id, chunk_number))
    }
}

/// Manager for chunked uploads
pub struct ChunkedUploadManager {
    sessions: Arc<Mutex<HashMap<String, ChunkedUploadSession>>>,
    temp_base_dir: PathBuf,
}

impl ChunkedUploadManager {
    /// Create a new chunked upload manager
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::ChunkedUploadManager;
    /// use std::path::PathBuf;
    ///
    /// let manager = ChunkedUploadManager::new(PathBuf::from("/tmp/chunked_uploads"));
    /// ```
    pub fn new(temp_base_dir: PathBuf) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            temp_base_dir,
        }
    }

    /// Start a new upload session
    ///
    /// # Examples
    ///
    /// ```
    /// use reinhardt_forms::ChunkedUploadManager;
    /// use std::path::PathBuf;
    ///
    /// let manager = ChunkedUploadManager::new(PathBuf::from("/tmp/chunked_uploads"));
    /// let session = manager.start_session(
    ///     "session123".to_string(),
    ///     "large_file.bin".to_string(),
    ///     10_000_000, // 10MB
    ///     1_000_000,  // 1MB chunks
    /// ).unwrap();
    /// assert_eq!(session.total_chunks, 10);
    /// ```
    pub fn start_session(
        &self,
        session_id: String,
        filename: String,
        total_size: usize,
        chunk_size: usize,
    ) -> Result<ChunkedUploadSession, ChunkedUploadError> {
        let temp_dir = self.temp_base_dir.join(&session_id);
        fs::create_dir_all(&temp_dir)?;

        let session = ChunkedUploadSession::new(
            session_id.clone(),
            filename,
            total_size,
            chunk_size,
            temp_dir,
        );

        let mut sessions = self.sessions.lock().unwrap();
        sessions.insert(session_id, session.clone());

        Ok(session)
    }

    /// Upload a chunk
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use reinhardt_forms::ChunkedUploadManager;
    /// use std::path::PathBuf;
    ///
    /// let manager = ChunkedUploadManager::new(PathBuf::from("/tmp/chunked_uploads"));
    /// manager.start_session("session123".to_string(), "file.bin".to_string(), 1000, 100).unwrap();
    ///
    /// let chunk_data = vec![0u8; 100];
    /// manager.upload_chunk("session123", 0, &chunk_data).unwrap();
    /// ```
    pub fn upload_chunk(
        &self,
        session_id: &str,
        chunk_number: usize,
        data: &[u8],
    ) -> Result<ChunkedUploadSession, ChunkedUploadError> {
        let mut sessions = self.sessions.lock().unwrap();
        let session = sessions
            .get_mut(session_id)
            .ok_or_else(|| ChunkedUploadError::SessionNotFound(session_id.to_string()))?;

        if session.completed {
            return Err(ChunkedUploadError::AlreadyCompleted);
        }

        // Validate chunk number
        if chunk_number >= session.total_chunks {
            return Err(ChunkedUploadError::InvalidChunk {
                expected: session.total_chunks - 1,
                actual: chunk_number,
            });
        }

        // Write chunk to disk
        let chunk_path = session.chunk_path(chunk_number);
        let mut file = File::create(chunk_path)?;
        file.write_all(data)?;

        session.received_chunks += 1;

        if session.is_complete() {
            session.completed = true;
        }

        Ok(session.clone())
    }

    /// Assemble all chunks into final file
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use reinhardt_forms::ChunkedUploadManager;
    /// use std::path::PathBuf;
    ///
    /// let manager = ChunkedUploadManager::new(PathBuf::from("/tmp/chunked_uploads"));
    /// let output_path = manager.assemble_chunks("session123", PathBuf::from("/tmp/final_file.bin")).unwrap();
    /// ```
    pub fn assemble_chunks(
        &self,
        session_id: &str,
        output_path: PathBuf,
    ) -> Result<PathBuf, ChunkedUploadError> {
        let sessions = self.sessions.lock().unwrap();
        let session = sessions
            .get(session_id)
            .ok_or_else(|| ChunkedUploadError::SessionNotFound(session_id.to_string()))?;

        if !session.is_complete() {
            return Err(ChunkedUploadError::InvalidChunk {
                expected: session.total_chunks,
                actual: session.received_chunks,
            });
        }

        // Create output file
        let mut output_file = File::create(&output_path)?;

        // Assemble chunks in order
        for i in 0..session.total_chunks {
            let chunk_path = session.chunk_path(i);
            let chunk_data = fs::read(&chunk_path)?;
            output_file.write_all(&chunk_data)?;
        }

        Ok(output_path)
    }

    /// Clean up a session (delete temporary files)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use reinhardt_forms::ChunkedUploadManager;
    /// use std::path::PathBuf;
    ///
    /// let manager = ChunkedUploadManager::new(PathBuf::from("/tmp/chunked_uploads"));
    /// manager.cleanup_session("session123").unwrap();
    /// ```
    pub fn cleanup_session(&self, session_id: &str) -> Result<(), ChunkedUploadError> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.remove(session_id) {
            if session.temp_dir.exists() {
                fs::remove_dir_all(session.temp_dir)?;
            }
        }
        Ok(())
    }

    /// Get session information
    pub fn get_session(&self, session_id: &str) -> Option<ChunkedUploadSession> {
        let sessions = self.sessions.lock().unwrap();
        sessions.get(session_id).cloned()
    }

    /// List all active sessions
    pub fn list_sessions(&self) -> Vec<ChunkedUploadSession> {
        let sessions = self.sessions.lock().unwrap();
        sessions.values().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let session = ChunkedUploadSession::new(
            "test123".to_string(),
            "file.bin".to_string(),
            1000,
            100,
            PathBuf::from("/tmp"),
        );

        assert_eq!(session.session_id, "test123");
        assert_eq!(session.filename, "file.bin");
        assert_eq!(session.total_size, 1000);
        assert_eq!(session.chunk_size, 100);
        assert_eq!(session.total_chunks, 10);
        assert_eq!(session.received_chunks, 0);
        assert!(!session.completed);
    }

    #[test]
    fn test_session_progress() {
        let mut session = ChunkedUploadSession::new(
            "test123".to_string(),
            "file.bin".to_string(),
            1000,
            100,
            PathBuf::from("/tmp"),
        );

        assert_eq!(session.progress(), 0.0);

        session.received_chunks = 5;
        assert_eq!(session.progress(), 50.0);

        session.received_chunks = 10;
        assert_eq!(session.progress(), 100.0);
    }

    #[test]
    fn test_manager_creation() {
        let manager = ChunkedUploadManager::new(PathBuf::from("/tmp/test_chunks"));
        assert_eq!(manager.list_sessions().len(), 0);
    }

    #[test]
    fn test_start_session() {
        let manager = ChunkedUploadManager::new(PathBuf::from("/tmp/test_chunks"));
        let session = manager
            .start_session("session1".to_string(), "file.bin".to_string(), 1000, 100)
            .unwrap();

        assert_eq!(session.session_id, "session1");
        assert_eq!(session.total_chunks, 10);
        assert_eq!(manager.list_sessions().len(), 1);
    }

    #[test]
    fn test_upload_chunk() {
        let temp_dir = PathBuf::from("/tmp/test_chunks_upload");
        let manager = ChunkedUploadManager::new(temp_dir.clone());

        manager
            .start_session("session2".to_string(), "file.bin".to_string(), 300, 100)
            .unwrap();

        let chunk_data = vec![0u8; 100];
        let result = manager.upload_chunk("session2", 0, &chunk_data);
        assert!(result.is_ok());

        let session = manager.get_session("session2").unwrap();
        assert_eq!(session.received_chunks, 1);

        manager.cleanup_session("session2").unwrap();
    }

    #[test]
    fn test_invalid_session() {
        let manager = ChunkedUploadManager::new(PathBuf::from("/tmp/test_chunks"));
        let chunk_data = vec![0u8; 100];
        let result = manager.upload_chunk("nonexistent", 0, &chunk_data);

        assert!(result.is_err());
        if let Err(ChunkedUploadError::SessionNotFound(id)) = result {
            assert_eq!(id, "nonexistent");
        } else {
            panic!("Expected SessionNotFound error");
        }
    }

    #[test]
    fn test_chunk_assembly() {
        let temp_dir = PathBuf::from("/tmp/test_chunks_assembly");
        let manager = ChunkedUploadManager::new(temp_dir.clone());

        manager
            .start_session("session3".to_string(), "file.bin".to_string(), 300, 100)
            .unwrap();

        // Upload 3 chunks
        for i in 0..3 {
            let chunk_data = vec![i as u8; 100];
            manager.upload_chunk("session3", i, &chunk_data).unwrap();
        }

        let output_path = temp_dir.join("assembled.bin");
        let result = manager.assemble_chunks("session3", output_path.clone());
        assert!(result.is_ok());

        assert!(output_path.exists());
        let content = fs::read(&output_path).unwrap();
        assert_eq!(content.len(), 300);

        // Cleanup
        fs::remove_file(output_path).unwrap();
        manager.cleanup_session("session3").unwrap();
    }

    #[test]
    fn test_session_completion() {
        let temp_dir = PathBuf::from("/tmp/test_chunks_completion");
        let manager = ChunkedUploadManager::new(temp_dir.clone());

        manager
            .start_session("session4".to_string(), "file.bin".to_string(), 200, 100)
            .unwrap();

        let chunk_data = vec![0u8; 100];

        manager.upload_chunk("session4", 0, &chunk_data).unwrap();
        let session = manager.get_session("session4").unwrap();
        assert!(!session.is_complete());

        manager.upload_chunk("session4", 1, &chunk_data).unwrap();
        let session = manager.get_session("session4").unwrap();
        assert!(session.is_complete());

        manager.cleanup_session("session4").unwrap();
    }
}
