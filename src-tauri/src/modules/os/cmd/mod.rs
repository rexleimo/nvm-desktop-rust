use std::{
    fs::File,
    io::{BufRead, Write},
    process::{Child, Command, Stdio},
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
};

use regex::Regex;

pub mod windows;

pub struct CmdManager {
    file_log: Option<Arc<Mutex<File>>>,
    command: Command,
}

impl CmdManager {
    pub fn set_log(&mut self, file_log: File) {
        let mutex_file = Arc::new(Mutex::new(file_log));
        self.file_log = Some(mutex_file);
    }
    pub fn run(&mut self) -> Child {
        #[cfg(target_os = "windows")]
        {
            self.command.spawn().unwrap()
        }

        #[cfg(not(target_os = "windows"))]
        {}
    }

    pub fn run_async(&mut self) -> u32 {
        let mut child;

        if let Some(file) = &self.file_log {
            child = self.command.stdout(Stdio::piped()).spawn().unwrap();
            println!("{:?}", child);

            let stdout = child.stdout.take().unwrap();
            let mutex_file = file.clone();
            thread::spawn(move || {
                let stdout_reader = std::io::BufReader::new(stdout);
                let lines = stdout_reader.lines();

                for line in lines {
                    if let Ok(line) = line {
                        let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
                        let text = re.replace_all(&line, "").to_string();
                        let mut file_log = mutex_file.lock().unwrap();
                        file_log.write_all(text.as_bytes()).unwrap();
                        file_log.write_all(b"\n").unwrap();
                    }
                }
            });
        } else {
            child = self.command.spawn().unwrap();
        }

        child.id()
    }
}

pub fn new(args: &[&str]) -> CmdManager {
    #[cfg(target_os = "windows")]
    {
        println!("{:?}", args);
        let cmd = windows::cmd::new(args);
        println!("{:?}", cmd);
        CmdManager {
            file_log: None,
            command: cmd,
        }
    }
}

pub fn open(path: &str) {
    #[cfg(target_os = "windows")]
    {
        let mut child = new(&vec!["/c", "start", "cmd", "/K", "cd /d", path]);
        child.run().wait().unwrap();
    }
}
