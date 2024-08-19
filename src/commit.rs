use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Error, Read, Write};
use sha1::{Digest, Sha1};
use crate::arguments::init::{find_info, find_objects, find_staged, get_object_path, open_object_file};
use crate::utils::{write_hash_file, NULL_HASH};

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

    pub fn set_parent(&mut self, parent: String){
        self.parent = parent;
    }

    pub fn get_tree(&self) -> &String {
        &self.tree
    }

    pub fn set_tree(&mut self, tree: String) {
        self.tree = tree;
    }

    pub fn get_description(&self) -> &String {
        &self.description
    }

    pub fn set_description(&mut self, description: String){
        self.description = description;
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
                .append(false)
                .create(false)
                .open(info_path).unwrap();
            
            write_hash_file(self.hash.clone(),&info_file,5).unwrap();

            let staged_file = OpenOptions::new()
                .write(true)
                .append(false)
                .create(false)
                .open(staged_path).unwrap();

            write_hash_file(String::from(NULL_HASH) ,&staged_file,0).unwrap();
        }
    }
    
    fn write_commit(&self, writer: &mut BufWriter<File>) -> Result<(),Error> {
        writeln!(writer, "tree {}", self.tree)?;
        writeln!(writer, "pare {}", self.parent)?;
        write!(writer,"{}", self.description)?;
        Ok(())
    }

    pub fn get_commit_from_file(hash: String) -> Commit{
        let file = open_object_file(hash);
        let mut reader = BufReader::new(file);
        let mut tree_line: String = Default::default();
        reader.read_line(&mut tree_line).expect("Failed to read commit file");
        let tree = &tree_line[5..45];
    

        let mut parent_line: String = Default::default();
        reader.read_line(&mut parent_line).expect("Failed to read commit file");
        let parent = &parent_line[5..45];

        let mut description: String = Default::default();
        reader.read_to_string(&mut description).expect("Failed to read commit file");

        println!("{tree}");
        println!("{parent}");
        println!("{description}");
    
        Commit::new(String::from(tree), String::from(parent), description)
    } 
}