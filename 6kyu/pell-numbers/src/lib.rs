//! <https://www.codewars.com/kata/5818d00a559ff57a2f000082/train/rust>

pub const fn pell(n: u32) -> u128 {
    TABLE[n as usize]
}

const TABLE: [u128; 102] = {
    let mut res = [0; 102];
    res[1] = 1;
    let mut i = 2;
    while i < res.len() {
        res[i] = 2 * res[i - 1] + res[i - 2];
        i += 1;
    }
    res
};
