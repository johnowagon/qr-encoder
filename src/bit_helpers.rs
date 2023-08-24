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