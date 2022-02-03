use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

use crate::app::App;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone)]
pub enum DirectoryItem {
    // File(path)
    File(String),
    // Directory(path, is_open, content_len)
    Directory(String),
}

pub fn get_files_for_current_directory(app: &mut App) -> Result<Vec<DirectoryItem>, io::Error> {
    //Get list, unwrap, and convert results to &Path
    let dir_items: Vec<PathBuf> = match read_dir(&app.current_directory) {
        Ok(val) => val.map(|f| f.unwrap().path()).collect(),
        Err(err) => return Err(err),
    };

    //Convert items to DirectoryItem
    let mut files: Vec<DirectoryItem> = Vec::new();
    for item in dir_items {
        if item.is_file() {
            // Check whether it is an audio file
            if check_audio_file(&item)? {
                let file = DirectoryItem::File(String::from(item.to_string_lossy()));
                files.push(file);
            }
        } else {
            let file = DirectoryItem::Directory(String::from(item.to_string_lossy()));
            files.push(file);
        }
    }

    Ok(files)
}

pub fn check_audio_file(path: &PathBuf) -> Result<bool, io::Error> {
    if let Some(t) = infer::get_from_path(path.to_str().unwrap())? {
        let mime_type = t.mime_type();

        return Ok(mime_type.contains("audio") || mime_type.contains("video"));
    }

    Ok(false)
}

pub fn read_audio_file() {
    
}