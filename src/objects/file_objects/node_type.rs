use std::{
    fs::File,
    io::BufWriter,
    path::{Path, PathBuf},
};

use sha1::{Digest, Sha1};

use crate::{
    features::init::get_object_path,
    utils::{path_from_dit, read_content_file_from_path},
};
use crate::error::DitError;
use crate::objects::file_objects::blob::Blob;
use crate::objects::file_objects::tree::Tree;
use crate::utils::set_current_dir_to_project_dir;

#[derive(Clone, Debug)]
pub enum NodeType {
    Blob(Blob),
    Tree(Tree),
}

impl NodeType {
    pub fn display(&self) {
        match self {
            NodeType::Blob(blob) => blob.display(),
            NodeType::Tree(tree) => tree.display(),
        }
    }

    pub fn create_node_hash(&mut self) -> String {
        if let NodeType::Tree(ref mut tree) = self {
            let mut hashes: Vec<String> = Vec::new();

            for node in tree.get_mut_nodes() {
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
        } else if let NodeType::Blob(ref mut blob) = self {
            return blob.get_hash().clone();
        } else {
            return String::new();
        }
    }

    pub fn get_name(&self) -> String {
        match self {
            NodeType::Tree(tree) => tree.get_name().clone(),
            NodeType::Blob(blob) => blob.get_name().clone(),
        }
    }

    pub fn get_hash(&self) -> String {
        match self {
            NodeType::Tree(t) => t.get_hash().clone(),
            NodeType::Blob(b) => b.get_hash().clone(),
        }
    }

    pub fn get_nodes(&self) -> Vec<NodeType> {
        if let NodeType::Tree(tree) = self {
            return tree.get_nodes();
        }
        return Vec::new();
    }

    pub fn add_node(&mut self, node: NodeType) {
        if let NodeType::Tree(ref mut tree) = self {
            tree.add_node(node);
        }
    }

    pub fn remove_node(&mut self, node: &NodeType) {
        if let NodeType::Tree(ref mut tree) = self {
            tree.remove_node(&node);
        }
    }

    pub fn replace(&mut self, node: NodeType) {
        for n in self.get_nodes().iter() {
            if n.get_name() == node.get_name() && Self::is_same_type(n, &node) {
                self.remove_node(n);
                self.add_node(node);
                return;
            }
        }
    }

    pub fn is_same_type(n1: &NodeType, n2: &NodeType) -> bool {
        match (n1, n2) {
            (NodeType::Blob(_), NodeType::Blob(_)) => true,
            (NodeType::Tree(_), NodeType::Tree(_)) => true,
            _ => false,
        }
    }

    /// Create a representation tree of the repository starting with this node
    pub fn create_repository_tree(&mut self, element: &String) -> Result<(), DitError> {
        let path_from_dit = path_from_dit(element)?;

        set_current_dir_to_project_dir().map_err(DitError::IoError)?;

        let mut ancestors: Vec<_> = path_from_dit.ancestors().collect();
        ancestors.pop();
        ancestors.reverse();

        self._create_repository_tree(&mut ancestors)?;

        Ok(())
    }

    /// Sub function create_repository_tree
    pub fn _create_repository_tree<'a>(&mut self, paths: &mut Vec<&Path>) -> Result<(), DitError> {
        if paths.is_empty() {
            return Ok(());
        }
        self.create_specific_node(paths)?;
        Ok(())
    }

    /// Create specific node depending on the situation
    fn create_specific_node(&mut self, paths: &mut Vec<&Path>) -> Result<(), DitError> {
        let path = paths[0];
        if let Some(file_name) = path.file_name() {
            match file_name.to_str() {
                Some(name) => {
                    if path.is_dir() {
                        self.create_tree_node(name, paths)?;
                    } else {
                        self.create_blob_node(paths, &path, name)?;
                    }
                }
                None => {
                    return Err(DitError::UnexpectedComportement(
                        "Fail to get file name".to_string(),
                    ))
                }
            }
        }
        Ok(())
    }

    fn create_blob_node(
        &mut self,
        paths: &mut Vec<&Path>,
        path: &&Path,
        file_name: &str,
    ) -> Result<(), DitError> {
        let content = read_content_file_from_path(&path).map_err(DitError::IoError)?;
        let mut file_blob: Blob = Blob::new(String::from(file_name), content, String::from(""));
        file_blob.create_hash();
        let node: NodeType = NodeType::Blob(file_blob);

        if let NodeType::Tree(ref mut tree) = self {
            if !tree.exist_node_with_same_name(node.get_name()) {
                paths.remove(0);
                tree.add_node(node);
            } else {
                if let Some(old_blob) = tree.find_blob_by_name(node.get_name()) {
                    let to_remove = old_blob.clone();
                    tree.remove_blob_same_name(to_remove.get_name());
                }
                paths.remove(0);
                tree.add_node(node);
            }
        }
        Ok(())
    }

