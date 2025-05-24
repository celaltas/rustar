use super::{constants::*, error::HeaderError};

pub struct HeaderValidator;

impl HeaderValidator {
    pub fn validate(header: &[u8]) -> Result<(), HeaderError> {
        Self::validate_magic(header)?;
        Self::validate_checksum(header)?;
        Ok(())
    }
    fn validate_magic(header: &[u8]) -> Result<(), HeaderError> {
        if &header[MAGIC_FIELD] != USTAR_MAGIC || &header[VERSION_FIELD] != USTAR_VERSION {
            return Err(HeaderError::InvalidHeaderFormat);
        }
        Ok(())
    }
    pub fn validate_checksum(header: &[u8]) -> Result<(), HeaderError> {
        let file_checksum = header[CHECKSUM_FIELD]
            .iter()
            .map(|&b| b as u64)
            .sum::<u64>();
        let actual_checksum = header.iter().map(|&b| b as u64).sum::<u64>()
            - header[CHECKSUM_FIELD]
                .iter()
                .map(|&b| b as u64)
                .sum::<u64>();
        if file_checksum != actual_checksum {
            return Err(HeaderError::ChecksumMisatch);
        }
        Ok(())
    }
}
