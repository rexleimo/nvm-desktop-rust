use std::process::Command;

use sysinfo::{Pid, System};

pub fn new(args: &[&str]) -> Command {
    let mut sh = Command::new("cmd.exe");
    sh.args(args.as_ref());
    sh
}

pub fn get_process_info(pid: usize) -> Option<crate::modules::os::cmd::ProcessInfo> {
    let mut system = System::new_all();
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
        Some(crate::modules::os::cmd::ProcessInfo {
            memory: all_memory,
            cpu_usage: all_cpu_usage,
        })
    } else {
        None
    }
}