    fn create_tree_node(
        &mut self,
        file_name: &str,
        paths: &mut Vec<&Path>,
    ) -> Result<(), DitError> {
        let dir_tree = Tree::new(String::from(file_name), Vec::new(), String::from(""));
        let mut node = NodeType::Tree(dir_tree);
        if let NodeType::Tree(ref mut tree) = self {
            if !tree.exist_node_with_same_name_and_type(&node) {
                paths.remove(0);
                node._create_repository_tree(paths)?;
                tree.add_node(node);
            } else {
                if let Some(old_node) = tree.find_tree_by_name(node.get_name()) {
                    paths.remove(0);
                    old_node._create_repository_tree(paths)?;
                } else {
                    return Err(DitError::UnexpectedComportement(
                        "Error while finding directory, already existing".to_string(),
                    ));
                }
            }
        }

        Ok(())
    }

    pub fn transcript_to_files(&mut self, objects_path: &PathBuf) -> Result<(), DitError> {
        if let NodeType::Tree(tree) = self {
            let hash = tree.get_hash().clone();

            let node_path = get_object_path(objects_path, &hash).map_err(DitError::IoError)?;
            if !node_path.exists() {
                let file = File::create(&node_path).map_err(DitError::IoError)?;
                let mut writer = BufWriter::new(file);

                tree.write_tree_node_to_file(&objects_path, &mut writer)?;
            }
        }

        if let NodeType::Blob(blob) = self {
            let hash = blob.get_hash().clone();

            let node_path = get_object_path(objects_path, &hash).map_err(DitError::IoError)?;
            if !node_path.exists() {
                let file = File::create(&node_path).map_err(DitError::IoError)?;
                blob.write_content_to_file(file)
                    .map_err(DitError::IoError)?;
            }
        }

        Ok(())
    }

    pub fn find_node_with_same_name(reference: &NodeType, node: &NodeType) -> Option<NodeType> {
        for n in node.get_nodes() {
            if n.get_name() == reference.get_name() && Self::is_same_type(&n, reference) {
                return Some(n);
            }
        }
        return None;
    }

    pub fn fuse(r1: NodeType, r2: NodeType) -> Option<NodeType> {
        if r1 == r2 {
            return None;
        }

        let mut new = r1.clone();

        new.complete_node_from_another_node(r2.clone());

        for node in new.get_nodes().iter_mut() {
            if let Some(equi_node) = Self::find_node_with_same_name(&node, &r2) {
                if let Some(merged_node) = Self::fuse(node.clone(), equi_node) {
                    new.replace(merged_node);
                }
            }
        }

        Some(new)
    }

    fn complete_node_from_another_node(&mut self, node: NodeType) {
        match self {
            NodeType::Tree(tree) => {
                tree.complete_node_from_another_node(node);
            }
            _ => (),
        }
    }

    pub fn create_element(&mut self, path_buf: PathBuf) -> Result<(), DitError> {
        match self {
            NodeType::Tree(t) => t.create_directory_from_tree(path_buf)?,
            NodeType::Blob(b) => b
                .create_file_from_blob(path_buf)
                .map_err(DitError::IoError)?,
        }
        Ok(())
    }

    pub fn delete_element(&mut self, path_buf: PathBuf) -> Result<(), DitError> {
        match self {
            NodeType::Tree(t) => Ok(t.delete_directory(path_buf)?),
            NodeType::Blob(b) => Ok(b.delete_file(path_buf).map_err(DitError::IoError)?),
        }
    }

    pub fn merge(n1: NodeType, n2: NodeType) -> Option<NodeType> {
        if n1 == n2 && n1.get_name() != "" {
            return None;
        }

        match (n1, n2) {
            (NodeType::Blob(mut b1), NodeType::Blob(b2)) => {
                b1.merge(b2);
                Some(NodeType::Blob(b1))
            }
            (NodeType::Tree(mut t1), NodeType::Tree(t2)) => {
                for node1 in t1.get_nodes() {
                    let name_node1 = node1.get_name();
                    for node2 in t2.get_nodes() {
                        if name_node1 == node2.get_name() && Self::is_same_type(&node1, &node2) {
                            if let Some(result) = Self::merge(node1.clone(), node2) {
                                t1.replace_node(result);
                            }
                        }
                    }
                }

                for node in t2.get_nodes().iter() {
                    if !t1.exist_node_with_same_name_and_type(node) {
                        t1.add_node(node.to_owned())
                    }
                }

                Some(NodeType::Tree(t1))
            }
            _ => None,
        }
    }
}

