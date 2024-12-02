use std::fs::OpenOptions;
use std::path::Path;
use std::process;

use repository_tree_creator::features::get_repository_tree_from_object_files::get_repository_tree_from_object_files;
use repository_tree_creator::features::transcript_repository_tree_to_object_files::transcript_repository_to_object_files;
use repository_tree_creator::models::node::Node;
use repository_tree_creator::models::node::Node::TreeNode;
use repository_tree_creator::models::tree::Tree;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{find_objects, find_staged, get_staged_hash, is_init};
use crate::utils::{NULL_HASH, path_from_dit, write_hash_file};

pub fn rm(elements: Vec<&String>) -> Result<(), DitError> {
    if !is_init() {
        display_message("dit repository is not initialized.", Color::RED);
        process::exit(1);
    }

    let staged_hash = get_staged_hash()?;
    let object_path = find_objects();
    let staged_path = find_staged();

    if elements.is_empty() {
        display_message("You need to specify files to remove", Color::BLUE);
    } else if staged_hash == NULL_HASH {
        display_message("You need to add files before remove them", Color::BLUE);
    } else {
        let mut tree = Tree::default();

        get_repository_tree_from_object_files(&mut tree, &staged_hash, &object_path).map_err(|e| {
            display_message("Error getting files", Color::RED);
            DitError::UnexpectedComportement(format!("Details: {}", e))
        })?;

        let mut root = TreeNode(tree);

        for element in elements {
            let real_path = path_from_dit(element)?;

            let mut ancestors: Vec<_> = real_path.ancestors().collect();
            ancestors.pop();
            ancestors.reverse();

            find_element_to_remove(&mut root, &mut ancestors);
        }
        transcript_repository_to_object_files(&root, &object_path).map_err(|e1| {
            display_message("Error recreating files.", Color::RED);
            DitError::UnexpectedComportement(format!("Details: {}.", e1))
        })?;

        let file = OpenOptions::new()
            .write(true)
            .append(false)
            .create(true)
            .open(staged_path).unwrap();

        write_hash_file(root.get_id(), &file, 0).unwrap();
    }

    Ok(())
}

pub fn find_element_to_remove(root: &mut Node, paths: &mut Vec<&Path>) {
    if paths.is_empty() {
        return;
    }
    let path = paths[0];
    let file_name = path.file_name().unwrap().to_str().unwrap();

    if let TreeNode(ref mut tree) = root {
        let mut tree_copy = tree.clone();
        if let Some(node) = tree.get_mut_children()
            .iter()
            .find(|x| x.get_name() == file_name) {
            if paths.len() == 1 {
                if let Some(index) = tree_copy
                    .get_mut_children()
                    .iter()
                    .position(|x1| x1.get_path() == node.get_path() && Node::is_same_type(x1, node)) {
                    tree.get_mut_children().remove(index);
                }
            } else {
                if let Some(index) = tree_copy
                    .get_mut_children()
                    .iter()
                    .position(|x1| x1.get_path() == node.get_path() && Node::is_same_type(x1, node)) {
                    let children = tree.get_mut_children();
                    paths.remove(0);
                    find_element_to_remove(&mut children[index], paths);
                }
            }
        }
    }
}