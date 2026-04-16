use std::time::{Duration, Instant};

pub const SEARCH_TEXT: &str = "Search:";
pub const LOAD_FILE_TEXT: &str = "Filename:";
pub const SAVE_FILE_TEXT: &str = "Set new filename:";

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
    pub fn time(&self) -> Instant {
        self.start
    }
    pub fn is_expired(&self) -> bool {
        self.start.elapsed() > self.delay
    }
}
