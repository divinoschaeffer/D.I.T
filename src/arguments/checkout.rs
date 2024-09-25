use crate::arguments::init::{find_refs, get_head_hash, is_init};
use crate::error::DitError;
use crate::objects::branch::Branch;
use crate::objects::commit::Commit;
use crate::utils::{NULL_HASH};

pub fn checkout(name: &String) -> Result<(), DitError>{

    if !is_init() {
        return Err(DitError::NotInitialized)
    }

    let branch_path = find_refs().join(name.to_owned());
    
    if branch_path.exists() {
        Commit::get_commit_from_file(get_head_hash()?).map_err(DitError::IoError)?.delete_files()?;
        
        let branch_commits = Commit::get_commit_list(name.to_string()).map_err(DitError::IoError)?;
        
        match branch_commits.last() {
            Some(c) => { 
                c.recreate_files()?;
                Branch::set_info_file(name.clone(),c.get_hash().clone()).map_err(DitError::IoError)?;
            },
            None => {
                Branch::set_info_file(name.clone(), NULL_HASH.to_string()).map_err(DitError::IoError)?;
            }
        }
    }
    
    Ok(())
}