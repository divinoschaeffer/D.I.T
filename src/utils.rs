use std::{env, io};
use std::env::set_current_dir;
use std::fs::File;
use std::io::{BufReader, Read};
use std::os::unix::fs::FileExt;
use std::path::{Component, Path, PathBuf};

use crate::error::DitError;
use crate::features::init::find_dit;

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
pub fn path_from_dit(path: &String) -> Result<PathBuf, DitError> {
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

pub fn read_hash_file(file: File, pos: u64) -> String {
    let mut buf = [0u8; 40];

    file.read_at(&mut buf, pos).unwrap_or_else(|e| {
        panic!("Error while reading hash: {e}");
    });

    let hash = String::from_utf8(Vec::from(buf)).unwrap();
    hash
}

pub fn write_hash_file(hash: String, file: &File, pos: u64) -> Result<(), io::Error> {
    let buf = &Vec::from(hash)[..];
    file.write_at(buf, pos)?;
    Ok(())
}

pub fn write_header_file(header: String, file: &File, pos: u64) -> Result<(), io::Error> {
    let buf = &Vec::from(header)[..];
    file.write_at(buf, pos)?;
    Ok(())
}

pub fn write_footer_file(footer: String, file: File, pos: u64) -> Result<(), io::Error> {
    let buf = &Vec::from(footer)[..];
    file.write_at(buf, pos)?;
    Ok(())
}

pub fn read_content_file_from_path(path: &&Path) -> Result<String, io::Error> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn merge_text(text1: String, text2: String) -> String {
    let mut result = String::new();
    let mut in_conflict = false;

    for diff in diff::lines(&text1, &text2) {
        match diff {
            diff::Result::Left(old) => {
                if !in_conflict {
                    result.push_str("\n<<<<<< HEAD (current change)\n");
                    in_conflict = true;
                }
                result.push_str(old);
            }
            diff::Result::Right(new) => {
                if in_conflict {
                    result.push_str("\n======\n");
                }
                result.push_str(new);
                result.push_str("\n>>>>>> (incoming change)\n");
                in_conflict = false; // End the conflict block
            }
            diff::Result::Both(common, _) => {
                if in_conflict {
                    result.push_str("\n======\n>>>>>> (incoming change)\n");
                    in_conflict = false; // Close the conflict block
                }
                result.push_str(common);
            }
        }
    }

    if in_conflict {
        result.push_str("\n======\n>>>>>> (incoming change)\n");
    }

    result
}

pub fn set_current_dir_to_project_dir() -> Result<(), io::Error> {
    if let Some(dit_path) = find_dit() {
        match dit_path.parent() {
            Some(parent) => set_current_dir(parent)?,
            _ => return Err(io::Error::new(io::ErrorKind::NotFound, "Error root project not found"))
        }
        return Ok(());
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "Error setting current dir"))
}
