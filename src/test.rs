#[cfg(test)]
mod test {
    use crate::{config::BagLogConfig, BagLog};
    use std::io::Read;

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

        let log_res = logger.write_to_file("log_to_file");
        assert!(&file_path.exists());

        if let Err(err) = log_res {
            panic!("{}", err);
        }

        let mut contents = String::new();
        std::fs::File::open(&file_path)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();
        assert_eq!(contents, "log_to_file");
    }

    #[test]
    fn log_to_terminal() {
        let cfg = BagLogConfig {
            ..Default::default()
        };

        let logger = BagLog::new(Some(cfg));

        let mut stdout: Vec<u8> = Vec::new();
        let log_res = logger.write_to_terminal("log_to_terminal", Some(&mut stdout));

        if let Err(err) = log_res {
            panic!("{}", err);
        }

        assert_eq!(stdout, b"log_to_terminal\n");
    }
}
