use crate::arguments::commit::create_commit;
use crate::arguments::init::find_objects;
use crate::error::DitError;
use crate::objects::branch::Branch;
use crate::objects::commit::Commit;
use crate::objects::file_objects::node_type::NodeType;
use crate::objects::file_objects::tree::Tree;

pub fn merge(name: &String) -> Result<(), DitError> {
    
    let target_branch = Branch::get_branch(name.to_owned())?;
    let current_branch = Branch::get_current_branch()?;
    
    let current_commit = Commit::get_commit_from_file(current_branch.get_head().to_owned()).map_err(DitError::IoError)?;
    let target_commmit = Commit::get_commit_from_file(target_branch.get_head().to_owned()).map_err(DitError::IoError)?;
    
    let mut target_tree = Tree::default();
    target_tree.get_tree_from_file(target_commmit.get_tree().to_owned())?;
    target_tree.display();
    let target_node: NodeType = NodeType::Tree(target_tree);
    
    
    let mut current_tree = Tree::default();
    current_tree.get_tree_from_file(current_commit.get_tree().to_owned())?;
    current_tree.display();
    let current_node: NodeType = NodeType::Tree(current_tree);
    
    if let Some(mut merge) = NodeType::merge(current_node, target_node) {
        merge.display();
        let tree: String = merge.create_node_hash();
        merge.transcript_to_files(&find_objects())?;
        let desc: String = format!("merge {} and {}", current_branch.get_name(), target_branch.get_name());
        create_commit(desc, current_commit.get_hash().to_owned(), tree)?;
    }
    Ok(())
}