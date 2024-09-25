use crate::utils::{NULL_HASH, read_hash_file, write_hash_file};
use std::fs::{create_dir, File};
use std::path::{Path, PathBuf};
use std::{fs, io};
use colored::Colorize;
use crate::error::DitError;
use crate::objects::branch::Branch;

pub fn init_repository() -> Result<(), io::Error> {
    if Path::new("./.dit").exists() {
        println!("{}", "dit is already initialized".blue());
        return Ok(());
    }

    fs::create_dir_all("./.dit/objects")?;
    fs::create_dir_all("./.dit/refs/")?;

    init_info_file()?;
    
    init_staged_file()?;
    
    File::create("./.dit/deleted")?;
    
    File::create("./.dit/commit")?;

    Ok(())
}

fn init_info_file() -> Result<(), io::Error> {
    File::create("./.dit/info")?;
    
    Branch::new_branch(String::from("main"), String::from(NULL_HASH)).unwrap();

    Ok(())
}

fn init_staged_file() -> Result<(), io::Error> {
    let file = File::create("./.dit/staged")?;

    write_hash_file(String::from(NULL_HASH), &file, 0)?;
    
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

pub fn get_head_hash()  -> Result<String, DitError> {
    let info_path = find_info();
    let file = File::open(info_path).map_err(DitError::IoError)?;
    Ok(read_hash_file(file, 5))
}

pub fn find_objects() -> PathBuf{
    let dit_path = find_dit();
    dit_path.join("objects")
}

pub fn find_refs() -> PathBuf{
    find_dit().join("refs")
}

pub fn find_staged() -> PathBuf {
    let dit_path = find_dit();
    dit_path.join("staged")
}

pub fn find_info() -> PathBuf {
    let dit_path = find_dit();
    dit_path.join("info")
}

pub fn find_dit() -> PathBuf {
    PathBuf::from("./.dit")
}

pub fn is_init() -> bool {
    find_dit().is_dir()
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