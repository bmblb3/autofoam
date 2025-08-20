use std::{
    fs::File,
    io::{Read, Seek},
};

pub fn is_ascii(file: &mut File) -> bool {
    let mut header = [0u8; 5];
    file.read_exact(&mut header).expect("File too short");
    file.rewind().expect("Seek failed");
    header.starts_with(b"solid")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Seek, SeekFrom, Write};
    use tempfile::tempfile;

    #[test]
    fn test_is_ascii_with_ascii_stl() {
        let mut file = tempfile().unwrap();
        file.write_all(b"solid\nrest of file").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        assert!(is_ascii(&mut file));
    }

    #[test]
    fn test_is_ascii_with_binary_stl() {
        let mut file = tempfile().unwrap();
        file.write_all(b"\x00\x01\x02\x03\x04rest of file").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        assert!(!is_ascii(&mut file));
    }

    #[test]
    #[should_panic(expected = "File too short")]
    fn test_is_ascii_with_short_file() {
        let mut file = tempfile().unwrap();
        file.write_all(b"soli").unwrap();
        file.seek(SeekFrom::Start(0)).unwrap();
        is_ascii(&mut file);
    }
}
