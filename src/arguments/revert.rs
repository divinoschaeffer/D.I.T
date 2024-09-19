use std::io;
use crate::objects::commit::Commit;

pub fn revert(hash: String) -> Result<(), io::Error>{
    
    if Commit::commit_exist(&hash) {
        println!("Commit ID recognized");
        let commit  = Commit::get_commit_from_file(hash);
        commit.recreate_files();
    } else { 
        println!("Commit ID not recognized");
    }
    Ok(())
}