use baglog::{BagLog, BagLogConfig};
use chrono::{DateTime, Local};
use std::{fs, str};

#[test]
fn write_to_terminal() {
    let config = BagLogConfig {
        out_file: String::from("test.log"),
        ..Default::default()
    };

    let logger = BagLog::new(Some(config));
    let date_format = &logger.get_config().date_format;

    let mut result = Vec::new();

    let date: DateTime<Local> = Local::now();
    let formated_date = &date.format(&date_format);

    let log_res = logger.write(String::from("test message"), &mut result);

    match log_res {
        Err(err) => panic!("Something went wrong in the logging: {}", err),
        Ok(_) => (),
    }

    let str_result = match str::from_utf8(&result) {
        Ok(v) => v,
        Err(err) => panic!("Invalid UTF-8 sequence: {}", err),
    };

    let formated_test_string = format!("[{formated_date}]: test message\n");

    assert_eq!(str_result, formated_test_string);

    let file_remove_result = fs::remove_file(&logger.get_config().out_file);

    match file_remove_result {
        Err(err) => panic!(
            "Something went wrong when cleaning up the test log file: {}",
            err
        ),
        Ok(_) => (),
    }
}
