use std::io;
use std::io::Write;
use std::path::PathBuf;

use dit_file_encryptor::CompressedFile;

pub fn message(message: String) -> Result<(), io::Error> {
    let desc_path = PathBuf::from("./.dit/commit");

    let mut writer = CompressedFile::create_file(desc_path)?
        .open_for_write()?;
    write!(writer, "{}", message)?;
    Ok(())
}