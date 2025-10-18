//! Task worker

use crate::TaskBackend;

#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub name: String,
    pub concurrency: usize,
}

impl WorkerConfig {
    pub fn new(name: String) -> Self {
        Self {
            name,
            concurrency: 4,
        }
    }
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self::new("worker".to_string())
    }
}

pub struct Worker {
    #[allow(dead_code)]
    config: WorkerConfig,
}

impl Worker {
    pub fn new(config: WorkerConfig) -> Self {
        Self { config }
    }

    pub async fn start(&self, _backend: &dyn TaskBackend) {
        // Worker logic
    }

    pub async fn stop(&self) {
        // Shutdown logic
    }
}

impl Default for Worker {
    fn default() -> Self {
        Self::new(WorkerConfig::default())
    }
}
