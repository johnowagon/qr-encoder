use std::convert::TryInto;

use bitvec::vec::BitVec;

use crate::{consts::from_alphanum_table, bit_helpers::{pad_to_size, to_bitvec}};

// The major concession amde with these encoding algorithms is that they currently only support ASCII text.
// TODO: make this more general to accept different types of text?
// TODO cont. : Make the input data more generalized by creating a custom struct
// This may be beneficial for encoding to allow for custom iterators?
// may be beneficial for easier polymorphism as well. 

// Enum to make numeric encoding easier
enum NumDigits {
    ThreeDigit,
    TwoDigit,
    OneDigit
}

pub fn numeric_encoding(s: String) -> BitVec {
    // Info about numeric encoding can be found here: https://www.thonky.com/qr-code-tutorial/numeric-mode-encoding
    let mut result: BitVec = BitVec::new();
    let data_len = s.len();
    
    fn interpret_triple(s: String) -> NumDigits {
        println!("{s}");
        let mut len = 0; 
        let mut leading = 0;
        for (i, c) in s.chars().enumerate() {
            if c != '0' && i == 0 {
                // If there are no leading zeroes
                return NumDigits::ThreeDigit;
            }
            if c == '0' {
                leading += 1;
            }
            len += 1;
        }
        if len == leading {
            match len {
                1 => NumDigits::OneDigit,
                2 => NumDigits::TwoDigit,
                3 => NumDigits::ThreeDigit,
                _ => todo!() // how to handle default cases?
            }
        } else {
            match leading {
                1 => NumDigits::TwoDigit,
                2 => NumDigits::OneDigit,
                3 => NumDigits::ThreeDigit,
                _ => todo!()
            }
        }
    }

    for i in (0..data_len).step_by(3) {
        let numlen: NumDigits;
        if i+3 > data_len {
            numlen = interpret_triple(String::from(&s[i..]));
        } else {
            numlen = interpret_triple(String::from(&s[i..i+3]));
        }
        match numlen {
            // convert string into int
            NumDigits::OneDigit => {
                println!("ONE DIGIT");
            },
            NumDigits::TwoDigit => { 
                println!("TWO");
            },
            NumDigits::ThreeDigit => {
                println!("THREE");
            }
        }
    }

    result
}

pub fn alphanumeric_encoding(s: String) -> BitVec {
    // AlphaNumeric encoding is described here: https://www.thonky.com/qr-code-tutorial/alphanumeric-mode-encoding
    let mut result: BitVec = BitVec::new();
    let data_len = s.len();

    fn encode_equation(c1: char, c2: char) -> u16 {
        // helper for alphanumeric encoding, calculates encoded number from pair of chars
        let num1 = from_alphanum_table(c1);
        let num2 = from_alphanum_table(c2);
        (45 * num1) + num2
    }
    
    for i in (0..data_len).step_by(2) {
        println!("{i}");
        if i == data_len-1 {
            // handle odd number case
            let mut tmp: BitVec = BitVec::new();
            tmp = to_bitvec(
                from_alphanum_table(s.as_bytes()[i] as char)
            );
            pad_to_size(&mut tmp, 6);
            result.append(&mut tmp);
        } else {
            let mut tmp: BitVec = BitVec::new();
            tmp = to_bitvec(
                encode_equation(s.as_bytes()[i] as char, s.as_bytes()[i+1] as char)
            );
            pad_to_size(&mut tmp, 11); //encoded size should be 11
            result.append(&mut tmp);
        }
    }
    result
}

pub fn byte_encoding(s: String) -> BitVec {
    todo!();
}

pub fn kanji_encoding(s: String) -> BitVec {
    todo!();
}