use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::is_init;
use crate::objects::branch::Branch;
use crate::objects::commit::Commit;

pub fn show_commit() -> Result<(), DitError> {
    if !is_init() {
        return Err(DitError::NotInitialized);
    }

    let branch: Branch = Branch::get_current_branch()?;
    display_message(format!("Branch: {}\nCommit tree:\n", branch.get_name()).as_str(), Color::GREEN);
    Commit::display_commit_tree()?;
    Ok(())
}