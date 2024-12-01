use crate::error::DitError;
use crate::features::init::{get_head_hash, is_init};
use crate::objects::branch::Branch;

pub fn new_branch(name: &String) -> Result<(), DitError> {
    if !is_init() {
        println!("init");
        return Err(DitError::NotInitialized);
    }

    let head = get_head_hash()?;
    Branch::new_branch(name.to_owned(), head)?;

    Ok(())
}