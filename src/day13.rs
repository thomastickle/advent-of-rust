// Import the necessary modules

use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum ParseError {
    // 1. Add variants here (read description)
    NoName,
    NoGoodDeeds,
    NoBadDeeds,
    InvalidGoodDeeds,
    InvalidBadDeeds,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ParseError::NoName => write!(f, "Name field is missing"),
            ParseError::NoGoodDeeds => write!(f, "Good deeds field is missing"),
            ParseError::NoBadDeeds => write!(f, "Bad deeds field is missing"),
            ParseError::InvalidGoodDeeds => write!(f, "Good deeds value is invalid"),
            ParseError::InvalidBadDeeds => write!(f, "Bad deeds value is invalid"),
        }
    }
}

// 2. Implement the Error trait for ParseError
impl Error for ParseError {}

pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}

impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };

        Kid { name, niceness }
    }

    pub fn parse_row(csv_row: &str) -> Result<Kid, ParseError> {
        // 3. Update the code to return meaningful errors
        let mut fields = csv_row.split(',');
        let name = fields
            .next()
            .filter(|n| !n.is_empty())
            .ok_or(ParseError::NoName)?
            .to_string();
        let good_deeds = fields
            .next()
            .filter(|deed| !deed.is_empty())
            .ok_or(ParseError::NoGoodDeeds)?
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidGoodDeeds)?;
        let bad_deeds = fields
            .next()
            .filter(|deed| !deed.is_empty())
            .ok_or(ParseError::NoBadDeeds)?
            .parse::<u32>()
            .map_err(|_| ParseError::InvalidBadDeeds)?;

        Ok(Kid::new(name, good_deeds, bad_deeds))
    }

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
    fn test_parse_error_display() {
        assert_eq!(format!("{}", ParseError::NoName), "Name field is missing");
        assert_eq!(format!("{}", ParseError::NoGoodDeeds), "Good deeds field is missing");
        assert_eq!(format!("{}", ParseError::NoBadDeeds), "Bad deeds field is missing");
        assert_eq!(format!("{}", ParseError::InvalidGoodDeeds), "Good deeds value is invalid");
        assert_eq!(format!("{}", ParseError::InvalidBadDeeds), "Bad deeds value is invalid");
    }

    #[test]
    fn test_parse_row_success() {
        let row = "Alice,12,2";
        let kid = Kid::parse_row(row).unwrap();
        assert_eq!(kid.name, "Alice");
        match kid.niceness {
            Niceness::Nice(good) => assert_eq!(good, 12),
            _ => panic!("Expected Nice"),
        }
    }

    #[test]
    fn test_parse_row_no_name() {
        let row = ",10,2";
        let result = Kid::parse_row(row);
        assert!(matches!(result, Err(ParseError::NoName)));
    }

    #[test]
    fn test_parse_row_no_good_deeds() {
        let row = "Alice,,2";
        let result = Kid::parse_row(row);
        assert!(matches!(result, Err(ParseError::NoGoodDeeds)));
    }

    #[test]
    fn test_parse_row_no_bad_deeds() {
        let row = "Alice,10,";
        let result = Kid::parse_row(row);
        assert!(matches!(result, Err(ParseError::NoBadDeeds)));
    }

    #[test]
    fn test_parse_row_invalid_good_deeds() {
        let row = "Alice,ten,2";
        let result = Kid::parse_row(row);
        assert!(matches!(result, Err(ParseError::InvalidGoodDeeds)));
    }

    #[test]
    fn test_parse_row_invalid_bad_deeds() {
        let row = "Alice,10,two";
        let result = Kid::parse_row(row);
        assert!(matches!(result, Err(ParseError::InvalidBadDeeds)));
    }
}

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}
