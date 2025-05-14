use std::{
    fs::{self, File, OpenOptions, metadata},
    io::{BufReader, Read, Seek, SeekFrom, Write},
    os::unix::fs::MetadataExt,
    path::Path,
    str::from_utf8,
};

type ArchiveResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct Archiver {}

impl Archiver {
    pub fn new() -> Self {
        Archiver {}
    }

    pub fn create(&self, archive_name: &str, files: Vec<impl AsRef<Path>>) -> ArchiveResult<()> {
        let mut archive = std::fs::File::create(archive_name)?;
        for file in files {
            let header = self.generate_header(&file)?;
            archive.write_all(&header)?;
            let file = File::open(&file)?;
            let mut buffer = [0; 8192];
            let mut reader = BufReader::new(file);
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                archive.write_all(&buffer[..bytes_read])?;
            }
            let pos = archive.stream_position()?;
            let padding = ((512 - (pos % 512)) % 512) as usize;
            archive.write_all(&vec![0; padding])?;
        }
        archive.write_all(&[0; 1024])?;
        Ok(())
    }

    fn generate_header(&self, file_name: impl AsRef<Path>) -> ArchiveResult<[u8; 512]> {
        let mut header = [0u8; 512];
        let metadata = fs::metadata(file_name.as_ref())?;
        let name = Path::new(file_name.as_ref())
            .file_name()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid filename",
            ))?
            .to_str()
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Non-UTF-8 filename",
            ))?;
        let name_bytes = name.as_bytes();
        header[0..name_bytes.len()].copy_from_slice(name_bytes);
        self.write_octal(&mut header[100..108], metadata.mode() as u64, 8)?;
        self.write_octal(&mut header[108..116], metadata.uid() as u64, 8)?;
        self.write_octal(&mut header[116..124], metadata.gid() as u64, 8)?;
        self.write_octal(&mut header[124..136], metadata.size(), 12)?;
        self.write_octal(&mut header[136..148], metadata.mtime() as u64, 12)?;
        for i in 148..156 {
            header[i] = b' ';
        }
        header[156] = b'0';
        header[257..263].copy_from_slice(b"ustar\0");
        header[263..265].copy_from_slice(b"00");

        let checksum = header.iter().map(|&b| b as u32).sum::<u32>();
        self.write_octal(&mut header[148..156], checksum as u64, 8)?;
        Ok(header)
    }

    fn write_octal(&self, dst: &mut [u8], value: u64, len: usize) -> ArchiveResult<()> {
        let s = format!("{:0len$o}", value, len = len - 1);
        dst[..s.len()].copy_from_slice(s.as_bytes());
        dst[s.len()] = b'\0';
        Ok(())
    }

    fn read_octal(&self, src: &[u8]) -> ArchiveResult<u64> {
        let s = from_utf8(src)?.trim_end_matches('\0').trim();
        let v = u64::from_str_radix(&s, 8)?;
        Ok(v)
    }

    fn parse_header(&self, header: &[u8]) -> ArchiveResult<(String, u64)> {
        let name = from_utf8(&header[0..100])?.trim_end_matches('\0');
        let _mode = self.read_octal(&header[100..108])?;
        let _uid = self.read_octal(&header[108..116])?;
        let _gid = self.read_octal(&header[116..124])?;
        let size = self.read_octal(&header[124..136])?;
        let _mtime = self.read_octal(&header[136..148])?;
        let _checksum = self.read_octal(&header[148..156])?;
        Ok((name.to_string(), size))
    }

    pub fn extract(&self, archive_name: &str) -> ArchiveResult<()> {
        let archive = File::open(archive_name)?;
        let mut reader = BufReader::new(archive);
        loop {
            let mut header = [0u8; 512];
            let bytes_read = reader.read(&mut header)?;
            if bytes_read == 0 {
                break;
            }
            if header.iter().all(|&b| b == 0) {
                break;
            }
            let (file_name, size) = self.parse_header(&header)?;
            let mut file_data = vec![0; size as usize];
            reader.read_exact(&mut file_data)?;
            fs::write(file_name, &file_data)?;
            let padding = (512 - (size % 512)) % 512;
            reader.seek_relative(padding as i64)?;
        }
        Ok(())
    }

    pub fn list(&self, archive_name: &str) -> ArchiveResult<Vec<String>> {
        let archive = File::open(archive_name)?;
        let mut file_list = Vec::new();
        let mut reader = BufReader::new(archive);
        loop {
            let mut header = [0u8; 512];
            let bytes_read = reader.read(&mut header)?;
            if bytes_read == 0 {
                break;
            }
            if header.iter().all(|&b| b == 0) {
                break;
            }
            let (name, size) = self.parse_header(&header)?;
            let padding = (512 - (size % 512)) % 512;
            reader.seek_relative((size + padding) as i64)?;
            file_list.push(name);
        }
        println!("all files in tar: {:#?}", file_list);
        Ok(file_list)
    }

    pub fn append(&self, archive_name: &str, files: Vec<impl AsRef<Path>>) -> ArchiveResult<()> {
        let meta = metadata(archive_name)?;
        let archive_size = meta.len();
        let mut archive = OpenOptions::new()
            .read(true)
            .write(true)
            .open(archive_name)?;
        archive.seek(SeekFrom::Start(archive_size - 1024))?;
        for file in files {
            let header = self.generate_header(file.as_ref())?;
            archive.write_all(&header)?;
            let mut buffer = [0; 8192];
            let file = File::open(file.as_ref())?;
            let mut reader = BufReader::new(file);
            loop {
                let bytes_read = reader.read(&mut buffer)?;
                if bytes_read == 0 {
                    break;
                }
                archive.write_all(&buffer[..bytes_read])?;
            }
            let pos = archive.stream_position()?;
            let padding = ((512 - (pos % 512)) % 512) as usize;
            archive.write_all(&vec![0; padding])?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_octal() {
        let mut buffer = [0u8; 8];
        let archiver = Archiver::new();
        archiver.write_octal(&mut buffer, 12345, 8).unwrap();
        assert_eq!(&buffer[..], b"0030071\0");
    }
    #[test]
    fn test_read_octal() {
        let archiver = Archiver::new();
        let buffer = b"000000000000\0";
        let result = archiver.read_octal(buffer).unwrap();
        assert_eq!(result, 0);
        let buffer = b"0030071\0";
        let result = archiver.read_octal(buffer).unwrap();
        assert_eq!(result, 12345);
    }
    #[test]
    fn test_generate_header() {
        let archiver = Archiver::new();
        let header = archiver.generate_header("tests/foo.txt").unwrap();
        assert_eq!(header.len(), 512);
        let name = from_utf8(&header[0..100]).unwrap().trim_end_matches('\0');
        assert_eq!(name, "foo.txt");
        let mode = archiver.read_octal(&header[100..108]).unwrap();
        println!("mode: {}", mode);
        assert_eq!(mode, 0o100664);
    }

    #[test]
    fn test_parse_hearder() {
        let archiver = Archiver::new();
        let header = archiver.generate_header("tests/foo.txt").unwrap();
        let (name, _size) = archiver.parse_header(&header).unwrap();
        assert_eq!(name, "foo.txt");
    }

    #[test]
    fn test_extract() {
        let archiver = Archiver::new();
        let archive_name = "test.tar";
        let files = vec!["tests/foo.txt".to_string(), "tests/bar.txt".to_string()];
        archiver.create(archive_name, files).unwrap();
        archiver.extract(archive_name).unwrap();
    }

    #[test]
    fn test_list() {
        let archiver = Archiver::new();
        let archive_name = "test.tar";
        archiver.list(archive_name).unwrap();
    }

    #[test]
    fn test_append() {
        let archiver = Archiver::new();
        let archive_name = "test.tar";
        let files = vec!["tests/foo.txt".to_string(), "tests/bar.txt".to_string()];
        archiver.create(archive_name, files).unwrap();
        let new_files = vec!["tests/foo_eng.txt", "tests/bar_eng.txt"];
        archiver.append(archive_name, new_files).unwrap();
        archiver.list(archive_name).unwrap();
    }
}
