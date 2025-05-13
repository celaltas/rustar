use std::{
    fs::{self, File, metadata},
    io::{BufReader, Read, Write},
    os::unix::fs::MetadataExt,
    path::Path,
    str::from_utf8,
};

pub struct Archiver {}

impl Archiver {
    pub fn new() -> Self {
        Archiver {}
    }

    pub fn create(&self, archive_name: &str, files: Vec<String>) -> std::io::Result<()> {
        let mut all_bytes = Vec::new();
        for file in files {
            let mut file_bytes = Vec::new();
            let header = self.generate_header(&file);
            file_bytes.extend_from_slice(&header);
            let mut local_buf = [0; 512];
            let file = File::open(file)?;
            let mut reader = BufReader::new(file);
            loop {
                let bytes_read = reader.read(&mut local_buf)?;
                if bytes_read == 0 {
                    break;
                }
                file_bytes.extend_from_slice(&local_buf[..bytes_read]);
            }
            all_bytes.extend(file_bytes);
        }
        let mut archive = std::fs::File::create(archive_name)?;
        archive.write_all(&all_bytes)?;
        let padding = 512 - (all_bytes.len() % 512);
        archive.write_all(&vec![0; padding])?;
        let end_of_archive = [0; 512];
        archive.write_all(&end_of_archive)?;
        archive.write_all(&end_of_archive)?;
        Ok(())
    }

    fn generate_header(&self, file_name: &str) -> [u8; 512] {
        let mut header = [0u8; 512];
        let metadata = fs::metadata(file_name).unwrap();
        // [0..100]
        let name = Path::new(file_name).file_name().unwrap().to_str().unwrap();
        let name_bytes = name.as_bytes();
        header[0..name_bytes.len()].copy_from_slice(name_bytes);
        // mode: [100..108]
        self.write_octal(&mut header[100..108], metadata.mode() as u64, 8);
        // uid: [108..116]
        self.write_octal(&mut header[108..116], metadata.uid() as u64, 8);
        // gid: [116..124]
        self.write_octal(&mut header[116..124], metadata.gid() as u64, 8);
        //size [124..136]
        self.write_octal(&mut header[124..136], metadata.size(), 12);
        // mtime [136..148]
        self.write_octal(&mut header[136..148], metadata.mtime() as u64, 12);

        for i in 148..156 {
            header[i] = b' ';
        }

        // typeflag [156]
        header[156] = b'0';
        header[257..263].copy_from_slice(b"ustar\0");
        header[263..265].copy_from_slice(b"00");

        let checksum = header.iter().map(|&b| b as u32).sum::<u32>();
        self.write_octal(&mut header[148..156], checksum as u64, 8);
        header
    }

    fn write_octal(&self, dst: &mut [u8], value: u64, len: usize) {
        let s = format!("{:0len$o}", value, len = len - 1);
        println!("s: {}",s);
        dst[..s.len()].copy_from_slice(s.as_bytes());
        dst[s.len()] = b'\0';
    }

    fn read_octal(&self, src: &[u8], len: u32) -> u64 {
        let s = from_utf8(src).unwrap().trim_end_matches('\0').trim();
        u64::from_str_radix(&s, len).unwrap_or(0)
    }

    fn parse_header(&self, header: &[u8]) {
        let name = from_utf8(&header[0..100]).unwrap().trim_end_matches('\0');
        let mode = self.read_octal(&header[100..108], 8);
        let uid = self.read_octal(&header[108..116], 8);
        let gid = self.read_octal(&header[116..124], 8);
        let size = self.read_octal(&header[124..136], 12);
        let mtime = self.read_octal(&header[136..148], 12);
        let checksum = self.read_octal(&header[148..156], 8);
        println!(
            "Name: {}, Mode: {}, UID: {}, GID: {}, Size: {}, MTime: {}, Checksum: {}",
            name, mode, uid, gid, size, mtime, checksum
        );
    }

    pub fn extract(&self, archive_name: &str) -> std::io::Result<()> {
        Ok(())
    }

    pub fn list(&self, archive_name: &str) -> std::io::Result<()> {
        Ok(())
    }

    pub fn append(&self, archive_name: &str, files: Vec<&str>) -> std::io::Result<()> {
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
        archiver.write_octal(&mut buffer, 12345, 8);
        assert_eq!(&buffer[..], b"0030071\0");
    }
    #[test]
    fn test_read_octal() {
        let archiver = Archiver::new();
        let buffer = b"000000000000\0";
        let result = archiver.read_octal(buffer, 8);
        assert_eq!(result, 0);
        let buffer = b"0030071\0";
        let result = archiver.read_octal(buffer, 8);
        assert_eq!(result, 12345);
    }
}