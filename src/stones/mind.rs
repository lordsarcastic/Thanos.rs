use rand::{seq::SliceRandom, thread_rng};
use std::{fs, path::{PathBuf, Path}};
use uuid::Uuid;

use crate::utils;

pub fn shuffle_path_content(contents: Vec<PathBuf>) -> Result<(), std::io::Error> {
    let mut path_names: Vec<&str> = contents
        .iter()
        .map(|file| file.file_name().unwrap().to_str().unwrap())
        .collect();

    let mut new_path: Vec<String> = vec![];

    for path_name in contents.iter() {
        let uuid = Uuid::new_v4();
        let path = path_name.parent().unwrap().to_str().unwrap();
        fs::rename(path_name, format!("{}/{}", path, uuid.to_string()))?;
        new_path.push(format!("{}/{}", path, uuid.to_string()));
    }

    let mut rng = thread_rng();
    for uuid_path in new_path.iter() {
        let file = Path::new(uuid_path);
        let path = file.parent().unwrap().to_str().unwrap();
        let temp_path = path_names.choose(&mut rng).unwrap().clone();
        path_names.retain(|&x| x != temp_path);
        fs::rename(file, format!("{}/{}", path, temp_path))?;
    }

    Ok(())
}

pub fn mind(directory: PathBuf) -> Result<(), std::io::Error> {
    let directory_content = utils::get_directory_content(
        directory.as_path()
    ).unwrap();

    shuffle_path_content(directory_content.get("files").unwrap().to_vec())?;

    for dir in directory_content.get("dirs").unwrap() {
        mind(dir.to_path_buf())?;
    }

    shuffle_path_content(directory_content.get("dirs").unwrap().to_vec())?;
    Ok(())
}
// pick folder and separate files from folders
// shuffle files
// pick folders;
//      folder one
//      repeat above