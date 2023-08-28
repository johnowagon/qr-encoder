use std::{ops::Index, fmt};
use bitvec::prelude::*;

use crate::{
    results::{
        ModeResult, ModeError
    }, 
    encoding::{
        alphanumeric_encoding, numeric_encoding, byte_encoding, kanji_encoding
    }
};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Mode {
    Numeric = 0,
    AlphaNumeric = 1,
    Byte = 2,
    Kanji = 3, // For completeness, unnecessary?
    ECI = 4, // For multiple QR codes chained together, not used rn
}

// Different encoding algorithms for specific mode types.

impl Mode {
    pub fn encode(&self, s: String) -> BitVec {
        match self {
            Mode::AlphaNumeric => alphanumeric_encoding(s),
            Mode::Numeric => numeric_encoding(s),
            Mode::Byte => byte_encoding(s),
            Mode::Kanji => kanji_encoding(s),
            Mode::ECI => BitVec::new() // not supported yet
        }
    }
}

impl Index<Mode> for [u16; 4] {
    type Output = u16;

    fn index(&self, ec: Mode) -> &Self::Output {
        &self[ec as usize]
    }
}

pub fn analyze_mode(s: String) -> Mode {
    // Analyze a string (for now) and return the proper encoding style for this string.
    if s.chars().all(|c| is_numeric(c)) {
        Mode::Numeric
    } else if s.chars().all(|c| is_alphanumeric(c)) {
        Mode::AlphaNumeric
    } else {
        Mode::Byte
    }
    // Kanji mode can be implemented at a later date
}

// TODO: Change return type to Result<BitVec, Error>
pub fn get_mode_indicator(m: Mode) -> ModeResult<BitVec> {
    match m {
        Mode::Numeric => Ok(bitvec![0, 0, 0, 1]),
        Mode::AlphaNumeric => Ok(bitvec![0, 0, 1, 0]),
        Mode::Byte => Ok(bitvec![0, 1, 0, 0]), 
        Mode::Kanji => Ok(bitvec![1, 0, 0, 0]),
        Mode::ECI => Ok(bitvec![0, 1, 1, 1]),
        _ => Err(ModeError)
    }
}

pub fn get_mode_charcountlen(version: u16, m: Mode) -> ModeResult<u16> {
    // Returns number of bits the charcount bitstring must be based on the version
    // and the mode.
    // TODO: Make this better somehow.
    match m {
        Mode::AlphaNumeric => {
            if 1 <= version && version <= 9 {
                Ok(9)
            } else if 10 <= version && version <= 26 {
                Ok(11)
            } else if 27 <= version && version <= 40 {
                Ok(13)
            } else {
                Err(ModeError)
            }
        },
        Mode::Numeric => {
            if 1 <= version && version <= 9 {
                Ok(10)
            } else if 10 <= version && version <= 26 {
                Ok(12)
            } else if 27 <= version && version <= 40 {
                Ok(14)
            } else {
                Err(ModeError)
            }
        },
        Mode::Byte => {
            if 1 <= version && version <= 9 {
                Ok(8)
            } else if 10 <= version && version <= 26 {
                Ok(16)
            } else if 27 <= version && version <= 40 {
                Ok(16)
            } else {
                Err(ModeError)
            }
        },
        Mode::Kanji => {
            if 1 <= version && version <= 9 {
                Ok(8)
            } else if 10 <= version && version <= 26 {
                Ok(10)
            } else if 27 <= version && version <= 40 {
                Ok(12)
            } else {
                Err(ModeError)
            }
        },
        Mode::ECI => {
            // not implemented yet
            Err(ModeError)
        }
        _ => Err(ModeError)
    }
}
// Not using built in char functions because they are too wide ranging
// for encoding purposes.

fn is_alphanumeric(c: char) -> bool {
    let alpha_digits: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789$%*+-./: ";
    return alpha_digits.contains(c);
}

fn is_numeric(c: char) -> bool {
    let digits: &str = "0123456789";
    return digits.contains(c);
}

