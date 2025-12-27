use std::fmt;

// 1. Add the `is_wrapped` field to the gift structs
pub struct KidsGift {
    pub name: String,
    pub is_wrapped: bool,
}

impl Gift for KidsGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
}

pub struct ElvesGift {
    pub name: String,
    pub is_wrapped: bool,
}

impl Gift for ElvesGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
}

pub struct ReindeerGift {
    pub name: String,
    pub is_wrapped: bool,
}

impl Gift for ReindeerGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
}

// 2. Finish the trait definition //
pub trait Gift  {
    fn wrap(&mut self);
}

// 3. Update the function signature
pub fn prepare_gift<T: fmt::Display + Gift>(gift: &mut T) {
    println!("Preparing gift for {}", &gift);
    gift.wrap();
    println!("Gift wrapped for {}", &gift);
}

// 4. Implement the Gift trait to the gift structs

impl fmt::Display for KidsGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for ElvesGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for ReindeerGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn main() {
    let mut kids_gift = KidsGift {
        name: "toy car".to_string(),
        is_wrapped: false,
    };
    let mut elves_gift = ElvesGift {
        name: "vertical monitor".to_string(),
        is_wrapped: false,
    };
    let mut reindeer_gift = ReindeerGift {
        name: "carrot".to_string(),
        is_wrapped: false,
    };

    prepare_gift(&mut kids_gift);
    prepare_gift(&mut elves_gift);
    prepare_gift(&mut reindeer_gift);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kids_gift_wrap() {
        let mut gift = KidsGift {
            name: "toy car".to_string(),
            is_wrapped: false,
        };
        gift.wrap();
        assert!(gift.is_wrapped);
    }

    #[test]
    fn test_elves_gift_wrap() {
        let mut gift = ElvesGift {
            name: "vertical monitor".to_string(),
            is_wrapped: false,
        };
        gift.wrap();
        assert!(gift.is_wrapped);
    }

    #[test]
    fn test_reindeer_gift_wrap() {
        let mut gift = ReindeerGift {
            name: "carrot".to_string(),
            is_wrapped: false,
        };
        gift.wrap();
        assert!(gift.is_wrapped);
    }

    #[test]
    fn test_kids_gift_display() {
        let gift = KidsGift {
            name: "toy car".to_string(),
            is_wrapped: false,
        };
        assert_eq!(format!("{}", gift), "toy car");
    }

    #[test]
    fn test_elves_gift_display() {
        let gift = ElvesGift {
            name: "vertical monitor".to_string(),
            is_wrapped: false,
        };
        assert_eq!(format!("{}", gift), "vertical monitor");
    }

    #[test]
    fn test_reindeer_gift_display() {
        let gift = ReindeerGift {
            name: "carrot".to_string(),
            is_wrapped: false,
        };
        assert_eq!(format!("{}", gift), "carrot");
    }

    #[test]
    fn test_prepare_gift() {
        let mut gift = KidsGift {
            name: "toy car".to_string(),
            is_wrapped: false,
        };
        prepare_gift(&mut gift);
        assert!(gift.is_wrapped);
    }
}
