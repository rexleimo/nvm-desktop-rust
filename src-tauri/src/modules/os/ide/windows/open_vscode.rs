use std::process::Child;
#[cfg(target_os = "windows")]
pub fn open(directory: &str) -> Child {
    use crate::modules::os::cmd;
    println!("directory is:{}", &directory);
    let mut manager = cmd::new(&vec!["/c", "start", "code", directory]);
    manager.run()
}
