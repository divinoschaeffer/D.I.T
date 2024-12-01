use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Error, Read, Write};
use std::path::PathBuf;

use dit_id_generator::features::generator::generate;
use dit_id_generator::traits::generator::Generator;
use ptree2::print_tree;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{
    find_info, find_objects, find_refs, find_staged, get_object_path, open_object_file,
};
use crate::objects::branch::Branch;
use crate::objects::file_objects::tree::Tree;
use crate::objects::node::Node;
use crate::utils::{NULL_HASH, write_hash_file};

#[derive(Clone, Debug)]
pub struct Commit {
    hash: String,
    tree: String,
    parent: String,
    description: String,
}

impl Commit {
    pub fn new(tree: String, parent: String, description: String) -> Commit {
        let mut commit = Commit {
            hash: String::from(""),
            tree,
            parent,
            description,
        };
        let _ = commit.generate_id();
        commit
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }

    pub fn set_hash(&mut self, hash: String) {
        self.hash = hash;
    }

    pub fn get_parent(&self) -> &String {
        &self.parent
    }

    pub fn set_parent(&mut self, parent: String) {
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

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn display(&self) {
        println!("hash: {} \n description: {}", self.hash, self.description);
    }

    pub fn transcript_commit_to_file(&self) -> Result<(), DitError> {
        let object_path = find_objects();
        let commit_path = get_object_path(&object_path, &self.hash).map_err(DitError::IoError)?;
        let info_path = find_info();
        let staged_path = find_staged();

        if !commit_path.exists() {
            let commit_file = File::create(&commit_path).map_err(DitError::IoError)?;

            let mut writer = BufWriter::new(commit_file);
            self.write_commit(&mut writer).map_err(DitError::IoError)?;

            self.reference_commit()
                .expect("Error while referencing commit");

            let info_file = OpenOptions::new()
                .write(true)
                .append(false)
                .create(false)
                .open(info_path)
                .map_err(DitError::IoError)?;

            write_hash_file(self.hash.clone(), &info_file, 5).map_err(DitError::IoError)?;

            let staged_file = OpenOptions::new()
                .write(true)
                .append(false)
                .create(false)
                .open(staged_path)
                .map_err(DitError::IoError)?;

            write_hash_file(String::from(NULL_HASH), &staged_file, 0).map_err(DitError::IoError)?;
        }
        Ok(())
    }

    fn write_commit(&self, writer: &mut BufWriter<File>) -> Result<(), Error> {
        writeln!(writer, "tree {}", self.tree)?;
        writeln!(writer, "pare {}", self.parent)?;
        write!(writer, "{}", self.description)?;
        Ok(())
    }

    fn reference_commit(&self) -> Result<(), DitError> {
        let branch = Branch::get_current_branch()?;
        let branch_path = format!("./.dit/refs/{}", branch.get_name());

        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(branch_path)
            .map_err(DitError::IoError)?;

        let mut writer = BufWriter::new(file);
        writeln!(writer, "{}", self.hash).map_err(DitError::IoError)?;
        Ok(())
    }

    pub fn reset_description_file() -> Result<(), Error> {
        let path = PathBuf::from("./.dit/commit");
        let file = OpenOptions::new().write(true).truncate(true).open(path)?;
        let mut writer = BufWriter::new(file);
        write!(writer, "")?;
        Ok(())
    }

    pub fn get_commit_from_file(hash: String) -> Result<Commit, Error> {
        let file = open_object_file(hash)?;
        let mut reader = BufReader::new(file);
        let mut tree_line: String = Default::default();
        reader.read_line(&mut tree_line)?;
        let tree = &tree_line[5..45];

        let mut parent_line: String = Default::default();
        reader.read_line(&mut parent_line)?;
        let parent = &parent_line[5..45];

        let mut description: String = Default::default();
        reader
            .read_to_string(&mut description)
            .expect("Failed to read commit file");

        Ok(Commit::new(
            String::from(tree),
            String::from(parent),
            description,
        ))
    }

    pub fn create_commit_tree(branch: Branch) -> Result<Option<Node>, Error> {
        let commits = Self::get_commit_list(branch.get_name().to_owned())?;

        if commits.is_empty() {
            return Ok(None);
        }

        let mut root = None;

        for commit in commits.iter() {
            let node = Node::new(commit.clone(), Vec::new());

            if root.is_none() {
                root = Some(node.clone());
            }

            root.as_mut().unwrap().add_child_to_tree(&node);
        }

        return Ok(root);
    }

    pub fn get_commit_list(branch_name: String) -> Result<Vec<Commit>, Error> {
        let branch_path = find_refs().join(branch_name);
        let file = OpenOptions::new().read(true).open(branch_path)?;

        let reader = BufReader::new(file);

        let mut commits: Vec<Commit> = Vec::new();

        for line in reader.lines() {
            let content = line.unwrap();
            let c = Commit::get_commit_from_file(content)?;
            commits.push(c);
        }

        Ok(commits)
    }
    pub fn commit_exist(hash: &String) -> Result<bool, DitError> {
        let branch = Branch::get_current_branch()?;
        let commits =
            Self::get_commit_list(branch.get_name().to_string()).map_err(DitError::IoError)?;
        for c in commits.iter() {
            if *c.get_hash() == *hash {
                return Ok(true);
            }
        }
        return Ok(false);
    }

    pub fn display_commit_tree() -> Result<(), DitError> {
        let root =
            Commit::create_commit_tree(Branch::get_current_branch()?).map_err(DitError::IoError)?;
        match root {
            Some(tree) => print_tree(&tree).expect("Error while displaying commit tree"),
            None => display_message("No commit on this branch", Color::BLUE),
        }
        Ok(())
    }

    pub fn recreate_files(&self) -> Result<(), DitError> {
        let mut tree = Tree::default();
        tree.get_tree_from_file(self.tree.clone())?;
        let path = PathBuf::from("./");
        tree.create_directory_from_tree(path)?;
        Ok(())
    }

    pub fn delete_files(&self) -> Result<(), DitError> {
        let mut tree = Tree::default();
        tree.get_tree_from_file(self.tree.clone())?;
        let path = PathBuf::from("./");
        tree.delete_directory(path)?;
        Ok(())
    }
}

impl Generator for Commit {
    fn generate_id(&mut self) -> String {
        let content = self.tree.clone() + &*self.parent + &*self.description;
        let hash = generate(content);
        self.set_hash(hash.clone());
        hash
    }
}
