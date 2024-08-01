use crate::objects::{Blob as StructBlob, Blob, BLOB, NodeType, Tree as StructTree, Tree};
use crate::utils::{NULL_HASH, read_content_file_from_path, real_path, write_hash_file};
use std::fs::{ File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use crate::arguments::init::{find_dit, get_object_path, get_staged_hash, open_object_file};

pub fn add(elements: Vec<&String>) -> Result<(), io::Error> {
    let dit_path = find_dit().unwrap_or_else(|| {
        panic!("dit is not initialized");
    });
    
    let object_path = dit_path.join("objects");
    let staged_path = dit_path.join("staged");
    let staged_hash = get_staged_hash();
    
    if elements.is_empty() {
        println!("You need to specify files to add");
    } else if staged_hash == NULL_HASH {
        
        let tree = StructTree::new(String::from(""), Vec::new(), String::from(""));
        let mut root: NodeType = NodeType::Tree(tree);

        add_elements(&elements, &object_path, &staged_path, &mut root);

    } else {
        let mut tree = StructTree::new(String::from(""), Vec::new(), String::from(staged_hash.clone()));
        create_tree_node_from_file(staged_hash, &mut tree);
        let mut root = NodeType::Tree(tree);

        add_elements(&elements, &object_path, &staged_path, &mut root);
    }
    Ok(())
}

fn add_elements(elements: &Vec<&String>, object_path: &PathBuf, staged_path: &PathBuf, mut root: &mut NodeType) {
    for element in elements {
        create_repository_tree(&mut root, element);
    }

    let root_hash = NodeType::create_node_hash(&mut root);
    
    transcript_tree_to_files(&mut root, &object_path);

    let file = OpenOptions::new()
        .write(true)
        .append(false)
        .create(true)
        .open(staged_path).unwrap();

    write_hash_file(root_hash, &file, 0).unwrap();
}

pub(crate) fn create_tree_node_from_file(staged_hash: String, mut tree: &mut Tree) {
    let root_file = open_object_file(staged_hash);
    let reader = BufReader::new(root_file);

    for line in reader.lines() {
        let content = match line {
            Ok(content) => content,
            Err(e) => panic!("Error while reading object file: {}",e),
        };
        let hash = &content[5..45];
        let name = &content[46..];

        if &content[0..4] == BLOB {
            create_blob_node_from_file(String::from(name), String::from(hash), &mut tree)
        } else {
            let mut new_tree = StructTree::new(String::from(name), Vec::new(), String::from(hash));
            create_tree_node_from_file(String::from(hash), &mut new_tree);
            let node = NodeType::Tree(new_tree);
            tree.add_node(node);
        }
    }
}

fn create_blob_node_from_file(file_name: String, hash: String, root: &mut Tree) {
    let blob_file = open_object_file(String::from(hash.clone()));
    let mut buf_reader = BufReader::new(blob_file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    let blob = StructBlob::new(file_name,contents, hash);
    let node = NodeType::Blob(blob);
    root.add_node(node);
}

fn create_repository_tree(root: &mut NodeType, element: &String) {
    let real_path = real_path(element);
    if !real_path.exists() {
        println!("path: {} does not exist", real_path.display());
        return;
    }

    let mut ancestors: Vec<_> = real_path.ancestors().collect();
    ancestors.pop();
    ancestors.reverse();

    _create_repository_tree(root, &mut ancestors);
}

fn _create_repository_tree<'a>(root: &'a mut NodeType, paths: &mut Vec<&Path>) {
    if paths.is_empty() {
        return;
    }

    let path = paths[0];
    let file_name = path.file_name().unwrap().to_str().unwrap();

    if path.is_dir() {
        create_tree_node(root,file_name,paths);
    } else {
        create_blob_node(root, paths, &path, file_name);
    }
}

fn create_blob_node(root: &mut NodeType, paths: &mut Vec<&Path>, path: &&Path, file_name: &str) {
    let content = read_content_file_from_path(&path).unwrap_or_else(|e| {
        panic!("Error while reading {:?} : {e}", file_name);
    });
    let mut file_blob: StructBlob =
        StructBlob::new(String::from(file_name), content, String::from(""));
    file_blob.create_hash();
    let node: NodeType = NodeType::Blob(file_blob);

    if let NodeType::Tree(ref mut tree) = root {
        if !tree.exist_node(&node) {
            paths.remove(0);
            tree.add_node(node);
        } else {
            if let Some(old_blob) = tree.find_node(&node) {
                let to_remove = old_blob.clone();
                tree.remove_node(&to_remove);
            }
            paths.remove(0);
            tree.add_node(node);
        }
    }
}

fn create_tree_node(root: &mut NodeType, file_name: &str, paths: &mut Vec<&Path>) {
    let dir_tree = StructTree::new(String::from(file_name), Vec::new(), String::from(""));
    let mut node = NodeType::Tree(dir_tree);
    if let NodeType::Tree(ref mut tree) = root {
        if !tree.contains(&node) {
            paths.remove(0);
            _create_repository_tree(&mut node, paths);
            tree.add_node(node);
        } else {
            if let Some(old_node) = tree.find_node(&node) {
                paths.remove(0);
                _create_repository_tree(old_node, paths);
            } else {
                panic!("Error while finding directory already existing");
            }
        }
    }
}

pub(crate) fn transcript_tree_to_files(root: &mut NodeType, objects_path: &PathBuf){
    if let NodeType::Tree(tree) = root {
        let hash = tree.get_hash().clone();

        let node_path = get_object_path(objects_path, &hash);
        if !node_path.exists() {
            let file = File::create(&node_path).unwrap_or_else(|e1| {
                panic!("Error while creating file in objects directory: {e1}");
            });
            let mut writer = BufWriter::new(file);

            write_tree_node_to_file(&objects_path, tree, &mut writer);
        }
    }
    
    if let NodeType::Blob(blob) = root {
        let hash = blob.get_hash().clone();
        
        let node_path = get_object_path(objects_path, &hash);
        if !node_path.exists() {
            let file = File::create(&node_path).unwrap_or_else(|e1| {
                panic!("Error while creating file in objects directory: {e1}");
            });
            write_blob_content_to_file(blob, file);
        }
    }
}

fn write_blob_content_to_file(blob: &Blob, file: File) {
    let mut writer = BufWriter::new(file);

    writeln!(writer, "{}", blob.get_content())
        .unwrap_or_else(|e| {
            panic!("Error while writing to file: {e}");
        });
}

fn write_tree_node_to_file(objects_path: &&PathBuf, tree: &mut Tree, writer: &mut BufWriter<File>) {
    for node in tree.get_nodes() {
        if let NodeType::Tree(current_tree) = node {
            writeln!(writer, "tree {} {}", current_tree.get_hash(), current_tree.get_name())
                .unwrap_or_else(|e| {
                    panic!("Error while writing to file: {e}");
                });
        }
        if let NodeType::Blob(current_blob) = node {
            writeln!(writer, "blob {} {}", current_blob.get_hash(), current_blob.get_name())
                .unwrap_or_else(|e| {
                    panic!("Error while writing to file: {e}");
                });
        }
        transcript_tree_to_files(node, &objects_path);
    }
}

