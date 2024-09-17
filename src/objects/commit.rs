use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Error, Read, Write};
use ptree2::print_tree;
use sha1::{Digest, Sha1};
use crate::arguments::init::{find_info, find_objects, find_refs, find_staged, get_object_path, open_object_file};
use crate::objects::branch::Branch;
use crate::objects::file_objects::tree::Tree;
use crate::objects::node::Node;
use crate::utils::{write_hash_file, NULL_HASH};

#[derive(Clone)]
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
        println!("hash: {} \n description: {}", self.hash, self.description);
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
            
            self.reference_commit().expect("Error while referencing commit");
            
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
    
    fn reference_commit(&self) -> Result<(), io::Error> {
        let branch = Branch::get_branch_from_file();
        let branch_path = format!("./.dit/refs/{}", branch.get_name());
        
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(branch_path)?;
        
        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}",self.hash)?;
        Ok(())
    }
    
    pub fn delete_description_file(){
        std::fs::remove_file("./.dit/commit").unwrap();
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
    
        Commit::new(String::from(tree), String::from(parent), description)
    }
    
    pub fn create_commit_tree(branch: Branch) -> Option<Node>{
        let commits = Self::get_commit_list(branch);
        
        if commits.is_empty() {
            return None
        }
        
        let mut root = None;
        
        for commit in commits.iter() {
            
            let node = Node::new(commit.clone(), Vec::new());
            
            if root.is_none() {
                root = Some(node.clone());
            }
            
            root.as_mut().unwrap().add_child_to_tree(&node);
        }
        
        return root
    }
    
    
    pub fn get_commit_list(branch: Branch) -> Vec<Commit>{
        let name_branch = branch.get_name();

        let branch_path = find_refs().join(name_branch);
        let file = OpenOptions::new().read(true).open(branch_path).unwrap();

        let reader = BufReader::new(file);

        let mut commits: Vec<Commit> = Vec::new();

        for line in reader.lines() {
            let content = line.unwrap();

            let commit = Commit::get_commit_from_file(content);
            commits.push(commit);
        }
        
        commits
    }
    pub fn commit_exist(hash: &String) -> bool {
        let branch = Branch::get_branch_from_file();
        let commits = Self::get_commit_list(branch);
        for c in commits.iter() {
            if *c.get_hash() == *hash {
                return true
            }
        }
        return false
    }
    
    pub fn display_commit_tree(){
        let root = Commit::create_commit_tree(Branch::get_branch_from_file());
        match root { 
            Some(tree) => print_tree(&tree).expect("Error while displaying commit tree"),
            None => println!("No commit on this branch")
        }
    }
    
    pub fn recreate_files(&self){
        let mut tree = Tree::default();
        tree.get_tree_from_file(self.tree.clone());
        tree.display();
    }
}