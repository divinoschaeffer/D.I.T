use std::fs::OpenOptions;
use colored::Colorize;
use crate::arguments::init::{find_info, get_head_hash, is_init};
use crate::error::DitError;
use crate::objects::commit::Commit;
use crate::utils::{NULL_HASH, write_hash_file};

pub fn revert(hash: String) -> Result<(), DitError>{

    if !is_init() {
        return Err(DitError::NotInitialized)
    }
    
    if Commit::commit_exist(&hash)? {
        let head = get_head_hash()?;
        if head == NULL_HASH {
            println!("{}", "Commit staged files before revert".blue())
        } else {
            let info_path = find_info();
            
            Commit::get_commit_from_file(head).map_err(DitError::IoError)?.delete_files()?;
            let commit  = Commit::get_commit_from_file(hash).map_err(DitError::IoError)?;
            commit.recreate_files()?;

            let info_file = OpenOptions::new()
                .write(true)
                .append(false)
                .create(false)
                .open(info_path).map_err(DitError::IoError)?;

            write_hash_file(commit.get_hash().clone(),&info_file,5).map_err(DitError::IoError)?;
        }
    } else { 
        println!("{}", "Commit ID not recognized".blue());
    }
    Ok(())
}