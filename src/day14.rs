use std::fmt;
use std::fmt::Display;

pub struct KidsGift {
    pub name: String,
}

impl Display for KidsGift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Kids gift: {}", self.name)
    }
}

pub struct ElvesGift {
    pub name: String,
}

impl Display for ElvesGift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Elves gift: {}", self.name)
    }
}

pub struct ReindeerGift {
    pub name: String,
}

impl Display for ReindeerGift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Reindeer gift: {}", self.name)
    }
}

pub fn display_gift<T: fmt::Display>(gift: T)
where T: {
    println!("{}", gift);
}

pub fn main() {
    let kids_gift = KidsGift {
        name: "toy car".to_string(),
    };
    let elves_gift = ElvesGift {
        name: "vertical monitor".to_string(),
    };
    let reindeer_gift = ReindeerGift {
        name: "carrot".to_string(),
    };

    display_gift(&kids_gift);
    display_gift(&elves_gift);
    display_gift(&reindeer_gift);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kids_gift_display() {
        let gift = KidsGift {
            name: "toy car".to_string(),
        };
        assert_eq!(format!("{}", gift), "Kids gift: toy car");
    }

    #[test]
    fn test_elves_gift_display() {
        let gift = ElvesGift {
            name: "vertical monitor".to_string(),
        };
        assert_eq!(format!("{}", gift), "Elves gift: vertical monitor");
    }

    #[test]
    fn test_reindeer_gift_display() {
        let gift = ReindeerGift {
            name: "carrot".to_string(),
        };
        assert_eq!(format!("{}", gift), "Reindeer gift: carrot");
    }
}
