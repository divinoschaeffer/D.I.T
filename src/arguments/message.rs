use std::fs::{OpenOptions};
use std::io;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

pub fn message(message: String) -> Result<(), io::Error> {
    
    let desc_path = PathBuf::from("./.dit/commit");
    
    let file = OpenOptions::new()   
        .write(true)
        .truncate(true)
        .append(false)
        .create(true)
        .open(desc_path).unwrap();
    
    let mut writer = BufWriter::new(file);
    write!(writer,"{}",message).unwrap_or_else(|e| {
        panic!("Error while writing commit message: {e}");
    });
    Ok(())
}