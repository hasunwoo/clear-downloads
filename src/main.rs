#![windows_subsystem = "windows"]

use std::{
    error::Error,
    fs, io,
    path::{Path, PathBuf},
};

use directories::UserDirs;

fn main() -> Result<(), Box<dyn Error>> {
    let download_dir = get_download_dir().ok_or("다운로드 경로를 찾을 수 없습니다.")?;
    if !download_dir.exists() {
        println!("다운로드 폴더가 없습니다.");
        return Ok(());
    }
    remove_dir_files(download_dir)
        .map_err(|e| format!("다운로드 폴더를 삭제할 수 없습니다: {}", e))?;
    println!("다운로드 폴더를 비웠습니다.");
    Ok(())
}

fn remove_dir_files<P: AsRef<Path>>(dir: P) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if entry.file_type()?.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

fn get_download_dir() -> Option<PathBuf> {
    let user_dir = UserDirs::new()?;
    Some(user_dir.download_dir()?.to_owned())
}
