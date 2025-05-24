use crate::header::{HeaderParser, HeaderValidator, constants::*};
use std::{
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::Path,
};

use super::error::ValidationError;

pub struct ArchiveStructureValidator;

impl ArchiveStructureValidator {
    pub fn validate(archive_path: &Path) -> Result<(), ValidationError> {
        let file = File::open(archive_path)?;
        let mut reader = BufReader::new(file);
        let file_size = reader.get_ref().metadata()?.len();

        if file_size < (BLOCK_SIZE * END_MARKER_BLOCKS) as u64 {
            return Err(ValidationError::InvalidStructure(format!(
                "Archive too small ({} bytes), must be at least {} bytes",
                file_size,
                BLOCK_SIZE * END_MARKER_BLOCKS
            )));
        }

        loop {
            let mut header = [0u8; BLOCK_SIZE];
            let bytes_read = reader.read(&mut header)?;
            if bytes_read == 0 {
                break;
            }

            if bytes_read != BLOCK_SIZE {
                return Err(ValidationError::InvalidStructure(
                    "Incomplete header block (not 512 bytes)".to_string(),
                ));
            }
            if header.iter().all(|&b| b == 0) {
                continue;
            }
            HeaderValidator::validate(&header)
                .map_err(|e| ValidationError::InvalidStructure(e.to_string()))?;
            let size = HeaderParser::parse_size(&header)
                .map_err(|e| ValidationError::InvalidStructure(e.to_string()))?
                as usize;
            let padding = (BLOCK_SIZE - (size % BLOCK_SIZE)) % BLOCK_SIZE;
            reader.seek_relative((size + padding) as i64)?;
        }

        let end_pos = (BLOCK_SIZE * END_MARKER_BLOCKS) as i64;
        reader.seek(SeekFrom::End(-end_pos))?;
        let mut end = [0u8; BLOCK_SIZE * END_MARKER_BLOCKS];
        reader.read_exact(&mut end)?;
        if end.iter().any(|&b| b != 0) {
            return Err(ValidationError::InvalidStructure(
                "Archive missing proper zero padding at end".to_string(),
            ));
        }

        Ok(())
    }
}
