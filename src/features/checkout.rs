use std::process;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{find_refs, get_head_hash, is_init};
use crate::objects::branch::Branch;
use crate::objects::commit::Commit;
use crate::utils::NULL_HASH;

pub fn checkout(name: &String) -> Result<(), DitError> {
    if !is_init() {
        display_message("dit repository is not initialized.", Color::RED);
        process::exit(1);
    }

    let branch_path = find_refs().join(name.to_owned());

    if branch_path.exists() {
        Commit::get_commit_from_file(get_head_hash()?).map_err(DitError::IoError)?;

        let branch_commits = Commit::get_commit_list(name.to_string()).map_err(DitError::IoError)?;

        match branch_commits.last() {
            Some(commit) => {
                commit.recreate_files()?;
                Branch::set_info_file(name.clone(), commit.get_hash().clone()).map_err(DitError::IoError)?;
            }
            None => {
                Branch::set_info_file(name.clone(), NULL_HASH.to_string()).map_err(DitError::IoError)?;
            }
        }
    }

    Ok(())
}