extern crate qr_encoder;
extern crate bitvec;
#[cfg(test)]
mod tests {
    use qr_encoder::codes::QRCode;
    use qr_encoder::codes::ErrorCorrectionLevel;
    use bitvec::prelude::*;

    #[test]
    fn qrbits1() {
        let code = QRCode::new(ErrorCorrectionLevel::L, String::from("HELLO WORLD"));
        println!("{0:?}", code);
        assert_eq!(code.bitfield, bitvec![0,0,1,0,0,0,0,0,0,1,0,1,1]);
    }

    #[test]
    fn qrbits3() {
        let code = QRCode::new(ErrorCorrectionLevel::L, String::from("1234567890"));
        assert_eq!(code.bitfield, bitvec![0,0,0,1,0,0,0,0,0,0,1,0,1,0]);
    }

    #[test]
    fn qrbits4() {
        let code = QRCode::new(ErrorCorrectionLevel::L, String::from("this is a test! ÿ"));//17
        assert_eq!(code.bitfield, bitvec![0,1,0,0,0,0,0,1,0,0,0,1]);
    }
}