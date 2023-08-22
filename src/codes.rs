use std::error::Error;


// The level of Reed-Soloman error correction used in the code.
// Note: a higher error correction requires more bits, thus 
// resulting in a smaller amount of possible encoded data.
#[derive(Debug)]
enum ErrorCorrectionLevel {
    L, // 7%
    M, // 15%
    Q, // 25%
    H // 30%
}


// Object that contains necessary information about QR codes.
#[derive(Debug)]
pub struct QRCode {
    // TODO: how to find version based on size of input data??
    version: i32, // version of the code, determines how big it is.
    correction_level: ErrorCorrectionLevel,
    data: String, // String slice type right now, as file types are not supported.


}