use std::{process::Command, thread};

pub fn run_cmd_nonblocking(cmd: &mut Command, project_name: String) -> Result<u32, String> {
   

    // let stdout = child.stdout.as_mut().unwrap();
    // let stdout_reader = BufReader::new(stdout);
    // let lines = stdout_reader.lines();
    // let log_name = format!("./logs/{}", project_name);
    // let mut binding = OpenOptions::new();
    // let options: &mut OpenOptions = binding.write(true).create(true);
    // let mut log = options.open(&log_name).unwrap();
    // for line in lines {
    //     let line = line.unwrap();
    //     let re = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
    //     let write_line = re.replace_all(&line, "").to_string();
    //     log.write_all(write_line.as_bytes()).unwrap();
    //     log.write_all(b"\n").unwrap();
    // }
    Ok(0)
}
