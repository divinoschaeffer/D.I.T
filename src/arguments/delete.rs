use std::io;
use std::path::PathBuf;
use crate::objects::node_type::NodeType;
use crate::objects::tree::Tree;
use crate::utils::NULL_HASH;

use crate::arguments::init::{find_dit, get_staged_hash};

pub fn delete(elements: Vec<&String>) -> Result<(), io::Error> {

    let dit_path = find_dit().unwrap_or_else(|| {
        panic!("dit is not initialized");
    });
    
    let object_path = dit_path.join("objects");
    let staged_path = dit_path.join("staged");
    let staged_hash = get_staged_hash();

    if staged_hash == NULL_HASH {
        println!("Elements need to be commited first");
    }
    else {
        let mut tree = Tree::new(String::from(""), Vec::new(), String::from(staged_hash.clone()));
        tree.create_tree_node_from_file(staged_hash);
        let mut root = NodeType::Tree(tree);
    }
    Ok(())
}