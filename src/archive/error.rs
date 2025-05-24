use crate::{header::HeaderError, validation::ValidationError};
use std::{fmt, io};

#[derive(Debug)]
pub enum ArchiverError {
    Validation(ValidationError),
    Io(io::Error),
    UnsupportedFeature(String),
    HeaderError(HeaderError),
}

impl fmt::Display for ArchiverError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Validation(e) => write!(f, "Validation failed: {}", e),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::UnsupportedFeature(msg) => write!(f, "Unsupported: {}", msg),
            Self::HeaderError(e) => write!(f, "header error: {}", e),
        }
    }
}

impl From<io::Error> for ArchiverError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ValidationError> for ArchiverError {
    fn from(e: ValidationError) -> Self {
        Self::Validation(e)
    }
}
impl From<HeaderError> for ArchiverError {
    fn from(e: HeaderError) -> Self {
        Self::HeaderError(e)
    }
}

impl std::error::Error for ArchiverError {}
