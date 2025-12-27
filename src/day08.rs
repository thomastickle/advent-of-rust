use std::fs::File;
use std::io::{BufWriter, Write};

pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }

    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }

    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        // 🎁 Your code here! 🎁

        let log_file = File::create(path)?;
        let mut writer = BufWriter::new(log_file);
        let log_lines = self.search(keyword);
        for log_line in log_lines {
            writeln!(writer, "{}", log_line)?;
        }
        writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Read;

    #[test]
    fn test_export_to_file() {
        let logs = vec![
            "error: disk full".to_string(),
            "warning: low memory".to_string(),
            "error: network down".to_string(),
        ];
        let query = LogQuery::new(&logs);
        
        let mut temp_path = std::env::temp_dir();
        temp_path.push("test_logs.txt");
        let path_str = temp_path.to_str().unwrap();

        let result = query.export_to_file("error", path_str);
        assert!(result.is_ok());

        let mut file_content = String::new();
        let mut file = File::open(path_str).unwrap();
        file.read_to_string(&mut file_content).unwrap();

        let expected_content = "error: disk full\nerror: network down\n";
        assert_eq!(file_content, expected_content);

        // Clean up
        fs::remove_file(path_str).unwrap();
    }

    #[test]
    fn test_export_to_file_no_match() {
        let logs = vec!["info: starting".to_string()];
        let query = LogQuery::new(&logs);
        
        let mut temp_path = std::env::temp_dir();
        temp_path.push("test_logs_empty.txt");
        let path_str = temp_path.to_str().unwrap();

        let result = query.export_to_file("error", path_str);
        assert!(result.is_ok());

        let mut file_content = String::new();
        let mut file = File::open(path_str).unwrap();
        file.read_to_string(&mut file_content).unwrap();

        assert!(file_content.is_empty());

        // Clean up
        fs::remove_file(path_str).unwrap();
    }
}
