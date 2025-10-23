// Import the necessary modules
use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum ParseError {
    // 1. Add variants here (read description)
    NoName,
    NoGoodDeeds,
    NoBadDeeds,
    InvalidGoodDeeds,
    InvalidBadDeeds,
}

// When errors displayed, they should be human-readable:

// ParseError::NoName should display as "Name field is missing"
// ParseError::NoGoodDeeds should display as "Good deeds field is missing"
// ParseError::NoBadDeeds should display as "Bad deeds field is missing"
// ParseError::InvalidGoodDeeds should display as "Good deeds value is invalid"
// ParseError::InvalidBadDeeds should display as "Bad deeds value is invalid"
// 2. Implement the Error trait for ParseError

impl Error for ParseError {}


impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::NoName => write!(f, "Name field is missing"),
            ParseError::NoGoodDeeds => write!(f, "Good deeds field is missing"),
            ParseError::NoBadDeeds => write!(f, "Bad deeds field is missing"),
            ParseError::InvalidGoodDeeds => write!(f, "Good deeds value is invalid"),
            ParseError::InvalidBadDeeds => write!(f, "Bad deeds value is invalid"),
        }
    }
}

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
        // we need to check if the csv_row is empty 
        //and if it is like (alice, 0, 0) but if it becomes like (0,0) then it should return an error that is related to name error
        if csv_row.is_empty() {
            return Err(ParseError::NoName);
        }
        // we need to split the csv_row by commas
        let mut fields = csv_row.split(',');
        let name_str = fields.next().ok_or(ParseError::NoName)?;
        let name = name_str.to_string();
        
        // Check if name is empty after trimming whitespace
        if name.trim().is_empty() {
            return Err(ParseError::NoName);
        }
        // Get good_deeds field and check if it's empty
        let good_deeds_str = fields.next().ok_or(ParseError::NoGoodDeeds)?;
        if good_deeds_str.trim().is_empty() {
            return Err(ParseError::NoGoodDeeds);
        }
        let good_deeds = good_deeds_str.parse::<u32>().map_err(|_| ParseError::InvalidGoodDeeds)?;
        
        // Get bad_deeds field and check if it's empty
        let bad_deeds_str = fields.next().ok_or(ParseError::NoBadDeeds)?;
        if bad_deeds_str.trim().is_empty() {
            return Err(ParseError::NoBadDeeds);
        }
        let bad_deeds = bad_deeds_str.parse::<u32>().map_err(|_| ParseError::InvalidBadDeeds)?;

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

pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;

#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}
