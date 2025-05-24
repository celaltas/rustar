use super::{ParsedHeader, constants::*, error::HeaderError};
use std::str::from_utf8;
pub struct HeaderParser;

impl HeaderParser {
    pub fn parse(header: &[u8]) -> Result<ParsedHeader, HeaderError> {
        let name = from_utf8(&header[NAME_FIELD])?
            .trim_end_matches('\0')
            .to_string();

        let mode = Self::read_octal(&header[MODE_FIELD])?;
        let uid = Self::read_octal(&header[UID_FIELD])?;
        let gid = Self::read_octal(&header[GID_FIELD])?;
        let size = Self::read_octal(&header[SIZE_FIELD])?;
        let mtime = Self::read_octal(&header[MTIME_FIELD])?;

        Ok(ParsedHeader {
            name,
            size,
            mode,
            gid,
            uid,
            mtime,
        })
    }

    pub fn parse_size(header: &[u8]) -> Result<u64, HeaderError> {
        Self::read_octal(&header[SIZE_FIELD])
    }

    fn read_octal(src: &[u8]) -> Result<u64, HeaderError> {
        let s = from_utf8(src)?.trim_end_matches('\0').trim();
        let v = u64::from_str_radix(s, 8)?;
        Ok(v)
    }
}