impl PartialEq for NodeType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (NodeType::Blob(blob1), NodeType::Blob(blob2)) => blob1 == blob2,
            (NodeType::Tree(tree1), NodeType::Tree(tree2)) => tree1 == tree2,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod merge_test {
        use super::*;

        fn basic_setup() -> (NodeType, NodeType, NodeType) {
            let blob1 = Blob::new(
                "file1.txt".to_string(),
                "Ceci est le contenu du fichier 1.".to_string(),
                "hash_blob_1".to_string(),
            );

            let blob3 = Blob::new(
                "file3.txt".to_string(),
                "Ceci est le contenu du fichier 3.".to_string(),
                "hash_blob_3".to_string(),
            );

            let tree1 = Tree::new(
                "hello".to_string(),
                vec![NodeType::Blob(blob1.clone())],
                "hash_tree1".to_string(),
            );

            let tree2 = Tree::new("".to_string(), vec![NodeType::Tree(tree1)], "".to_string());

            let tree3 = Tree::new(
                "zio".to_string(),
                vec![NodeType::Blob(blob3.clone())],
                "hash_tree3".to_string(),
            );

            let tree4 = Tree::new(
                "".to_string(),
                vec![NodeType::Tree(tree3.clone())],
                "".to_string(),
            );

            let tree5 = Tree::new(
                "".to_string(),
                vec![NodeType::Tree(tree3), NodeType::Tree(tree2.clone())],
                "".to_string(),
            );

            (
                NodeType::Tree(tree2),
                NodeType::Tree(tree4),
                NodeType::Tree(tree5),
            )
        }

        fn multiple_same_name_setup() -> (NodeType, NodeType, NodeType) {
            let blob1 = Blob::new(
                "file1.txt".to_string(),
                "Ceci est le contenu du fichier 1.".to_string(),
                "hash_blob_1".to_string(),
            );

            let blob3 = Blob::new(
                "file3.txt".to_string(),
                "Ceci est le contenu du fichier 3.".to_string(),
                "hash_blob_3".to_string(),
            );

            let tree1 = Tree::new(
                "hello".to_string(),
                vec![NodeType::Blob(blob1.clone())],
                "hash_tree1".to_string(),
            );

            let tree2 = Tree::new("".to_string(), vec![NodeType::Tree(tree1)], "".to_string());

            let tree3 = Tree::new(
                "zio".to_string(),
                vec![NodeType::Blob(blob3.clone()), NodeType::Blob(blob1.clone())],
                "hash_tree3".to_string(),
            );

            let tree4 = Tree::new(
                "".to_string(),
                vec![NodeType::Tree(tree3.clone())],
                "".to_string(),
            );

            let tree5 = Tree::new(
                "".to_string(),
                vec![NodeType::Tree(tree3), NodeType::Tree(tree2.clone())],
                "".to_string(),
            );

            (
                NodeType::Tree(tree2),
                NodeType::Tree(tree4),
                NodeType::Tree(tree5),
            )
        }

        fn multiple_same_name_different_content_setup() -> (NodeType, NodeType, NodeType) {
            let blob1 = Blob::new(
                "file1.txt".to_string(),
                "Ceci est le contenu du fichier 1.".to_string(),
                "hash_blob_1".to_string(),
            );

            let blob2 = Blob::new(
                "file1.txt".to_string(),
                "Ceci est le contenu du fichier 1\nHAHAHAHAHAH.".to_string(),
                "hash_blob_1".to_string(),
            );

            let blob3 = Blob::new(
                "file3.txt".to_string(),
                "Ceci est le contenu du fichier 3.".to_string(),
                "hash_blob_3".to_string(),
            );

            let tree1 = Tree::new(
                "hello".to_string(),
                vec![NodeType::Blob(blob1.clone())],
                "hash_tree1".to_string(),
            );

            let tree2 = Tree::new("".to_string(), vec![NodeType::Tree(tree1)], "".to_string());

            let tree3 = Tree::new(
                "zio".to_string(),
                vec![NodeType::Blob(blob3.clone()), NodeType::Blob(blob2.clone())],
                "hash_tree3".to_string(),
            );

            let tree4 = Tree::new(
                "".to_string(),
                vec![NodeType::Tree(tree3.clone())],
                "".to_string(),
            );

            let tree5 = Tree::new(
                "".to_string(),
                vec![NodeType::Tree(tree3), NodeType::Tree(tree2.clone())],
                "".to_string(),
            );

            (
                NodeType::Tree(tree2),
                NodeType::Tree(tree4),
                NodeType::Tree(tree5),
            )
        }

        #[test]
        fn basic() {
            let (n1, n2, n3) = basic_setup();
            assert_eq!(NodeType::merge(n1, n2).unwrap(), n3)
        }

        #[test]
        fn multiple_same_name_same_content() {
            let (n1, n2, n3) = multiple_same_name_setup();
            assert_eq!(NodeType::merge(n1, n2).unwrap(), n3)
        }

        #[test]
        fn multiple_same_name_different_content() {
            let (n1, n2, n3) = multiple_same_name_different_content_setup();
            assert_eq!(NodeType::merge(n1, n2).unwrap(), n3)
        }
    }
}
