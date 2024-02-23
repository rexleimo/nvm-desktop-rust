use std::fs::File;

use flate2::read::GzDecoder;

use crate::folder;

pub fn linux_un_tar_gz(path: &str, out_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tar_gz = File::open(path).expect("File not found");
    let tar = GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path: std::borrow::Cow<'_, std::path::Path> = entry.path()?;

        let out_path = format!("{}", out_path);
        println!("{}", &out_path);
        entry.unpack(out_path)?;
    }

    Ok(())
}
