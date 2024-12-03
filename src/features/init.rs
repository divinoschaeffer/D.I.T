use std::{env, fs, io};
use std::fs::{create_dir, File};
use std::path::PathBuf;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::objects::branch::Branch;
use crate::utils::{NULL_HASH, read_hash_file, write_hash_file};

pub fn init_repository() -> Result<(), io::Error> {
    let dit_path = PathBuf::from("./.dit");
    if dit_path.is_dir() {
        fs::remove_dir_all(dit_path)?;
    }
    fs::create_dir_all("./.dit/objects")?;
    fs::create_dir("./.dit/refs/")?;
    
    init_object_dir()?;

    init_info_file()?;

    init_staged_file()?;

    File::create("./.dit/deleted")?;

    File::create("./.dit/commit")?;

    Ok(())
}

fn init_object_dir() -> Result<(), io::Error> {
    display_message("Initializing objects directory", Color::DEFAULT);
    display_message("Initialized 255 sub directory objects", Color::DEFAULT);

    let parent_dir = PathBuf::from("./.dit/objects");
    if !parent_dir.exists() {
        fs::create_dir(&parent_dir)?;
    }

    for i in 0..=255 {
        let folder_name = format!("{:02x}", i);

        let folder_path = parent_dir.join(folder_name);

        fs::create_dir(&folder_path)?;
    }

    display_message("Initialized 255 sub directory objects", Color::DEFAULT);
    display_message("Initialized objects directory", Color::DEFAULT);

    Ok(())
}

fn init_info_file() -> Result<(), io::Error> {
    display_message("Initializing info file", Color::DEFAULT);
    File::create("./.dit/info")?;

    Branch::new_branch(String::from("main"), String::from(NULL_HASH)).unwrap();
    display_message("Initialized info file", Color::DEFAULT);
    Ok(())
}

fn init_staged_file() -> Result<(), io::Error> {
    display_message("Initializing staged file", Color::DEFAULT);
    let file = File::create("./.dit/staged")?;

    write_hash_file(String::from(NULL_HASH), &file, 0)?;
    display_message("Initialized staged file", Color::DEFAULT);
    Ok(())
}

pub fn open_object_file(hash: String) -> Result<File, io::Error> {
    let b_hash = &hash[..2];
    let e_hash = &hash[2..];

    let object_dir = find_objects().join(b_hash);
    if object_dir.exists() {
        let object_file = object_dir.join(e_hash);
        if object_file.exists() {
            let file = File::open(object_file)?;
            return Ok(file);
        }
    }
    Err(io::Error::new(io::ErrorKind::NotFound, "Error file not found in objects: {hash}"))
}

pub fn get_staged_hash() -> Result<String, DitError> {
    let staged_path = find_staged();
    let file = File::open(staged_path).map_err(DitError::IoError)?;
    Ok(read_hash_file(file, 0))
}

pub fn get_head_hash() -> Result<String, DitError> {
    let info_path = find_info();
    let file = File::open(info_path).map_err(DitError::IoError)?;
    Ok(read_hash_file(file, 5))
}

pub fn find_objects() -> PathBuf {
    let dit_path = find_dit().unwrap();
    dit_path.join("objects")
}

pub fn find_refs() -> PathBuf {
    find_dit().unwrap().join("refs")
}

pub fn find_staged() -> PathBuf {
    let dit_path = find_dit().unwrap();
    dit_path.join("staged")
}

pub fn find_info() -> PathBuf {
    let dit_path = find_dit().unwrap();
    dit_path.join("info")
}

pub fn find_dit() -> Option<PathBuf> {
    let mut current_path = env::current_dir().ok()?;

    loop {
        let dit = current_path.join(".dit");
        if dit.is_dir() {
            return Some(dit);
        }
        current_path = current_path.parent()?.to_path_buf();
    }
}

pub fn is_init() -> bool {
    if let Some(_) = find_dit() {
        return true;
    }
    return false;
}

pub fn get_object_path(objects_path: &PathBuf, hash: &String) -> Result<PathBuf, io::Error> {
    let b_hash = &hash[..2];
    let e_hash = &hash[2..];

    let object_dir_path = objects_path.join(b_hash);
    if !object_dir_path.exists() {
        create_dir(&object_dir_path)?;
    }
    let node_path = object_dir_path.join(e_hash);
    Ok(node_path)
}
