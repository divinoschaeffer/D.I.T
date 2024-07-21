use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use crate::arguments::add::{create_tree_node_from_file, transcript_tree_to_files};
use crate::arguments::init::{find_dit, get_staged_hash};
use crate::objects::NodeType;
use crate::utils::{NULL_HASH, real_path, write_hash_file};

pub fn rm(elements: Vec<&String>) -> Result<(), io::Error> {
    let dit_path = find_dit().unwrap_or_else(|| {
        panic!("dit is not initialized");
    });
    
    let staged_hash = get_staged_hash();
    let object_path = dit_path.join("objects");
    let staged_path = dit_path.join("staged");
    
    if elements.is_empty() {
        println!("You need to specify files to remove");
    } else if staged_hash == NULL_HASH {
        println!("You need to add files before remove them");
    } else {
        let mut tree = crate::objects::Tree::new(String::from(""), Vec::new(), String::from(staged_hash.clone()));
        
        create_tree_node_from_file(staged_hash, &mut tree);

        let mut root = NodeType::Tree(tree);
        
        for element in elements {
            let real_path = real_path(element);
            
            let mut ancestors: Vec<_> = real_path.ancestors().collect();
            ancestors.pop();
            ancestors.reverse();
            
            find_element_to_remove(&mut root, &mut ancestors);
        }

        let root_hash = NodeType::create_node_hash(&mut root);

        transcript_tree_to_files(&mut root, &object_path);

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(staged_path).unwrap();

        write_hash_file(root_hash, &file, 0).unwrap();
    }
    
    Ok(())
}

fn find_element_to_remove(root: &mut NodeType, paths: &mut Vec<&Path>) {
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
                    tree.get_nodes().remove(index);
                }
            } else {
                if let Some(index) = tree_copy.find_node_index(node) {
                    let children = tree.get_nodes();
                    paths.remove(0);
                    find_element_to_remove(&mut children[index], paths);
                }
            }
        }
    }    
}