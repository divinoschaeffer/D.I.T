use std::{io, process};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, ErrorKind, Write};
use std::os::unix::fs::FileExt;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{find_info, find_refs, get_head_hash};
use crate::utils::{NULL_HASH, write_footer_file, write_hash_file, write_header_file};

pub struct Branch {
    head: String,
    name: String,
}

impl Branch {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_head(&self) -> &String {
        &self.head
    }

    pub fn new_branch(name: String, head: String) -> Result<Branch, DitError> {
        if !Self::is_name_ok(&name) {
            display_message("Branch name must not contains '/'.", Color::RED);
            process::exit(1);
        }
        let ref_path = find_refs();
        let file_path = ref_path.join(&name);

        if file_path.exists() {
            display_message("Branch with same name already exist, cannot create the branch", Color::RED);
            process::exit(1);
        }
        File::create(file_path).map_err(DitError::IoError)?;
        Self::set_info_file(name.clone(), head.clone()).map_err(DitError::IoError)?;
        if head != NULL_HASH {
            let branch_path = format!("./.dit/refs/{}", name);

            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(branch_path).map_err(DitError::IoError)?;

            let mut writer = BufWriter::new(file);
            writeln!(writer, "{}", head).map_err(DitError::IoError)?;
        }
        Ok(Branch {
            head,
            name,
        })
    }

    pub fn is_name_ok(name: &String) -> bool {
        !(name.contains('/') || name.contains('\\'))
    }

    pub fn exist(name: String) -> bool {
        let ref_path = find_refs();
        let file_path = ref_path.join(name.clone());

        if file_path.is_file() {
            return true;
        }

        false
    }

    pub fn set_info_file(name: String, head: String) -> Result<(), io::Error> {
        let file = File::create("./.dit/info")?;

        write_header_file(String::from("HEAD"), &file, 0)?;
        write_hash_file(head, &file, 5)?;
        write_footer_file(name, file, 46)?;

        Ok(())
    }

    pub fn get_current_branch() -> Result<Branch, DitError> {
        let head = get_head_hash()?;
        let info = find_info();

        let mut buf = [0u8; 100];

        let file = File::open(info).map_err(DitError::IoError)?;

        file.read_at(&mut buf, 46).map_err(DitError::IoError)?;
        let filtered_bytes: Vec<u8> = buf.iter().cloned().filter(|&b| b != 0).collect();

        let name = String::from_utf8(filtered_bytes).unwrap();

        Ok(Branch {
            head,
            name,
        })
    }

    pub fn get_branch(name: String) -> Result<Branch, DitError> {
        let ref_path = find_refs();
        let file_path = ref_path.join(name.clone());

        let file = OpenOptions::new()
            .read(true)
            .open(file_path).map_err(DitError::IoError)?;

        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().collect();
        let line = lines.last().ok_or_else(|| DitError::IoError(io::Error::new(io::ErrorKind::UnexpectedEof, "File is empty")))?;
        return match line {
            Ok(hash) => {
                let branch = Branch {
                    head: hash.to_string(),
                    name,
                };
                Ok(branch)
            }
            _ => Err(DitError::IoError(io::Error::new(ErrorKind::InvalidData, "Head not found")))
        };
    }
}
