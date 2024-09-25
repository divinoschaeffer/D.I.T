use std::fs::OpenOptions;
use std::path::Path;
use colored::Colorize;
use crate::arguments::init::{find_dit, get_staged_hash, is_init};
use crate::error::DitError;
use crate::objects::file_objects::node_type::NodeType;
use crate::objects::file_objects::tree::Tree;
use crate::utils::{NULL_HASH, real_path, write_hash_file};

pub fn rm(elements: Vec<&String>) -> Result<(), DitError> {

    if !is_init() {
        return Err(DitError::NotInitialized)
    }
    
    let dit_path = find_dit();
    
    let staged_hash = get_staged_hash()?;
    let object_path = dit_path.join("objects");
    let staged_path = dit_path.join("staged");
    
    if elements.is_empty() {
        println!("{}", "You need to specify files to remove".blue());
    } else if staged_hash == NULL_HASH {
        println!("{}", "You need to add files before remove them".blue());
    } else {
        let mut tree = Tree::new(String::from(""), Vec::new(), String::from(staged_hash.clone()));
        
        tree.get_tree_from_file(staged_hash)?;

        let mut root = NodeType::Tree(tree);
        
        for element in elements {
            let real_path = real_path(element)?;
            
            let mut ancestors: Vec<_> = real_path.ancestors().collect();
            ancestors.pop();
            ancestors.reverse();
            
            find_element_to_remove(&mut root, &mut ancestors);
        }

        let root_hash = root.create_node_hash();

        root.transcript_to_files(&object_path)?;

        let file = OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(staged_path).unwrap();

        write_hash_file(root_hash, &file, 0).unwrap();
    }
    
    Ok(())
}

pub fn find_element_to_remove(root: &mut NodeType, paths: &mut Vec<&Path>) {
    if paths.is_empty() { 
        return;
    }
    let path = paths[0];
    let file_name = path.file_name().unwrap().to_str().unwrap();
    
    if let NodeType::Tree(ref mut tree) = root {
        let tree_copy = tree.clone();
        if let Some(node) = tree.find_node_by_name(String::from(file_name)) {
            if paths.len() == 1 {
                if let Some(index) = tree_copy.find_node_index(node) {
                    tree.get_mut_nodes().remove(index);
                }
            } else {
                if let Some(index) = tree_copy.find_node_index(node) {
                    let children = tree.get_mut_nodes();
                    paths.remove(0);
                    find_element_to_remove(&mut children[index], paths);
                }
            }
        }
    }    
}