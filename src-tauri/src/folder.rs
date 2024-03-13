use std::{fs, path::Path};

#[cfg(not(target_os = "windows"))]
pub fn no_exists_create_dir(path: &str) -> Result<(), std::io::Error> {
    let path = Path::new(&path);
    if !path.exists() {
        fs::create_dir_all(&path);
        let permission = fs::Permissions::from(0o775);
        fs::set_permissions(&path, permission);
        return Ok(());
    } else {
        // 已经存在所以不处理
        return Ok(());
    }
}

#[cfg(target_os = "windows")]
pub fn no_exists_create_dir(path: &str) -> Result<(), std::io::Error> {
    let path = Path::new(&path);
    if !path.exists() {
        return fs::create_dir_all(&path);
    } else {
        // 已经存在所以不处理
        return Ok(());
    }
}
