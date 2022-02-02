use std::fs::read_dir;
use std::path::PathBuf;

use exitfailure::ExitFailure;

#[derive(Eq, PartialEq, PartialOrd, Ord, Debug, Clone)]
pub enum DirectoryItem {
    // File(path)
    File(String),
    // Directory(path, is_open, content_len)
    Directory(String, bool),
}

pub fn get_files_for_current_directory() -> Result<Vec<DirectoryItem>, ExitFailure> {
    //Get list, unwrap, and convert results to &Path
    let dir_items: Vec<PathBuf> = match read_dir(".") {
        Ok(val) => val.map(|f| f.unwrap().path()).collect(),
        Err(err) => return Err(ExitFailure::from(err)),
    };

    //Convert items to DirectoryItem
    let mut files: Vec<DirectoryItem> = Vec::new();
    for item in dir_items {
        if item.is_file() {
            let file = DirectoryItem::File(String::from(item.to_string_lossy()));
            files.push(file);
        } else {
            let file = DirectoryItem::Directory(String::from(item.to_string_lossy()), false);
            files.push(file);
        }
    }

    Ok(files)
}
