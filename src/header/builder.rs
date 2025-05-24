use super::{constants::*, error::HeaderError};
use std::{fs, os::unix::fs::MetadataExt, path::Path};

pub struct HeaderBuilder;

impl HeaderBuilder {
    pub fn build_from_path(path: impl AsRef<Path>) -> Result<[u8; BLOCK_SIZE], HeaderError> {
        let metadata = fs::metadata(path.as_ref())?;
        let name = path
            .as_ref()
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| {
                HeaderError::InvalidFileName(path.as_ref().to_string_lossy().into_owned())
            })?;
        Self::build(name, &metadata)
    }

    pub fn build(name: &str, metadata: &fs::Metadata) -> Result<[u8; BLOCK_SIZE], HeaderError> {
        let mut header = [0u8; BLOCK_SIZE];
        let name_bytes = name.as_bytes();
        header[NAME_FIELD][..name_bytes.len()].copy_from_slice(name_bytes);

        Self::write_octal(&mut header[MODE_FIELD], metadata.mode() as u64, 8)?;
        Self::write_octal(&mut header[UID_FIELD], metadata.uid() as u64, 8)?;
        Self::write_octal(&mut header[GID_FIELD], metadata.gid() as u64, 8)?;
        Self::write_octal(&mut header[SIZE_FIELD], metadata.size(), 12)?;
        Self::write_octal(&mut header[MTIME_FIELD], metadata.mtime() as u64, 12)?;

        header[CHECKSUM_FIELD].fill(b' ');
        header[TYPEFLAG_FIELD] = TYPEFLAG_REGULAR;
        header[MAGIC_FIELD].copy_from_slice(USTAR_MAGIC);
        header[VERSION_FIELD].copy_from_slice(USTAR_VERSION);

        let checksum = header.iter().map(|&b| b as u32).sum::<u32>();
        Self::write_octal(&mut header[CHECKSUM_FIELD], checksum as u64, 8)?;

        Ok(header)
    }

    fn write_octal(dst: &mut [u8], value: u64, len: usize) -> Result<(), HeaderError> {
        let s = format!("{:0len$o}", value, len = len - 1);
        dst[..s.len()].copy_from_slice(s.as_bytes());
        dst[s.len()] = b'\0';
        Ok(())
    }
}
