//! <https://www.codewars.com/kata/5a057ec846d843c81a0000ad/train/rust>

pub const fn cycle(n: i64) -> i64 {
    if n % 2 == 0 || n % 5 == 0 {
        return -1;
    }
    let mut x = 10 % n;
    let mut res = 1;
    while x != 1 {
        x = x * 10 % n;
        res += 1;
    }
    res
}
