use crate::arguments::init::get_head_hash;
use crate::error::DitError;
use crate::objects::branch::Branch;

pub fn new_branch(name: &String) -> Result<(), DitError> {
    
    let head = get_head_hash()?;
    Branch::new_branch(name.to_owned(), head)?;
    
    Ok(())
}