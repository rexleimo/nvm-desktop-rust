#[cfg(target_os = "windows")]
pub mod windows;

// #[cfg(not(target_os = "windows"))]
pub mod unix;

pub fn open(path: &str) -> u32 {
    #[cfg(target_os = "windows")]
    let result = windows::open_vscode::open(path);

    #[cfg(not(target_os = "windows"))]
    let result = unix::open_vscode::open(path);

    result.id()
}
