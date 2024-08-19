use std::{fs::File, io::{BufRead, BufReader, BufWriter, Read, Write}, path::PathBuf};

use crate::arguments::init::open_object_file;

use super::{blob::Blob, node_type::NodeType, BLOB};

#[derive(Clone)]
pub struct Tree {
    hash: String,
    name: String,
    nodes: Vec<NodeType>
}

impl Tree {
    pub fn new(name: String, nodes: Vec<NodeType>, hash: String) -> Tree {
        let tree = Tree {
            hash,
            name,
            nodes
        };
        tree
    }

    pub fn add_node(&mut self, node: NodeType) {
        self.nodes.push(node)
    }

    pub fn find_node(&mut self, node: &NodeType) -> Option<&mut NodeType>{
        self.nodes.iter_mut().find(|x| *x == node)
    }

    pub fn find_node_index(&self, node: &NodeType) -> Option<usize> {
        self.nodes.iter().position(|n| n == node)
    }
    
    pub fn find_node_by_name(&mut self, file_name: String) -> Option<&mut NodeType> {
        self.nodes.iter_mut().find(|x| {
            *x.get_name() == file_name
        })
    }

    pub fn remove_node(&mut self, node: &NodeType) {
        if let Some(index) = self.find_node_index(node) {
            self.nodes.remove(index);
        }
    }

    pub fn contains(&self, node: &NodeType) -> bool {
        self.nodes.contains(node)
    }
    
    pub fn exist_node(&mut self, node: &NodeType) -> bool {
        match self.find_node(node) {
            Some(_) => true,
            None => false
        }
    }

    pub fn get_nodes(&mut self) -> &mut Vec<NodeType> {
        &mut self.nodes
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn display(&self) {

        println!("hash: {}, name: {}, nodes: {}", self.hash, self.name, self.nodes.len());
        for node in &self.nodes[..] {
            node.display()
        }
    }

    pub fn set_name(&mut self, name: String){
        self.name = name;
    }

    pub fn set_hash(&mut self, hash: String){
        self.hash = hash;
    }

    pub fn get_tree_from_file(&mut self, staged_hash: String){
        let root_file = open_object_file(staged_hash);
        let reader = BufReader::new(root_file);

        for line in reader.lines() {
            let content = match line {
                Ok(content) => content,
                Err(e) => panic!("Error while reading object file: {}",e),
            };
            let hash = &content[5..45];
            let name = &content[46..];

            if &content[0..4] == BLOB {
                self.get_blob_from_file(String::from(name), String::from(hash))
            } else {
                let mut new_tree = Tree::new(String::from(name), Vec::new(), String::from(hash));
                new_tree.get_tree_from_file(String::from(hash));
                let node = NodeType::Tree(new_tree);
                self.add_node(node);
            }
        }
    }

    pub fn get_blob_from_file(&mut self, file_name: String, hash: String) {
        let blob_file = open_object_file(String::from(hash.clone()));
        let mut buf_reader = BufReader::new(blob_file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        let blob = Blob::new(file_name,contents, hash);
        let node = NodeType::Blob(blob);
        self.add_node(node);
    }

    pub fn write_tree_node_to_file(&mut self, objects_path: &&PathBuf, writer: &mut BufWriter<File>) {
        for node in self.get_nodes() {
            if let NodeType::Tree(current_tree) = node {
                writeln!(writer, "tree {} {}", current_tree.get_hash(), current_tree.get_name())
                    .unwrap_or_else(|e| {
                        panic!("Error while writing to file: {e}");
                    });
            }
            if let NodeType::Blob(current_blob) = node {
                writeln!(writer, "blob {} {}", current_blob.get_hash(), current_blob.get_name())
                    .unwrap_or_else(|e| {
                        panic!("Error while writing to file: {e}");
                    });
            }
            node.transcript_to_files( &objects_path);
        }
    }

}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}