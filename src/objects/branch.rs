use std::fs::File;
use std::io;
use std::os::unix::fs::FileExt;
use crate::arguments::init::{find_dit, find_info, get_head_hash};
use crate::utils::{ write_footer_file, write_hash_file, write_header_file};

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
    
    pub fn create_branch(name: String, head: String) -> Branch{
        let ref_path = find_dit().unwrap().join("refs");
        File::create(ref_path.join(name.clone())).unwrap();
        
        Self::set_info_file(name.clone(), head.clone()).unwrap();
        
        Branch {
            head,
            name
        }
    }

    fn set_info_file(name: String, head: String) -> Result<(), io::Error> {
        let file = File::create("./.dit/info")?;

        write_header_file(String::from("HEAD"), &file, 0)?;
        write_hash_file(head, &file, 5, )?;
        write_footer_file(name, file, 46)?;

        Ok(())
    }
    
    pub fn get_branch_from_file() -> Branch {
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
    
}
