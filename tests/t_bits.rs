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

    #[test]
    fn bits4() {
        let mut v: BitVec = to_bitvec(8);
        pad_to_size(&mut v, 8);
        assert_eq!(v, bitvec![0, 0, 0, 0, 1, 0, 0, 0]);
    }

    #[test]
    fn bits5() {
        let mut v: BitVec = to_bitvec(8);
        pad_to_size(&mut v, 5);
        assert_eq!(v, bitvec![0, 1, 0, 0, 0]);
    }

    #[test]
    fn bits6() {
        let mut v: BitVec = to_bitvec(8);
        pad_to_size(&mut v, 4);
        assert_eq!(v, bitvec![1, 0, 0, 0]);
    }
}