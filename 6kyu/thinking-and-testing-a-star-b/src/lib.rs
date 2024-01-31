//! <https://www.codewars.com/kata/5a90c9ecb171012b47000077/train/rust>

pub const fn test_it(a: u64, b: u64) -> u64 {
    sum_digits(a) * sum_digits(b)
}

const fn sum_digits(mut n: u64) -> u64 {
    let mut res = 0;
    while n != 0 {
        res += n % 10;
        n /= 10;
    }
    res
}
