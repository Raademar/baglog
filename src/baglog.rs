pub mod message;

pub mod baglog {

    use chrono::{DateTime, Local};
    use std::{
        fs::{File, OpenOptions},
        io::{ErrorKind, Write},
    };

    use super::message::BagLogMessage;

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

    pub struct BagLog {
        config: BagLogConfig,
    }

    impl BagLog {
        pub fn new(config: Option<BagLogConfig>) -> Self {
            match config {
                Some(cfg) => BagLog { config: { cfg } },
                None => BagLog {
                    config: { BagLogConfig::default() },
                },
            }
        }

        pub fn get_config(&self) -> &BagLogConfig {
            return &self.config;
        }

        pub fn write(
            &self,
            message: String,
            mut writer: impl std::io::Write,
        ) -> std::io::Result<()> {
            let date: DateTime<Local> = Local::now();
            let bag_message = BagLogMessage { date, message };

            if self.config.write_to_terminal {
                let log_res = self.write_to_terminal(&bag_message, &mut writer);
                match log_res {
                    Err(err) => panic!("Something went wrong when writing to terminal: {}", err),
                    Ok(_) => (),
                }
            }

            if self.config.write_to_file {
                return self.write_to_file(&bag_message);
            } else {
                return Ok(());
            }
        }

        fn format_message(&self, bag_message: &BagLogMessage) -> String {
            let date = &bag_message.date.format(&self.config.date_format);
            let msg = &bag_message.message;

            return format!("[{date}]: {msg}");
        }

        pub fn write_to_terminal(
            &self,
            bag_message: &BagLogMessage,
            mut writer: &mut impl std::io::Write,
        ) -> Result<(), std::io::Error> {
            let msg = self.format_message(bag_message);
            writeln!(&mut writer, "{}", msg)
        }

        fn write_to_file(&self, bag_message: &BagLogMessage) -> std::io::Result<()> {
            let file_to_write = OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(&self.config.out_file);

            let mut log_file = match file_to_write {
                Ok(file) => file,
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => match File::create(&self.config.out_file) {
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem creating the file: {:?}", e),
                    },
                    other_error => {
                        panic!("Something went wrong when opening file: {:?}", other_error);
                    }
                },
            };
            let msg = &self.format_message(bag_message);
            log_file.write_all(msg.as_bytes())?;

            Ok(())
        }
    }
}
