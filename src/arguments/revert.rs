use std::fs::OpenOptions;
use std::io;
use crate::arguments::init::{find_info, get_head_hash};
use crate::objects::commit::Commit;
use crate::utils::{NULL_HASH, write_hash_file};

pub fn revert(hash: String) -> Result<(), io::Error>{
    
    if Commit::commit_exist(&hash) {
        let head = get_head_hash();
        if head == NULL_HASH {
            println!("Commit staged files before revert")
        } else {
            let info_path = find_info();
            
            Commit::get_commit_from_file(head).delete_files();
            let commit  = Commit::get_commit_from_file(hash);
            commit.recreate_files();

            let info_file = OpenOptions::new()
                .write(true)
                .append(false)
                .create(false)
                .open(info_path).unwrap();

            write_hash_file(commit.get_hash().clone(),&info_file,5).unwrap();
        }
    } else { 
        println!("Commit ID not recognized");
    }
    Ok(())
}