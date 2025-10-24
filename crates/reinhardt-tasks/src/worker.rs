//! Task worker

use crate::{TaskBackend, TaskStatus};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::sleep;

/// Worker configuration
///
/// Controls worker behavior including name, concurrency, and polling interval.
///
/// # Examples
///
/// ```rust
/// use reinhardt_tasks::WorkerConfig;
/// use std::time::Duration;
///
/// let config = WorkerConfig::new("my-worker".to_string())
///     .with_concurrency(8)
///     .with_poll_interval(Duration::from_millis(100));
///
/// assert_eq!(config.name, "my-worker");
/// assert_eq!(config.concurrency, 8);
/// ```
#[derive(Debug, Clone)]
pub struct WorkerConfig {
    pub name: String,
    pub concurrency: usize,
    pub poll_interval: Duration,
}

impl WorkerConfig {
    /// Create a new worker configuration with default values
    ///
    /// # Examples
    ///
    /// ```rust
    /// use reinhardt_tasks::WorkerConfig;
    ///
    /// let config = WorkerConfig::new("worker-1".to_string());
    /// assert_eq!(config.name, "worker-1");
    /// assert_eq!(config.concurrency, 4);
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            concurrency: 4,
            poll_interval: Duration::from_secs(1),
        }
    }

    /// Set the concurrency level
    ///
    /// # Examples
    ///
    /// ```rust
    /// use reinhardt_tasks::WorkerConfig;
    ///
    /// let config = WorkerConfig::new("worker".to_string()).with_concurrency(8);
    /// assert_eq!(config.concurrency, 8);
    /// ```
    pub fn with_concurrency(mut self, concurrency: usize) -> Self {
        self.concurrency = concurrency;
        self
    }

    /// Set the poll interval
    ///
    /// # Examples
    ///
    /// ```rust
    /// use reinhardt_tasks::WorkerConfig;
    /// use std::time::Duration;
    ///
    /// let config = WorkerConfig::new("worker".to_string())
    ///     .with_poll_interval(Duration::from_millis(500));
    /// assert_eq!(config.poll_interval, Duration::from_millis(500));
    /// ```
    pub fn with_poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self::new("worker".to_string())
    }
}

/// Task worker
///
/// Polls the backend for tasks and executes them concurrently.
///
/// # Examples
///
/// ```rust,no_run
/// use reinhardt_tasks::{Worker, WorkerConfig, DummyBackend};
/// use std::sync::Arc;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let config = WorkerConfig::new("worker-1".to_string());
/// let worker = Worker::new(config);
/// let backend = Arc::new(DummyBackend::new());
///
/// // Start worker in background
/// let handle = tokio::spawn(async move {
///     worker.run(backend).await
/// });
///
/// // Later: stop the worker
/// handle.abort();
/// # Ok(())
/// # }
/// ```
pub struct Worker {
    config: WorkerConfig,
    shutdown_tx: broadcast::Sender<()>,
}

impl Worker {
    /// Create a new worker
    ///
    /// # Examples
    ///
    /// ```rust
    /// use reinhardt_tasks::{Worker, WorkerConfig};
    ///
    /// let config = WorkerConfig::new("worker-1".to_string());
    /// let worker = Worker::new(config);
    /// ```
    pub fn new(config: WorkerConfig) -> Self {
        let (shutdown_tx, _) = broadcast::channel(1);
        Self {
            config,
            shutdown_tx,
        }
    }

    /// Run the worker loop
    ///
    /// This method blocks until the worker is stopped via `stop()`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use reinhardt_tasks::{Worker, WorkerConfig, DummyBackend};
    /// use std::sync::Arc;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let worker = Worker::new(WorkerConfig::default());
    /// let backend = Arc::new(DummyBackend::new());
    ///
    /// worker.run(backend).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run(
        &self,
        backend: Arc<dyn TaskBackend>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut shutdown_rx = self.shutdown_tx.subscribe();

