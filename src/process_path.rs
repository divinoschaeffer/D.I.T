use std::path::PathBuf;
use std::fs;
use std::io;

pub fn get_all_files_in_directory(dir: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    if !dir.is_dir() {
        return Ok(vec![dir.clone()]);
    } 
    
    let mut files: Vec<PathBuf> = Vec::new();
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            files.extend(get_all_files_in_directory(&path)?)
        } else if path.is_file() {
            files.push(path);
        }
    }

    return Ok(files);
}
