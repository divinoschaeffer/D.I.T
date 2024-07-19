use crate::objects::{Blob as StructBlob, Blob, NodeType, Tree as StructTree, Tree};
use crate::utils::{find_dit, read_content_file, read_hash_file, real_path};
use std::fs::{create_dir, File};
use std::io;
use std::io:: {BufWriter, Write};
use std::path::{Path, PathBuf};

pub fn add_files_or_directory(elements: Vec<&String>) -> Result<(), io::Error> {
    let object_path = find_dit().unwrap().join("objects");
    if elements.is_empty() {
        println!("You need to specify files to add");
    } else if is_first_commit() {
        let tree = StructTree::new(String::from(""), Vec::new(), String::from(""));
        let mut racine: NodeType = NodeType::Tree(tree);

        for element in elements {
            create_repository_tree(&mut racine, element);
        }

        NodeType::create_node_hash(&mut racine);

        racine.display();
        
        transcript_tree_to_files(&racine, &object_path);
    }
    Ok(())
}

fn is_first_commit() -> bool {
    let dit_path = find_dit().unwrap();
    let info_path = dit_path.join("info");
    let file = File::open(info_path).unwrap();
    read_hash_file(file, 5) == "0000000000000000000000000000000000000000"
}

fn create_repository_tree(racine: &mut NodeType, element: &String) {
    let real_path = real_path(element);
    if !real_path.exists() {
        println!("path: {} does not exist", real_path.display());
        return;
    }

    let mut ancestors: Vec<_> = real_path.ancestors().collect();
    ancestors.pop();
    ancestors.reverse();

    _create_repository_tree(racine, &mut ancestors);
}

fn _create_repository_tree<'a>(racine: &'a mut NodeType, paths: &mut Vec<&Path>) {
    if paths.is_empty() {
        return;
    }

    let path = paths[0];
    let file_name = path.file_name().unwrap().to_str().unwrap();

    if path.is_dir() {
        create_tree_node(racine,file_name,paths);
    } else {
        create_blob_node(racine, paths, &path, file_name);
    }
}

fn create_blob_node(racine: &mut NodeType, paths: &mut Vec<&Path>, path: &&Path, file_name: &str) {
    let content = read_content_file(&path).unwrap_or_else(|e| {
        panic!("Error while reading {:?} : {e}", file_name);
    });
    let mut file_blob: StructBlob =
        StructBlob::new(String::from(file_name), content, String::from(""));
    file_blob.create_hash();
    let node: NodeType = NodeType::Blob(file_blob);
    if let NodeType::Tree(ref mut tree) = racine {
        if !tree.contains(&node) {
            paths.remove(0);
            tree.add_node(node);
        }
    }
}

fn create_tree_node(racine: &mut NodeType, file_name: &str, paths: &mut Vec<&Path>) {
    let dir_tree = StructTree::new(String::from(file_name), Vec::new(), String::from(""));
    let mut node = NodeType::Tree(dir_tree);
    if let NodeType::Tree(ref mut tree) = racine {
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

fn transcript_tree_to_files(racine: &NodeType, objects_path: &PathBuf){
    if let NodeType::Tree(tree) = racine {
        let hash = tree.get_hash().clone();

        let node_path = get_node_path(objects_path, &hash);
        if !node_path.exists() {
            let file = File::create(&node_path).unwrap_or_else(|e1| {
                panic!("Error while creating file in objects directory: {e1}");
            });
            let mut writer = BufWriter::new(file);

            write_tree_to_file(&objects_path, tree, &mut writer);
        }
    }
    
    if let NodeType::Blob(blob) = racine {
        let hash = blob.get_hash().clone();
        
        let node_path = get_node_path(objects_path,&hash);
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

fn write_tree_to_file(objects_path: &&PathBuf, tree: &Tree, writer: &mut BufWriter<File>) {
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

fn get_node_path(objects_path: &PathBuf, hash: &String) -> PathBuf {
    let b_hash = &hash[..2];
    let e_hash = &hash[2..];


    let node_dir_path = objects_path.join(b_hash);
    if !node_dir_path.exists() {
        create_dir(&node_dir_path).unwrap_or_else(|e| {
            panic!("Error while creating directory in objects directory: {e}");
        })
    }
    let node_path = node_dir_path.join(e_hash);
    node_path
}