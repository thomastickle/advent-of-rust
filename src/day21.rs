use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;
use std::time::SystemTime;

pub struct TempFile {
    file_path: PathBuf,
    file: File,
    content: Option<String>,
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
        Ok(Self {
            file_path,
            file,
            content: None,
        })
    }

    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        use std::io::Seek;
        self.file.set_len(0)?;
        self.file.rewind()?;
        self.file.write_all(data)?;
        self.content = None;
        Ok(())
    }

    pub fn read_to_string(&mut self) -> Result<&str, std::io::Error> {
        if self.content.is_none() {
            let mut file = OpenOptions::new().read(true).open(&self.file_path)?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            self.content = Some(buffer);
        }

        Ok(self.content.as_ref().unwrap())
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
    fn test_temp_file_caching() {
        let mut temp_file = TempFile::new().expect("Failed to create temp file");
        let data = b"Cached data";
        temp_file.write(data).expect("Failed to write to temp file");

        // First read - populates cache
        {
            let content1 = temp_file.read_to_string().expect("Failed first read");
            assert_eq!(content1, "Cached data");
        } // content1 (borrow) dropped here

        // Second read - uses cache
        {
            let content2 = temp_file.read_to_string().expect("Failed second read");
            assert_eq!(content2, "Cached data");
        }
    }

    #[test]
    fn test_temp_file_cache_invalidation() {
        let mut temp_file = TempFile::new().expect("Failed to create temp file");

        temp_file.write(b"Initial").expect("First write failed");
        assert_eq!(temp_file.read_to_string().unwrap(), "Initial");

        temp_file.write(b"Updated").expect("Second write failed");
        // After write, the cache should be None and new read should get updated content
        assert_eq!(temp_file.read_to_string().unwrap(), "Updated");
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
