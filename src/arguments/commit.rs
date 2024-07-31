use std::fs::File;
use std::io;
use std::process::Command;
use crate::arguments::init::{find_dit, get_staged_hash};
use crate::commit::Commit;
use crate::utils::{NULL_HASH, read_content_file_from_path, read_hash_file};

pub fn commit() -> Result<(), io::Error>{

    let dit_path = find_dit().unwrap_or_else(|| {
        panic!("dit is not initialized");
    });
    let desc_path = dit_path.join("commit");
    let staged_hash = get_staged_hash();
    
    if staged_hash == NULL_HASH { 
        println!("You need to stage elements before commiting");
        return Ok(())
    } else if is_first_commit() {
        Command::new("nano")
            .arg(desc_path.clone())
            .spawn()
            .expect("Failed to open nano")
            .wait()
            .expect("Error with running nano");
        
        let description = read_content_file_from_path(&desc_path.as_path()).unwrap();
        let parent = NULL_HASH;
        let tree = staged_hash;
        let commit = Commit::new(tree, String::from(parent), description);
        
        commit.display();
        
        commit.transcript_commit_to_file();
    }
    Ok(())
}

fn is_first_commit() -> bool {
    let dit_path = find_dit().unwrap();
    let info_path = dit_path.join("info");
    let file = File::open(info_path).unwrap();
    read_hash_file(file, 5) == NULL_HASH
}

