// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate lazy_static;

use nvm_desktop_rust::modules::os::{ide, system_info};
use project::Project;
use regex::Regex;
use reqwest::Client;
use std::{
    collections::HashMap,
    env,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::Mutex,
};
use sysinfo::System;
use tauri::Result;

mod cmd;
mod folder;
mod log;
mod project;
mod uzip;
mod version;

#[derive(Debug)]
#[warn(dead_code)]
struct SystemInfo {
    name: String,
    cpu_arch: String,
}

static NODE_URL: &str = "https://nodejs.org/dist/";
static NODE_DIR: &str = "./nodes/";
static VERSION_DIR: &str = "./versions/";

lazy_static! {
    static ref CMD_MAP: Mutex<HashMap<String, u32>> = {
        let map = HashMap::new();
        Mutex::new(map)
    };
}

fn get_system_info() -> SystemInfo {
    let mut system = System::new_all();
    system.refresh_all();
    let name = String::from(std::env::consts::OS);
    SystemInfo {
        name: name,
        cpu_arch: String::from(std::env::consts::ARCH),
    }
}

async fn get_download_node_url(version_str: String, app_handle: &tauri::AppHandle) -> Result<bool> {
    // https://nodejs.org/dist/v21.6.1/node-v21.6.1-win-x86.zip
    let mut node_url = String::new();
    let unzip_path = format!("{}{}.zip", NODE_DIR, &version_str);

    match folder::no_exists_create_dir(NODE_DIR) {
        Ok(_) => {
            node_url.push_str(NODE_URL);
            let system = get_system_info();
            println!("{}", system.cpu_arch);
            if "windows" == system.name {
                if "x86_64" == system.cpu_arch {
                    let push_str: String =
                        format!("v{}/node-v{}-win-x86.zip", &version_str, &version_str);
                    node_url.push_str(push_str.as_str());
                }
            }

            if "linux" == system.name {
                if "x86" == system.cpu_arch {
                    //https://nodejs.org/dist/v19.6.1/node-v19.6.1-linux-x64.tar.gz
                    let push_str =
                        format!("v{}/node-v{}-linux-x64.tar.xz", &version_str, &version_str);
                    node_url.push_str(push_str.as_str());
                }
            }

            if "macos" == system.name {
                if "x86" == system.cpu_arch {
                    //https://nodejs.org/dist/v19.6.1/node-v19.6.1-darwin-x64.tar.gz
                    let push_str =
                        format!("v{}/node-v{}-darwin-x64.tar.xz", &version_str, &version_str);
                    node_url.push_str(push_str.as_str());
                }
            }

            println!("download url:{}", node_url);
            let mut binding = OpenOptions::new();
            let options = binding.read(true).write(true).truncate(true).create(true);
            let path = Path::new(&unzip_path);
            match fs::metadata(path) {
                Ok(_) => {
                    return Ok(true);
                }
                Err(_) => {
                    let mut file = options.open(path)?;
                    let client = Client::new();
                    let mut response: reqwest::Response =
                        client.get(node_url).send().await.unwrap();
                    if response.status().is_success() {
                        while let Some(chunk) = response.chunk().await.unwrap() {
                            file.write_all(&chunk)?;
                        }
                        log::send_log(&app_handle, format!("下载版本{},成功", &version_str));
                        return Ok(true);
                    }
                }
            }
            // 没有
            Ok(false)
        }
        Err(e) => {
            log::send_log(&app_handle, e.to_string());
            Ok(false)
        }
    }
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

async fn remote_install_node(
    version_str: String,
    app_handle: &tauri::AppHandle,
) -> Vec<version::Version> {
    let save = get_download_node_url(version_str.clone(), app_handle)
        .await
        .unwrap();
    let version_list = version::get_all_version();
    if save {
        let mut cur = version::get_version(&version_str).unwrap();
        cur.status = 1;
        let ex = version::update_version(&cur).unwrap();
        if ex {
            return version::get_all_version();
        } else {
            return version_list;
        }
    }
    version_list
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn download_node(
    version_str: String,
    app_handle: tauri::AppHandle,
) -> tauri::Result<Vec<version::Version>> {
    let result = remote_install_node(version_str, &app_handle).await;
    Ok(result)
}

#[tauri::command]
async fn get_version_list() -> Vec<version::Version> {
    let versions = version::get_all_version();
    versions
}

#[cfg(target_os = "windows")]
#[tauri::command]
async fn unzip_version(version_str: String, app_handle: tauri::AppHandle) -> Vec<version::Version> {
    // 解压node文件
    let node_zip_path = format!("{}{}.zip", NODE_DIR, &version_str);
    let path = Path::new(&node_zip_path);

    let mut node_unzip_path = String::new();
    node_unzip_path.push_str(VERSION_DIR);

    folder::no_exists_create_dir(VERSION_DIR).unwrap();

    let new_versions = match fs::metadata(&path) {
        Err(_why) => Vec::new(),
        Ok(_) => {
            let versions = version::get_all_version();
            let result = match uzip::window_unzip(&node_zip_path, &node_unzip_path) {
                Ok(_zip_resp) => {
                    let mut current = version::get_version(&version_str).unwrap();
                    current.status = 2;
                    let ex = version::update_version(&current);
                    match ex {
                        Ok(ex) => {
                            if ex {
                                log::send_log(&app_handle, format!("安装版本{}成功", &version_str));
                                version::get_all_version()
                            } else {
                                versions
                            }
                        }
                        Err(_) => versions,
                    }
                }
                Err(_) => versions,
            };
            result
        }
    };
    new_versions
}

// macos 和 linux 走的tar.gz 双提取
#[cfg(not(target_os = "windows"))]
#[tauri::command]
async fn unzip_version(version_str: String, app_handle: tauri::AppHandle) -> Vec<version::Version> {
    let node_zip_path = format!("{}{}.tar.gz", NODE_DIR, &version_str);
    let path = Path::new(&node_zip_path);

    let mut node_unzip_path: String = String::new();
    node_unzip_path.push_str(VERSION_DIR);

    folder::no_exists_create_dir(VERSION_DIR).unwrap();

    match fs::metadata(&path) {
        Err(_why) => Vec::new(),
        Ok(_) => {
            node_unzip_path.push_str(format!("{}", &version_str).as_str());
            uzip::linux_un_tar_gz(path.to_str().unwrap(), &node_unzip_path.to_string()).unwrap();
            log::send_log(&app_handle, format!("安装版本{}成功", &version_str));
            Vec::new()
        }
    }
}

#[tauri::command]
fn use_version(version_str: String, app_handle: tauri::AppHandle) -> Vec<version::Version> {
    let mut target_dir = String::new();
    target_dir.push_str("./versions/");
    target_dir.push_str("node-v");
    target_dir.push_str(&version_str);
    target_dir.push_str("-win-x86");

    let mut link_dir = String::new();
    link_dir.push_str("./versions/");
    link_dir.push_str("default");
    // 先取消链接
    match symlink::remove_symlink_dir(link_dir.clone()) {
        Ok(_) => true,
        Err(_) => false,
    };

    let target_path = fs::canonicalize(PathBuf::from(target_dir)).unwrap();
    symlink::symlink_dir(target_path, link_dir).unwrap();

    let current = version::get_version(&version_str).unwrap();

    let update_id = match current.id {
        Some(id) => id,
        None => 0,
    };
    let _ = version::update_version_is_use(&update_id);
    let version_list = version::get_all_version();
    log::send_log(&app_handle, format!("切换到版本{}", &version_str));
    version_list
}

#[tauri::command]
async fn download_remote(
    version_str: String,
    app_handle: tauri::AppHandle,
) -> Vec<version::Version> {
    let mut version_list = version::get_all_version();
    if version_str.is_empty() {
        return version_list;
    }

    match get_download_node_url(version_str.clone(), &app_handle).await {
        Ok(_) => {
            let payload = version::Version {
                id: None,
                name: version_str,
                status: 1,
                is_use: 0,
            };

            match version::insert_version(&payload) {
                Ok(ex) => {
                    if ex {
                        version_list.push(payload);
                        version_list
                    } else {
                        version_list
                    }
                }
                Err(_) => version_list,
            }
        }
        Err(_) => version_list,
    }
}

#[tauri::command]
async fn create_project(body: Project, app_handle: tauri::AppHandle) -> Result<bool> {
    let execute = project::add_project(&body);
    match execute {
        Ok(_) => {
            log::send_log(&app_handle, "创建项目成功".to_string());
            Ok(true)
        }
        Err(_) => Ok(false),
    }
}

#[tauri::command]
fn run_project(project_name: String, app_handle: tauri::AppHandle) {
    let (os_cmd, args) = project::get_cmd_args(project_name.clone());

    let mut cmd = Command::new(os_cmd);
    cmd.args(args);

    let pid = cmd::run(cmd, project_name.clone());

    if pid > 0 {
        CMD_MAP.lock().unwrap().insert(project_name, pid);
        let txt = format!("项目启动： 进程ID为：{}，可在日志中查看运行情况", &pid);
        log::send_log(&app_handle, txt);
    }
}

#[tauri::command]
async fn stop_project(project_name: String, app_handle: tauri::AppHandle) {
    let mut lock = CMD_MAP.lock().unwrap();
    let pid = lock.get(&project_name).unwrap();
    let borrow_pid = pid.clone();
    if let Ok(mut run) = Command::new("cmd.exe")
        .args(&[
            "/c",
            "taskkill /f /t /pid",
            &borrow_pid.to_string().as_str(),
        ])
        .spawn()
    {
        lock.remove(&project_name);
        run.wait().unwrap();
    }

    println!("success taskkill");
    log::send_log(&app_handle, format!("项目关闭： 进程ID为：{}", &borrow_pid));
}

#[tauri::command]
async fn get_project_list() -> Result<Vec<Project>> {
    let projects = project::get_projects();
    Ok(projects)
}

#[tauri::command]
async fn get_project_info(project_name: String) -> Option<Project> {
    let project = project::get_project(&project_name);
    project
}

#[tauri::command]
async fn delete_project(id: i32, app_handle: tauri::AppHandle) -> Vec<Project> {
    let borrow_id = &id;
    project::delete_project(borrow_id).unwrap();
    let projects = project::get_projects();
    log::send_log(&app_handle, format!("项目删除： {}", borrow_id));
    projects
}

#[tauri::command]
async fn open_project(project_name: String) -> Result<()> {
    let row = project::get_project(&project_name).unwrap();

    let os_info = system_info::new();
    if "windows" == os_info.name {
        let directory = row.dir.replace("\\", "/");
        ide::open(directory.as_str());
    } else {
        ide::open(row.dir.as_str());
    }

    Ok(())
}

#[tauri::command]
async fn open_log(project_name: String) -> Result<()> {
    let directory = Path::new("logs").join(format!("{}.log", &project_name));
    println!("{}", directory.to_str().unwrap());
    let os_cmd = "cmd.exe";
    let mut cmd = Command::new(os_cmd);
    cmd.args(vec!["/c", "start", "Notepad", directory.to_str().unwrap()]);
    let mut child = cmd.spawn().unwrap();
    println!("{}", child.id());
    child.wait().unwrap();
    Ok(())
}

#[tauri::command]
async fn open_cmd(project_name: String) {
    let row = project::get_project(&project_name).unwrap();
    let directory = row.dir.replace("\\", "/");
    println!("{}", directory);
    let os_cmd = "cmd.exe";
    let mut cmd = Command::new(os_cmd);
    cmd.args(vec![
        "/c",
        "start",
        "cmd",
        "/K",
        "cd /d",
        directory.as_str(),
    ]);
    let mut child = cmd.spawn().unwrap();
    println!("{}", child.id());
    child.wait().unwrap();
}

#[tauri::command]
async fn get_process_info(project_name: String) -> Option<cmd::ProcessInfo> {
    let lock = CMD_MAP.lock().unwrap();

    let pid: Option<&u32> = lock.get(&project_name);

    match pid {
        Some(pid) => {
            let target_pid = &(*pid as usize);
            if let Some(process) = cmd::get_process_info(*target_pid) {
                Some(process)
            } else {
                None
            }
        }
        None => None,
    }
}

fn main() {
    project::create_project().unwrap();
    version::create_or_update_table();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_version_list,
            download_node,
            unzip_version,
            use_version,
            download_remote,
            get_project_list,
            get_project_info,
            create_project,
            run_project,
            stop_project,
            delete_project,
            open_project,
            open_log,
            open_cmd,
            get_process_info
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
