use std::{fmt, io};

#[derive(Debug)]
pub enum ValidationError {
    InvalidStructure(String),
    InvalidContent(String),
    InvalidExtension(String),
    Io(io::Error),
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidStructure(msg) => write!(f, "Invalid archive structure: {}", msg),
            Self::InvalidContent(msg) => write!(f, "Content validation failed: {}", msg),
            Self::InvalidExtension(msg) => write!(f, "Invalid extension: {}", msg),
            Self::Io(e) => write!(f, "IO error during validation: {}", e),
        }
    }
}

impl From<io::Error> for ValidationError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl std::error::Error for ValidationError {}
