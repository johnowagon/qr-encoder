use std::convert::TryInto;

use bitvec::vec::BitVec;


pub fn to_bitvec(n: u16) -> BitVec {
    // Converts a given u16 to binary by first converting to string and iterating through chars.
    let s = format!("{n:b}");
    let mut ret_vec = BitVec::new();
    for c in s.chars() {
        if c == '1' {
            ret_vec.push(true);
        } else if c == '0' {
            ret_vec.push(false);
        }
    }

    ret_vec
}

pub fn pad_to_size(v: &mut BitVec, size: u16) {
    // Pads a bitvec with 0s to the specified size in place
    let cur_size: u16 = v.len().try_into().unwrap();
    if size == cur_size {
        return;
    }

    for _ in 0..(size - cur_size) {
        v.insert(0, false);
    }
}