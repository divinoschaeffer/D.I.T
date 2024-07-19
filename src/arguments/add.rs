use crate::objects::{Blob as StructBlob, NodeType, Tree as StructTree};
use crate::utils::{find_dit, read_content_file, read_hash_file, real_path};
use std::fs::File;
use std::io;
use std::path::{Path};

pub fn add_files_or_directory(elements: Vec<&String>) -> Result<(), io::Error> {
    if elements.is_empty() {
        println!("You need to specify files to add");
    } else if is_first_commit() {
        let tree = StructTree::new(String::from(""), Vec::new(), String::from(""));
        let mut racine: NodeType = NodeType::Tree(tree);

        for element in elements {
            create_repository_tree(&mut racine, element);
        }

        NodeType::create_node_hash(&mut racine);

        racine.display();
    }
    Ok(())
}

fn is_first_commit() -> bool {
    let dit_path = find_dit().unwrap();
    let info_path = dit_path.join("info");
    let file = File::open(info_path).unwrap();
    read_hash_file(file, 5) == "0000000000000000000000000000000000000000"
}

fn create_repository_tree(racine: &mut NodeType, element: &String) {
    let real_path = real_path(element);
    if !real_path.exists() {
        println!("path: {} does not exist", real_path.display());
        return;
    }

    let mut ancestors: Vec<_> = real_path.ancestors().collect();
    ancestors.pop();
    ancestors.reverse();

    _create_repository_tree(racine, &mut ancestors);
}

fn _create_repository_tree<'a>(racine: &'a mut NodeType, paths: &mut Vec<&Path>) {
    if paths.is_empty() {
        return;
    }

    let path = paths[0];
    let file_name = path.file_name().unwrap().to_str().unwrap();

    if path.is_dir() {
        create_tree_node(racine,file_name,paths);
    } else {
        create_blob_node(racine, paths, &path, file_name);
    }
}

fn create_blob_node(racine: &mut NodeType, paths: &mut Vec<&Path>, path: &&Path, file_name: &str) {
    let content = read_content_file(&path).unwrap_or_else(|e| {
        panic!("Error while reading {:?} : {e}", file_name);
    });
    let mut file_blob: StructBlob =
        StructBlob::new(String::from(file_name), content, String::from(""));
    file_blob.create_hash();
    let node: NodeType = NodeType::Blob(file_blob);
    if let NodeType::Tree(ref mut tree) = racine {
        if !tree.contains(&node) {
            paths.remove(0);
            tree.add_node(node);
        }
    }
}

fn create_tree_node(racine: &mut NodeType, file_name: &str, paths: &mut Vec<&Path>) {

    let dir_tree = StructTree::new(String::from(file_name), Vec::new(), String::from(""));
    let mut node = NodeType::Tree(dir_tree);
    if let NodeType::Tree(ref mut tree) = racine {
        if !tree.contains(&node) {
            paths.remove(0);
            _create_repository_tree(&mut node, paths);
            tree.add_node(node);
        } else {
            if let Some(old_node) = tree.find_node(&node) {
                paths.remove(0);
                _create_repository_tree(old_node, paths);
            } else {
                panic!("Error while finding directory already existing");
            }
        }
    }
}