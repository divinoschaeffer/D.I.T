use std::{env, io};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Read};
use std::path::{Component, Path, PathBuf};

use dit_file_encryptor::CompressedFile;
use dit_file_encryptor::write_string_file_gz;

use crate::error::DitError;
use crate::process_path::get_all_files_in_directory;

pub const NULL_HASH: &str = "0000000000000000000000000000000000000000";

pub fn relative_path_to_dit() -> Option<PathBuf> {
    let initial_dir = env::current_dir()
        .expect("Failed to get current directory");

    let mut current_dir = initial_dir.clone();

    loop {
        let dit_path = current_dir.join(".dit");
        if dit_path.exists() && dit_path.is_dir() {
            return match initial_dir.strip_prefix(&current_dir) {
                Ok(path) => Some(PathBuf::from(path)),
                _ => None
            };
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => break,
        }
    }

    None
}

pub fn normalize_path(path: PathBuf) -> PathBuf {
    let mut components = vec![];

    for component in path.components() {
        match component {
            Component::ParentDir => {
                if components.last() != Some(&Component::RootDir) && components.last() != Some(&Component::ParentDir) {
                    components.pop();
                } else {
                    components.push(component);
                }
            }
            Component::CurDir => {}
            _ => components.push(component),
        }
    }

    components.iter().collect()
}

/// Return a result with normalize path from .dit to the specify path
pub fn path_from_dit(path: &PathBuf) -> Result<PathBuf, DitError> {
    let rel_to_dit = match relative_path_to_dit() {
        Some(rel_to_dit) => rel_to_dit,
        None => {
            return Err(DitError::NotInitialized)
        }
    };
    let path_file = rel_to_dit.join(path);
    let norm_path = normalize_path(path_file);
    Ok(norm_path)
}

pub fn read_hash_file(file_path: PathBuf, pos: usize) -> Result<String, DitError> {
    let mut reader = CompressedFile::new(file_path)
        .open_for_read()
        .map_err(|e| DitError::IoError(e))?;

    let mut buf = String::new();

    reader.read_to_string(&mut buf)
        .map_err(|e1| DitError::IoError(e1))?;

    let infos: Vec<&str> = buf
        .split_whitespace()
        .collect();

    Ok(String::from(infos[pos]))
}

pub fn write_hash_file(hash: String, path: PathBuf, pos: u64) -> Result<(), io::Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    write_string_file_gz(hash, &mut file, pos)?;
    Ok(())
}

pub fn read_content_from_non_encrypted_file(path: &&Path) -> Result<String, io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn clean_path(paths: Vec<PathBuf>) -> Result<Vec<PathBuf>, DitError> {
    let mut all_files_path: Vec<PathBuf> = vec![];

    for element in paths {
        let total_files = get_all_files_in_directory(&element).map_err(|e| {
            DitError::UnexpectedComportement(format!("Details: {}", e))
        })?;
        let clean_files: Result<Vec<PathBuf>, DitError> = total_files
            .into_iter()
            .map(|file| path_from_dit(&file))
            .collect();

        let clean_files = clean_files?;

        all_files_path.extend(clean_files);
    }

    Ok(all_files_path)
}