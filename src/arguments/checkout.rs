use std::io;
use crate::arguments::init::find_refs;
use crate::objects::commit::Commit;

pub fn checkout(name: &String) -> Result<(), io::Error>{

    let branch_path = find_refs().join(name.clone());
    
    if branch_path.exists() {
        let commits = Commit::get_commit_list(name.to_string());
        match commits.last() {
            Some(c) => c.recreate_files(),
            None => ()
        }
    }
    
    Ok(())
}