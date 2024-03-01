use std::process::{Child, Command};

pub fn new(args: &[&str]) -> Command {
    let mut sh = Command::new("cmd.exe");
    sh.args(args.as_ref());
    sh
}
