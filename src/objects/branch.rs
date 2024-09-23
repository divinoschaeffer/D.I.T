use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Write, BufWriter, BufReader, BufRead, Error, ErrorKind};
use std::os::unix::fs::FileExt;
use crate::arguments::init::{find_dit, find_info, get_head_hash};
use crate::utils::{NULL_HASH, write_footer_file, write_hash_file, write_header_file};

pub struct Branch {
    head: String,
    name: String
}

impl Branch {
    pub fn get_name(&self) -> &String {
        &self.name
    }
    
    pub fn get_head(&self) -> &String {
        &self.head
    }
    
    pub fn new_branch(name: String, head: String) -> Branch{
        let ref_path = find_dit().unwrap().join("refs");
        let file_path = ref_path.join(name.clone());
        
        if file_path.is_file() {
            panic!("Can't have two branch with same name");
        }
        
        File::create(file_path).unwrap();
        
        Self::set_info_file(name.clone(), head.clone()).unwrap();
        
        if head != NULL_HASH {
            let branch_path = format!("./.dit/refs/{}", name);
            
            let file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(branch_path).unwrap();

            let mut writer = BufWriter::new(file);
            writeln!(writer, "{}",head).unwrap();
        }
        
        Branch {
            head,
            name
        }
    }

    pub fn exist(name: String) -> bool{
        let ref_path = find_dit().unwrap().join("refs");
        let file_path = ref_path.join(name.clone());

        if file_path.is_file() {
            return true
        }

        false
    }

    pub fn set_info_file(name: String, head: String) -> Result<(), io::Error> {
        let file = File::create("./.dit/info")?;

        write_header_file(String::from("HEAD"), &file, 0)?;
        write_hash_file(head, &file, 5, )?;
        write_footer_file(name, file, 46)?;

        Ok(())
    }
    
    pub fn get_current_branch() -> Branch {
        let head = get_head_hash();
        let info = find_info();

        let mut buf = [0u8; 100];
        
        let file = File::open(info).unwrap();

        file.read_at(&mut buf, 46).unwrap();
        let filtered_bytes: Vec<u8> = buf.iter().cloned().filter(|&b| b != 0).collect();

        let name =  String::from_utf8(filtered_bytes).unwrap();
        
        Branch {
            head,
            name
        }
    }

    pub fn get_branch(name: String) -> Result<Branch, io::Error> {
        let ref_path = find_dit().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Ref directory not found"))?.join("refs");
        let file_path = ref_path.join(name.clone());

        let file = OpenOptions::new()
            .read(true)
            .open(file_path)?;

        let reader = BufReader::new(file);
        let lines: Vec<_> = reader.lines().collect();
        let line = lines.last().ok_or_else( || io::Error::new(io::ErrorKind::UnexpectedEof, "File is empty"))?;
        return match line {
            Ok(hash) => {
                let branch = Branch {
                    head: hash.to_string(),
                    name
                };
                Ok(branch)
            }
            _ => Err(Error::new(ErrorKind::InvalidData, "Head not found"))
        };
    }
}
