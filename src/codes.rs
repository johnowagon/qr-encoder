use std::{error::Error, ops::Index};


// The level of Reed-Soloman error correction used in the code.
// Note: a higher error correction requires more bits, thus 
// resulting in a smaller amount of possible encoded data.
#[derive(Debug)]
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

// Object that contains necessary information about QR codes.
#[derive(Debug)]
pub struct QRCode {
    // TODO: how to find version based on size of input data??
    version: i32, // version of the code, determines how big it is.
    correction_level: ErrorCorrectionLevel,
    data: String, // String slice type right now, as file types are not supported.


}