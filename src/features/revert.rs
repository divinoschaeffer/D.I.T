use std::fs::OpenOptions;
use std::process;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{find_info, get_head_hash, is_init};
use crate::objects::commit::Commit;
use crate::utils::{NULL_HASH, write_hash_file};

pub fn revert(hash: String) -> Result<(), DitError> {
    if !is_init() {
        display_message("dit repository is not initialized.", Color::RED);
        process::exit(1);
    }

    if Commit::commit_exist(&hash)? {
        let head = get_head_hash()?;
        if head == NULL_HASH {
            display_message("Commit staged files before revert", Color::BLUE);
        } else {
            let info_path = find_info();

            Commit::get_commit_from_file(head).map_err(DitError::IoError)?;
            let commit = Commit::get_commit_from_file(hash).map_err(DitError::IoError)?;
            commit.recreate_files()?;

            let info_file = OpenOptions::new()
                .write(true)
                .append(false)
                .create(false)
                .open(info_path).map_err(DitError::IoError)?;

            write_hash_file(commit.get_hash().clone(), &info_file, 5).map_err(DitError::IoError)?;
        }
    } else {
        display_message("Commit ID not recognized", Color::BLUE);
    }
    Ok(())
}