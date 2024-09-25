use colored::Colorize;
use crate::error::DitError;
use crate::objects::branch::Branch;
use crate::objects::commit::Commit;

pub fn show_commit() -> Result<(), DitError>{
    let branch: Branch = Branch::get_current_branch()?;
    println!("Branch: {}\nCommit tree:\n", branch.get_name().green());
    Commit::display_commit_tree()?;
    Ok(())
}