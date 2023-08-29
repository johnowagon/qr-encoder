extern crate qr_encoder;
extern crate bitvec;

#[cfg(test)]
mod tests {
    use qr_encoder::encoding::{self, alphanumeric_encoding, numeric_encoding};
    use bitvec::prelude::*;

    #[test]
    fn alphanum_encode1() {
        let res: String = String::from("HELLO WORLD");
        assert_eq!(alphanumeric_encoding(res), 
            bitvec![0,1,1,0,0,0,0,1,0,1,1,0,1,1,1,1,0,0,0,1,1,0,1,0,
                    0,0,1,0,1,1,1,0,0,1,0,1,1,0,1,1,1,0,0,0,1,0,0,1,
                    1,0,1,0,1,0,0,0,0,1,1,0,1]);
    }

    #[test]
    fn alphanum_encode2() {
        let res: String = String::from("DEADBEEF LIVES");
        assert_eq!(alphanumeric_encoding(res), 
            bitvec![0,1,0,0,1,0,1,0,1,1,1,0,0,1,1,1,0,0,1,1,1,1,0,0,
                    1,1,1,1,1,1,1,0,1,0,1,0,1,0,0,0,0,1,0,1,1,1,0,0,
                    1,1,0,1,0,0,1,0,1,1,0,1,0,0,1,0,0,1,0,1,0,1,0,0,
                    1,0,0,1,0]);
    }

    #[test]
    fn num_encode1() {
        let res: String = String::from("123456789000");
        assert_eq!(numeric_encoding(res), 
            bitvec![0,0,0,1,1,1,1,0,1,1,1,0,1,1,1,0,0,1,0,0,0,1,1,0,
                    0,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0]);
    }

    #[test]
    fn num_encode2() {
        let res: String = String::from("1234567890");
        assert_eq!(numeric_encoding(res), 
            bitvec![0,0,0,1,1,1,1,0,1,1,1,0,1,1,1,0,0,1,0,0,0,1,1,0,
                    0,0,1,0,1,0,1,0,0,0,0]);
    }

    #[test]
    fn num_encode3() {
        let res: String = String::from("12345678900");
        assert_eq!(numeric_encoding(res), 
            bitvec![0,0,0,1,1,1,1,0,1,1,1,0,1,1,1,0,0,1,0,0,0,1,1,0,
                    0,0,1,0,1,0,1,0,0,0,0,0,0,0]);
    }

    #[test]
    fn num_encode4() {
        let res: String = String::from("1234567890000");
        assert_eq!(numeric_encoding(res), 
            bitvec![0,0,0,1,1,1,1,0,1,1,1,0,1,1,1,0,0,1,0,0,0,1,1,0,
                    0,0,1,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0]);
    }
}