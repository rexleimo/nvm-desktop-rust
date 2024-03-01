// #[cfg(not(target_os = "windows"))]
pub fn open(directory: &str) -> u32 {
    use std::process::{Command, Stdio};
    println!("directory is:{}", &directory);
    let mut shell = Command::new("bash");
    shell.args(vec!["-c", "code", directory]);

    let child = shell
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");
    return child.id();
}
