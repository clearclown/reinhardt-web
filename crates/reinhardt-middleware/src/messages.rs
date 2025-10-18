//! Messages middleware

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageLevel {
    Debug,
    Info,
    Success,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub level: MessageLevel,
    pub text: String,
}

impl Message {
    pub fn new(level: MessageLevel, text: String) -> Self {
        Self { level, text }
    }
}

pub trait MessageStorage {
    fn add_message(&mut self, message: Message);
    fn get_messages(&self) -> Vec<Message>;
}

pub struct SessionStorage;

impl SessionStorage {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SessionStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageStorage for SessionStorage {
    fn add_message(&mut self, _message: Message) {}
    fn get_messages(&self) -> Vec<Message> {
        Vec::new()
    }
}

pub struct CookieStorage;

impl CookieStorage {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CookieStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageStorage for CookieStorage {
    fn add_message(&mut self, _message: Message) {}
    fn get_messages(&self) -> Vec<Message> {
        Vec::new()
    }
}
