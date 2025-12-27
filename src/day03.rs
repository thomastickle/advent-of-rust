// We need to find the nice and naughty kids for santa

// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    // Calculate the ratio of good deeds to total deeds
    // Any ratio greater than or equal to 0.75 is considered nice
    // e.g. 10 good deeds and 2 bad deeds =
    // (10 * 1) / ((10 * 1) + (2 * 2)) = 10 / 14 = 0.714... (not nice)
    // If both good and bad deeds are 0, the child is naughty

    if good_deeds == 0 && bad_deeds == 0 {
       return false;
    }

    let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
    let bad_deeds = bad_deeds as f32 * BAD_WEIGHT ;

    let good_to_bad = good_deeds / (good_deeds + bad_deeds);

    good_to_bad >= 0.75f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nice_zero_deeds() {
        assert!(!is_nice(0, 0));
    }

    #[test]
    fn test_is_nice_example_naughty() {
        // (10 * 1) / ((10 * 1) + (2 * 2)) = 10 / 14 = 0.714... (not nice)
        assert!(!is_nice(10, 2));
    }

    #[test]
    fn test_is_nice_all_good_deeds() {
        assert!(is_nice(3, 0));
    }

    #[test]
    fn test_is_nice_boundary_pass() {
        // (12 * 1) / (12 * 1 + 2 * 2) = 12 / 16 = 0.75 (true)
        assert!(is_nice(12, 2));
    }

    #[test]
    fn test_is_nice_boundary_fail() {
        // (5 * 1) / (5 * 1 + 1 * 2) = 5 / 7 ≈ 0.714 (false)
        assert!(!is_nice(5, 1));
    }

    #[test]
    fn test_is_nice_mostly_good() {
        // (9 * 1) / (9 * 1 + 1 * 2) = 9 / 11 ≈ 0.818 (true)
        assert!(is_nice(9, 1));
    }
}
