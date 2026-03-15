//! <https://www.codewars.com/kata/54fb963d3fe32351f2000102/train/rust>

pub const fn collatz(n: u64) -> u64 {
    if n == 1 {
        1
    } else if n.is_multiple_of(2) {
        1 + collatz(n / 2)
    } else {
        1 + collatz(3 * n + 1)
    }
}
