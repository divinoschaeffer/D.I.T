use std::path::PathBuf;
use std::process;

use repository_tree_creator::features::create_repository_tree as crt;
use repository_tree_creator::features::get_repository_tree_from_object_files::get_repository_tree_from_object_files;
use repository_tree_creator::features::transcript_repository_tree_to_object_files::transcript_repository_to_object_files;
use repository_tree_creator::models::node::Node;
use repository_tree_creator::models::tree::Tree;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{find_objects, find_staged, get_staged_hash, is_init};
use crate::utils::{clean_path, NULL_HASH, write_hash_file};

pub fn add(elements: Vec<&String>) -> Result<(), DitError> {
    if !is_init() {
        display_message("dit repository is not initialized.", Color::RED);
        process::exit(1);
    }

    let object_path = find_objects();
    let staged_path = find_staged();
    let staged_hash = get_staged_hash()?;

    let new_elements: Vec<String> = elements.iter().map(|s| s.to_string()).collect();

    let new_elements: Vec<PathBuf> = new_elements
        .into_iter()
        .map(|p| PathBuf::from(p))
        .collect();

    let new_elements = clean_path(new_elements)?;

    if new_elements.is_empty() {
        display_message("You need to specify files to add.", Color::DEFAULT);
    } else if staged_hash == NULL_HASH {
        let tree: Tree = Default::default();
        add_elements(new_elements, &object_path, &staged_path, tree)?;
    } else {
        let mut tree: Tree = Default::default();
        get_repository_tree_from_object_files(&mut tree, &staged_hash, &object_path).map_err(|e| {
            display_message("Error getting previous staged files", Color::RED);
            DitError::UnexpectedComportement(format!("Details: {}", e))
        })?;
        add_elements(new_elements, &object_path, &staged_path, tree)?;
    }
    Ok(())
}

/// Add files to dit repository
fn add_elements(
    elements: Vec<PathBuf>,
    object_path: &PathBuf,
    staged_path: &PathBuf,
    root: Tree,
) -> Result<(), DitError> {
    let root: Node = crt::create_repository_tree(root, elements).map_err(|e| {
        display_message("Error creating repository tree", Color::RED);
        DitError::UnexpectedComportement(format!("Details: {}", e))
    })?;
    transcript_repository_to_object_files(&root, &object_path).map_err(|e| {
        display_message("Error saving repository tree", Color::RED);
        DitError::UnexpectedComportement(format!("Details: {}", e))
    })?;

    write_hash_file(root.get_id(), staged_path.clone(), 0).map_err(DitError::IoError)?;
    Ok(())
}
