extern crate qr_encoder;
extern crate bitvec;

#[cfg(test)]
mod tests {
    use qr_encoder::bit_helpers::*;
    use bitvec::prelude::*;

    #[test]
    fn bits1() {
        assert_eq!(to_bitvec(25), bitvec![1, 1, 0, 0, 1]);
    }

    #[test]
    fn bits2() {
        assert_eq!(to_bitvec(1), bitvec![1]);
    }

    #[test]
    fn bits3() {
        assert_eq!(to_bitvec(128), bitvec![1, 0, 0, 0, 0, 0, 0, 0]);
    }
}