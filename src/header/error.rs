use std::{fmt::Display, io, num::ParseIntError, str::Utf8Error};


#[derive(Debug)]
pub enum HeaderError {
    InvalidFileName(String),
    IntConversion(ParseIntError),
    ChecksumMisatch,
    InvalidHeaderFormat,
    Io(io::Error),
    Utf8Conversion(Utf8Error),
}

impl Display for HeaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidFileName(name) => write!(f, "invalid file name {name}"),
            Self::IntConversion(e) => write!(f, "parse error {e}"),
            Self::ChecksumMisatch => {
                write!(f, "header checksum mismatch - possibly corrupted ")
            }
            HeaderError::InvalidHeaderFormat => write!(f, "Invalid USTAR header format"),
            Self::Io(e) => write!(f, "IO error: {}", e),
            Self::Utf8Conversion(e) => write!(f, "UTF-8 conversion error: {}", e),
        }
    }
}

impl From<io::Error> for HeaderError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<Utf8Error> for HeaderError {
    fn from(value: Utf8Error) -> Self {
        Self::Utf8Conversion(value)
    }
}
impl From<ParseIntError> for HeaderError {
    fn from(value: ParseIntError) -> Self {
        Self::IntConversion(value)
    }
}
