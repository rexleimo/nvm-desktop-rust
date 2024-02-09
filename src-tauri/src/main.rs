// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use regex::Regex;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_version_list() -> Vec<String> {
    let resp = reqwest::get("https://nodejs.org/dist/").await;
    match resp {
        Ok(body) => {
            let text = body.text().await.unwrap();
            let re = Regex::new(r#"<a href="(v[^"/]+/)">"#).unwrap();
            let captures_iter = re.captures_iter(&text);
            let mut versions = Vec::new();
            for capture in captures_iter {
                if let Some(version) = capture.get(1) {
                    versions.push(version.as_str().replace("/", "").to_string());
                }
            }
            versions
        }
        Err(_) => Vec::new(),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![get_version_list])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
