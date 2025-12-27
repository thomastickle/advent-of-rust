pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
    // Create the enum variants `Nice` and `Naughty`
    // Variant `Nice` is a tuple struct that holds the number of good deeds
    Nice(u32),
    Naughty
}

pub struct Kid {
    // Add a field `name` of type `String`
    // and `niceness` field of type `Niceness`
    // Make all fields public
    pub name: String,
    pub niceness: Niceness
}




impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let niceness = if Kid::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Kid {name, niceness }
    }

    // Move yesterday's function to an associated function in the struct
    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }

        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;

        let ratio = good_deeds / (good_deeds + bad_deeds);

        ratio >= 0.75
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_niceness_variants() {
        let nice = Niceness::Nice(10);
        let naughty = Niceness::Naughty;

        assert_eq!(nice, Niceness::Nice(10));
        assert_eq!(naughty, Niceness::Naughty);
        assert_ne!(nice, naughty);
    }

    #[test]
    fn test_kid_new_nice() {
        // (10 * 1) / (10 * 1 + 1 * 2) = 10 / 12 = 0.833... >= 0.75
        let kid = Kid::new("Alice".to_string(), 10, 1);
        assert_eq!(kid.name, "Alice");
        assert_eq!(kid.niceness, Niceness::Nice(10));
    }

    #[test]
    fn test_kid_new_naughty() {
        // (10 * 1) / (10 * 1 + 2 * 2) = 10 / 14 = 0.714... < 0.75
        let kid = Kid::new("Bob".to_string(), 10, 2);
        assert_eq!(kid.name, "Bob");
        assert_eq!(kid.niceness, Niceness::Naughty);
    }

    #[test]
    fn test_kid_new_zero_deeds() {
        let kid = Kid::new("Charlie".to_string(), 0, 0);
        assert_eq!(kid.niceness, Niceness::Naughty);
    }

    #[test]
    fn test_kid_new_boundary_pass() {
        // (12 * 1) / (12 * 1 + 2 * 2) = 12 / 16 = 0.75
        let kid = Kid::new("David".to_string(), 12, 2);
        assert_eq!(kid.niceness, Niceness::Nice(12));
    }
}
