use tauri::Manager;

#[derive(serde::Serialize, Clone)]
struct Logger {
    message: String,
}

pub fn send_log(app_handle: &tauri::AppHandle, log_message: String) {
    let logger = Logger {
        message: log_message,
    };
    app_handle.emit_all("logs_event", logger).unwrap();
}
