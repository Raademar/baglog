use crate::message::BagLogMessage;

pub struct BagLogConfig {
    pub date_format: String,
    pub out_file: String,
    pub write_to_terminal: bool,
    pub write_to_file: bool,
}

impl Default for BagLogConfig {
    fn default() -> Self {
        Self {
            date_format: String::from("%Y-%m-%d %H:%M:%S"),
            out_file: String::from("log.log"),
            write_to_terminal: true,
            write_to_file: true,
        }
    }
}
impl BagLogConfig {
    pub(crate) fn format_message(&self, bag_message: &BagLogMessage) -> String {
        let date = &bag_message.date.format(&self.date_format);
        let msg = &bag_message.message;

        format!("[{date}]: {msg}")
    }
}
