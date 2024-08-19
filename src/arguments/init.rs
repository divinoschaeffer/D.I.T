use crate::utils::{NULL_HASH, read_hash_file, write_footer_file, write_hash_file, write_header_file};
use std::fs::{create_dir, File};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub fn init_repository() -> Result<(), io::Error> {
    if Path::new("./.dit").exists() {
        println!("dit is already initialized");
        return Ok(());
    }

    fs::create_dir_all("./.dit/objects")?;
    fs::create_dir("./.dit/refs")?;

    init_info_file()?;
    
    init_staged_file()?;

    Ok(())
}

fn init_info_file() -> Result<(), io::Error> {
    let file = File::create("./.dit/info")?;

    write_header_file(String::from("HEAD"), &file, 0)?;
    write_hash_file(
        String::from(NULL_HASH),
        &file,
        5,
    )?;
    write_footer_file(String::from("main"), file, 46)?;

    Ok(())
}

fn init_staged_file() -> Result<(), io::Error> {
    let file = File::create("./.dit/staged")?;

    write_hash_file(String::from(NULL_HASH), &file, 0)?;
    
    Ok(())
}

pub fn open_object_file(hash: String) -> File {
    let b_hash = &hash[..2];
    let e_hash = &hash[2..];
    
    let object_dir = find_objects().join(b_hash);
    if object_dir.exists() {
        let object_file = object_dir.join(e_hash);
        if object_file.exists() {
            let file = File::open(object_file).unwrap_or_else(|e| {
                panic!("Error while opening object file : {e}")
            });
            return file;
        }
    }
    panic!("Error file not found in objects: {hash}");
}

pub fn get_staged_hash() -> String {
    let staged_path = find_staged();
    let file = File::open(staged_path).unwrap_or_else(|e| {
        panic!("Error while opening staged file: {e}");
    });
    read_hash_file(file, 0)
}

pub fn get_head_hash()  -> String {
    let info_path = find_info();
    let file = File::open(info_path).unwrap_or_else(|e| {
        panic!("Error while opening staged file: {e}");
    });
    read_hash_file(file, 5)
}

pub fn find_objects() -> PathBuf{
    let dit_path = find_dit().unwrap();
    dit_path.join("objects")
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
    let initial_dir  = env::current_dir()
        .expect("Failed to get current directory");

    let mut current_dir = initial_dir.clone();

    loop {
        let dit_path = current_dir.join(".dit");
        if dit_path.exists() && dit_path.is_dir() {
            return Some(PathBuf::from(dit_path))
        }

        match current_dir.parent() {
            Some(parent) => current_dir = parent.to_path_buf(),
            None => break,
        }
    }

    None
}

pub fn get_object_path(objects_path: &PathBuf, hash: &String) -> PathBuf {
    let b_hash = &hash[..2];
    let e_hash = &hash[2..];

    let object_dir_path = objects_path.join(b_hash);
    if !object_dir_path.exists() {
        create_dir(&object_dir_path).unwrap_or_else(|e| {
            panic!("Error while creating directory in objects directory: {e}");
        })
    }
    let node_path = object_dir_path.join(e_hash);
    node_path
}