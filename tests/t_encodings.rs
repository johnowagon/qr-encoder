extern crate qr_encoder;
extern crate bitvec;

#[cfg(test)]
mod tests {
    use qr_encoder::encoding::{self, alphanumeric_encoding};
    use bitvec::prelude::*;

    #[test]
    fn alphanum_encode1() {
        let res: String = String::from("HELLO WORLD");
        assert_eq!(alphanumeric_encoding(res), 
            bitvec![0,1,1,0,0,0,0,1,0,1,1,0,1,1,1,1,0,0,0,1,1,0,1,0,
                    0,0,1,0,1,1,1,0,0,1,0,1,1,0,1,1,1,0,0,0,1,0,0,1,
                    1,0,1,0,1,0,0,0,0,1,1,0,1]);
    }
}