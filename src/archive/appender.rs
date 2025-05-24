use crate::{header::HeaderBuilder, validation::ArchiveValidator};
use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Read, Seek, SeekFrom, Write},
    path::Path,
};

use super::ArchiverError;

pub struct ArchiveAppender {
    validator: ArchiveValidator,
    buffer_size: usize,
}

impl ArchiveAppender {
    pub fn new(validator: ArchiveValidator) -> Self {
        Self {
            validator,
            buffer_size: 8192,
        }
    }

    pub fn append(
        &self,
        archive_path: &Path,
        files: Vec<impl AsRef<Path>>,
    ) -> Result<(), ArchiverError> {
        self.validator.validate(archive_path)?;
        let mut archive = OpenOptions::new()
            .read(true)
            .write(true)
            .open(archive_path)?;
        archive.seek(SeekFrom::End(-1024))?;
        for file in files {
            self.append_file(&mut archive, file)?;
        }
        Ok(())
    }

    fn append_file(
        &self,
        archive: &mut File,
        file_path: impl AsRef<Path>,
    ) -> Result<(), ArchiverError> {
        let header = HeaderBuilder::build_from_path(&file_path)?;
        archive.write_all(&header)?;
        let mut reader = BufReader::new(File::open(&file_path)?);
        let mut buffer = vec![0u8; self.buffer_size];
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            archive.write_all(&buffer[..bytes_read])?;
        }

        let pos = archive.stream_position()?;
        let padding = ((512 - (pos % 512)) % 512) as usize;
        archive.write_all(&vec![0u8; padding])?;

        Ok(())
    }
}
