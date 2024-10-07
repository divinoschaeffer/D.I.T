use std::process::Command;

use crate::arguments::init::{find_dit, find_objects, get_staged_hash, is_init};
use crate::display_message::{Color, display_message};
use crate::error::DitError;
use crate::objects::commit::Commit;
use crate::objects::file_objects::node_type::NodeType;
use crate::objects::file_objects::tree::Tree;
use crate::utils::{NULL_HASH, path_from_dit, read_content_file_from_path};

use super::delete::get_deleted_elements;
use super::init::get_head_hash;
use super::rm::find_element_to_remove;

pub fn commit(desc_already_set: bool) -> Result<(), DitError> {
    if !is_init() {
        return Err(DitError::NotInitialized);
    }

    let dit_path = find_dit().unwrap();
    let desc_path = dit_path.join("commit");
    let staged_hash = get_staged_hash()?;

    if staged_hash == NULL_HASH {
        display_message("You need to stage elements before commiting", Color::BLUE);
        return Ok(());
    } else if is_first_commit()? {
        if !desc_already_set {
            Command::new("nano")
                .arg(desc_path.clone())
                .spawn()
                .expect("Failed to open nano")
                .wait()
                .expect("Error with running nano");
        }

        let description = read_content_file_from_path(&desc_path.as_path()).unwrap_or_default();

        create_commit(description, String::from(NULL_HASH), staged_hash)?;
    } else {
        if !desc_already_set {
            Command::new("nano")
                .arg(desc_path.clone())
                .spawn()
                .expect("Failed to open nano")
                .wait()
                .expect("Error with running nano");
        }

        let description = read_content_file_from_path(&desc_path.as_path()).unwrap_or_default();
        let last_commit_hash = get_head_hash()?;

        let last_commit = Commit::get_commit_from_file(last_commit_hash.clone()).map_err(DitError::IoError)?;

        let mut staged_tree = Tree::new(String::from(""), Vec::new(), String::from(""));
        staged_tree.get_tree_from_file(staged_hash.clone())?;
        staged_tree.set_hash(staged_hash);
        let mut staged_root = NodeType::Tree(staged_tree);

        let mut last_commit_tree = Tree::new(String::from(""), Vec::new(), String::from(""));
        last_commit_tree.get_tree_from_file(last_commit.get_tree().to_string())?;
        last_commit_tree.set_hash(last_commit.get_tree().to_string());
        let mut last_commit_root: NodeType = NodeType::Tree(last_commit_tree);

        let option_deleted_elements = get_deleted_elements()?;

        match option_deleted_elements {
            Some(deleted_elements) => {
                for deleted_element in deleted_elements {
                    let real_path = path_from_dit(&deleted_element)?;

                    let mut ancestors: Vec<_> = real_path.ancestors().collect();
                    ancestors.pop();
                    ancestors.reverse();

                    find_element_to_remove(&mut last_commit_root, &mut ancestors);
                    find_element_to_remove(&mut staged_root, &mut ancestors);
                }
            }
            None => ()
        }

        if let Some(mut result) = NodeType::fuse(last_commit_root, staged_root) {
            let commit_tree_hash = result.create_node_hash();
            result.transcript_to_files(&find_objects())?;

            create_commit(description, last_commit_hash, commit_tree_hash)?;
        } else {
            Err(DitError::UnexpectedComportement("Fail to create commit".to_string()))?
        }
    }
    Ok(())
}

pub fn create_commit(description: String, last_commit_hash: String, commit_tree_hash: String) -> Result<(), DitError> {
    let parent = last_commit_hash;
    let tree = commit_tree_hash;
    let commit: Commit = Commit::new(tree, String::from(parent), description);

    commit.transcript_commit_to_file()?;

    Commit::reset_description_file().map_err(DitError::IoError)?;
    Ok(())
}

fn is_first_commit() -> Result<bool, DitError> {
    Ok(get_head_hash()? == NULL_HASH)
}

