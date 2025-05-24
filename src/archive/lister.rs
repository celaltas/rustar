use crate::{header::HeaderParser, validation::ArchiveValidator};
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

use super::error::ArchiverError;

#[derive(Debug)]
pub struct ArchiveEntry {
    pub name: String,
    pub size: u64,
    pub mode: u64,
    pub uid: u64,
    pub gid: u64,
    pub mtime: u64,
}

pub struct ArchiveLister {
    validator: ArchiveValidator,
    show_metadata: bool,
}

impl ArchiveLister {
    pub fn new(validator: ArchiveValidator) -> Self {
        Self {
            validator,
            show_metadata: false,
        }
    }

    pub fn show_metadata(&mut self, show: bool) -> &mut Self {
        self.show_metadata = show;
        self
    }

    pub fn list(&self, archive_path: &Path) -> Result<Vec<ArchiveEntry>, ArchiverError> {
        self.validator.validate(archive_path)?;

        let file = File::open(archive_path)?;
        let mut reader = BufReader::new(file);
        let mut entries = Vec::new();
        let mut buffer = [0u8; 512];

        loop {
            reader.read_exact(&mut buffer)?;
            if buffer.iter().all(|&b| b == 0) {
                break;
            }

            let header = HeaderParser::parse(&buffer)?;
            let entry = ArchiveEntry {
                name: header.name,
                size: header.size,
                mode: header.mode,
                uid: header.uid,
                gid: header.gid,
                mtime: header.mtime,
            };
            entries.push(entry);
            let padding = (512 - (header.size % 512)) % 512;
            reader.seek(SeekFrom::Current((header.size + padding) as i64))?;
        }

        Ok(entries)
    }

    pub fn _list_names(&self, archive_path: &Path) -> Result<Vec<String>, ArchiverError> {
        Ok(self
            .list(archive_path)?
            .into_iter()
            .map(|e| e.name)
            .collect())
    }
}
