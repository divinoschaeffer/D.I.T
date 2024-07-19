use std::cmp::PartialEq;
use sha1::{Sha1, Digest};

#[derive(Clone)]
pub struct Blob {
    hash: String,
    name: String,
    content: String
}

#[derive(Clone)]
pub struct Tree {
    hash: String,
    name: String,
    nodes: Vec<NodeType>
}

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
            
            for node in &mut tree.nodes {
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
            return blob.hash.clone()
        } else { 
            return String::new()
        }
        
    }
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
    
    pub fn contains(&self, node: &NodeType) -> bool {
        self.nodes.contains(node)
    }
    
    pub fn get_nodes(&self) -> &Vec<NodeType> {
        &self.nodes
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

}

impl Blob {
    
    pub fn new(name: String, content: String, hash: String) -> Blob {
        let blob = Blob {
            name,
            hash,
            content
        };
        blob
    }
    
    pub fn set_name(&mut self, name: String){
        self.name = name;
    }
    
    pub fn set_hash(&mut self, hash: String){
        self.hash = hash;
    }
    
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
    
    pub fn display(&self) {
        println!("hash: {}, name: {}", self.hash, self.name);
    }
    
    pub fn create_hash(&mut self) {
        let total  = self.name.clone() + &*self.content.clone();
        let hash = Sha1::digest(total.as_bytes());
        let string_hash = hex::encode(hash);
        self.set_hash(string_hash)
    }
}

impl PartialEq for Blob {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.content == other.content
    }
}

impl PartialEq for Tree {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
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