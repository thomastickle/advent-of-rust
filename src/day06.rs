// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let s1_len = s1.trim().chars().count();
    let s2_len = s2.trim().chars().count();

    match s1_len.cmp(&s2_len) {
        std::cmp::Ordering::Greater => Some(s1),
        std::cmp::Ordering::Less => Some(s2),
        std::cmp::Ordering::Equal => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longer_wish() {
        assert_eq!(longer_wish("  Santa  ", "  Elf  "), Some("  Santa  "));
        assert_eq!(longer_wish("Elf", "Santa"), Some("Santa"));
        assert_eq!(longer_wish("Same", "Size"), None);
    }

    #[test]
    fn test_longer_wish_unicode() {
        // "❄️" is U+2744 (Snowflake) + U+FE0F (Variation Selector-16), so 2 chars
        // "🎁" is U+1F381 (Wrapped Gift), so 1 char
        assert_eq!(longer_wish("❄️", "🎁"), Some("❄️"));
        assert_eq!(longer_wish("❄️", "❄️"), None);
    }
}
