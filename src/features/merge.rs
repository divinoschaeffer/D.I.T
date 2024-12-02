use std::process;

use repository_tree_creator::features::get_repository_tree_from_object_files::get_repository_tree_from_object_files;
use repository_tree_creator::features::merge_repository_trees::{merge_repository_trees, Mode};
use repository_tree_creator::features::transcript_repository_tree_to_object_files::transcript_repository_to_object_files;
use repository_tree_creator::models::node::Node;
use repository_tree_creator::models::node::Node::TreeNode;
use repository_tree_creator::models::tree::Tree;

use crate::error::DitError;
use crate::features::commit::create_commit;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{find_objects, is_init};
use crate::objects::branch::Branch;
use crate::objects::commit::Commit;

pub fn merge(name: &String) -> Result<(), DitError> {
    if !is_init() {
        display_message("dit repository is not initialized.", Color::RED);
        process::exit(1);
    }

    let target_branch = Branch::get_branch(name.to_owned())?;
    let current_branch = Branch::get_current_branch()?;

    let current_commit = Commit::get_commit_from_file(current_branch.get_head().to_owned()).map_err(DitError::IoError)?;
    let target_commit = Commit::get_commit_from_file(target_branch.get_head().to_owned()).map_err(DitError::IoError)?;

    let mut target_tree = Tree::default();
    get_repository_tree_from_object_files(&mut target_tree, target_commit.get_tree(), &find_objects()).map_err(|e| {
        display_message("Error recreating files", Color::RED);
        DitError::UnexpectedComportement(format!("Details: {}", e))
    })?;
    let target_node: Node = TreeNode(target_tree);


    let mut current_tree = Tree::default();
    get_repository_tree_from_object_files(&mut current_tree, target_commit.get_tree(), &find_objects()).map_err(|e| {
        display_message("Error recreating files", Color::RED);
        DitError::UnexpectedComportement(format!("Details: {}", e))
    })?;
    let current_node: Node = TreeNode(current_tree);

    if let Some(merge) = merge_repository_trees(current_node, target_node, &Mode::Complete) {
        transcript_repository_to_object_files(&merge, &find_objects()).map_err(|e1| {
            display_message("Error recreating files.", Color::RED);
            DitError::UnexpectedComportement(format!("Details: {}.", e1))
        })?;
        let desc: String = format!("merge {} and {}", current_branch.get_name(), target_branch.get_name());
        create_commit(desc, current_commit.get_hash().to_owned(), merge.get_id())?;
    }
    Ok(())
}