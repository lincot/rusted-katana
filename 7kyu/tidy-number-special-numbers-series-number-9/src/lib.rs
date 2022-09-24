//! <https://www.codewars.com/kata/5a87449ab1710171300000fd/train/rust>

#![no_std]

pub const fn tidy_number(mut n: u64) -> bool {
    let mut prev = 9;

    while n != 0 {
        let cur = (n % 10) as u8;

        if prev < cur {
            return false;
        }

        prev = cur;
        n /= 10;
    }

    true
}
