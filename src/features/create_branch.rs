use std::process;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{get_head_hash, is_init};
use crate::objects::branch::Branch;

pub fn new_branch(name: &String) -> Result<(), DitError> {
    if !is_init() {
        display_message("dit repository is not initialized.", Color::RED);
        process::exit(1);
    }

    let head = get_head_hash()?;
    Branch::new_branch(name.to_owned(), head)?;
    display_message(format!("branch {} created.", name).as_str(), Color::GREEN);
    Ok(())
}