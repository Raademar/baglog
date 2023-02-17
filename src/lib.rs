pub mod config;
pub mod message;
mod test;

use chrono::{DateTime, Local};
use config::BagLogConfig;
use core::panic;
use message::BagLogMessage;
use std::{
    fs::{File, OpenOptions},
    io::{ErrorKind, Write},
};

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
        &self.config
    }

    pub fn write(&self, message: String) -> std::io::Result<()> {
        let date: DateTime<Local> = Local::now();
        let bag_message = BagLogMessage { date, message };

        let formated_message = &self.config.format_message(&bag_message);

        if self.config.write_to_terminal {
            let res = self.write_to_terminal(formated_message, None::<Vec<u8>>);

            if let Err(err) = res {
                panic!("{err}");
            }
        }

        if self.config.write_to_file {
            self.write_to_file(formated_message)
        } else {
            Ok(())
        }
    }

    pub fn write_to_terminal(
        &self,
        message: &str,
        writer: Option<impl Write>,
    ) -> Result<(), std::io::Error> {
        match writer {
            Some(mut w) => writeln!(&mut w, "{}", message),
            None => todo!(),
        }
    }

    pub fn write_to_file(&self, message: &str) -> std::io::Result<()> {
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
        log_file.write_all(message.as_bytes())?;

        Ok(())
    }
}
