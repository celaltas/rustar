use crate::{header::HeaderParser, validation::ArchiveValidator};
use std::{
    fs::{self, File},
    io::{BufReader, Read, Write},
    path::Path,
};

use super::error::ArchiverError;

pub struct ArchiveExtractor {
    validator: ArchiveValidator,
    overwrite: bool,
}

impl ArchiveExtractor {
    pub fn new(validator: ArchiveValidator) -> Self {
        Self {
            validator,
            overwrite: false,
        }
    }

    pub fn _set_overwrite(&mut self, overwrite: bool) -> &mut Self {
        self.overwrite = overwrite;
        self
    }

    pub fn extract(&self, archive_path: &Path, output_dir: &Path) -> Result<(), ArchiverError> {
        self.validator.validate(archive_path)?;
        let archive = File::open(archive_path)?;
        let mut reader = BufReader::new(archive);
        let mut buffer = [0u8; 512];
        loop {
            reader.read_exact(&mut buffer)?;
            if buffer.iter().all(|&b| b == 0) {
                break;
            }
            let header = HeaderParser::parse(&buffer)?;
            let output_path = output_dir.join(&header.name);
            if output_path.exists() && !self.overwrite {
                return Err(ArchiverError::UnsupportedFeature(
                    "File exists and overwrite disabled".into(),
                ));
            }

            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut file = File::create(&output_path)?;
            let mut remaining = header.size;
            let mut chunk = vec![0u8; 8192.min(remaining as usize)];

            while remaining > 0 {
                let to_read = chunk.len().min(remaining as usize);
                reader.read_exact(&mut chunk[..to_read])?;
                file.write_all(&chunk[..to_read])?;
                remaining -= to_read as u64;
            }
            let padding = (512 - (header.size % 512)) % 512;
            reader.seek_relative(padding as i64)?;
        }

        Ok(())
    }
}
