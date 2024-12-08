use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::path::PathBuf;

use dit_file_encryptor::CompressedFile;
use dit_id_generator::features::generator::generate;
use dit_id_generator::traits::generator::Generator;
use ptree2::print_tree;
use repository_tree_creator::features::get_repository_tree_from_object_files::get_repository_tree_from_object_files;
use repository_tree_creator::features::transcript_repository_to_files::{Mode, transcript_repository_tree_to_files};
use repository_tree_creator::models::node::Node::TreeNode;
use repository_tree_creator::models::tree::Tree;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{find_dit, find_info, find_objects, find_refs, find_staged, get_object_path, get_path_object_file};
use crate::objects::branch::Branch;
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
            let _ = File::create(&commit_path).map_err(DitError::IoError)?;

            let mut writer = CompressedFile::new(commit_path)
                .open_for_write()
                .map_err(|e| DitError::IoError(e))?;

            self.write_commit(&mut writer).map_err(DitError::IoError)?;

            self.reference_commit()?;

            write_hash_file(self.hash.clone(), info_path, 5)
                .map_err(DitError::IoError)?;

            write_hash_file(String::from(NULL_HASH), staged_path, 0)
                .map_err(DitError::IoError)?;
        }
        Ok(())
    }

    fn write_commit(&self, writer: &mut Box<dyn Write>) -> Result<(), Error> {
        writeln!(writer, "tree {}", self.tree)?;
        writeln!(writer, "pare {}", self.parent)?;
        write!(writer, "{}", self.description)?;
        Ok(())
    }

    fn reference_commit(&self) -> Result<(), DitError> {
        let branch = Branch::get_current_branch()?;
        let branch_path = PathBuf::from(format!("./.dit/refs/{}", branch.get_name()));

        let mut commit_hash: String = self.hash.clone();
        commit_hash.push('\n');

        CompressedFile::new(branch_path)
            .append_to_file(commit_hash.as_bytes())
            .map_err(|e| DitError::IoError(e))?;
        Ok(())
    }

    pub fn reset_description_file() -> Result<(), Error> {
        let path = PathBuf::from("./.dit/commit");
        File::open(path)?;
        Ok(())
    }

    pub fn get_commit_from_file(hash: String) -> Result<Commit, Error> {
        let file = get_path_object_file(hash)?;
        let reader = CompressedFile::new(file)
            .open_for_read()?;

        let mut buf_reader = BufReader::new(reader);

        let mut tree_line: String = Default::default();
        buf_reader.read_line(&mut tree_line)?;
        let tree = &tree_line[5..45];

        let mut parent_line: String = Default::default();
        buf_reader.read_line(&mut parent_line)?;
        let parent = &parent_line[5..45];

        let mut description: String = Default::default();
        buf_reader.read_to_string(&mut description)?;

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

        let reader = BufReader::new(
            CompressedFile::new(branch_path)
                .open_for_read()?
        );

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
        let dit_path = find_dit().unwrap();
        let project_path = dit_path.parent().unwrap();
        get_repository_tree_from_object_files(&mut tree, &self.tree, &find_objects()).map_err(|e| {
            display_message("Error getting files", Color::RED);
            DitError::UnexpectedComportement(format!("Details: {}", e))
        })?;
        transcript_repository_tree_to_files(&TreeNode(tree), &project_path.to_path_buf(), &Mode::Complete).map_err(|e1| {
            display_message("Error recreating files.", Color::RED);
            DitError::UnexpectedComportement(format!("Details: {}.", e1))
        })?;
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
