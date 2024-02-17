use std::{fs, path::Path};

pub fn no_exists_create_dir(path: &str) -> Result<(), std::io::Error> {
    let path = Path::new(path);
    if !path.exists() {
        return fs::create_dir_all(path);
    } else {
        // 已经存在所以不处理
        return Ok(());
    }
}
