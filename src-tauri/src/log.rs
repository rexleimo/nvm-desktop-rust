use std::{fs::OpenOptions, io::Write};

use chrono::Local;
use tauri::Manager;

#[derive(serde::Serialize, Clone)]
struct Logger {
    message: String,
}

pub fn send_log(app_handle: &tauri::AppHandle, log_message: String) {
    let logger = Logger {
        message: log_message,
    };
    // 只是借用给别
    app_handle.emit_all("logs_event", &logger).unwrap();
    let mut binding = OpenOptions::new();
    let options = binding
        .write(true)
        .create(true)
        .append(true)
        .open("log.txt");
    match options {
        Ok(mut file) => {
            let now = Local::now();
            file.write_all(
                format!(
                    "{}: {}\n",
                    now.format("%Y-%m-%d %H:%M:%S"),
                    logger.message
                )
                .as_bytes(),
            )
            .unwrap();
        }
        Err(e) => {
            println!("Error opening file: {}", e);
        }
    }
}
