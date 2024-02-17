use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
    thread,
};

use regex::Regex;
use serde::{Deserialize, Serialize};
use sysinfo::{Pid, System};

use crate::folder;

const LOG_DIR: &str = "./logs/";

pub fn run(mut cmd: Command, project_name: String) -> u32 {
    let mut child = cmd
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    // 将数据放到task线程进行处理
    let stdout = child.stdout.take().unwrap();

    thread::spawn(move || {
        let stdout_reader = BufReader::new(stdout);
        let lines = stdout_reader.lines();

        if let Ok(_) = folder::no_exists_create_dir(LOG_DIR) {
            let mut binding = OpenOptions::new();
            let options = binding.write(true).truncate(true).create(true);
            let mut file = options
                .open(format!("{}{}.log", LOG_DIR, &project_name))
                .unwrap();

            for line in lines {
                let line = line.unwrap();
                let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
                let writer = re.replace_all(&line, "").to_string();
                file.write_all(writer.as_bytes()).unwrap();
                file.write_all(b"\n").unwrap();
            }
        }
    });

    child.id()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessInfo {
    pub memory: u64,
    pub cpu_usage: f32,
}

pub fn get_process_info(pid: usize) -> Option<ProcessInfo> {
    let mut system = System::new();
    system.refresh_all();
    let pid = Pid::from(pid);
    let borrow_pid = &pid;
    if let Some(process) = system.process(*borrow_pid) {
        let mut all_memory = process.memory();
        let mut all_cpu_usage = process.cpu_usage();

        system.processes().iter().for_each(|(_, row_process)| {
            if let Some(parent_pid) = row_process.parent() {
                if borrow_pid.eq(&parent_pid) {
                    all_memory = all_memory + row_process.memory();
                    all_cpu_usage = all_cpu_usage + row_process.cpu_usage()
                }
            }
        });

        Some(ProcessInfo {
            memory: all_memory,
            cpu_usage: all_cpu_usage,
        })
    } else {
        None
    }
}
