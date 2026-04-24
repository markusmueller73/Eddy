// Part of Eddy - A lightweight text editor for the terminal.
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct StatusMessage {
    message: String,
    start: Instant,
    delay: Duration,
}

impl Default for StatusMessage {
    fn default() -> Self {
        Self {
            message: String::new(),
            start: Instant::now(),
            delay: Duration::from_millis(1),
        }
    }
}

impl StatusMessage {
    pub fn new(message: &str, delay: Duration) -> StatusMessage {
        StatusMessage {
            message: message.to_string(),
            start: Instant::now(),
            delay,
        }
    }
    pub fn get(&self) -> &str {
        &self.message
    }
    pub fn is_expired(&self) -> bool {
        self.start.elapsed() > self.delay
    }
}
