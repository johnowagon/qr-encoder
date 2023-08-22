#[derive(Debug, Eq, PartialEq)]
pub enum Mode {
    Numeric,
    AlphaNumeric,
    Byte,
    Kanji // For completeness, unnecessary?
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

