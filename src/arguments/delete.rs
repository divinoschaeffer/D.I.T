use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};
use crate::utils::NULL_HASH;

use crate::arguments::init::{find_dit, get_staged_hash};

pub fn delete(elements: Vec<&String>) -> Result<(), io::Error> {

    let dit_path = find_dit().unwrap_or_else(|| {
        panic!("dit is not initialized");
    });
    let deleted_path = dit_path.join("deleted");
    let staged_hash = get_staged_hash();

    if staged_hash == NULL_HASH {
        println!("Elements need to be commited first");
    }
    else {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(deleted_path).unwrap();
        
        let mut writer = BufWriter::new(file);
        for element in elements {
            writeln!(writer, "{}", element).unwrap_or_else(|e| {
                panic!("Error while writing in deleted files: {e}");
            });
        }
    }
    Ok(())
}