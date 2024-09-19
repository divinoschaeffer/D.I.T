use std::{fs::File, io::BufWriter, path::{Path, PathBuf}};

use sha1::{Digest, Sha1};

use crate::{arguments::init::get_object_path, utils::{read_content_file_from_path, real_path}};

use crate::objects::file_objects::tree::Tree;
use crate::objects::file_objects::blob::Blob;

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

            for node in tree.get_mut_nodes() {
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
    
    pub fn get_hash(&self) -> String {
        match self { 
            NodeType::Tree(t) => t.get_hash().clone(),
            NodeType::Blob(b) => b.get_hash().clone(),
        }
    }
    
    pub fn get_nodes(&self) -> Vec<NodeType> {
        if let NodeType::Tree(tree) = self {
            return tree.get_nodes()
        }
        return Vec::new()
    }
    
    pub fn add_node(&mut self, node: NodeType){
        if let NodeType::Tree(ref mut tree) = self {
            tree.add_node(node);
        }    
    }
    
    pub fn remove_node(&mut self, node: &NodeType) {
        if let NodeType::Tree(ref mut tree) = self {
            tree.remove_node(&node);
        }
    }

    pub fn replace(&mut self, node: NodeType) {
        for n in self.get_nodes().iter() {
            if n.get_name() == node.get_name() && Self::is_same_type(n,&node) {
                self.remove_node(n);
                self.add_node(node);
                return;
            }
        }
    }

    pub fn is_same_type(n1: &NodeType, n2: &NodeType) -> bool {
        match (n1, n2) {
            (NodeType::Blob(_), NodeType::Blob(_)) => true,
            (NodeType::Tree(_), NodeType::Tree(_)) => true,
            _ => false,
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
            if !tree.exist_node_with_same_name(node.get_name()) {
                paths.remove(0);
                tree.add_node(node);
            } else {
                if let Some(old_blob) = tree.find_blob_by_name(node.get_name()) {
                    let to_remove = old_blob.clone();
                    tree.remove_blob_same_name(to_remove.get_name());
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
            if !tree.exist_node_with_same_name_and_type(&node) {
                paths.remove(0);
                node._create_repository_tree(paths);
                tree.add_node(node);
            } else {
                if let Some(old_node) = tree.find_tree_by_name(node.get_name()) {
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

    pub fn find_node_with_same_name(reference: &NodeType, node: &NodeType) -> Option<NodeType> {
        for n in node.get_nodes() {
            if n.get_name() == reference.get_name() {
                return Some(n);
            }
        }
        return None
    }

    pub fn merge(r1: NodeType, r2: NodeType) -> Option<NodeType> {
        if r1 == r2 {
            return None;
        }

        let mut new = r1.clone();

        new.complete_node_from_another_node(r2.clone());

        for node in new.get_nodes().iter_mut() {
            if let Some(equi_node) = Self::find_node_with_same_name(&node, &r2) {
                if let Some(merged_node) = Self::merge(node.clone(), equi_node) {
                    new.replace(merged_node);
                }
            }
        }

        Some(new)
    }
    
    fn complete_node_from_another_node(&mut self, node: NodeType) {
        match self {
            NodeType::Tree(tree) => {
                tree.complete_node_from_another_node(node);
            },
            _ => ()
        }
    }
    
    pub fn create_element(&mut self, path_buf: PathBuf){
        match self{
            NodeType::Tree(t) => t.create_directory_from_tree(path_buf),
            NodeType::Blob(b) => b.create_file_from_blob(path_buf)
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