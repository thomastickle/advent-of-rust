pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}

// 1. Finish the implementation of LogQuery
impl<'a> LogQuery<'a> {

    // 2. Create a public associated function named `new()` that will take a reference to a vector of strings
    pub fn new(logs: &'a Vec<String>) -> Self {
        Self { logs }
    }

    // 3. Create a public method named `search` that accepts a string slice and finds it from the logs and
    //    returns a vector of references to those logs.
    pub fn search(&self, keyword: &str ) -> Vec<&'a String> {
        self.logs.iter().filter(|log| log.contains(keyword)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_query_new() {
        let logs = vec!["log1".to_string(), "log2".to_string()];
        let query = LogQuery::new(&logs);
        assert_eq!(query.logs, &logs);
    }

    #[test]
    fn test_log_query_search() {
        let logs = vec![
            "error: disk full".to_string(),
            "warning: low memory".to_string(),
            "error: network down".to_string(),
        ];
        let query = LogQuery::new(&logs);
        let results = query.search("error");
        
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], &logs[0]);
        assert_eq!(results[1], &logs[2]);
    }

    #[test]
    fn test_log_query_search_no_match() {
        let logs = vec!["info: starting".to_string()];
        let query = LogQuery::new(&logs);
        let results = query.search("error");
        
        assert!(results.is_empty());
    }
}
