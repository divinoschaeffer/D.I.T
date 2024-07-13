pub mod init {

    use std::{fs, io};

    pub fn init_repository() -> Result<(), io::Error> {
        if fs::metadata("./.dit").is_ok() {
            println!("dit is already initialized");
            return Ok(());
        }
        
        fs::create_dir_all("./.dit/objects")?;
        fs::create_dir("./.dit/refs")?;
        Ok(())
    }
}

