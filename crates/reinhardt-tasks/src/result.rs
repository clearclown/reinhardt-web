//! Task result types

use crate::TaskId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskOutput {
    pub task_id: TaskId,
    pub result: String,
}

impl TaskOutput {
    pub fn new(task_id: TaskId, result: String) -> Self {
        Self { task_id, result }
    }
}

pub type TaskResult = Result<TaskOutput, String>;
