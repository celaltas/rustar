use std::ops::Range;

pub const NAME_FIELD: Range<usize> = 0..100;
pub const MODE_FIELD: Range<usize> = 100..108;
pub const UID_FIELD: Range<usize> = 108..116;
pub const GID_FIELD: Range<usize> = 116..124;
pub const SIZE_FIELD: Range<usize> = 124..136;
pub const MTIME_FIELD: Range<usize> = 136..148;
pub const CHECKSUM_FIELD: Range<usize> = 148..156;
pub const TYPEFLAG_FIELD: usize = 156;
pub const MAGIC_FIELD: Range<usize> = 257..263;
pub const VERSION_FIELD: Range<usize> = 263..265;
pub const USTAR_MAGIC: &[u8; 6] = b"ustar\0";
pub const USTAR_VERSION: &[u8; 2] = b"00";
pub const TYPEFLAG_REGULAR: u8 = b'0';
pub const BLOCK_SIZE: usize = 512;
pub const END_MARKER_BLOCKS: usize = 2;

