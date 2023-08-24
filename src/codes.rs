use std::{ops::Index};
use crate::{modes::{Mode, analyze_mode, get_mode_indicator, get_mode_charcountlen}, capacities, bit_helpers::to_bitvec};
use bitvec::prelude::*;

// The level of Reed-Soloman error correction used in the code.
// Note: a higher error correction requires more bits, thus 
// resulting in a smaller amount of possible encoded data.
#[derive(Debug, Clone, Copy)]
pub enum ErrorCorrectionLevel {
    L = 0, // 7%
    M = 1, // 15%
    Q = 2, // 25%
    H = 3 // 30%
}

impl Index<ErrorCorrectionLevel> for [[[u16; 4]; 40]; 4] {
    type Output = [[u16; 4]; 40];

    fn index(&self, ec: ErrorCorrectionLevel) -> &Self::Output {
        &self[ec as usize]
    }
}

// Object that contains necessary information to construct QR codes.
#[derive(Debug)]
pub struct QRCode {
    // TODO: how to find version based on size of input data??
    version: u16, // version of the code, determines how big it is.
    correction_level: ErrorCorrectionLevel,
    char_count: u16,
    max_len: u16,
    mode: Mode, // AlphaNumeric, Numeric, Byte, Kanji, ECI
    data: String, // String type right now, as file types are not supported.
    pub bitfield: BitVec
}

// TODO: improve this by using custom iterators?
pub fn smallest_cap(ec: ErrorCorrectionLevel, s: String) -> (u16, u16) {
    // Gets the version and smallest capacity for a given string.
    let target: u16 = s.chars().count().try_into().unwrap();
    let mode: Mode = analyze_mode(s);
    let mut found: u16 = 0;
    let mut v: u16 = 1; // version should start at 1
    for list in capacities::CHAR_CAPS[ec] {
        if list[mode] >= target {
            found = list[mode];
            break;
        }
        v += 1;
    }

    (v, found)
}

impl QRCode {
    pub fn new(ec: ErrorCorrectionLevel, s: String) -> Self {
        // Construct QRCode with necessary data
        let new_mode: Mode = analyze_mode(s.clone());
        let (version, charc) = smallest_cap(ec.clone(), s.clone());
        let mut code = QRCode { 
            version: version, 
            correction_level: ec, 
            char_count: s.chars().count() as u16,
            max_len: charc,
            mode: new_mode, 
            data: s,
            bitfield: BitVec::new(),
        };

        code.append_modebits();
        code.append_charcount();

        code
    }

    pub fn append_modebits(&mut self) {
        self.bitfield.append(&mut get_mode_indicator(self.mode).unwrap());
    }

    pub fn append_charcount(&mut self) {
        let charc_len: u16 = get_mode_charcountlen(self.version, self.mode).unwrap();
        let mut count: BitVec = to_bitvec(self.char_count);
        let mut cc_bv: BitVec = BitVec::new();
        for _ in 0..(charc_len - count.len() as u16) {
            cc_bv.push(false); // Pad bitvec with necessary zeroes
        }
        // kind of weird
        cc_bv.append(&mut count);
        self.bitfield.append(&mut cc_bv);
    }
}