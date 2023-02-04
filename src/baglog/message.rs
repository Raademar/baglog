use chrono::{DateTime, Local};

pub struct BagLogMessage {
    pub message: String,
    pub date: DateTime<Local>,
}
