use std::io;
use std::process::Command;
use crate::arguments::init::{find_dit, find_objects, get_staged_hash};
use crate::commit::Commit;
use crate::objects::node_type::NodeType;
use crate::objects::tree::Tree;
use crate::utils::{read_content_file_from_path, real_path, NULL_HASH};

use super::delete::get_deleted_elements;
use super::init::get_head_hash;
use super::rm::find_element_to_remove;

pub fn commit() -> Result<(), io::Error>{

    let dit_path = find_dit().unwrap_or_else(|| {
        panic!("dit is not initialized");
    });
    let desc_path = dit_path.join("commit");
    let staged_hash = get_staged_hash();
    
    if staged_hash == NULL_HASH { 
        println!("You need to stage elements before commiting");
        return Ok(())
    } else if is_first_commit() {
        Command::new("nano")
            .arg(desc_path.clone())
            .spawn()
            .expect("Failed to open nano")
            .wait()
            .expect("Error with running nano");
        
        let description = read_content_file_from_path(&desc_path.as_path()).unwrap_or_default();
        let parent = NULL_HASH;
        let tree = staged_hash;
        let commit: Commit = Commit::new(tree, String::from(parent), description);
        
        commit.transcript_commit_to_file();
        
        Commit::delete_description_file();
        
    } else {
        Command::new("nano")
            .arg(desc_path.clone())
            .spawn()
            .expect("Failed to open nano")
            .wait()
            .expect("Error with running nano");
        
        let description = read_content_file_from_path(&desc_path.as_path()).unwrap_or_default();
        let last_commit_hash = get_head_hash();

        let last_commit = Commit::get_commit_from_file(last_commit_hash.clone());

        let mut staged_tree = Tree::new(String::from(""), Vec::new(), String::from(""));
        staged_tree.get_tree_from_file(staged_hash.clone());
        staged_tree.set_hash(staged_hash);
        let mut staged_root =  NodeType::Tree(staged_tree);

        let mut last_commit_tree = Tree::new(String::from(""), Vec::new(), String::from(""));
        last_commit_tree.get_tree_from_file(last_commit.get_tree().to_string());
        last_commit_tree.set_hash(last_commit.get_tree().to_string());
        let mut last_commit_root: NodeType = NodeType::Tree(last_commit_tree);

        let option_deleted_elements = get_deleted_elements();

        match option_deleted_elements {
            Some(deleted_elements) => {
                for deleted_element in deleted_elements {
                    let real_path = real_path(&deleted_element);

                    let mut ancestors: Vec<_> = real_path.ancestors().collect();
                    ancestors.pop();
                    ancestors.reverse();

                    find_element_to_remove(&mut last_commit_root, &mut ancestors);
                    find_element_to_remove(&mut staged_root, &mut ancestors);
                }
            }
            None => ()
        }

        let mut result = NodeType::merge(last_commit_root, staged_root).unwrap();
        let commit_tree_hash = NodeType::create_node_hash(&mut result);
        result.transcript_to_files(&find_objects());
        
        let parent = last_commit_hash;
        let tree = commit_tree_hash;
        let commit: Commit = Commit::new(tree, String::from(parent), description);
        
        commit.transcript_commit_to_file();
        
        Commit::delete_description_file();
    }
    Ok(())
}

fn is_first_commit() -> bool {
    get_head_hash() == NULL_HASH
}

