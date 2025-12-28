use std::collections::HashMap;

pub struct SantaList {
    // 1. Define the records field
    records: HashMap<String, bool>,
}

impl SantaList {
    // 2. Implement the new method
    pub fn new() -> Self {
        Self {
            records: HashMap::new(),
        }
    }

    // 3. Implement the add method
    pub fn add(&mut self, name: &str, is_nice: bool) {
        self.records.insert(name.to_string(), is_nice);
    }

    // 4. Implement the remove method
    pub fn remove(&mut self, name: &str) {
        self.records.remove(name);
    }

    // 5. Implement the get method
    pub fn get(&self, name: &str) -> Option<bool> {
        self.records.get(name).cloned()
    }

    // 6. Implement the count method
    pub fn count(&self) -> (usize, usize) {
        let nice = self.records.values().filter(|&b| *b).count();
        let naughty = self.records.values().filter(|&b| !*b).count();
        (nice, naughty)
    }

    // 7. Implement the list_by_behavior method
    pub fn list_by_behavior(&self, behavior: bool) -> Vec<String> {
        self.records
            .iter()
            .filter_map(|(name, &b)| {
                if b == behavior {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn main() {
    let mut santa_list = SantaList::new();

    santa_list.add("Alice", true);
    santa_list.add("Bob", false);
    santa_list.add("Charlie", true);

    if let Some(behavior) = santa_list.get("Alice") {
        println!(
            "Alice is on the {} list.",
            if behavior { "Nice" } else { "Naughty" }
        );
    }

    let (nice, naughty) = santa_list.count();
    println!(
        "Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );

    let nice_list = santa_list.list_by_behavior(true);
    println!("Nice children: {:?}", nice_list);

    let naughty_list = santa_list.list_by_behavior(false);
    println!("Naughty children: {:?}", naughty_list);

    santa_list.remove("Bob");
    let (nice, naughty) = santa_list.count();
    println!(
        "After removing Bob, Santa's list contains {} nice and {} naughty children.",
        nice, naughty
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_santa_list_new() {
        let list = SantaList::new();
        let (nice, naughty) = list.count();
        assert_eq!(nice, 0);
        assert_eq!(naughty, 0);
    }

    #[test]
    fn test_santa_list_add_get() {
        let mut list = SantaList::new();
        list.add("Alice", true);
        list.add("Bob", false);

        assert_eq!(list.get("Alice"), Some(true));
        assert_eq!(list.get("Bob"), Some(false));
        assert_eq!(list.get("Charlie"), None);
    }

    #[test]
    fn test_santa_list_remove() {
        let mut list = SantaList::new();
        list.add("Alice", true);
        assert_eq!(list.get("Alice"), Some(true));

        list.remove("Alice");
        assert_eq!(list.get("Alice"), None);
    }

    #[test]
    fn test_santa_list_count() {
        let mut list = SantaList::new();
        list.add("Alice", true);
        list.add("Bob", false);
        list.add("Charlie", true);

        let (nice, naughty) = list.count();
        assert_eq!(nice, 2);
        assert_eq!(naughty, 1);
    }

    #[test]
    fn test_santa_list_list_by_behavior() {
        let mut list = SantaList::new();
        list.add("Alice", true);
        list.add("Bob", false);
        list.add("Charlie", true);

        let mut nice_list = list.list_by_behavior(true);
        nice_list.sort();
        assert_eq!(nice_list, vec!["Alice", "Charlie"]);

        let naughty_list = list.list_by_behavior(false);
        assert_eq!(naughty_list, vec!["Bob"]);
    }
}
