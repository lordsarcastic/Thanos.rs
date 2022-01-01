use std::{collections::HashMap, io::Error, fs::read_dir, path::{Path, PathBuf}};

pub fn get_directory_content(directory: &Path) -> Result<HashMap<&str, Vec<PathBuf>>, Error> {
    let mut files: Vec<PathBuf> = vec![];
    let mut dirs: Vec<PathBuf> = vec![];
    let mut result = HashMap::new();

    for entry in read_dir(directory)? {
        let entry = entry?.path();
        if entry.is_dir() {
            dirs.push(entry);
        } else {
            files.push(entry);
        }
    }
    result.insert("files", files);
    result.insert("dirs", dirs);

    Ok(result)
}