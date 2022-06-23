//! <https://www.codewars.com/kata/5583090cbe83f4fd8c000051/train/rust>

pub fn digitize(mut n: u64) -> Vec<u8> {
    if n == 0 {
        return vec![0];
    }

    let mut digits = Vec::with_capacity(20);

    while n != 0 {
        digits.push((n % 10) as _);
        n /= 10;
    }

    digits
}