        println!(
            "[{}] Worker started with concurrency {}",
            self.config.name, self.config.concurrency
        );

        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    println!("[{}] Shutdown signal received", self.config.name);
                    break;
                }
                _ = self.process_tasks(backend.clone()) => {}
            }
        }

        println!("[{}] Worker stopped", self.config.name);
        Ok(())
    }

    /// Process tasks from the backend
    async fn process_tasks(&self, backend: Arc<dyn TaskBackend>) {
        match backend.dequeue().await {
            Ok(Some(task_id)) => {
                println!("[{}] Processing task: {}", self.config.name, task_id);

                // Execute task (placeholder - actual execution would happen here)
                match self.execute_task(task_id, backend.clone()).await {
                    Ok(_) => {
                        println!(
                            "[{}] Task {} completed successfully",
                            self.config.name, task_id
                        );
                        if let Err(e) = backend.update_status(task_id, TaskStatus::Success).await {
                            eprintln!(
                                "[{}] Failed to update task {} status: {}",
                                self.config.name, task_id, e
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("[{}] Task {} failed: {}", self.config.name, task_id, e);
                        if let Err(e) = backend.update_status(task_id, TaskStatus::Failure).await {
                            eprintln!(
                                "[{}] Failed to update task {} status: {}",
                                self.config.name, task_id, e
                            );
                        }
                    }
                }
            }
            Ok(None) => {
                // No tasks available, wait before polling again
                sleep(self.config.poll_interval).await;
            }
            Err(e) => {
                eprintln!("[{}] Failed to dequeue task: {}", self.config.name, e);
                sleep(self.config.poll_interval).await;
            }
        }
    }

    /// Execute a task
    async fn execute_task(
        &self,
        task_id: crate::TaskId,
        _backend: Arc<dyn TaskBackend>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Placeholder for actual task execution
        // In a real implementation, this would:
        // 1. Deserialize task data
        // 2. Call the task's execute method
        // 3. Store the result
        println!("[{}] Executing task: {}", self.config.name, task_id);
        Ok(())
    }

    /// Stop the worker
    ///
    /// Sends a shutdown signal to all worker loops.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use reinhardt_tasks::{Worker, WorkerConfig};
    ///
    /// # async fn example() {
    /// let worker = Worker::new(WorkerConfig::default());
    /// worker.stop().await;
    /// # }
    /// ```
    pub async fn stop(&self) {
        let _ = self.shutdown_tx.send(());
    }
}

impl Default for Worker {
    fn default() -> Self {
        Self::new(WorkerConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DummyBackend, Task, TaskId, TaskPriority};
    use std::time::Duration;

    #[allow(dead_code)]
    struct TestTask {
        id: TaskId,
        name: String,
    }

    impl Task for TestTask {
        fn id(&self) -> TaskId {
            self.id
        }

        fn name(&self) -> &str {
            &self.name
        }

        fn priority(&self) -> TaskPriority {
            TaskPriority::new(5)
        }
    }

    #[tokio::test]
    async fn test_worker_creation() {
        let config = WorkerConfig::new("test-worker".to_string());
        let worker = Worker::new(config);
        assert_eq!(worker.config.name, "test-worker");
    }

    #[tokio::test]
    async fn test_worker_config_builder() {
        let config = WorkerConfig::new("test".to_string())
            .with_concurrency(8)
            .with_poll_interval(Duration::from_millis(100));

        assert_eq!(config.concurrency, 8);
        assert_eq!(config.poll_interval, Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_worker_start_and_stop() {
        let worker = Worker::new(WorkerConfig::default());
        let backend = Arc::new(DummyBackend::new());
        let worker_clone = Worker {
            config: worker.config.clone(),
            shutdown_tx: worker.shutdown_tx.clone(),
        };

        let handle = tokio::spawn(async move { worker.run(backend).await });

        // Give worker time to start
        sleep(Duration::from_millis(100)).await;

        // Stop worker
        worker_clone.stop().await;

        // Wait for worker to finish
        let result = tokio::time::timeout(Duration::from_secs(2), handle).await;
        assert!(result.is_ok());
    }
}
