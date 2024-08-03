use std::{fs::File, io::BufWriter, path::{Path, PathBuf}};

use sha1::{Digest, Sha1};

use crate::{arguments::init::get_object_path, utils::{read_content_file_from_path, real_path}};

use super::{blob::Blob, tree::Tree};

#[derive(Clone)]
pub enum NodeType {
    Blob(Blob),
    Tree(Tree)
}

impl NodeType {
    pub fn display(&self) {
        match self {
            NodeType::Blob(blob) => blob.display(),
            NodeType::Tree(tree) => tree.display()
        }
    }

    pub fn create_node_hash(racine: &mut NodeType) -> String {
        if let  NodeType::Tree(ref mut tree) = racine {

            let mut hashes: Vec<String> = Vec::new();

            for node in tree.get_nodes() {
                let hash = Self::create_node_hash(node);
                hashes.push(hash);
            }

            let mut total = String::new();

            for h in &hashes {
                total += h;
            }

            let hash = Sha1::digest(total.as_bytes());
            let string_hash = hex::encode(hash);
            tree.set_hash(string_hash.clone());
            string_hash

        } else if let NodeType::Blob(ref mut blob) = racine {
            return blob.get_hash().clone()
        } else {
            return String::new()
        }

    }
    
    pub fn get_name(&self) -> String{
        match self { 
            NodeType::Tree(tree) => tree.get_name().clone(),
            NodeType::Blob(blob) => blob.get_name().clone()
        }
    }

    pub fn create_repository_tree(&mut self, element: &String) {
        let real_path = real_path(element);
        if !real_path.exists() {
            println!("path: {} does not exist", real_path.display());
            return;
        }
    
        let mut ancestors: Vec<_> = real_path.ancestors().collect();
        ancestors.pop();
        ancestors.reverse();
    
        self._create_repository_tree(&mut ancestors);
    }

    pub fn _create_repository_tree<'a>(&mut self, paths: &mut Vec<&Path>) {
        if paths.is_empty() {
            return;
        }
    
        let path = paths[0];
        let file_name = path.file_name().unwrap().to_str().unwrap();
    
        if path.is_dir() {
            self.create_tree_node(file_name,paths);
        } else {
            self.create_blob_node(paths, &path, file_name);
        }
    }

    fn create_blob_node(&mut self, paths: &mut Vec<&Path>, path: &&Path, file_name: &str) {
        let content = read_content_file_from_path(&path).unwrap_or_else(|e| {
            panic!("Error while reading {:?} : {e}", file_name);
        });
        let mut file_blob: Blob =
            Blob::new(String::from(file_name), content, String::from(""));
        file_blob.create_hash();
        let node: NodeType = NodeType::Blob(file_blob);
    
        if let NodeType::Tree(ref mut tree) = self {
            if !tree.exist_node(&node) {
                paths.remove(0);
                tree.add_node(node);
            } else {
                if let Some(old_blob) = tree.find_node(&node) {
                    let to_remove = old_blob.clone();
                    tree.remove_node(&to_remove);
                }
                paths.remove(0);
                tree.add_node(node);
            }
        }
    }

    fn create_tree_node(&mut self, file_name: &str, paths: &mut Vec<&Path>) {
        let dir_tree = Tree::new(String::from(file_name), Vec::new(), String::from(""));
        let mut node = NodeType::Tree(dir_tree);
        if let NodeType::Tree(ref mut tree) = self {
            if !tree.contains(&node) {
                paths.remove(0);
                node._create_repository_tree(paths);
                tree.add_node(node);
            } else {
                if let Some(old_node) = tree.find_node(&node) {
                    paths.remove(0);
                    old_node._create_repository_tree(paths);
                } else {
                    panic!("Error while finding directory already existing");
                }
            }
        }
    }

    pub fn transcript_to_files(&mut self, objects_path: &PathBuf){
        if let NodeType::Tree(tree) = self {
            let hash = tree.get_hash().clone();
    
            let node_path = get_object_path(objects_path, &hash);
            if !node_path.exists() {
                let file = File::create(&node_path).unwrap_or_else(|e1| {
                    panic!("Error while creating file in objects directory: {e1}");
                });
                let mut writer = BufWriter::new(file);
    
                tree.write_tree_node_to_file(&objects_path, &mut writer);
            }
        }
        
        if let NodeType::Blob(blob) = self {
            let hash = blob.get_hash().clone();
            
            let node_path = get_object_path(objects_path, &hash);
            if !node_path.exists() {
                let file = File::create(&node_path).unwrap_or_else(|e1| {
                    panic!("Error while creating file in objects directory: {e1}");
                });
                blob.write_content_to_file(file);
            }
        }
    }
}

impl PartialEq for NodeType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeType::Blob(blob1), NodeType::Blob(blob2)) => blob1 == blob2,
            (NodeType::Tree(tree1), NodeType::Tree(tree2)) => tree1 == tree2,
            _ => false
        }
    }
}