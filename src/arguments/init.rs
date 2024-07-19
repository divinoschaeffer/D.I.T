use crate::utils::{write_footer_file, write_hash_file, write_header_file};
use std::fs::File;
use std::path::Path;
use std::{fs, io};

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
        String::from("0000000000000000000000000000000000000000"),
        &file,
        5,
    )?;
    write_footer_file(String::from("main"), file, 46)?;

    Ok(())
}

fn init_staged_file() -> Result<(), io::Error> {
    let file = File::create("./.dit/staged")?;

    write_hash_file(String::from("0000000000000000000000000000000000000000"), &file, 0)?;
    
    Ok(())
}