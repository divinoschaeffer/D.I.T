use std::{io, process};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, ErrorKind, Read, Write};
use std::path::PathBuf;

use dit_file_encryptor::CompressedFile;

use crate::error::DitError;
use crate::features::display_message::{Color, display_message};
use crate::features::init::{find_info, find_refs, get_head_hash};
use crate::utils::NULL_HASH;

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
            process::exit(0);
        }
        let ref_path = find_refs();
        let file_path = ref_path.join(&name);

        if file_path.exists() {
            display_message("Branch with same name already exist, cannot create the branch", Color::RED);
            process::exit(0);
        }
        let file = CompressedFile::create_file(file_path)
            .map_err(|e| DitError::IoError(e))?;
        Self::set_info_file(name.clone(), head.clone()).map_err(DitError::IoError)?;
        if head != NULL_HASH {
            let writer = file
                .open_for_write()
                .map_err(|e| DitError::IoError(e))?;

            let mut writer = BufWriter::new(writer);
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
        let path_info_file = PathBuf::from("./.dit/info");
        let _ = File::create(path_info_file.clone())?;
        let mut writer = CompressedFile::new(path_info_file)
            .open_for_write()
            .map_err(|e| {
                io::Error::new(io::ErrorKind::InvalidData, e)
            })?;

        write!(writer, "{} {} {}", "HEAD", head, name)?;

        Ok(())
    }

    pub fn get_current_branch() -> Result<Branch, DitError> {
        let head = get_head_hash()?;
        let info_path = find_info();

        let mut reader = CompressedFile::new(info_path)
            .open_for_read()
            .map_err(|e| {
                DitError::IoError(e)
            })?;

        let mut buf = String::new();
        let _ = reader.read_to_string(&mut buf);
        let infos: Vec<_> = buf.split_whitespace().collect();
        let name = String::from(infos[2]);

        Ok(Branch {
            head,
            name,
        })
    }

    pub fn get_branch(name: String) -> Result<Branch, DitError> {
        let ref_path = find_refs();
        let file_path = ref_path.join(name.clone());

        let reader = CompressedFile::new(file_path)
            .open_for_read()
            .map_err(|e| {
                DitError::IoError(e)
            })?;
        let buf_reader = BufReader::new(reader);
        let lines: Vec<_> = buf_reader.lines().collect();
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
