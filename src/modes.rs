use std::ops::Index;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Mode {
    Numeric = 0,
    AlphaNumeric = 1,
    Byte = 2,
    Kanji = 3 // For completeness, unnecessary?
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

