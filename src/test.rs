#[cfg(test)]
mod test {
    use std::io::Read;

    use chrono::{DateTime, Local};

    use crate::{config::BagLogConfig, BagLog};

    #[test]

    fn log_to_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("log.log");

        let file_path = path.clone();

        let cfg = BagLogConfig {
            out_file: path.into_os_string().into_string().unwrap(),
            ..Default::default()
        };

        let logger = BagLog::new(Some(cfg));

        let log_res = logger.write(String::from("log_to_file"));
        assert!(&file_path.exists());

        if let Err(err) = log_res {
            panic!("{}", err);
        }

        let date_format = &logger.get_config().date_format;
        let date: DateTime<Local> = Local::now();
        let formatted = &date.format(&date_format);

        let mut contents = String::new();
        std::fs::File::open(&file_path)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert_eq!(contents, format!("[{formatted}]: log_to_file"));
    }
}
