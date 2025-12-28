use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::SystemTime;

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        // Your code here...
        let system_time_nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let file_path = std::env::temp_dir().join(format!("day21-{}.tmp", system_time_nanos));
        let file = File::create(file_path.clone())?;
        Ok(Self { file_path, file })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        self.file.write_all(data)?;
        Ok(())
    }

    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;
        Ok(buffer)
    }

    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn file(&self) -> &File {
        &self.file
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_file_new() {
        let temp_file = TempFile::new().expect("Failed to create temp file");
        assert!(temp_file.path().exists());
    }

    #[test]
    fn test_temp_file_write_and_read() {
        let mut temp_file = TempFile::new().expect("Failed to create temp file");
        let data = b"Hello, world!";
        temp_file.write(data).expect("Failed to write to temp file");
        
        let content = temp_file.read_to_string().expect("Failed to read from temp file");
        assert_eq!(content, "Hello, world!");
    }

    #[test]
    fn test_temp_file_drop() {
        let path = {
            let temp_file = TempFile::new().expect("Failed to create temp file");
            let path = temp_file.path().clone();
            assert!(path.exists());
            path
        };
        // temp_file is dropped here
        assert!(!path.exists());
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
       let _ = std::fs::remove_file(&self.file_path);
    }
}
