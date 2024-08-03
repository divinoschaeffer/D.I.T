use std::{fs::File, io::{BufWriter, Write}};

use sha1::{Digest, Sha1};

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

