//! Task backend implementations

use crate::{Task, TaskId, TaskStatus};
use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TaskExecutionError {
    #[error("Task execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Task not found: {0}")]
    NotFound(TaskId),

    #[error("Backend error: {0}")]
    BackendError(String),
}

pub type ResultStatus = TaskStatus;
pub type TaskResultStatus = TaskStatus;

#[async_trait]
pub trait TaskBackend: Send + Sync {
    async fn enqueue(&self, task: Box<dyn Task>) -> Result<TaskId, TaskExecutionError>;
    async fn get_status(&self, task_id: TaskId) -> Result<TaskStatus, TaskExecutionError>;
    fn backend_name(&self) -> &str;
}

pub struct TaskBackends;

impl TaskBackends {
    pub fn new() -> Self {
        Self
    }
}

impl Default for TaskBackends {
    fn default() -> Self {
        Self::new()
    }
}

pub struct DummyBackend;

impl DummyBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DummyBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TaskBackend for DummyBackend {
    async fn enqueue(&self, _task: Box<dyn Task>) -> Result<TaskId, TaskExecutionError> {
        Ok(TaskId::new())
    }

    async fn get_status(&self, _task_id: TaskId) -> Result<TaskStatus, TaskExecutionError> {
        Ok(TaskStatus::Success)
    }

    fn backend_name(&self) -> &str {
        "dummy"
    }
}

pub struct ImmediateBackend;

impl ImmediateBackend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ImmediateBackend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl TaskBackend for ImmediateBackend {
    async fn enqueue(&self, _task: Box<dyn Task>) -> Result<TaskId, TaskExecutionError> {
        Ok(TaskId::new())
    }

    async fn get_status(&self, _task_id: TaskId) -> Result<TaskStatus, TaskExecutionError> {
        Ok(TaskStatus::Success)
    }

    fn backend_name(&self) -> &str {
        "immediate"
    }
}
