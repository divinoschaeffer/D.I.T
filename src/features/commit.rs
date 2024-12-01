use std::process::Command;

use repository_tree_creator::features::get_repository_tree_from_object_files::get_repository_tree_from_object_files;
use repository_tree_creator::features::merge_repository_trees::{merge_repository_trees, Mode};
use repository_tree_creator::features::remove_element_from_repository_tree::remove_element_from_repository_tree;
use repository_tree_creator::features::transcript_repository_tree_to_object_files::transcript_repository_to_object_files;
use repository_tree_creator::models::node::Node;
use repository_tree_creator::models::node::Node::TreeNode;
use repository_tree_creator::models::tree::Tree;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{find_dit, get_staged_hash, is_init};
use crate::objects::commit::Commit;
use crate::utils::{NULL_HASH, path_from_dit, read_content_file_from_path};

use super::delete::get_deleted_elements;
use super::init::get_head_hash;

pub fn commit(desc_already_set: bool) -> Result<(), DitError> {
    if !is_init() {
        return Err(DitError::NotInitialized);
    }

    let dit_path = find_dit().unwrap();
    let desc_path = dit_path.join("commit");
    let objects_path = dit_path.join("objects");
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
            Command::new("vim")
                .arg(desc_path.clone())
                .spawn()
                .expect("Failed to open vim")
                .wait()
                .expect("Error with running vim");
        }

        let description = read_content_file_from_path(&desc_path.as_path()).unwrap_or_default();
        let last_commit_hash = get_head_hash()?;

        let last_commit = Commit::get_commit_from_file(last_commit_hash.clone()).map_err(DitError::IoError)?;

        let mut staged_tree = Tree::default();
        get_repository_tree_from_object_files(&mut staged_tree, &staged_hash, &objects_path).map_err(|e| {
            display_message("Error getting repository files from objects directory", Color::RED);
            DitError::UnexpectedComportement(format!("Error details: {}", e))
        })?;
        staged_tree.set_id(staged_hash);
        let mut staged_root = TreeNode(staged_tree);

        let mut last_commit_tree = Tree::default();
        get_repository_tree_from_object_files(&mut last_commit_tree, last_commit.get_tree(), &objects_path).map_err(|e| {
            display_message("Error getting repository files from objects directory", Color::RED);
            DitError::UnexpectedComportement(format!("Error details: {}", e))
        })?;
        last_commit_tree.set_id(last_commit.get_tree().to_string());
        let mut last_commit_root: Node = TreeNode(last_commit_tree);

        let option_deleted_elements = get_deleted_elements()?;

        match option_deleted_elements {
            Some(deleted_elements) => {
                for deleted_element in deleted_elements {
                    let real_path = path_from_dit(&deleted_element)?;
                    remove_element_from_repository_tree(&mut last_commit_root, &real_path).map_err(|e2| {
                        DitError::UnexpectedComportement(format!("{}", e2))
                    })?;
                    remove_element_from_repository_tree(&mut staged_root, &real_path).map_err(|e2| {
                        DitError::UnexpectedComportement(format!("{}", e2))
                    })?;
                }
            }
            None => ()
        }

        if let Some(result) = merge_repository_trees(last_commit_root, staged_root, &Mode::Partial) {
            transcript_repository_to_object_files(&result, &dit_path.join("objects")).map_err(|e1| {
                display_message("Error transcribing repository files from objects directory", Color::RED);
                DitError::UnexpectedComportement(format!("Error details: {}", e1))
            })?;
            create_commit(description, last_commit_hash, result.get_id())?;
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

