use rand::{seq::SliceRandom, thread_rng};
use std::fs;
use uuid::Uuid;

pub fn shuffle_path_content(contents: Vec<std::path::PathBuf>) -> Result<(), std::io::Error> {
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
        let file = std::path::Path::new(uuid_path);
        let path = file.parent().unwrap().to_str().unwrap();
        let temp_path = path_names.choose(&mut rng).unwrap().clone();
        path_names.retain(|&x| x != temp_path);
        fs::rename(file, format!("{}/{}", path, temp_path))?;
    }

    Ok(())
}

