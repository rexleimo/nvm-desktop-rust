// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, File, OpenOptions},
    io::{self, BufReader, Read, Write},
    path::{Path, PathBuf},
    process::Command,
    sync::Mutex,
};

use project::Project;
use regex::Regex;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sysinfo::System;
use tauri::Result;
use zip::ZipArchive;

mod cmd;
mod project;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Version {
    name: String,
    status: i16,
    is_use: bool,
}

#[derive(Debug)]
#[warn(dead_code)]
struct SystemInfo {
    name: String,
    cpu_arch: String,
}

static NODE_URL: &str = "https://nodejs.org/dist/";

lazy_static! {
    static ref CMD_MAP: Mutex<HashMap<String, u32>> = {
        let map = HashMap::new();
        Mutex::new(map)
    };
}

fn get_system_info() -> SystemInfo {
    let mut system = System::new_all();
    system.refresh_all();
    SystemInfo {
        name: System::name().unwrap(),
        cpu_arch: System::cpu_arch().unwrap(),
    }
}

fn unzip(zip_path: &str, dest_path: &str) -> Result<()> {
    let file = File::open(zip_path).unwrap();
    let mut zip = ZipArchive::new(BufReader::new(file)).unwrap();

    let target = Path::new(&dest_path);

    if !target.exists() {
        let _ = fs::create_dir_all(target).map_err(|e| {
            println!("{}", e);
        });
    }

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        if file.is_dir() {
            let target = target.join(Path::new(&file.name().replace("\\", "")));
            fs::create_dir_all(target).unwrap();
        } else {
            let file_path = target.join(Path::new(file.name()));
            let mut target_file = if !file_path.exists() {
                fs::File::create(file_path).unwrap()
            } else {
                fs::File::open(file_path).unwrap()
            };
            let copy_result = io::copy(&mut file, &mut target_file);
            match copy_result {
                Ok(size) => {
                    println!("{}", size);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
    Ok(())
}

async fn get_download_node_url(version_str: String) -> Result<bool> {
    // https://nodejs.org/dist/v21.6.1/node-v21.6.1-win-x86.zip
    let mut node_url = String::new();
    let mut save_path = String::new();
    node_url.push_str(NODE_URL);
    save_path.push_str("./node/");
    save_path.push_str(&version_str);
    save_path.push_str(".zip");
    let system = get_system_info();
    if "Windows" == system.name {
        if "x86" == system.cpu_arch {
            node_url.push_str("v");
            node_url.push_str(&version_str);
            node_url.push_str("/node-v");
            node_url.push_str(&version_str);
            node_url.push_str("-win-x86.zip");
        }
    }
    println!("download url:{}", node_url);
    let mut binding = OpenOptions::new();
    let options = binding.read(true).write(true).truncate(true).create(true);
    let path = Path::new(&save_path);
    match fs::metadata(path) {
        Ok(_) => {
            return Ok(true);
        }
        Err(_) => {
            let mut file = options.open(path)?;
            let client = Client::new();
            let mut response = client.get(node_url).send().await.unwrap();
            if response.status().is_success() {
                while let Some(chunk) = response.chunk().await.unwrap() {
                    file.write_all(&chunk)?;
                }
                return Ok(true);
            }
        }
    }
    // 没有
    Ok(false)
}

fn read_version_setting() -> Vec<Version> {
    let mut binding = OpenOptions::new();
    let options = binding.read(true).write(true).create(true);
    let setting = options.open("setting.json");
    match setting {
        Ok(mut file) => {
            let mut content: String = String::new();
            file.read_to_string(&mut content).expect("读取内容错误");
            let setting_json: Vec<Version> = serde_json::from_str(&content).expect("序列化错误");
            return setting_json;
        }
        Err(_) => Vec::new(),
    }
}

fn write_version_setting(settings: &Vec<Version>) -> Result<()> {
    let mut binding = OpenOptions::new();
    let options = binding.write(true).truncate(true);
    let mut setting_file = options.open("setting.json").expect("open");
    let json_str = serde_json::to_string_pretty(settings).unwrap();
    setting_file
        .write_all(json_str.as_bytes())
        .expect("Failed to open or create the file");
    Ok(())
}

async fn _remote_node_list() -> Vec<String> {
    let resp = reqwest::get(NODE_URL).await;
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

async fn remote_install_node(version_str: String) -> Vec<Version> {
    let save = get_download_node_url(version_str.clone()).await.unwrap();
    if save {
        let mut setting_json: Vec<Version> = read_version_setting();
        let row = setting_json
            .iter_mut()
            .find(|item| item.name == version_str)
            .unwrap();
        row.status = 1;
        write_version_setting(&setting_json).unwrap();
        return setting_json;
    }
    Vec::new()
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn download_node(version_str: String) -> tauri::Result<Vec<Version>> {
    let result = remote_install_node(version_str).await;
    Ok(result)
}

#[tauri::command]
async fn get_version_list() -> Vec<Version> {
    let versions = read_version_setting();
    versions
}

#[tauri::command]
async fn unzip_version(version_str: String) -> Vec<Version> {
    // 解压node文件
    let mut node_zip_path = String::new();
    node_zip_path.push_str("./node/");
    node_zip_path.push_str(&version_str);
    node_zip_path.push_str(".zip");
    let path = Path::new(&node_zip_path);

    let mut node_unzip_path = String::new();
    node_unzip_path.push_str("./versions/");

    let new_versions = match fs::metadata(&path) {
        Err(_why) => Vec::new(),
        Ok(_) => {
            let result = match unzip(&node_zip_path, &node_unzip_path) {
                Ok(_zip_resp) => {
                    let mut setting_json = read_version_setting();
                    let row = setting_json
                        .iter_mut()
                        .find(|item| item.name == version_str)
                        .unwrap();
                    row.status = 2;
                    write_version_setting(&setting_json).unwrap();
                    setting_json
                }
                Err(_) => Vec::new(),
            };
            result
        }
    };
    new_versions
}

#[tauri::command]
fn use_version(version_str: String) -> Vec<Version> {
    let mut target_dir = String::new();
    target_dir.push_str("./versions/");
    target_dir.push_str("node-v");
    target_dir.push_str(&version_str);
    target_dir.push_str("-win-x86");

    let mut link_dir = String::new();
    link_dir.push_str("./versions/");
    link_dir.push_str("default");
    // let mut binding = OpenOptions::new();
    // let options = binding.write(true).truncate(true).create(true).read(true);
    // 先取消链接
    match symlink::remove_symlink_dir(link_dir.clone()) {
        Ok(_) => true,
        Err(_) => false,
    };

    let target_path = fs::canonicalize(PathBuf::from(target_dir)).unwrap();
    println!("{:?}", target_path);
    println!("{}", link_dir);
    symlink::symlink_dir(target_path, link_dir).unwrap();

    let mut settings_json = read_version_setting();
    settings_json
        .iter_mut()
        .for_each(|item| item.is_use = false);
    let row = settings_json
        .iter_mut()
        .find(|item| item.name == version_str)
        .unwrap();

    row.is_use = true;
    write_version_setting(&settings_json).unwrap();
    settings_json
}

#[tauri::command]
async fn download_remote(version_str: String) -> Vec<Version> {
    match get_download_node_url(version_str.clone()).await {
        Ok(is_save) => {
            let mut settings_json = read_version_setting();
            if is_save {
                let version = Version {
                    name: version_str,
                    status: 1,
                    is_use: false,
                };
                settings_json.push(version);
                write_version_setting(&settings_json).unwrap();
                settings_json
            } else {
                settings_json
            }
        }
        Err(_) => Vec::new(),
    }
}

#[tauri::command]
async fn create_project(body: Project) -> Result<bool> {
    let execute = project::add_project(&body);
    match execute {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

#[tauri::command]
fn run_project(project_name: String) {
    let row = project::get_project(&project_name).unwrap();
    let directory = row.dir.replace("\\", "/");
    let os_cmd = "cmd.exe";
    let args = &["/c", &format!("cd /d {} && {}", directory, row.run_cmd)];
    let mut cmd = Command::new(os_cmd);

    if let Ok(run) = cmd.args(args).spawn() {
        CMD_MAP.lock().unwrap().insert(project_name, run.id());
        println!("{}", run.id());
    }
}

#[tauri::command]
async fn stop_project(project_name: String) {
    let mut lock = CMD_MAP.lock().unwrap();
    let pid = lock.get(&project_name).unwrap();

    if let Ok(mut run) = Command::new("cmd.exe")
        .args(&["/c", "taskkill /f /t /pid", pid.to_string().as_str()])
        .spawn()
    {
        lock.remove(&project_name);
        run.wait().unwrap();
    }

    println!("success taskkill");
}

#[tauri::command]
async fn get_project_list() -> Result<Vec<Project>> {
    let projects = project::get_projects();
    Ok(projects)
}

#[tauri::command]
async fn delete_project(project_name: String) -> Vec<Project> {
    project::delete_project(&project_name).unwrap();
    let projects = project::get_projects();
    projects
}

fn main() {
    project::create_project().unwrap();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_version_list,
            download_node,
            unzip_version,
            use_version,
            download_remote,
            get_project_list,
            create_project,
            run_project,
            stop_project,
            delete_project
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
