use std::error::Error;
use std::fmt;
// For a better understanding of the problem, have a look at the end of the file and see the `main`
// function to see how the structs are being used.

pub struct Kid {
    pub name: String,
    pub gifted: bool,
}

pub struct Reindeer {
    pub name: String,
    pub gifted: bool,
}

pub struct Elf {
    pub name: String,
    pub gifted: bool,
}

pub trait Giftable {
    // 1. Define the trait definition
    // Add a function named `receive_gift`
    fn receive_gift(&mut self);
}

impl Giftable for Kid {
    fn receive_gift(&mut self) {
        self.gifted = true;
    }
}

impl Giftable for Reindeer {
    fn receive_gift(&mut self) {
        self.gifted = true;
    }
}

impl Giftable for Elf {
    fn receive_gift(&mut self) {
        self.gifted = true;
    }
}

// 2. Implement `Giftable` for `Kid`, `Reindeer`, and `Elf`

pub trait Gift {
    fn wrap(&mut self);
    // 3. Define a function named `is_wrapped` that returns a boolean

    fn is_wrapped(&self) -> bool;
}

// 4. Update the `Gift` trait implementation for `KidsGift`, `ElvesGift`, and `ReindeerGift` to
//    include the `is_wrapped` function

impl Gift for KidsGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }

    // Update implementation
    fn is_wrapped(&self) -> bool {
        self.is_wrapped
    }
}

impl Gift for ElvesGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }

    // Update implementation
    fn is_wrapped(&self) -> bool {
        self.is_wrapped
    }
}

impl Gift for ReindeerGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }

    // Update implementation
    fn is_wrapped(&self) -> bool {
        self.is_wrapped
    }
}

pub struct Santa;

impl Santa {
    pub fn give_gift<T, V>(&self, recipient: &mut T, gift: &V) -> Result<(), Box<dyn Error>>
    where
        T: Giftable,
        V: Gift,
    {
        // 5. Update the function signature to accept any type of recipient and gift
        if !gift.is_wrapped() {
            return Err("Gift is not wrapped".into());
        }

        recipient.receive_gift();

        Ok(())
    }
}

pub struct KidsGift {
    pub name: String,
    pub is_wrapped: bool,
}

impl fmt::Display for KidsGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct ElvesGift {
    pub name: String,
    pub is_wrapped: bool,
}

impl fmt::Display for ElvesGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct ReindeerGift {
    pub name: String,
    pub is_wrapped: bool,
}

impl fmt::Display for ReindeerGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub fn prepare_gift<T: Gift + fmt::Display>(gift: &mut T) {
    println!("Preparing gift for {}", &gift);
    gift.wrap();
    println!("Gift wrapped for {}", &gift);
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

    let mut alice = Kid {
        name: "Alice".to_string(),
        gifted: false,
    };

    let mut prancer = Reindeer {
        name: "Prancer".to_string(),
        gifted: false,
    };

    let mut bernard = Elf {
        name: "Buddy".to_string(),
        gifted: false,
    };

    let santa = Santa;

    prepare_gift(&mut kids_gift);
    prepare_gift(&mut elves_gift);
    prepare_gift(&mut reindeer_gift);

    if let Ok(_) = santa.give_gift(&mut alice, &kids_gift) {
        println!("{} received {}", alice.name, kids_gift);
        assert_eq!(alice.gifted, true);
    } else {
        panic!("{} should have received {}", alice.name, kids_gift);
    }

    if let Ok(_) = santa.give_gift(&mut prancer, &reindeer_gift) {
        println!("{} received {}", prancer.name, reindeer_gift);
        assert_eq!(prancer.gifted, true);
    } else {
        panic!("{} should have received {}", prancer.name, reindeer_gift);
    }

    if let Ok(_) = santa.give_gift(&mut bernard, &elves_gift) {
        println!("{} received {}", bernard.name, elves_gift);
        assert_eq!(bernard.gifted, true);
    } else {
        panic!("{} should have received {}", bernard.name, elves_gift);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_receive_gift() {
        let mut kid = Kid {
            name: "Alice".to_string(),
            gifted: false,
        };
        kid.receive_gift();
        assert!(kid.gifted);

        let mut reindeer = Reindeer {
            name: "Prancer".to_string(),
            gifted: false,
        };
        reindeer.receive_gift();
        assert!(reindeer.gifted);

        let mut elf = Elf {
            name: "Buddy".to_string(),
            gifted: false,
        };
        elf.receive_gift();
        assert!(elf.gifted);
    }

    #[test]
    fn test_is_wrapped() {
        let mut kids_gift = KidsGift {
            name: "toy car".to_string(),
            is_wrapped: false,
        };
        assert!(!kids_gift.is_wrapped());
        kids_gift.wrap();
        assert!(kids_gift.is_wrapped());

        let mut elves_gift = ElvesGift {
            name: "monitor".to_string(),
            is_wrapped: false,
        };
        assert!(!elves_gift.is_wrapped());
        elves_gift.wrap();
        assert!(elves_gift.is_wrapped());

        let mut reindeer_gift = ReindeerGift {
            name: "carrot".to_string(),
            is_wrapped: false,
        };
        assert!(!reindeer_gift.is_wrapped());
        reindeer_gift.wrap();
        assert!(reindeer_gift.is_wrapped());
    }

    #[test]
    fn test_santa_give_gift_success() {
        let mut kid = Kid {
            name: "Alice".to_string(),
            gifted: false,
        };
        let gift = KidsGift {
            name: "toy car".to_string(),
            is_wrapped: true,
        };
        let santa = Santa;
        let result = santa.give_gift(&mut kid, &gift);
        assert!(result.is_ok());
        assert!(kid.gifted);
    }

    #[test]
    fn test_santa_give_gift_not_wrapped() {
        let mut kid = Kid {
            name: "Alice".to_string(),
            gifted: false,
        };
        let gift = KidsGift {
            name: "toy car".to_string(),
            is_wrapped: false,
        };
        let santa = Santa;
        let result = santa.give_gift(&mut kid, &gift);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Gift is not wrapped");
        assert!(!kid.gifted);
    }
}
