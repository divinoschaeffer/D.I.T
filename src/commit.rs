use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Error, Write};
use sha1::{Digest, Sha1};
use crate::arguments::init::{find_info, find_objects, find_staged, get_object_path};
use crate::utils::write_hash_file;

pub struct Commit {
    hash: String,
    tree: String,
    parent: String,
    description: String
}

impl Commit {
    pub fn new(tree: String, parent: String, description: String) -> Commit{
        let ens  = tree.clone() + &*parent + &*description;
        let hash = Sha1::digest(ens.as_bytes());
        let string_hash = hex::encode(hash);
        Commit {
            hash: string_hash,
            tree,
            parent,
            description,
        }
    }

    pub fn get_hash(&self) -> &String{
        &self.hash
    }

    pub fn get_parent(&self) -> &String {
        &self.parent
    }

    pub fn get_tree(&self) -> &String {
        &self.tree
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn display(&self) {
        println!("hash: {}, parent: {}, tree: {}, description: {}", self.hash, self.parent, self.tree, self.description);
    }

    pub fn transcript_commit_to_file(&self){
        let object_path = find_objects();
        let commit_path = get_object_path(&object_path, &self.hash);
        let info_path = find_info();
        let staged_path = find_staged();
        
        if !commit_path.exists() {
            let commit_file = File::create(&commit_path).unwrap_or_else(|e| {
                panic!("Error while creating file in objects directory: {e}");
            });
            let mut writer = BufWriter::new(commit_file);
            self.write_commit(&mut writer).unwrap_or_else(|e1| {
                panic!("Error while writing commit: {e1}");
            });
            
            let info_file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(false)
                .open(info_path).unwrap();
            
            write_hash_file(self.hash.clone(),&info_file,5).unwrap();

            let staged_file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(false)
                .open(staged_path).unwrap();

            write_hash_file(self.hash.clone(),&staged_file,0).unwrap();
        }
    }
    
    fn write_commit(&self, writer: &mut BufWriter<File>) -> Result<(),Error> {
        writeln!(writer, "tree {}", self.tree)?;
        writeln!(writer, "pare {}", self.parent)?;
        writeln!(writer,"{}", self.description)?;
        Ok(())
    }
}