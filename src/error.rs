use std::fmt;
use std::fmt::{Formatter};

#[derive(Debug)]
pub enum DitError {
    NotInitialized,
    IoError(std::io::Error),
    UnexpectedComportement(String)
}

impl fmt::Display for DitError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            DitError::NotInitialized => write!(f, "Repository dit not initialized"),
            DitError::IoError(e) => write!(f, "IO error: {}",e),
            DitError::UnexpectedComportement(message) => write!(f, "{}", message),
        }
    }
}

impl std::error::Error for DitError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            DitError::IoError(e) => Some(e),
            _ => None,
        }
    }
}