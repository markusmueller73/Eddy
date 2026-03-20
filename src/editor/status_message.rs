use std::time::{Duration, Instant};

pub struct StatusMessage {
    message: String,
    start: Instant,
    delay: Duration,
}

impl StatusMessage {
    pub fn new(message: &str) -> StatusMessage {
        StatusMessage {
            message: message.to_string(),
            start: Instant::now(),
            delay: Duration::from_millis(5000),
        }
    }
    pub fn get(&self) -> &str {
        &self.message
    }
    pub fn is_expired(&self) -> bool {
        self.start.elapsed() > self.delay
    }
}
