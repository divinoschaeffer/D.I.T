use std::fs::OpenOptions;
use std::path::PathBuf;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{find_objects, find_staged, get_staged_hash, is_init};
use crate::objects::file_objects::node_type::NodeType;
use crate::objects::file_objects::tree::Tree;
use crate::utils::{NULL_HASH, write_hash_file};

pub fn add(elements: Vec<&String>) -> Result<(), DitError> {
    if !is_init() {
        return Err(DitError::NotInitialized);
    }

    let object_path = find_objects();
    let staged_path = find_staged();
    let staged_hash = get_staged_hash()?;

    let new_elements: Vec<String> = elements.iter().map(|s| s.to_string()).collect();

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

/// Add files to dit repository
fn add_elements(
    elements: &Vec<String>,
    object_path: &PathBuf,
    staged_path: &PathBuf,
    root: &mut NodeType,
) -> Result<(), DitError> {
    for element in elements {
        root.create_repository_tree(element)?;
    }

    let root_hash = root.create_node_hash();

    root.transcript_to_files(&object_path)?;

    let file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(staged_path)
        .map_err(DitError::IoError)?;

    write_hash_file(root_hash, &file, 0).map_err(DitError::IoError)?;

    Ok(())
}