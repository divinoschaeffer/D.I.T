use std::io::{BufRead, BufReader};
use std::process;

use dit_file_encryptor::CompressedFile;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{find_dit, get_staged_hash, is_init};
use crate::utils::NULL_HASH;

pub fn delete(elements: Vec<&String>) -> Result<(), DitError> {
    if !is_init() {
        display_message("dit repository is not initialized.", Color::RED);
        process::exit(1);
    }

    let dit_path = find_dit().unwrap();
    let deleted_path = dit_path.join("deleted");
    let staged_hash = get_staged_hash()?;

    if staged_hash == NULL_HASH {
        display_message("Elements need to be commited first", Color::BLUE);
    } else {
        for element in elements {
            CompressedFile::new(deleted_path.clone())
                .append_to_file(format!("{}", element).as_bytes())
                .map_err(|e| DitError::IoError(e))?;
        }
    }
    Ok(())
}

pub fn get_deleted_elements() -> Result<Option<Vec<String>>, DitError> {
    let dit_path = find_dit().unwrap();
    let deleted_path = dit_path.join("deleted");

    let mut elements: Vec<String> = Vec::new();

    if deleted_path.metadata()
        .map_err(|e| DitError::IoError(e))?
        .len() != 0 {
        let reader = CompressedFile::new(deleted_path)
            .open_for_read()
            .map_err(|e| DitError::IoError(e))?;

        let buf_reader = BufReader::new(reader);

        for line in buf_reader.lines() {
            let content = match line {
                Ok(content) => content,
                Err(e) => return Err(DitError::UnexpectedComportement(format!("deleted element not found, e: {}", e))),
            };
            elements.push(content);
        }
    }

    Ok(Some(elements))
}
