use std::io;
use crate::objects::branch::Branch;
use crate::objects::commit::Commit;

pub fn show_commit() -> Result<(), io::Error>{
    let branch: Branch = Branch::get_branch_from_file();
    println!("Branch: {}\nCommit tree:\n", branch.get_name());
    Commit::display_commit_tree();
    Ok(())
}