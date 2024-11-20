use std::{fs, fs::File, io::{BufRead, BufReader, BufWriter, Read, Write}, path::PathBuf};

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::open_object_file;
use crate::objects::BLOB;
use crate::objects::file_objects::blob::Blob;
use crate::objects::file_objects::node_type::NodeType;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
pub struct Tree {
    hash: String,
    name: String,
    nodes: Vec<NodeType>,
}

impl Tree {
    pub fn new(name: String, nodes: Vec<NodeType>, hash: String) -> Tree {
        let tree = Tree {
            hash,
            name,
            nodes,
        };
        tree
    }

    pub fn default() -> Tree {
        Tree::new(String::from(""), Vec::new(), String::from(""))
    }

    pub fn add_node(&mut self, node: NodeType) {
        self.nodes.push(node)
    }

    pub fn find_node(&mut self, node: &NodeType) -> Option<&mut NodeType> {
        self.nodes.iter_mut().find(|x| *x == node && Tree::is_tree(node))
    }

    pub fn find_node_index(&self, node: &NodeType) -> Option<usize> {
        self.nodes.iter().position(|n| n == node)
    }

    pub fn find_node_by_name(&mut self, file_name: String) -> Option<&mut NodeType> {
        self.nodes.iter_mut().find(|x| {
            *x.get_name() == file_name
        })
    }

    pub fn find_tree_by_name(&mut self, file_name: String) -> Option<&mut NodeType> {
        self.nodes.iter_mut().find(|x| {
            *x.get_name() == file_name && Tree::is_tree(x)
        })
    }

    pub fn find_blob_by_name(&mut self, file_name: String) -> Option<&mut NodeType> {
        self.nodes.iter_mut().find(|x| {
            *x.get_name() == file_name && Blob::is_blob(x)
        })
    }

    pub fn replace_node(&mut self, node: NodeType) {
        if let Some(existing_node) = self.nodes.iter_mut().find(|n| n.get_name() == node.get_name() && NodeType::is_same_type(n, &node)) {
            *existing_node = node;
        }
    }


    pub fn remove_node(&mut self, node: &NodeType) {
        if let Some(index) = self.find_node_index(node) {
            self.nodes.remove(index);
        }
    }

    pub fn remove_blob_same_name(&mut self, name: String) {
        if let Some(index) = self.nodes.iter().position(|node| node.get_name() == name && Blob::is_blob(&node)) {
            self.nodes.remove(index);
        }
    }

    pub fn contains(&self, node: &NodeType) -> bool {
        self.nodes.contains(node)
    }

    pub fn exist_node_with_same_name(&mut self, name: String) -> bool {
        self.nodes.iter().any(|n| n.get_name() == name)
    }

    pub fn exist_node_with_same_name_and_type(&mut self, node: &NodeType) -> bool {
        self.nodes.iter().any(|n| n.get_name() == node.get_name() && Self::is_tree(node))
    }

    pub fn is_tree(n1: &NodeType) -> bool {
        match n1 {
            NodeType::Tree(_) => true,
            _ => false
        }
    }

    pub fn complete_node(&mut self, node: NodeType) {
        let node_name = node.get_name();

        if !self.exist_node_with_same_name(node_name) {
            self.add_node(node);
        } else {
            match node {
                NodeType::Blob(_) => {
                    self.replace_node(node);
                }
                _ => (),
            }
        }
    }

    pub fn complete_node_from_another_node(&mut self, node: NodeType) {
        for n in node.get_nodes() {
            self.complete_node(n);
        }
    }

    pub fn get_mut_nodes(&mut self) -> &mut Vec<NodeType> {
        &mut self.nodes
    }

    pub fn get_nodes(&self) -> Vec<NodeType> {
        self.nodes.clone()
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

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    pub fn get_tree_from_file(&mut self, staged_hash: String) -> Result<(), DitError> {
        let root_file = open_object_file(staged_hash).map_err(DitError::IoError)?;
        let reader = BufReader::new(root_file);

        for line in reader.lines() {
            let content = match line {
                Ok(content) => content,
                Err(e) => {
                    display_message(format!("Error while reading object file: {}", e).as_str(), Color::RED);
                    return Err(DitError::UnexpectedComportement("Error while reading a file".to_string()));
                }
            };
            let hash = &content[5..45];
            let name = &content[46..];

            if &content[0..4] == BLOB {
                self.get_blob_from_file(String::from(name), String::from(hash))?
            } else {
                let mut new_tree = Tree::new(String::from(name), Vec::new(), String::from(hash));
                new_tree.get_tree_from_file(String::from(hash))?;
                let node = NodeType::Tree(new_tree);
                self.add_node(node);
            }
        }
        Ok(())
    }

    pub fn get_blob_from_file(&mut self, file_name: String, hash: String) -> Result<(), DitError> {
        let blob_file = open_object_file(String::from(hash.clone())).map_err(DitError::IoError)?;
        let mut buf_reader = BufReader::new(blob_file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).map_err(DitError::IoError)?;
        let blob = Blob::new(file_name, contents, hash);
        let node = NodeType::Blob(blob);
        self.add_node(node);
        Ok(())
    }

    pub fn write_tree_node_to_file(&mut self, objects_path: &&PathBuf, writer: &mut BufWriter<File>) -> Result<(), DitError> {
        for node in self.get_mut_nodes() {
            if let NodeType::Tree(current_tree) = node {
                writeln!(writer, "tree {} {}", current_tree.get_hash(), current_tree.get_name()).map_err(DitError::IoError)?;
            }
            if let NodeType::Blob(current_blob) = node {
                writeln!(writer, "blob {} {}", current_blob.get_hash(), current_blob.get_name()).map_err(DitError::IoError)?;
            }
            node.transcript_to_files(&objects_path)?;
        }
        Ok(())
    }

    pub fn create_directory_from_tree(&mut self, directory_path: PathBuf) -> Result<(), DitError> {
        let name = self.name.to_owned();
        let path = directory_path.join(name);

        if self.name != "" && !path.is_dir() {
            fs::create_dir(path.clone()).map_err(DitError::IoError)?;
        }

        for n in self.nodes.iter_mut() {
            n.create_element(path.to_owned())?;
        }

        Ok(())
    }

    pub fn delete_directory(&mut self, directory_path: PathBuf) -> Result<(), DitError> {
        let name = self.name.to_owned();
        let path = directory_path.join(name);

        if self.name == "" {
            for n in self.nodes.iter_mut() {
                n.delete_element(path.clone())?;
            }
        } else if path.is_dir() {
            fs::remove_dir_all(path).map_err(DitError::IoError)?;
        }
        Ok(())
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}