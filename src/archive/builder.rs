use std::{
    fs::File,
    io::{BufReader, Read, Seek, Write},
    path::Path,
};

use crate::header::HeaderBuilder;

use super::error::ArchiverError;

pub struct ArchiveBuilder {}

impl ArchiveBuilder {
    pub const DEFAULT_BUFFER_SIZE: usize = 8192;
    pub const DEFAULT_BLOCK_SIZE: usize = 512;

    pub fn new() -> Self {
        Self {}
    }

    pub fn build(
        &self,
        archive_path: &Path,
        files: Vec<impl AsRef<Path>>,
    ) -> Result<(), ArchiverError> {
        let mut archive = File::create(archive_path)?;
        for file in files {
            self.add_file(&mut archive, file)?;
        }
        self.write_end_marker(&mut archive)
    }

    fn add_file(
        &self,
        archive: &mut File,
        file_path: impl AsRef<Path>,
    ) -> Result<(), ArchiverError> {
        let header = HeaderBuilder::build_from_path(&file_path)?;
        archive.write_all(&header)?;
        let file = File::open(&file_path)?;
        let mut reader = BufReader::new(file);
        let mut buffer = [0; Self::DEFAULT_BUFFER_SIZE];
        loop {
            let bytes_read = reader.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            archive.write_all(&buffer[..bytes_read])?;
        }
        let pos = archive.stream_position()?;
        let padding = ((Self::DEFAULT_BLOCK_SIZE - (pos as usize % Self::DEFAULT_BLOCK_SIZE))
            % Self::DEFAULT_BLOCK_SIZE);
        archive.write_all(&vec![0; padding])?;

        Ok(())
    }

    fn write_end_marker(&self, archive: &mut File) -> Result<(), ArchiverError> {
        archive.write_all(&vec![0u8; Self::DEFAULT_BLOCK_SIZE * 2])?;
        Ok(())
    }
}
