use std::fs::{self, OpenOptions};
use std::{io};
use std::path::PathBuf;
use crate::arguments::init::{find_objects, find_staged, get_staged_hash, is_init};
use crate::display_message::{Color, display_message};
use crate::error::DitError;
use crate::objects::file_objects::node_type::NodeType;
use crate::objects::file_objects::tree::Tree;
use crate::utils::{NULL_HASH, write_hash_file};

pub fn add(elements: Vec<&String>) -> Result<(), DitError> {
    if !is_init() {
        return Err(DitError::NotInitialized)
    }

    let object_path = find_objects();
    let staged_path = find_staged();
    let staged_hash = get_staged_hash()?;

    let new_elements = process_elements(elements)?;

    if new_elements.is_empty() {
        display_message("You need to specify files to add", Color::BLUE);
    } else if staged_hash == NULL_HASH {
        
        let tree: Tree = Default::default();
        let mut root: NodeType = NodeType::Tree(tree);

        add_elements(&new_elements, &object_path, &staged_path, &mut root)?;

    } else {
        let mut tree: Tree = Default::default();
        tree.get_tree_from_file(staged_hash)?;
        let mut root = NodeType::Tree(tree);

        add_elements(&new_elements, &object_path, &staged_path, &mut root)?;
    }
    Ok(())
}

fn add_elements(elements: &Vec<String>, object_path: &PathBuf, staged_path: &PathBuf, root: &mut NodeType) -> Result<(), DitError> {
    for element in elements {
        root.create_repository_tree(element)?;
    }

    let root_hash = root.create_node_hash();
    
    root.transcript_to_files(&object_path)?;

    let file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(staged_path).map_err(DitError::IoError)?;

    write_hash_file(root_hash, &file, 0).map_err(DitError::IoError)?;

    Ok(())
}

fn process_elements(elements: Vec<&String>) -> Result<Vec<String>, DitError> {
    let mut owned_elements: Vec<String> = elements.iter().map(|s| s.to_string()).collect();
    let mut index = 0;

    while index < owned_elements.len() {
        let element = &owned_elements[index];
        let path = PathBuf::from(element);

        if path.is_dir() {
            match fs::read_dir(path) {
                Ok(paths) => {
                    for path in paths {
                        if let Ok(entry) = path {
                            if let Some(literal) = entry.path().to_str() {
                                owned_elements.push(literal.to_string());
                            }
                        }
                    }
                }
                Err(e) => Err(DitError::IoError(io::Error::new(io::ErrorKind::NotFound,e)))?
            }
        }
        index += 1;
    }
    Ok(owned_elements)
}