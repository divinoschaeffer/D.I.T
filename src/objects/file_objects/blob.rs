use std::{fs, fs::File, io::{BufWriter, Write}};
use std::fs::OpenOptions;
use std::path::PathBuf;
use sha1::{Digest, Sha1};
use crate::objects::file_objects::node_type::NodeType;

#[derive(Clone)]
pub struct Blob {
    hash: String,
    name: String,
    content: String
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

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }

    pub fn set_hash(&mut self, hash: String){
        self.hash = hash;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn get_content(&self) -> &String {
        &self.content
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

    pub fn write_content_to_file(&self, file: File) {
        let mut writer = BufWriter::new(file);
    
        writeln!(writer, "{}", self.get_content())
            .unwrap_or_else(|e| {
                panic!("Error while writing to file: {e}");
            });
    }
    
    pub fn is_blob(node: &NodeType) -> bool {
        match node {
            NodeType::Blob(_) => true,
            _ => false
        }
    }

    pub fn create_file_from_blob(&self, directory_path: PathBuf){
        let filename = self.name.to_owned();
        let file_path = directory_path.join(filename);
        
        if file_path.is_file() {
            fs::remove_file(file_path.clone()).unwrap();
        }
        
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path).unwrap();
        
        let mut writer = BufWriter::new(file);
        write!(writer, "{}", self.content.to_owned()).unwrap();
    }
    
    pub fn delete_file(&self, directory_path: PathBuf){
        let filename = self.name.to_owned();
        let file_path = directory_path.join(filename);

        if file_path.is_file() {
            fs::remove_file(file_path).unwrap();
        }
    }
}

impl PartialEq for Blob {
    fn eq(&self, other: &Self) -> bool {
        let empty_string = String::from("");
        if self.hash != empty_string && other.hash != empty_string {
            return self.hash == other.hash;
        }
        self.name == other.name
    }
}

