#[cfg(target_os = "windows")]
use std::fs::{self, File};
use std::io::{self, BufReader};
use std::path::Path;
use zip::ZipArchive;

#[cfg(not(target_os = "windows"))]
use flate2::read::GzDecoder;

#[cfg(not(target_os = "windows"))]
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

#[cfg(target_os = "windows")]
pub fn window_unzip(input_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&input_path).unwrap();
    let mut zip = ZipArchive::new(BufReader::new(file)).unwrap();

    let target = Path::new(&output_path);

    if !target.exists() {
        let _ = fs::create_dir_all(target).map_err(|e| {
            println!("{}", e);
        });
    }

    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        if file.is_dir() {
            let target = target.join(Path::new(&file.name().replace("\\", "")));
            fs::create_dir_all(target).unwrap();
        } else {
            let file_path = target.join(Path::new(file.name()));
            let mut target_file = if !file_path.exists() {
                fs::File::create(file_path).unwrap()
            } else {
                fs::File::open(file_path).unwrap()
            };
            let copy_result = io::copy(&mut file, &mut target_file);
            match copy_result {
                Ok(size) => {
                    println!("{}", size);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
    Ok(())
}
