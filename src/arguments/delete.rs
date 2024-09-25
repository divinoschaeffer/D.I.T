use std::fs::OpenOptions;
use std::io::{ BufRead, BufReader, BufWriter, Write};
use colored::Colorize;
use crate::utils::NULL_HASH;

use crate::arguments::init::{find_dit, get_staged_hash, is_init};
use crate::error::DitError;

pub fn delete(elements: Vec<&String>) -> Result<(), DitError> {

    if !is_init() {
        return Err(DitError::NotInitialized)
    }

    let dit_path = find_dit();
    let deleted_path = dit_path.join("deleted");
    let staged_hash = get_staged_hash()?;

    if staged_hash == NULL_HASH {
        println!("{}", "Elements need to be commited first".blue());
    }
    else {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(deleted_path).unwrap();
        
        let mut writer = BufWriter::new(file);
        for element in elements {
            writeln!(writer, "{}", element).map_err(DitError::IoError)?;
        }
    }
    Ok(())
}

pub fn get_deleted_elements() -> Result<Option<Vec<String>>, DitError>{
    let dit_path = find_dit();
    let deleted_path = dit_path.join("deleted");

    let file = OpenOptions::new()
        .read(true)
        .open(deleted_path).map_err(DitError::IoError)?;

        let reader = BufReader::new(file);
        let mut elements: Vec<String> = Vec::new();

        for line in reader.lines() {
            let content = match line {
                Ok(content) => content,
                Err(_e) => return Err(DitError::UnexpectedComportement("deleted element not found".to_string())),
            };
            elements.push(content);

        }
    
        Ok(Some(elements))

}