//! <https://www.codewars.com/kata/56968ce7753513604b000055/train/rust>

pub const fn pyramid_height(n: u32) -> u32 {
    let mut i = 1;
    let mut s = 1;
    while s <= n {
        i += 1;
        s += i * i;
    }
    i - 1
}
