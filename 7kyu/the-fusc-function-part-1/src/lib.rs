//! <https://www.codewars.com/kata/570409d3d80ec699af001bf9/train/rust>

pub fn fusc(n: u32) -> u32 {
    let mut a = 1;
    let mut b = 0;
    for i in 0..u32::BITS {
        if n >> i & 1 == 0 {
            a += b;
        } else {
            b += a;
        }
    }
    b
}
