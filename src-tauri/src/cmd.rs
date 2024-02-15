use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    process::{Command, Stdio},
    thread,
};

use regex::Regex;

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

        let mut binding = OpenOptions::new();
        let options = binding.write(true).truncate(true).create(true);
        let mut file = options
            .open(format!("./logs/{}.log", &project_name))
            .unwrap();

        for line in lines {
            let line = line.unwrap();
            let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
            let writer = re.replace_all(&line, "").to_string();
            file.write_all(writer.as_bytes()).unwrap();
            file.write_all(b"\n").unwrap();
        }
    });

    child.id()
}
