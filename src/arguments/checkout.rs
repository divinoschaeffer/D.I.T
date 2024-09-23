use std::io;
use crate::arguments::init::{ find_refs, get_head_hash};
use crate::objects::branch::Branch;
use crate::objects::commit::Commit;
use crate::utils::{NULL_HASH};

pub fn checkout(name: &String) -> Result<(), io::Error>{

    let branch_path = find_refs().join(name.to_owned());
    
    if branch_path.exists() {
        Commit::get_commit_from_file(get_head_hash()).delete_files();
        
        let branch_commits = Commit::get_commit_list(name.to_string());
        
        match branch_commits.last() {
            Some(c) => { 
                c.recreate_files();
                Branch::set_info_file(name.clone(),c.get_hash().clone())?;
            },
            None => {
                Branch::set_info_file(name.clone(), NULL_HASH.to_string())?;
            }
        }
    }
    
    Ok(())
}