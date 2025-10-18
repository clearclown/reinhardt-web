use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[async_trait::async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, record: &LogRecord);
    fn level(&self) -> LogLevel;
    fn set_level(&mut self, level: LogLevel);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone)]
pub struct LogRecord {
    pub level: LogLevel,
    pub logger_name: String,
    pub message: String,
    pub extra: HashMap<String, serde_json::Value>,
}

impl LogRecord {
    pub fn new(level: LogLevel, logger_name: String, message: String) -> Self {
        Self {
            level,
            logger_name,
            message,
            extra: HashMap::new(),
        }
    }
}

pub struct Logger {
    name: String,
    handlers: Arc<Mutex<Vec<Box<dyn Handler>>>>,
    level: Arc<Mutex<LogLevel>>,
}

impl Logger {
    pub fn new(name: String) -> Self {
        Self {
            name,
            handlers: Arc::new(Mutex::new(Vec::new())),
            level: Arc::new(Mutex::new(LogLevel::Debug)),
        }
    }

    pub async fn add_handler(&self, handler: Box<dyn Handler>) {
        self.handlers.lock().unwrap().push(handler);
    }

    pub async fn set_level(&self, level: LogLevel) {
        *self.level.lock().unwrap() = level;
    }

    pub async fn log_record(&self, record: &LogRecord) {
        let num_handlers = {
            let handlers_guard = self.handlers.lock().unwrap();
            handlers_guard.len()
        };

        for i in 0..num_handlers {
            let handlers_guard = self.handlers.lock().unwrap();
            if let Some(handler) = handlers_guard.get(i) {
                handler.handle(record).await;
            }
            drop(handlers_guard);
        }
    }

    async fn log(&self, level: LogLevel, message: String) {
        let current_level = *self.level.lock().unwrap();
        if level < current_level {
            return;
        }

        let record = LogRecord::new(level, self.name.clone(), message);

        // Create a scoped block to release the lock before awaiting
        let handlers_guard = self.handlers.lock().unwrap();
        let num_handlers = handlers_guard.len();
        drop(handlers_guard);

        // Process handlers one by one
        for i in 0..num_handlers {
            let handlers_guard = self.handlers.lock().unwrap();
            if let Some(handler) = handlers_guard.get(i) {
                // We can't store a reference across await, so we need to handle this differently
                // For now, we'll just handle the record directly without storing references
                handler.handle(&record).await;
            }
            drop(handlers_guard);
        }
    }

    pub async fn debug(&self, message: String) {
        self.log(LogLevel::Debug, message).await;
    }

    pub async fn info(&self, message: String) {
        self.log(LogLevel::Info, message).await;
    }

    pub async fn warning(&self, message: String) {
        self.log(LogLevel::Warning, message).await;
    }

    pub fn warning_sync(&self, message: &str) {
        // Synchronous version for compatibility
        let _ = message;
        todo!("Implement synchronous warning")
    }

    pub async fn error(&self, message: String) {
        self.log(LogLevel::Error, message).await;
    }
}
